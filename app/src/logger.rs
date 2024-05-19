use crate::common::file_manager::{open_file, read_file, write_line};
/// El logger guarda en alto nivel las acciones de todas las aplicaciones,
/// que pasan por el servidor.
/// Cuando el servidor recibe una accion de su protocolo, llama al logger
/// para asentarla.
///
/// El logger entonces:
///      * encola la accion
///          * la parsea a traves del protocolo
///          * le agrega un timestamp y la pasa al file manager para
///            persistirla
///
/// En un principio solo hay un archivo de log, en donde se guardaran los campos:
///      * timestamp
///      * client_id
///      * accion parseada
///
/// El log define el archivo, y su formato. (en un principio .csv)
use chrono;
use chrono::prelude::*;
use std::{
    fs::File,
    io::Error,
    sync::mpsc::{channel, Receiver, Sender},
    thread::{self, JoinHandle},
};

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
pub struct LoggerHandler {
    write_pipe: Sender<String>,
    log_file_path: String,
    threads: Vec<JoinHandle<Result<(), std::io::Error>>>,
}

impl LoggerHandler {
    pub fn create_logger_handler(w_pipe: Sender<String>, route: &String) -> LoggerHandler {
        LoggerHandler {
            write_pipe: w_pipe,
            log_file_path: String::from(route),
            threads: vec![],
        }
    }

    // must be called once
    pub fn initiate_listener(&mut self, reader: Receiver<String>) -> Result<(), Error> {
        let path = String::from(&self.log_file_path);
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
                Ok(())
            }
            Err(..) => Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "Thread handler recv error",
            )),
        }
    }

    // parsea el mensaje en el formato definido.
    // separator = ',' --> .csv
    pub fn log_event(&self, msg: &String, client_id: &String, separator: &String) {
        let message = if !msg.contains('\n') {
            msg.to_string() + "\n"
        } else {
            msg.to_string()
        };

        let logger_msg: String = separator.to_string() + &client_id + &separator + &message;
        let _ = &self.enqueue_message(&logger_msg);
    }

    fn enqueue_message(&self, msg: &String) -> Result<(), Error> {
        match &self.write_pipe.send(msg.to_string()) {
            Ok(..) => Ok(()),
            Err(e) => Err(Error::new(std::io::ErrorKind::InvalidData, e.to_string())),
        }
    }

    // must be called once
    pub fn close_logger(self) {
        drop(self.write_pipe);
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

#[cfg(test)]
mod test {
    use super::LoggerHandler;
    use crate::common::file_manager::{open_file, read_file};
    use std::{fs::remove_file, sync::mpsc::channel};

    #[test]
    fn the_logger_can_log_2_events() {
        let log_file_path = String::from("log1.tmp");
        let header = "Time,Client_ID,Action".to_string();
        let str1 = "Initiate logger ...".to_string();
        let str2 = "Closing logger ...".to_string();

        let mut writed_lines = vec![];
        writed_lines.push(header.to_string());
        writed_lines.push(str1.to_string());
        writed_lines.push(str2.to_string());

        let (tw, tr) = channel();
        let mut logger_handler = LoggerHandler::create_logger_handler(tw, &log_file_path);

        let _ = match logger_handler.initiate_listener(tr) {
            Err(..) => {
                println!("Logger fails to initiate");
                assert!(false)
            }
            Ok(..) => (),
        };

        logger_handler.log_event(&str1, &0.to_string(), &",".to_string());
        logger_handler.log_event(&str2, &0.to_string(), &",".to_string());
        logger_handler.close_logger();

        // testing
        let mut file = match open_file(&log_file_path) {
            Ok(f) => f,
            Err(e) => {
                println!("Error al abrir archivo de lectura: {}\n", &e.to_string());
                let _ = remove_file(&log_file_path);
                return assert!(false);
            }
        };

        let readed_lines = read_file(&mut file).unwrap();

        for line in readed_lines {
            if line.contains(&header) || line.contains(&str1) || line.contains(&str2) {
                continue;
            };
            println!("{}", line);
            let _ = remove_file(&log_file_path);
            assert!(false);
        }
        let _ = remove_file(&log_file_path);
        assert!(true)
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

        let _ = match logger_handler.initiate_listener(tr) {
            Err(..) => {
                println!("Logger fails to initiate");
                assert!(false)
            }
            Ok(..) => (),
        };

        logger_handler.log_event(&str1, &0.to_string(), &",".to_string());
        line_counter += 1;
        logger_handler.log_event(&str2, &0.to_string(), &",".to_string());
        line_counter += 1;
        logger_handler.close_logger();

        // testing
        let mut file = match open_file(&log_file_path) {
            Ok(f) => f,
            Err(e) => {
                println!("Error al abrir archivo de lectura: {}\n", &e.to_string());
                return assert!(false);
            }
        };

        let mut readed_lines = read_file(&mut file).unwrap();

        for line in &readed_lines {
            if line.contains(&header) || line.contains(&str1) || line.contains(&str2) {
                continue;
            };

            println!("Unknow line: [{}]", line);
            let _ = remove_file(&log_file_path);
            assert!(false);
        }

        // reseting logger:
        let (twc, trc) = channel();
        logger_handler = LoggerHandler::create_logger_handler(twc, &log_file_path);
        writed_lines.push(str3.to_string());

        let _ = match logger_handler.initiate_listener(trc) {
            Err(..) => {
                println!("Logger fails to initiate");
                assert!(false)
            }
            Ok(..) => (),
        };

        logger_handler.log_event(&str1, &0.to_string(), &",".to_string());
        line_counter += 1;
        logger_handler.log_event(&str3, &0.to_string(), &",".to_string());
        line_counter += 1;
        logger_handler.log_event(&str2, &0.to_string(), &",".to_string());
        line_counter += 1;
        logger_handler.close_logger();

        file = match open_file(&log_file_path) {
            Ok(f) => f,
            Err(e) => {
                println!("Error al abrir archivo de lectura: {}\n", &e.to_string());
                let _ = remove_file(&log_file_path);
                return assert!(false);
            }
        };

        readed_lines = read_file(&mut file).unwrap();

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
            assert!(false);
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

        let mut writed_lines = vec![];
        writed_lines.push(header.to_string());
        writed_lines.push(str1.to_string());
        writed_lines.push(str2.to_string());

        let (tw, tr) = channel();
        let mut logger_handler = LoggerHandler::create_logger_handler(tw, &log_file_path);

        let _ = match logger_handler.initiate_listener(tr) {
            Err(..) => {
                println!("Logger 1 fails to initiate");
                assert!(false)
            }
            Ok(..) => (),
        };

        let (tw2, tr2) = channel();
        let mut logger_handler2 = LoggerHandler::create_logger_handler(tw2, &log_file_path);

        let _ = match logger_handler2.initiate_listener(tr2) {
            Err(..) => {
                println!("Logger 2 fails to initiate");
                assert!(false)
            }
            Ok(..) => (),
        };

        logger_handler.log_event(&str1, &0.to_string(), &",".to_string());
        line_counter += 1;
        logger_handler2.log_event(&str2, &0.to_string(), &",".to_string());
        line_counter += 1;
        logger_handler.close_logger();
        logger_handler2.close_logger();

        // testing
        let mut file = match open_file(&log_file_path) {
            Ok(f) => f,
            Err(e) => {
                println!("Error al abrir archivo de lectura: {}\n", &e.to_string());
                return assert!(false);
            }
        };

        let readed_lines = read_file(&mut file).unwrap();

        for line in &readed_lines {
            if line.contains(&header) || line.contains(&str1) || line.contains(&str2) {
                continue;
            };

            println!("Unknow line: [{}]", line);
            let _ = remove_file(&log_file_path);
            assert!(false);
        }

        // deleting the file:
        let _ = remove_file(&log_file_path);
        // plus 1 for the unique header
        assert_eq!(line_counter + 1, readed_lines.len());
    }
}
