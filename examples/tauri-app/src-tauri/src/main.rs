// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;

use serde::{Deserialize, Serialize};
use tauri::{async_runtime::spawn, AppHandle};
use tauri_plugin_channel::{channel, Channel, Receiver, Sender};
use tokio::{pin, select, time::sleep};

#[derive(Clone, Serialize)]
enum BackendEvents {
    Progress(u8),
    Done,
    Stopped,
}

#[derive(Debug, Deserialize)]
enum FrontendEvents {
    Stop,
}

#[derive(Debug, Serialize, Deserialize)]
enum BackEndTrigger {
    ProgressDone(Duration),
    Interrupted(Duration),
}

#[tauri::command]
fn fast_progress(app_handle: AppHandle) -> Channel {
    let (sender, receiver, channel) = channel(app_handle);
    progress(sender, receiver, Duration::from_millis(100));
    channel
}

#[tauri::command]
fn slow_progress(app_handle: AppHandle) -> Channel {
    let (sender, receiver, channel) = channel(app_handle);
    progress(sender, receiver, Duration::from_secs(1));
    channel
}

fn progress(sender: Sender, receiver: Receiver, duration: Duration) {
    spawn(async move {
        let receive_task = receiver.once::<FrontendEvents>();
        pin!(receive_task);
        for i in 0..101 {
            select! {
                _ = sleep(duration) => {
                    sender.emit(BackendEvents::Progress(i)).await;
                    if i == 100 {
                        sender.emit(BackendEvents::Done).await;
                    }
                }
                received = &mut receive_task => {
                    if let Some(FrontendEvents::Stop) = received {
                        sender.emit(BackendEvents::Stopped).await;
                        break;
                    }
                }
            }
        }
    });
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_channel::init())
        .invoke_handler(tauri::generate_handler![fast_progress, slow_progress])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
