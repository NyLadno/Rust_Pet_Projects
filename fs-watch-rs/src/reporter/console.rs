use crate::errors::AppError;
use crate::event::{EventKind, WatchEvent};
use crate::reporter::Reporter;
use colored::Colorize;

/// Репортер, выводящий события файловой системы в стандартный вывод (консоль).
pub struct ConsoleReporter {
    no_color: bool,
}

impl ConsoleReporter {
    /// Создает новый экземпляр `ConsoleReporter`.
    pub fn new(no_color: bool) -> Self {
        Self { no_color }
    }
}

impl Reporter for ConsoleReporter {
    fn report(&self, event: &WatchEvent) -> Result<(), AppError> {
        let timestamp_str = event.timestamp.format("%Y-%m-%d %H:%M:%S").to_string();

        let (icon, kind_str) = match event.kind {
            EventKind::Created => ("✚", "CREATED"),
            EventKind::Modified => ("✎", "MODIFIED"),
            EventKind::Deleted => ("✖", "DELETED"),
            EventKind::Renamed => ("⇄", "RENAMED"),
        };

        if self.no_color {
            println!(
                "[{}] {} {:<9} {}",
                timestamp_str,
                icon,
                kind_str,
                event.path.display()
            );
        } else {
            let colored_kind = match event.kind {
                EventKind::Created => kind_str.green(),
                EventKind::Modified => kind_str.yellow(),
                EventKind::Deleted => kind_str.red(),
                EventKind::Renamed => kind_str.blue(),
            };

            println!(
                "[{}] {} {:<9} {}",
                timestamp_str.dimmed(),
                icon,
                colored_kind,
                event.path.display()
            );
        }

        Ok(())
    }
}
