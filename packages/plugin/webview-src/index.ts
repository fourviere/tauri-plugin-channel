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
  private unlisten?: UnlistenFn;

  constructor(end_point: string) {
    super(end_point + "_fe");
  }

  async listen<T>(handler: ReceiveCallback<T>): Promise<void> {
    const wrapped_handler = (event: Event<Data<T>>) => {
      if (event.payload === "Unlisten") {
        if (this.unlisten !== undefined) this.unlisten();
        return;
      }
      handler(event.payload.Message);
    };
    this.unlisten = await listen_event<Data<T>>(
      this.end_point,
      wrapped_handler,
    );
  }

  async once<T>(handler: ReceiveCallback<T>): Promise<void> {
    const wrapped_handler = (event: Event<Data<T>>) => {
      if (event.payload !== "Unlisten") {
        handler(event.payload.Message);
        return;
      }
      if (this.unlisten !== undefined) this.unlisten();
    };
    this.unlisten = await once_event<Data<T>>(this.end_point, wrapped_handler);
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
