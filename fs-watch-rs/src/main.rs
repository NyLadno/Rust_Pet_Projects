use fs_watch_rs::cli::Cli;
use fs_watch_rs::config::{AppSettings, FileConfig};
use fs_watch_rs::errors::AppError;
use fs_watch_rs::event::EventFilter;
use fs_watch_rs::reporter::Reporter;
use fs_watch_rs::reporter::console::ConsoleReporter;
use fs_watch_rs::reporter::file::FileReporter;
use fs_watch_rs::reporter::webhook::WebhookReporter;
use fs_watch_rs::watcher;
use std::path::PathBuf;
use std::process;

/// Точка входа в приложение fs-watch-rs.
#[tokio::main]
async fn main() {
    let args = Cli::parse_args();

    if let Err(e) = run_app(args) {
        eprintln!("Ошибка: {}", e);
        process::exit(1);
    }
}

/// Основная логика работы приложения, инкапсулирующая возврат `Result`.
fn run_app(args: Cli) -> Result<(), AppError> {
    // 1. Загружаем конфигурацию (если указан ключ --config или если есть config.toml в текущей папке)
    let file_config = if let Some(ref config_path) = args.config {
        Some(FileConfig::load(config_path)?)
    } else {
        let default_config = PathBuf::from("config.toml");
        if default_config.exists() {
            Some(FileConfig::load(default_config)?)
        } else {
            None
        }
    };

    // 2. Сливаем настройки CLI и конфига (CLI в приоритете)
    let settings = AppSettings::merge(args, file_config);

    // 3. Инициализируем фильтр событий на основе итоговых настроек
    let filter = EventFilter::new(settings.filters, settings.pattern.as_deref())?;

    // 4. Инициализируем репортеры
    let mut reporters: Vec<Box<dyn Reporter>> = Vec::new();

    // Консольный репортер используется всегда
    reporters.push(Box::new(ConsoleReporter::new(settings.no_color)));

    // Если указан файл для логов, добавляем файловый репортер
    if let Some(output_path) = settings.output_file {
        let file_reporter = FileReporter::new(output_path)?;
        reporters.push(Box::new(file_reporter));
    }

    // Если указан URL для webhook, добавляем вебхук репортер
    if let Some(webhook_url) = settings.webhook_url {
        let webhook_reporter = WebhookReporter::new(webhook_url, settings.webhook_timeout_sec)?;
        reporters.push(Box::new(webhook_reporter));
    }

    // 5. Запускаем наблюдателя
    watcher::run_watcher(
        &settings.watch_path,
        settings.recursive,
        settings.debounce_ms,
        filter,
        &reporters,
    )?;

    Ok(())
}
