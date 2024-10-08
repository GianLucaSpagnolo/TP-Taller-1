pub mod flags_handler {
    use std::io::Error;

    /// ## apply_mask_to_n_bits
    ///
    /// Aplica una mascara a un byte y devuelve los bits
    ///
    fn apply_mask_to_n_bits(flags: u8, shifts: u8, len: u8) -> u8 {
        let mask = (1 << len) - 1;
        (flags >> shifts) & mask
    }

    /// CONNECT FLAG: RESERVED
    pub fn get_connect_flag_reserved(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 0, 1)
    }

    /// CONNECT FLAG: CLEAN START
    pub fn get_connect_flag_clean_start(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 1, 1)
    }

    /// CONNECT FLAG: WILL FLAG
    pub fn get_connect_flag_will_flag(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 2, 1)
    }

    /// CONNECT FLAG: WILL QoS
    pub fn get_connect_flag_will_qos(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 3, 2)
    }

    /// CONNECT FLAG: WILL RETAIN
    pub fn get_connect_flag_will_retain(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 5, 1)
    }

    /// CONNECT FLAG: PASSWORD
    pub fn get_connect_flag_password(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 6, 1)
    }

    /// CONNECT FLAG: USERNAME
    pub fn get_connect_flag_username(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 7, 1)
    }

    /// CONNECT FLAG: SESSION PRESENT
    /// This flag is used in the CONNACK packet to indicate if a session is already present
    pub fn get_connect_acknowledge_flag_session_present(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 0, 1)
    }

    /// PUBLISH FLAG: DUP
    pub fn get_publish_dup_flag(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 3, 1)
    }

    /// PUBLISH FLAG: QoS
    pub fn get_publish_qos_level(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 1, 2)
    }

    /// PUBLISH FLAG: RETAIN
    pub fn get_publish_retain(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 0, 1)
    }

    /// SUBSCRIBE FLAG: MAX QoS
    pub fn get_subscribe_max_qos(subscription_options: u8) -> u8 {
        apply_mask_to_n_bits(subscription_options, 0, 2)
    }

    /// SUBSCRIBE FLAG: NO LOCAL Option
    pub fn get_subscribe_no_local_option(subscription_options: u8) -> u8 {
        apply_mask_to_n_bits(subscription_options, 2, 1)
    }

    /// SUBSCRIBE FLAG: RETAIN AS PUBLISHED
    pub fn get_subscribe_retain_as_published(subscription_options: u8) -> u8 {
        apply_mask_to_n_bits(subscription_options, 3, 1)
    }

    /// SUBSCRIBE FLAG: RETAIN HANDLING
    pub fn get_subscribe_retain_handling(subscription_options: u8) -> u8 {
        apply_mask_to_n_bits(subscription_options, 4, 2)
    }

    /// ## add_connect_flag_clean_start
    ///
    /// Agrega un flag de tipo CLEAN START al byte de flags
    ///
    /// ### Parametros
    /// - `connect_flags`: byte de flags
    /// - `clean_start`: valor del flag CLEAN START (string booleano)
    ///
    /// ### Retorno
    /// - `Result<u8, Error>`:
    ///   - Ok: byte de flags actualizado
    ///   - Err: error al agregar el flag (std::io::Error)
    ///
    pub fn add_connect_flag_clean_start(
        mut connect_flags: u8,
        clean_start: String,
    ) -> Result<u8, Error> {
        let clean_start = match clean_start.parse::<bool>() {
            Ok(p) => {
                if p {
                    1
                } else {
                    0
                }
            }
            Err(e) => return Err(Error::new(std::io::ErrorKind::InvalidData, e.to_string())),
        };
        connect_flags |= clean_start << 1;
        Ok(connect_flags)
    }

    /// ## add_connect_flag_will_flag
    ///
    /// Agrega un flag de tipo WILL FLAG al byte de flags
    ///
    /// ### Parametros
    /// - `connect_flags`: byte de flags
    /// - `will_flag`: valor del flag WILL FLAG (string booleano)
    ///
    /// ### Retorno
    /// - `Result<u8, Error>`:
    ///    - Ok: byte de flags actualizado
    ///    - Err: error al agregar el flag (std::io::Error)
    ///
    pub fn add_connect_flag_will_flag(
        mut connect_flags: u8,
        will_flag: String,
    ) -> Result<u8, Error> {
        let will_flag = match will_flag.parse::<bool>() {
            Ok(p) => {
                if p {
                    1
                } else {
                    0
                }
            }
            Err(e) => return Err(Error::new(std::io::ErrorKind::InvalidData, e.to_string())),
        };
        connect_flags |= will_flag << 2;
        Ok(connect_flags)
    }

    /// ## add_connect_flag_will_qos
    ///
    /// Agrega un flag de tipo WILL QoS al byte de flags
    ///
    /// ### Parametros
    /// - `connect_flags`: byte de flags
    /// - `will_qos`: valor del flag WILL QoS (u8)
    ///
    /// ### Retorno
    /// - `Result<u8, Error>`:
    ///     - Ok: byte de flags actualizado
    ///     - Err: error al agregar el flag (std::io::Error)
    ///
    pub fn add_connect_flag_will_qos(mut connect_flags: u8, will_qos: String) -> Result<u8, Error> {
        let will_qos = match will_qos.parse::<u8>() {
            Ok(p) => p,
            Err(e) => return Err(Error::new(std::io::ErrorKind::InvalidData, e.to_string())),
        };
        connect_flags |= will_qos << 3;
        Ok(connect_flags)
    }

    /// ## add_connect_flag_will_retain
    ///
    /// Agrega un flag de tipo WILL RETAIN al byte de flags
    ///
    /// ### Parametros
    /// - `connect_flags`: byte de flags
    /// - `will_retain`: valor del flag WILL RETAIN (string booleano)
    ///
    /// ### Retorno
    /// - `Result<u8, Error>`:
    ///   - Ok: byte de flags actualizado
    ///   - Err: error al agregar el flag (std::io::Error)
    ///
    pub fn add_connect_flag_will_retain(
        mut connect_flags: u8,
        will_retain: String,
    ) -> Result<u8, Error> {
        let will_retain = match will_retain.parse::<bool>() {
            Ok(p) => {
                if p {
                    1
                } else {
                    0
                }
            }
            Err(e) => return Err(Error::new(std::io::ErrorKind::InvalidData, e.to_string())),
        };
        connect_flags |= will_retain << 5;
        Ok(connect_flags)
    }

    /// ## add_connect_flag_password
    ///
    /// Agrega un flag de tipo PASSWORD al byte de flags
    ///
    /// ### Parametros
    /// - `connect_flags`: byte de flags
    /// - `password`: valor del flag PASSWORD (string booleano)
    ///
    /// ### Retorno
    /// - `Result<u8, Error>`:
    ///     - Ok: byte de flags actualizado
    ///     - Err: error al agregar el flag (std::io::Error)
    ///
    pub fn add_connect_flag_password(mut connect_flags: u8, password: String) -> Result<u8, Error> {
        let password = match password.parse::<bool>() {
            Ok(p) => {
                if p {
                    1
                } else {
                    0
                }
            }
            Err(e) => return Err(Error::new(std::io::ErrorKind::InvalidData, e.to_string())),
        };
        connect_flags |= password << 6;
        Ok(connect_flags)
    }

    /// ## add_connect_flag_username
    ///
    /// Agrega un flag de tipo USERNAME al byte de flags
    ///
    /// ### Parametros
    /// - `connect_flags`: byte de flags
    /// - `username`: valor del flag USERNAME (string booleano)
    ///
    /// ### Retorno
    /// - `Result<u8, Error>`:
    ///   - Ok: byte de flags actualizado
    ///   - Err: error al agregar el flag (std::io::Error)
    ///
    pub fn add_connect_flag_username(mut connect_flags: u8, username: String) -> Result<u8, Error> {
        let username = match username.parse::<bool>() {
            Ok(p) => {
                if p {
                    1
                } else {
                    0
                }
            }
            Err(e) => return Err(Error::new(std::io::ErrorKind::InvalidData, e.to_string())),
        };
        connect_flags |= username << 7;
        Ok(connect_flags)
    }
}
