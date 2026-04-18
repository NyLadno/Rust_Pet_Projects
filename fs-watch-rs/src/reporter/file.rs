use crate::errors::AppError;
use crate::event::{EventKind, WatchEvent};
use crate::reporter::Reporter;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::Mutex;

/// Репортер, записывающий события файловой системы в лог-файл.
pub struct FileReporter {
    writer: Mutex<BufWriter<File>>,
}

impl FileReporter {
    /// Создает новый `FileReporter`, открывая файл для дозаписи (или создавая новый).
    pub fn new<P: AsRef<Path>>(file_path: P) -> Result<Self, AppError> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)?;

        Ok(Self {
            writer: Mutex::new(BufWriter::new(file)),
        })
    }
}

impl Reporter for FileReporter {
    fn report(&self, event: &WatchEvent) -> Result<(), AppError> {
        let timestamp_str = event.timestamp.format("%Y-%m-%d %H:%M:%S").to_string();

        let (icon, kind_str) = match event.kind {
            EventKind::Created => ("✚", "CREATED"),
            EventKind::Modified => ("✎", "MODIFIED"),
            EventKind::Deleted => ("✖", "DELETED"),
            EventKind::Renamed => ("⇄", "RENAMED"),
        };

        let log_line = format!(
            "[{}] {} {:<9} {}\n",
            timestamp_str,
            icon,
            kind_str,
            event.path.display()
        );

        // Используем Mutex для потокобезопасной записи в файл из разных тредов (или async)
        let mut writer = self.writer.lock().map_err(|_| {
            AppError::Io(std::io::Error::other(
                "Ошибка блокировки мьютекса при записи в файл",
            ))
        })?;

        writer.write_all(log_line.as_bytes())?;

        Ok(())
    }

    fn flush(&self) -> Result<(), AppError> {
        let mut writer = self.writer.lock().map_err(|_| {
            AppError::Io(std::io::Error::other(
                "Ошибка блокировки мьютекса при сбросе буфера",
            ))
        })?;
        writer.flush()?;
        Ok(())
    }
}
