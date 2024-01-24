import { emit as emit_event, listen as listen_event, once as once_event, Event, UnlistenFn } from '@tauri-apps/api/event'

type ReceiveCallback<T> = (payload: T) => void;

export type Channel = {
  readonly end_point: string
  unlisten: UnlistenFn;
}

type Data<T> = { Message: T } | "Unlisten"

export async function listen<T>(receiver: Channel, handler: ReceiveCallback<T>): Promise<void> {
  let wrapped_handler = (event: Event<Data<T>>) => {
    if(event.payload === "Unlisten") {
      receiver.unlisten()
      return;
    }
    handler(event.payload.Message);
  }
  receiver.unlisten = await listen_event<Data<T>>(receiver.end_point, wrapped_handler)
}

export async function once<T>(receiver: Channel, handler: ReceiveCallback<T>): Promise<void> {
  let wrapped_handler = (event: Event<Data<T>>) => {
    if(event.payload !== "Unlisten") {
      handler(event.payload.Message);
      return;
    }
    receiver.unlisten()
  }
  receiver.unlisten = await once_event<Data<T>>(receiver.end_point, wrapped_handler)
}

export async function emit<T>(transmitter: Channel, data: T): Promise<void> {
  await emit_event(transmitter.end_point, data)
}
