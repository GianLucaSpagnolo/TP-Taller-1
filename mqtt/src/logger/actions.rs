use super::logger_handler::LoggerHandler;

#[allow(dead_code)]
pub trait MqttActions<Role = Self> {
    fn register_action(self) -> Self;

    fn log_action(&self, logger: &LoggerHandler);
}
