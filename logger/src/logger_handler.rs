
/// El logger guarda en alto nivel las acciones de todas las aplicaciones,
/// que pasan por el servidor.
///
/// El logger define el archivo, y su formato. (en un principio .csv)
use chrono;
use chrono::prelude::*;
use std::{
    fs::File,
    io::Error,
    sync::mpsc::{channel, Receiver, Sender},
    thread::{self, JoinHandle},
};

use crate::file_manager::{open_file, read_file, write_line};

// file manager ------------------------------------------------
fn file_was_created(file: &File) -> bool {
    match read_file(file) {
        Some(lineas) => !lineas.is_empty(),
        None => false,
    }
}

fn open_log_file(route: &String) -> Result<File, Error> {
    let header = "Time,Client_ID,Action\n".to_string();
    match open_file(route) {
        Ok(mut file) => {
            let mut fields = header;

            if file_was_created(&file) {
                return Ok(file);
            };

            match write_line(&mut fields, &mut file) {
                Ok(..) => Ok(file),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

// Logger ----------------------------------------------------------
/// Crea un logger handler y devuelve un handler para manejarlo
pub fn create_logger_handler(log_file_path: &String) -> Result<LoggerHandler, Error> {
    let (tw, tr) = channel();
    let mut logger_handler = LoggerHandler::create_logger_handler(tw, log_file_path);

    match logger_handler.initiate_listener(tr) {
        Err(e) => Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "Logger fails to initiate by error: ".to_string() + &e.to_string(),
        )),
        Ok(..) => Ok(logger_handler),
    }
}

#[derive(Clone)]
pub struct Logger {
    write_pipe: Sender<String>,
    log_file_path: String,
}

impl Logger {
    pub fn create_logger(w_pipe: Sender<String>, route: &String) -> Logger {
        Logger {
            write_pipe: w_pipe,
            log_file_path: String::from(route),
        }
    }

    pub fn get_path(&self) -> String{
        self.log_file_path.to_string()
    }

    // parsea el mensaje en el formato definido.
    // separator = ',' --> .csv
    pub fn log_event(&self, msg: &String, client_id: &String) {
        let separator = ",";

        let message = if !msg.contains('\n') {
            msg.to_string() + "\n"
        } else {
            msg.to_string()
        };

        let logger_msg: String = separator.to_string() + client_id + separator + &message;
        let _ = &self.enqueue_message(&logger_msg);
    }

    fn enqueue_message(&self, msg: &String) -> Result<(), Error> {
        match &self.write_pipe.send(msg.to_string()) {
            Ok(..) => Ok(()),
            Err(e) => Err(Error::new(std::io::ErrorKind::InvalidData, e.to_string())),
        }
    }

    // must be called once
    pub fn close(self) {
        drop(self.write_pipe);
    }
}

pub struct LoggerHandler {
    logger: Logger,
    threads: Vec<JoinHandle<Result<(), std::io::Error>>>,
}

impl LoggerHandler {
    pub fn create_logger_handler(w_pipe: Sender<String>, route: &String) -> LoggerHandler {
        LoggerHandler {
            logger: Logger::create_logger(w_pipe.clone(), route),
            threads: vec![],
        }
    }

    // must be called once
    pub fn initiate_listener(&mut self, reader: Receiver<String>) -> Result<Logger, Error> {
        let path = self.logger.get_path();
        let (tw, tr) = channel();

        self.threads
            .push(thread::spawn(move || log_actions(&path, reader, &tw)));

        match tr.recv() {
            Ok(r) => {
                if !r.contains("Ok") {
                    return Err(Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Error at open log file",
                    ));
                };
                Ok(self.logger.clone())
            }
            Err(..) => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Thread handler recv error",
            )),
        }
    }

    // ver de sacar
    pub fn log_event(&self, msg: &String, client_id: &String) {
        self.logger.log_event(msg, client_id)
    }

    // must be called once
    // para poder cerrar el thread, se deben cerrar
    // todas las referencias al receiver, es decir,
    // todos los loggers clonados.
    pub fn close_logger(self) {
        self.logger.close();
        for thread in self.threads {
            let _ = thread.join();
        }
    }
}

// se recibe el evento parseado, es decir, ya viene traducido
pub fn log_actions(
    log_file_route: &String,
    read_pipe: Receiver<String>,
    write_pipe: &Sender<String>,
) -> Result<(), Error> {
    let mut log_file: File = match open_log_file(log_file_route) {
        Ok(file) => {
            let _ = write_pipe.send(String::from("Ok"));
            file
        }
        Err(e) => {
            let _ = write_pipe.send(e.to_string());
            return Err(e);
        }
    };

    while let Ok(received) = read_pipe.recv() {
        let _ = log_action(&mut received.to_string(), &mut log_file);
    }
    Ok(())
}

// Logging -------------------------------------------------
fn get_actual_timestamp() -> String {
    let dt = Local::now();
    let naive_utc = dt.naive_utc();
    let offset = dt.offset();
    let dt_new = DateTime::<Local>::from_naive_utc_and_offset(naive_utc, *offset);
    dt_new.format("%Y-%m-%d %H:%M:%S:%3f").to_string()
}

fn log_action(action: &mut String, file: &mut File) -> Result<(), Error> {
    let mut line = get_actual_timestamp() + action;
    write_line(&mut line, file)
}

// Testing -------------------------------------------------
#[cfg(test)]
mod test {
    use crate::file_manager::{open_file, read_file};

    use super::LoggerHandler;
    use core::panic;
    use std::{fs::remove_file, sync::mpsc::channel};

    #[test]
    fn the_logger_can_log_2_events() {
        let log_file_path = String::from("log1.tmp");
        let header = "Time,Client_ID,Action".to_string();
        let str1 = "Initiate logger ...".to_string();
        let str2 = "Closing logger ...".to_string();

        let (tw, tr) = channel();
        let mut logger_handler = LoggerHandler::create_logger_handler(tw, &log_file_path);

        if logger_handler.initiate_listener(tr).is_err() {
            println!("Logger fails to initiate");
            panic!()
        };

        logger_handler.log_event(&str1, &0.to_string());
        logger_handler.log_event(&str2, &0.to_string());
        logger_handler.close_logger();

        // testing
        let file = match open_file(&log_file_path) {
            Ok(f) => f,
            Err(e) => {
                println!("Error al abrir archivo de lectura: {}\n", &e.to_string());
                let _ = remove_file(&log_file_path);
                panic!()
            }
        };

        let readed_lines = read_file(&file).unwrap();

        for line in readed_lines {
            if line.contains(&header) || line.contains(&str1) || line.contains(&str2) {
                continue;
            };
            println!("{}", line);
            let _ = remove_file(&log_file_path);
            panic!()
        }
        let _ = remove_file(&log_file_path);
        //assert!(true)
    }

    #[test]
    fn the_logger_can_append_events_at_crash_and_not_re_write_the_fields() {
        let log_file_path = String::from("log2.tmp");
        let header = "Time,Client_ID,Action".to_string();
        let str1 = "Initiating logger ...".to_string();
        let str2 = "Closing logger ...".to_string();
        let str3 = "Appened event".to_string();
        let mut line_counter = 0;

        let mut writed_lines = vec![];
        writed_lines.push(header.to_string());
        writed_lines.push(str1.to_string());
        writed_lines.push(str2.to_string());

        let (tw, tr) = channel();
        let mut logger_handler = LoggerHandler::create_logger_handler(tw, &log_file_path);

        if logger_handler.initiate_listener(tr).is_err() {
            println!("Logger fails to initiate");
            panic!()
        };

        logger_handler.log_event(&str1, &0.to_string());
        line_counter += 1;
        logger_handler.log_event(&str2, &0.to_string());
        line_counter += 1;
        logger_handler.close_logger();

        // testing
        let mut file = match open_file(&log_file_path) {
            Ok(f) => f,
            Err(e) => {
                println!("Error al abrir archivo de lectura: {}\n", &e.to_string());
                panic!()
            }
        };

        let mut readed_lines = read_file(&file).unwrap();

        for line in &readed_lines {
            if line.contains(&header) || line.contains(&str1) || line.contains(&str2) {
                continue;
            };

            println!("Unknow line: [{}]", line);
            let _ = remove_file(&log_file_path);
            panic!()
        }

        // reseting logger:
        let (twc, trc) = channel();
        logger_handler = LoggerHandler::create_logger_handler(twc, &log_file_path);
        writed_lines.push(str3.to_string());

        if logger_handler.initiate_listener(trc).is_err() {
            println!("Logger fails to initiate");
            panic!()
        };

        logger_handler.log_event(&str1, &0.to_string());
        line_counter += 1;
        logger_handler.log_event(&str3, &0.to_string());
        line_counter += 1;
        logger_handler.log_event(&str2, &0.to_string());
        line_counter += 1;
        logger_handler.close_logger();

        file = match open_file(&log_file_path) {
            Ok(f) => f,
            Err(e) => {
                println!("Error al abrir archivo de lectura: {}\n", &e.to_string());
                let _ = remove_file(&log_file_path);
                panic!()
            }
        };

        readed_lines = read_file(&file).unwrap();

        for line in &readed_lines {
            if line.contains(&header)
                || line.contains(&str1)
                || line.contains(&str2)
                || line.contains(&str3)
            {
                continue;
            };

            println!("Unknow line: [{}]", line);
            let _ = remove_file(&log_file_path);
            //assert!(false);
            panic!()
        }

        // deleting the file:
        let _ = remove_file(&log_file_path);
        // plus 1 for the unique header
        assert_eq!(line_counter + 1, readed_lines.len());
    }

    #[test]
    fn one_file_colud_be_handled_by_2_loggers() {
        let log_file_path = String::from("log3.tmp");
        let header = "Time,Client_ID,Action".to_string();
        let str1 = "Initiating logger ...".to_string();
        let str2 = "Closing logger ...".to_string();
        //let str3 = "Appened event".to_string();
        let mut line_counter = 0;

        let (tw, tr) = channel();
        let mut logger_handler = LoggerHandler::create_logger_handler(tw, &log_file_path);

        if logger_handler.initiate_listener(tr).is_err() {
            println!("Logger 1 fails to initiate");
            panic!()
        };

        let (tw2, tr2) = channel();
        let mut logger_handler2 = LoggerHandler::create_logger_handler(tw2, &log_file_path);

        if logger_handler2.initiate_listener(tr2).is_err() {
            println!("Logger 1 fails to initiate");
            panic!()
        };

        logger_handler.log_event(&str1, &0.to_string());
        line_counter += 1;
        logger_handler2.log_event(&str2, &0.to_string());
        line_counter += 1;
        logger_handler.close_logger();
        logger_handler2.close_logger();

        // testing
        let file = match open_file(&log_file_path) {
            Ok(f) => f,
            Err(e) => {
                println!("Error al abrir archivo de lectura: {}\n", &e.to_string());
                panic!()
            }
        };

        let readed_lines = read_file(&file).unwrap();

        for line in &readed_lines {
            if line.contains(&header) || line.contains(&str1) || line.contains(&str2) {
                continue;
            };

            println!("Unknow line: [{}]", line);
            let _ = remove_file(&log_file_path);
            panic!()
        }

        // deleting the file:
        let _ = remove_file(&log_file_path);
        // plus 1 for the unique header
        assert_eq!(line_counter + 1, readed_lines.len());
    }

    #[test]
    fn the_logger_handler_can_manage_the_logger_listener() {
        let log_file_path = String::from("log4.tmp");
        let header = "Time,Client_ID,Action".to_string();
        let str1 = "Initiating logger ...".to_string();
        let str2 = "Closing logger ...".to_string();
        let mut line_counter = 0;

        let (tw, tr) = channel();
        let mut logger_handler = LoggerHandler::create_logger_handler(tw, &log_file_path);

        let logger = match logger_handler.initiate_listener(tr) {
            Ok(log) => log,
            Err(e) => {
                println!("Logger 1 fails to initiate by: {}", e);
                panic!();
            }
        };

        logger.log_event(&str1, &0.to_string());
        line_counter += 1;
        logger.log_event(&str2, &0.to_string());
        line_counter += 1;
        logger.close();
        logger_handler.close_logger();

        // testing
        let file = match open_file(&log_file_path) {
            Ok(f) => f,
            Err(e) => {
                println!("Error al abrir archivo de lectura: {}\n", &e.to_string());
                panic!()
            }
        };

        let readed_lines = read_file(&file).unwrap();

        for line in &readed_lines {
            if line.contains(&header) || line.contains(&str1) || line.contains(&str2) {
                continue;
            };

            println!("Unknow line: [{}]", line);
            let _ = remove_file(&log_file_path);
            panic!()
        }

        // deleting the file:
        let _ = remove_file(&log_file_path);
        // plus 1 for the unique header
        assert_eq!(line_counter + 1, readed_lines.len());
    }

    #[test]
    fn the_logger_can_be_moved_between_threads() {
        let log_file_path = String::from("log5.tmp");
        let header = "Time,Client_ID,Action".to_string();
        let str1 = "Initiating logger ...".to_string();
        let str2 = "Closing logger ...".to_string();
        let mut line_counter = 0;

        let (tw, tr) = channel();
        let mut logger_handler = LoggerHandler::create_logger_handler(tw, &log_file_path);

        let logger = match logger_handler.initiate_listener(tr) {
            Ok(log) => log,
            Err(e) => {
                println!("Logger 1 fails to initiate by: {}", e);
                panic!();
            }
        };

        let logger_cpy = logger.clone();
        let str1_cpy = str1.to_string();
        let str2_cpy = str1.to_string();
        let mut threads = vec![];
        threads.push(std::thread::spawn(move || {
            logger_cpy.log_event(&str1_cpy, &0.to_string());
            logger_cpy.log_event(&str1_cpy, &0.to_string());
            logger_cpy.close();
        }));
        
        let logger_cpy_2 = logger.clone();
        threads.push(std::thread::spawn(move || {
            logger_cpy_2.log_event(&str2_cpy, &0.to_string());
            logger_cpy_2.log_event(&str2_cpy, &0.to_string());
            logger_cpy_2.close();
        }));

        line_counter += 1;
        line_counter += 1;
        line_counter += 1;
        line_counter += 1;

        logger.close();
        logger_handler.close_logger();

        for t in threads {
            let _ = t.join();
        }

        // testing
        let file = match open_file(&log_file_path) {
            Ok(f) => f,
            Err(e) => {
                println!("Error al abrir archivo de lectura: {}\n", &e.to_string());
                panic!()
            }
        };

        let readed_lines = read_file(&file).unwrap();

        for line in &readed_lines {
            if line.contains(&header) || line.contains(&str1) || line.contains(&str2) {
                continue;
            };

            println!("Unknow line: [{}]", line);
            let _ = remove_file(&log_file_path);
            panic!()
        }

        // deleting the file:
        let _ = remove_file(&log_file_path);
        // plus 1 for the unique header
        assert_eq!(line_counter + 1, readed_lines.len());
    }
}
