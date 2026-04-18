pub mod console;
pub mod file;
pub mod webhook;

use crate::errors::AppError;
use crate::event::WatchEvent;

/// Трейт, определяющий интерфейс для получателей уведомлений о событиях файловой системы.
pub trait Reporter: Send + Sync {
    /// Обрабатывает и доставляет (или выводит) событие.
    fn report(&self, event: &WatchEvent) -> Result<(), AppError>;

    /// Сбрасывает внутренние буферы. По умолчанию не делает ничего.
    #[allow(dead_code)]
    fn flush(&self) -> Result<(), AppError> {
        Ok(())
    }
}
