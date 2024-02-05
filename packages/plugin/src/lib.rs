use std::sync::{mpsc::sync_channel, Arc, OnceLock};

use async_stream::stream;
use futures_core::Stream;
use serde::{Deserialize, Serialize};
use tauri::{
    async_runtime::spawn_blocking,
    plugin::{Builder, TauriPlugin},
    AppHandle, Manager, Runtime,
};
use ustr::ustr;
use uuid::Uuid;

static PLUGIN_NAME: &str = env!("CARGO_PKG_NAME");
static INTIALIZED: OnceLock<()> = OnceLock::new();

/// Created from the Rust backend to emit events to frontend
#[derive(Clone)]
pub struct Sender(Arc<InternalSender>);

struct InternalSender {
    end_point: String,
    app_handle: AppHandle,
}

/// Created from the Rust backend to receive events from frontend
pub struct Receiver {
    end_point: String,
    app_handle: AppHandle,
}

/// Struct used from the frontend to communicate with the backend
#[derive(Serialize)]
pub struct Channel {
    end_point: String,
}

#[derive(Clone, Debug, Serialize)]
enum Data<T> {
    Unlisten,
    Message(T),
}

impl Drop for InternalSender {
    fn drop(&mut self) {
        let _ = self
            .app_handle
            .emit_all(&self.end_point, Data::<()>::Unlisten);
    }
}

impl InternalSender {
    /// Emit event to frontend
    async fn emit<S: Serialize + Clone + Send + 'static>(&self, payload: S) {
        let send_event = self.end_point.clone();
        let app_handle = self.app_handle.clone();

        let _ = spawn_blocking(move || {
            let data = Data::Message(payload);
            let _ = app_handle.emit_all(&send_event, data);
        })
        .await;
    }
}
impl Sender {
    pub async fn emit<S: Serialize + Clone + Send + 'static>(&self, payload: S) {
        self.0.emit(payload).await;
    }
}
impl Receiver {
    /// Listen to an event pubblished by frontend only once.
    pub async fn once<D: Deserialize<'static> + Send + 'static>(&self) -> Option<D> {
        let (sender, receiver) = sync_channel(1);
        let end_point = self.end_point.clone();

        self.app_handle.once_global(end_point, move |event| {
            let _ = sender.send(event.payload().map(ustr));
        });

        spawn_blocking(move || {
            receiver
                .recv()
                .ok()
                .flatten()
                .and_then(|val| serde_json::from_str(val.as_str()).ok())
        })
        .await
        .ok()
        .flatten()
    }

    pub async fn listen<D: Deserialize<'static> + Send + 'static>(&self) -> impl Stream<Item = D> {
        let (sender, receiver) = sync_channel(10);
        let end_point = self.end_point.clone();

        self.app_handle.listen_global(end_point, move |event| {
            let _ = sender.send(event.payload().map(ustr));
        });
        stream! {
            while let Ok(Some(val)) = receiver.recv() {
                if let Ok(decoded) = serde_json::from_str(val.as_str()) {
                    yield decoded;
                }
            }
        }
    }
}

/// Initializes the plugin. Must be callend in [tauri::Builder::plugin]
// Actually this method does nothing but may be required in the future
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new(PLUGIN_NAME)
        .setup(|_app| {
            assert!(INTIALIZED.get().is_none());
            assert!(INTIALIZED.set(()).is_ok());
            Ok(())
        })
        .build()
}

/// Create event channel components
pub fn channel(app_handle: AppHandle) -> (Sender, Receiver, Channel) {
    ensure_is_initialized();

    let id = Uuid::now_v7();
    let end_point = endpoint(id);

    (
        Sender(Arc::new(InternalSender {
            end_point: format!("{end_point}_fe"),
            app_handle: app_handle.clone(),
        })),
        Receiver {
            end_point: format!("{end_point}_be"),
            app_handle: app_handle.clone(),
        },
        Channel { end_point },
    )
}

fn endpoint(id: Uuid) -> String {
    format!("{PLUGIN_NAME}://{id}_mailbox")
}

fn ensure_is_initialized() {
    INTIALIZED
        .get()
        .expect("Plugin must be initilized from tauri::Builder::plugin");
}
