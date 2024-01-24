import { UnlistenFn } from '@tauri-apps/api/event';
declare type ReceiveCallback<T> = (payload: T) => void;
export declare type Channel = {
    readonly end_point: string;
    unlisten: UnlistenFn;
};
export declare function listen<T>(receiver: Channel, handler: ReceiveCallback<T>): Promise<void>;
export declare function once<T>(receiver: Channel, handler: ReceiveCallback<T>): Promise<void>;
export declare function emit<T>(transmitter: Channel, data: T): Promise<void>;
export {};
