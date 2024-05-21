use std::fmt::Debug;

pub trait MqttActions<Role = Self>
where
    Self: Debug,
{
    fn register_action(self) -> Self;

    fn log_action(self) -> Self;
}
