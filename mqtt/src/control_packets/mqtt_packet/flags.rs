pub mod flags_handler {
    use std::io::Error;

    fn apply_mask_to_n_bits(flags: u8, shifts: u8, len: u8) -> u8 {
        let mask = (1 << len) - 1;
        (flags >> shifts) & mask
    }

    /// CONNECT FLAG: RESERVED
    #[allow(dead_code)]
    pub fn get_connect_flag_reserved(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 0, 1)
    }

    /// CONNECT FLAG: CLEAN START
    #[allow(dead_code)]
    pub fn get_connect_flag_clean_start(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 1, 1)
    }

    /// CONNECT FLAG: WILL FLAG
    #[allow(dead_code)]
    pub fn get_connect_flag_will_flag(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 2, 1)
    }

    /// CONNECT FLAG: WILL QoS
    #[allow(dead_code)]
    pub fn get_connect_flag_will_qos(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 3, 2)
    }

    /// CONNECT FLAG: WILL RETAIN
    #[allow(dead_code)]
    pub fn get_connect_flag_will_retain(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 5, 1)
    }

    /// CONNECT FLAG: PASSWORD
    #[allow(dead_code)]
    pub fn get_connect_flag_password(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 6, 1)
    }

    /// CONNECT FLAG: USERNAME
    #[allow(dead_code)]
    pub fn get_connect_flag_username(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 7, 1)
    }

    /// CONNECT FLAG: SESSION PRESENT
    /// This flag is used in the CONNACK packet to indicate if a session is already present
    #[allow(dead_code)]
    pub fn get_connect_acknowledge_flag_session_present(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 0, 1)
    }

    /// PUBLISH FLAG: DUP
    #[allow(dead_code)]
    pub fn get_publish_dup_flag(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 3, 1)
    }

    /// PUBLISH FLAG: QoS
    #[allow(dead_code)]
    pub fn get_publish_qos_level(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 1, 2)
    }

    /// PUBLISH FLAG: RETAIN
    #[allow(dead_code)]
    pub fn get_publish_retain(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 0, 1)
    }

    /// SUBSCRIBE FLAG: MAX QoS
    #[allow(dead_code)]
    pub fn get_subscribe_max_qos(subscription_options: u8) -> u8 {
        apply_mask_to_n_bits(subscription_options, 0, 2)
    }

    /// SUBSCRIBE FLAG: NO LOCAL Option
    #[allow(dead_code)]
    pub fn get_subscribe_no_local_option(subscription_options: u8) -> u8 {
        apply_mask_to_n_bits(subscription_options, 2, 1)
    }

    /// SUBSCRIBE FLAG: RETAIN AS PUBLISHED
    #[allow(dead_code)]
    pub fn get_subscribe_retain_as_published(subscription_options: u8) -> u8 {
        apply_mask_to_n_bits(subscription_options, 3, 1)
    }

    /// SUBSCRIBE FLAG: RETAIN HANDLING
    #[allow(dead_code)]
    pub fn get_subscribe_retain_handling(subscription_options: u8) -> u8 {
        apply_mask_to_n_bits(subscription_options, 4, 2)
    }

    pub fn add_connect_flag_clean_start(
        mut connect_flags: u8,
        clean_start: String,
    ) -> Result<u8, Error> {
        let clean_start = match catch_true_false(&clean_start) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };
        connect_flags |= clean_start << 1;
        Ok(connect_flags)
    }

    pub fn add_connect_flag_will_flag(
        mut connect_flags: u8,
        will_flag: String,
    ) -> Result<u8, Error> {
        let will_flag = match catch_true_false(&will_flag) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };
        connect_flags |= will_flag << 2;
        Ok(connect_flags)
    }

    pub fn add_connect_flag_will_qos(mut connect_flags: u8, will_qos: String) -> Result<u8, Error> {
        let will_qos = match will_qos.parse::<u8>() {
            Ok(p) => p,
            Err(e) => return Err(Error::new(std::io::ErrorKind::InvalidData, e.to_string())),
        };
        connect_flags |= will_qos << 3;
        Ok(connect_flags)
    }

    pub fn add_connect_flag_will_retain(
        mut connect_flags: u8,
        will_retain: String,
    ) -> Result<u8, Error> {
        let will_retain = match catch_true_false(&will_retain) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };
        connect_flags |= will_retain << 5;
        Ok(connect_flags)
    }

    pub fn add_connect_flag_password(mut connect_flags: u8, password: String) -> Result<u8, Error> {
        let password = match catch_true_false(&password) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };
        connect_flags |= password << 6;
        Ok(connect_flags)
    }

    pub fn add_connect_flag_username(mut connect_flags: u8, username: String) -> Result<u8, Error> {
        let username = match catch_true_false(&username) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };
        connect_flags |= username << 7;
        Ok(connect_flags)
    }

    pub fn catch_true_false(flag: &String) -> Result<u8, Error> {
        if flag == "true" {
            return Ok(1);
        }
        if flag == "false" {
            return Ok(0);
        }
        Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid parameter value for flag",
        ))
    }

}
