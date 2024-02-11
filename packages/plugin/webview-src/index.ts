import { invoke } from "@tauri-apps/api";
import {
  emit as emit_event,
  listen as listen_event,
  once as once_event,
  Event,
  UnlistenFn,
} from "@tauri-apps/api/event";
import { InvokeArgs } from "@tauri-apps/api/tauri";

type ReceiveCallback<T> = (payload: T) => void;

type Data<T> = { Message: T } | "Unlisten";

type Channel = {
  readonly end_point: string;
};

export interface Sender {
  emit<T>(data: T): Promise<void>;
}

export interface Receiver {
  listen<T>(handler: ReceiveCallback<T>): Promise<void>;
  once<T>(handler: ReceiveCallback<T>): Promise<void>;
  unlisten(): void;
}

abstract class EndPoint {
  readonly end_point: string;

  constructor(end_point: string) {
    this.end_point = end_point;
  }
}

class InternalSender extends EndPoint implements Sender {
  constructor(end_point: string) {
    super(end_point + "_be");
  }

  async emit<T>(data: T): Promise<void> {
    await emit_event(this.end_point, data);
  }
}

class InternalReceiver extends EndPoint implements Receiver {
  private unlisten_fn?: UnlistenFn;
  private active: boolean = true;

  constructor(end_point: string) {
    super(end_point + "_fe");
  }

  private receiver_checks(): boolean {
    if (!this.active) {
      console.error("Rust Senders have been dropped");
      return false;
    }

    if (this.unlisten_fn !== undefined) {
      console.error("Another listen|once function is still running");
      return false;
    }

    return true;
  }

  unlisten(): void {
    if (this.unlisten_fn === undefined) return;
    this.unlisten_fn();
    delete this.unlisten_fn;
  }

  async listen<T>(handler: ReceiveCallback<T>): Promise<void> {
    if (!this.receiver_checks()) return;

    const wrapped_handler = (event: Event<Data<T>>) => {
      if (event.payload === "Unlisten") {
        this.active = false;
        this.unlisten();
        return;
      }
      handler(event.payload.Message);
    };

    this.unlisten_fn = await listen_event<Data<T>>(
      this.end_point,
      wrapped_handler,
    );
  }

  async once<T>(handler: ReceiveCallback<T>): Promise<void> {
    if (!this.receiver_checks()) return;

    const wrapped_handler = (event: Event<Data<T>>) => {
      if (event.payload !== "Unlisten") {
        handler(event.payload.Message);
      } else {
        this.active = false;
      }

      this.unlisten();
    };

    this.unlisten_fn = await once_event<Data<T>>(
      this.end_point,
      wrapped_handler,
    );
  }
}

export async function channel(
  cmd: string,
  args?: InvokeArgs,
): Promise<[Sender, Receiver]> {
  const channel: Channel = await invoke(cmd, args);
  const sender = new InternalSender(channel.end_point);
  const receiver = new InternalReceiver(channel.end_point);

  return [sender, receiver];
}
