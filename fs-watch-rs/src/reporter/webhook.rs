use crate::errors::AppError;
use crate::event::{EventKind, WatchEvent};
use crate::reporter::Reporter;
use reqwest::Client;
use serde::Serialize;
use std::time::Duration;

#[derive(Serialize)]
struct WebhookPayload {
    event: String,
    path: String,
    timestamp: String,
    extension: String,
}

/// Репортер, отправляющий POST-запрос с JSON при каждом событии.
pub struct WebhookReporter {
    url: String,
    client: Client,
}

impl WebhookReporter {
    /// Создает новый `WebhookReporter`.
    /// Настраивает HTTP-клиента с заданным или дефолтным таймаутом.
    pub fn new(url: String, timeout_sec: Option<u64>) -> Result<Self, AppError> {
        let timeout = timeout_sec.unwrap_or(5);
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout))
            .build()
            .map_err(|e| AppError::ConfigError(format!("Ошибка создания HTTP-клиента: {}", e)))?;

        Ok(Self { url, client })
    }
}

impl Reporter for WebhookReporter {
    fn report(&self, event: &WatchEvent) -> Result<(), AppError> {
        let event_name = match event.kind {
            EventKind::Created => "Created",
            EventKind::Modified => "Modified",
            EventKind::Deleted => "Deleted",
            EventKind::Renamed => "Renamed",
        }
        .to_string();

        let payload = WebhookPayload {
            event: event_name,
            path: event.path.display().to_string(),
            timestamp: event.timestamp.to_rfc3339(),
            extension: event
                .path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_string(),
        };

        // Клонируем клиент и URL для передачи в асинхронную задачу
        let client = self.client.clone();
        let url = self.url.clone();

        // Запускаем отправку webhook в фоне, чтобы не блокировать процесс мониторинга.
        // Ошибки логируем в терминал, но они не прерывают основной цикл.
        tokio::spawn(async move {
            match client.post(&url).json(&payload).send().await {
                Ok(response) => {
                    if !response.status().is_success() {
                        eprintln!("⚠️ Webhook вернул ошибку: {} ({})", response.status(), url);
                    }
                }
                Err(e) => {
                    eprintln!("⚠️ Ошибка сети при отправке webhook: {}", e);
                }
            }
        });

        Ok(())
    }
}
