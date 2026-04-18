use chrono::{DateTime, Local};
use glob::Pattern;
use notify::Event as NotifyEvent;
use notify::EventKind as NotifyEventKind;
use std::path::PathBuf;

/// Тип произошедшего события в файловой системе.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventKind {
    /// Файл или директория созданы.
    Created,
    /// Файл или директория изменены.
    Modified,
    /// Файл или директория удалены.
    Deleted,
    /// Файл или директория переименованы.
    Renamed,
}

/// Событие файловой системы, готовое для обработки и вывода репортерами.
#[derive(Debug, Clone)]
pub struct WatchEvent {
    /// Тип события.
    pub kind: EventKind,
    /// Полный путь к файлу, с которым произошло событие.
    pub path: PathBuf,
    /// Локальное время фиксации события.
    pub timestamp: DateTime<Local>,
}

impl WatchEvent {
    /// Конвертирует сырое событие `notify::Event` в вектор `WatchEvent`.
    ///
    /// Так как одно событие от ОС может содержать несколько путей,
    /// функция возвращает вектор структур `WatchEvent`. Неподдерживаемые
    /// или информационные события игнорируются (возвращается пустой вектор).
    pub fn from_notify_event(event: NotifyEvent) -> Vec<Self> {
        let kind = match event.kind {
            NotifyEventKind::Create(_) => Some(EventKind::Created),
            NotifyEventKind::Modify(notify::event::ModifyKind::Name(_)) => Some(EventKind::Renamed),
            NotifyEventKind::Modify(_) => Some(EventKind::Modified),
            NotifyEventKind::Remove(_) => Some(EventKind::Deleted),
            _ => None,
        };

        if let Some(event_kind) = kind {
            let timestamp = Local::now();
            event
                .paths
                .into_iter()
                .map(|path| WatchEvent {
                    kind: event_kind.clone(),
                    path,
                    timestamp,
                })
                .collect()
        } else {
            // Игнорируем события, которые нас не интересуют (например, Access)
            Vec::new()
        }
    }
}

/// Структура для фильтрации событий файловой системы.
#[derive(Debug, Clone)]
pub struct EventFilter {
    /// Список расширений для фильтрации (например, ["rs", "toml"]). Если пуст, фильтр не применяется.
    pub extensions: Vec<String>,
    /// Опциональный glob-паттерн для фильтрации по имени файла (например, "*.log").
    pub pattern: Option<Pattern>,
}

impl EventFilter {
    /// Создает новый фильтр из готового списка расширений и строкового представления паттерна.
    pub fn new(
        extensions: Vec<String>,
        glob_str: Option<&str>,
    ) -> Result<Self, crate::errors::AppError> {
        let pattern = match glob_str {
            Some(s) => Some(Pattern::new(s).map_err(|e| {
                crate::errors::AppError::ConfigError(format!("Неверный glob-паттерн: {}", e))
            })?),
            None => None,
        };

        Ok(Self {
            extensions,
            pattern,
        })
    }

    /// Проверяет, должно ли событие пройти фильтр (true = проходит, false = отбрасывается).
    pub fn matches(&self, event: &WatchEvent) -> bool {
        // Проверка расширения (если заданы)
        if !self.extensions.is_empty() {
            let path_ext = event
                .path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");
            let ext_matches = self.extensions.iter().any(|ext| {
                let ext = ext.trim_start_matches('.');
                ext == path_ext
            });

            if !ext_matches {
                return false;
            }
        }

        // Проверка паттерна (если задан)
        if let Some(ref pat) = self.pattern {
            // Применяем паттерн к имени файла
            let file_name = event
                .path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");
            if !pat.matches(file_name) {
                return false;
            }
        }

        true
    }
}
