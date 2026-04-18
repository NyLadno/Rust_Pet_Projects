use std::path::PathBuf;
use thiserror::Error;

/// Единый тип ошибки для приложения `fs-watch-rs`.
#[derive(Error, Debug)]
pub enum AppError {
    /// Ошибка ввода-вывода стандартной библиотеки.
    #[error("Ошибка ввода-вывода: {0}")]
    Io(#[from] std::io::Error),

    /// Ошибка при работе с подсистемой уведомлений ОС (notify).
    #[error("Ошибка наблюдателя файловой системы: {0}")]
    Notify(#[from] notify::Error),

    /// Указанный для мониторинга путь не найден в файловой системе.
    #[error("Указанный путь не существует: {0}")]
    PathNotFound(PathBuf),

    /// Ошибка конфигурации (парсинг, неверные значения).
    #[allow(dead_code)]
    #[error("Ошибка конфигурации: {0}")]
    ConfigError(String),
}
