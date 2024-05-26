use std::fmt::Display;

use super::logger_handler::LoggerHandler;

pub trait MqttActions<Role = Self> {
    fn log_action(&self, id: &String, logger: &LoggerHandler, log_in_term: &bool)
    where
        Self: Display,
    {
        if *log_in_term {
            println!("{} - {}", id, self);
        }
        logger.log_event(&self.to_string(), id);
    }
}

pub fn add_topics_names(msg: &mut String, topic: &String, i: &mut usize) {
    if *i == 0 {
        *msg = msg.to_string() + topic;
    } else {
        *msg = msg.to_string() + " - " + topic;
    }
    *i += 1;
}
