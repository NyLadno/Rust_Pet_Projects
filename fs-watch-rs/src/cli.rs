use clap::Parser;
use std::path::PathBuf;

/// Структура, описывающая доступные аргументы командной строки приложения.
#[derive(Parser, Debug, Clone)]
#[command(name = "fs-watch-rs")]
#[command(version, about = "CLI-инструмент мониторинга файловой системы", long_about = None)]
pub struct Cli {
    /// Путь к наблюдаемой директории
    #[arg(value_name = "PATH")]
    pub path: Option<PathBuf>,

    /// Наблюдать рекурсивно
    #[arg(short, long)]
    pub recursive: bool,

    /// Фильтр по расширению (например, .rs, .toml)
    #[arg(short, long, value_name = "EXT")]
    pub filter: Option<String>,

    /// Фильтр по glob-паттерну (например, *.log, src_*)
    #[arg(short, long, value_name = "GLOB")]
    pub pattern: Option<String>,

    /// Путь к файлу лога
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Путь к файлу конфигурации (config.toml)
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// URL для POST-уведомлений (webhook)
    #[arg(long, value_name = "URL")]
    pub webhook: Option<String>,

    /// Дебаунс событий в миллисекундах
    #[arg(long, value_name = "MS")]
    pub debounce: Option<u64>,

    /// Отключить цветной вывод в терминале
    #[arg(long)]
    pub no_color: bool,
}

impl Cli {
    /// Инициализирует и парсит аргументы командной строки.
    pub fn parse_args() -> Self {
        Cli::parse()
    }
}
