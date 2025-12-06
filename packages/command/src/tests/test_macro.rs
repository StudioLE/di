#![allow(dead_code)]
use crate::prelude::*;

define_commands! {
    Delay(DelayRequest),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_new() {
        // Arrange
        let request = DelayRequest::default();
        let handler = Arc::new(DelayHandler);
        let handler = CommandHandler::Delay(handler);

        // Act
        let command = Command::new(request, handler);

        // Assert
        assert!(matches!(command, Command::Delay(_, _)));
    }

    #[test]
    fn command_display() {
        // Arrange
        let request = DelayRequest::default();
        let handler = Arc::new(DelayHandler);
        let command = Command::Delay(request, handler);

        // Act
        let result = command.to_string();

        // Assert
        assert_eq!(result, "Delay 50 ms");
    }

    #[tokio::test]
    async fn command_execute() {
        init_elapsed_logger();

        // Arrange
        let request = DelayRequest::default();
        let handler = Arc::new(DelayHandler);
        let command = Command::Delay(request, handler);

        // Act
        let response = command.execute().await;

        // Assert
        assert!(matches!(response, CommandResult::Delay(_, Ok(()))));
    }
}
