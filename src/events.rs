use crate::errors::{ClientErrorKind, ErrorLogContext};
use grapher::prelude::{EVENT_DISPATCHER, GUIEvent};
use leptos::prelude::expect_context;
use log::info;

pub struct EventHandler;

impl EventHandler {
    pub async fn handle_event() {
        loop {
            match EVENT_DISPATCHER.gui_read_chan.recv().await {
                Ok(event) => {
                    match event {
                        GUIEvent::ShowMetadata(idx) => {
                            // call relevant signal
                            info!("{idx}");
                        }
                        GUIEvent::HideMetadata() => {
                            // call relevant signal
                        }
                    }
                }
                Err(e) => {
                    let error_context = expect_context::<ErrorLogContext>();
                    error_context.push(ClientErrorKind::EventHandlingError(e.to_string()).into());
                }
            }
        }
    }
}
