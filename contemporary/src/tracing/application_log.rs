use crate::tracing::application_log_entry::ApplicationLogEntry;
use async_channel::Receiver;
use gpui::{App, AsyncApp, Global};

pub struct ApplicationLog {
    log_entries: Vec<ApplicationLogEntry>,
}

impl ApplicationLog {
    pub fn new(cx: &mut App, receiver: Receiver<ApplicationLogEntry>) -> Self {
        cx.spawn(async move |cx: &mut AsyncApp| {
            loop {
                let Ok(entry) = receiver.recv().await else {
                    return;
                };
                cx.update_global::<ApplicationLog, ()>(|application_log, _| {
                    application_log.log_entries.push(entry);
                })
                .unwrap()
            }
        })
        .detach();

        Self {
            log_entries: Vec::new(),
        }
    }
}

impl Global for ApplicationLog {}
