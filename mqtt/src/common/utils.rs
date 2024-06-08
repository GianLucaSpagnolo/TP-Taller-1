use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// Abre el archivo recibiendo su ruta por parametro,
// devuelve el archivo encapsulado en un option
// si pudo encontrar su ruta o, devuelve None ante un caso
// de error e imprime el error por salida de error estandar
pub fn open_config_file(file_path: &String) -> Option<File> {
    let open_res = File::open(file_path);

    let open_file = match open_res {
        Ok(file) => file,
        Err(..) => {
            eprintln!(
                "\nNo se pudo encontrar el archivo: '{}' en la ruta indicada",
                file_path
            );
            return None;
        }
    };

    Some(open_file)
}

// Dadas las lineas de un archivo de configuragcion
// y la cantidad esperada de parametros devuelve
// un vector con todos los parametros ordenados
// devuelve un vector vacio si no se obtienen todos los parametrtos
pub fn get_file_parameters(lines: Vec<String>, param_qty: usize) -> Vec<(String, String)> {
    let mut params: Vec<(String, String)> = Vec::new();

    for line in lines {
        let mut split = line.split_whitespace();
        let (mut key, mut value) = (String::new(), String::new());
        match split.next() {
            None => None,
            Some(param) => {
                let mut aux = String::from(param);
                aux.pop(); // saco el ':'
                key = aux;
                Some(())
            }
        };
        match split.last() {
            None => None,
            Some(param) => {
                value = String::from(param);
                Some(())
            }
        };

        params.push((key, value));
    }

    if params.len() < param_qty {
        eprintln!("Parametros de servidor faltantes");
        return Vec::new();
    }
    params
}

// Recibe un archivo abierto, lo lee linea por linea y
// devuelve un vector de strings,m encapsulado en un option,
// con las lineas cargadas.
// Ante un caso de error, imprime por salidar de error estandar
// y devuelve None
pub fn read_config_file(archivo: &File) -> Option<Vec<String>> {
    let lector = BufReader::new(archivo);
    let mut lines: Vec<String> = Vec::new();

    for line in lector.lines() {
        match line {
            Err(e) => {
                eprintln!("Error at reading file: {}", e);
                return None;
            }
            Ok(line) => lines.push(line),
        };
    }

    Some(lines)
}
