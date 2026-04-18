use crate::errors::AppError;
use crate::event::{EventFilter, WatchEvent};
use crate::reporter::Reporter;
use colored::Colorize;
use notify::{RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::Duration;

/// Запускает бесконечный цикл наблюдения за указанной директорией.
///
/// # Аргументы
/// * `path` - Путь к директории для наблюдения.
/// * `recursive` - Флаг рекурсивного наблюдения.
/// * `debounce_ms` - Время ожидания в миллисекундах перед группировкой событий.
/// * `filter` - Структура фильтра событий.
/// * `reporters` - Список репортеров для доставки уведомлений.
pub fn run_watcher(
    path: &Path,
    recursive: bool,
    debounce_ms: u64,
    filter: EventFilter,
    reporters: &[Box<dyn Reporter>],
) -> Result<(), AppError> {
    if !path.exists() {
        return Err(AppError::PathNotFound(path.to_path_buf()));
    }

    // Создаем канал для получения событий от подсистемы notify
    let (tx, rx) = mpsc::channel();

    // Создаем платформозависимого наблюдателя
    let mut watcher = notify::recommended_watcher(tx)?;

    let mode = if recursive {
        RecursiveMode::Recursive
    } else {
        RecursiveMode::NonRecursive
    };

    // Даем команду наблюдателю следить за путем
    watcher.watch(path, mode)?;

    println!(
        "{} {}",
        "👀 Наблюдение запущено за директорией:".cyan().bold(),
        path.display()
    );

    let debounce_duration = Duration::from_millis(debounce_ms);
    let mut pending_events: HashMap<PathBuf, WatchEvent> = HashMap::new();

    // Блокирующий цикл обработки событий с поддержкой дебаунсинга
    loop {
        match rx.recv_timeout(debounce_duration) {
            Ok(Ok(notify_event)) => {
                let events = WatchEvent::from_notify_event(notify_event);

                // Фильтруем и помещаем в очередь pending_events для дебаунса
                for event in events {
                    if filter.matches(&event) {
                        // Перезаписываем старое событие для данного файла (дебаунсинг)
                        pending_events.insert(event.path.clone(), event);
                    }
                }
            }
            Ok(Err(e)) => {
                // Ошибки при получении события не должны "ронять" приложение
                eprintln!("{} {:?}", "⚠️ Ошибка наблюдения:".red(), e);
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                // Таймаут истек - если есть накопленные события, отправляем их репортерам
                if !pending_events.is_empty() {
                    for (_, event) in pending_events.drain() {
                        for reporter in reporters {
                            if let Err(e) = reporter.report(&event) {
                                eprintln!("{} {:?}", "⚠️ Ошибка репортера:".red(), e);
                            }
                        }
                    }
                }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                // Канал закрыт (наблюдатель остановлен)
                break;
            }
        }
    }

    Ok(())
}
