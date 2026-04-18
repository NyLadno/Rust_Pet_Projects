use crate::cli::Cli;
use crate::errors::AppError;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

/// Структура, отражающая формат файла `config.toml`.
#[derive(Debug, Deserialize, Default)]
pub struct FileConfig {
    pub watch_path: Option<PathBuf>,
    pub recursive: Option<bool>,
    pub debounce_ms: Option<u64>,
    pub filters: Option<Vec<String>>,
    pub pattern: Option<String>,
    pub output: Option<OutputConfig>,
    pub webhook: Option<WebhookConfig>,
}

#[derive(Debug, Deserialize)]
pub struct OutputConfig {
    pub file: PathBuf,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct WebhookConfig {
    pub url: String,
    pub timeout_sec: Option<u64>,
}

impl FileConfig {
    /// Загружает конфигурацию из указанного файла TOML.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, AppError> {
        let content = fs::read_to_string(&path).map_err(|e| {
            AppError::ConfigError(format!(
                "Не удалось прочитать файл конфигурации {}: {}",
                path.as_ref().display(),
                e
            ))
        })?;

        let config: FileConfig = toml::from_str(&content)
            .map_err(|e| AppError::ConfigError(format!("Ошибка парсинга TOML: {}", e)))?;

        Ok(config)
    }
}

/// Итоговые настройки приложения после слияния CLI и файла конфигурации.
#[derive(Debug)]
pub struct AppSettings {
    pub watch_path: PathBuf,
    pub recursive: bool,
    pub filters: Vec<String>,
    pub pattern: Option<String>,
    pub output_file: Option<PathBuf>,
    #[allow(dead_code)]
    pub webhook_url: Option<String>,
    pub webhook_timeout_sec: Option<u64>,
    #[allow(dead_code)]
    pub debounce_ms: u64,
    pub no_color: bool,
}

impl AppSettings {
    /// Выполняет слияние аргументов командной строки и файла конфигурации.
    /// Аргументы CLI всегда имеют наивысший приоритет.
    pub fn merge(cli: Cli, file_config: Option<FileConfig>) -> Self {
        let config = file_config.unwrap_or_default();

        // 1. Путь к наблюдаемой директории
        let watch_path = cli
            .path
            .or(config.watch_path)
            .unwrap_or_else(|| PathBuf::from("."));

        // 2. Рекурсивный обход (истина, если включено где-либо)
        let recursive = cli.recursive || config.recursive.unwrap_or(false);

        // 3. Фильтры расширений
        let filters = if let Some(cli_filter) = cli.filter {
            cli_filter
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        } else {
            config.filters.unwrap_or_default()
        };

        // 4. Glob-паттерн
        let pattern = cli.pattern.or(config.pattern);

        // 5. Выходной лог-файл
        let output_file = cli.output.or_else(|| config.output.map(|o| o.file));

        // 6. Webhook
        let webhook_url = cli
            .webhook
            .or_else(|| config.webhook.as_ref().map(|w| w.url.clone()));
        let webhook_timeout_sec = config.webhook.and_then(|w| w.timeout_sec);

        // 7. Дебаунсинг (по умолчанию 200 мс)
        let debounce_ms = cli.debounce.or(config.debounce_ms).unwrap_or(200);

        Self {
            watch_path,
            recursive,
            filters,
            pattern,
            output_file,
            webhook_url,
            webhook_timeout_sec,
            debounce_ms,
            no_color: cli.no_color,
        }
    }
}
