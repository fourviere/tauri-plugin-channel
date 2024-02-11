import { InvokeArgs } from "@tauri-apps/api/tauri";
type ReceiveCallback<T> = (payload: T) => void;
export interface Sender {
    emit<T>(data: T): Promise<void>;
}
export interface Receiver {
    listen<T>(handler: ReceiveCallback<T>): Promise<void>;
    once<T>(handler: ReceiveCallback<T>): Promise<void>;
    unlisten(): void;
}
export declare function channel(cmd: string, args?: InvokeArgs): Promise<[Sender, Receiver]>;
export {};
