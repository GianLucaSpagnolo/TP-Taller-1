pub mod flags_handler{

    fn apply_mask_to_n_bits(flags: u8, shifts: u8, len: u8) -> u8 {
        let mask = (1 << len) - 1;
        (flags >> shifts) & mask
    }

    /// FLAG: RESERVED
    pub fn get_connect_flag_reserved(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 0, 1)
    }

    /// FLAG: CLEAN START
    pub fn get_connect_flag_clean_start(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 1, 1)
    }

    /// FLAG: WILL FLAG
    pub fn get_connect_flag_will_flag(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 2, 1)
    }

    /// FLAG: WILL QoS
    pub fn get_connect_flag_will_qos(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 3, 2)
    }

    /// FLAG: WILL RETAIN
    pub fn get_connect_flag_will_retain(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 5, 1)
    }

    /// FLAG: PASSWORD
    pub fn get_connect_flag_password(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 6, 1)
    }

    /// FLAG: USERNAME
    pub fn get_connect_flag_username(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 7, 1)
    }

    /// FLAG: SESSION PRESENT
    /// This flag is used in the CONNACK packet to indicate if a session is already present
    pub fn _get_connect_acknowledge_flag_session_present(flags: u8) -> u8 {
        apply_mask_to_n_bits(flags, 0, 1)
    }

    pub fn create_connect_acknowledge_flags(session_present_flag: u8) -> u8 {
        let mut connect_acknowledge_flags: u8 = 0;
        connect_acknowledge_flags |= session_present_flag;
        connect_acknowledge_flags
    }

    pub fn create_connect_flags(
        reserver: u8,
        clean_start: u8,
        will_flag: u8,
        will_qos: u8,
        will_retain: u8,
        password: u8,
        username: u8,
    ) -> u8 {
        let mut connect_flags: u8 = 0;
        connect_flags |= reserver;
        connect_flags |= clean_start << 1;
        connect_flags |= will_flag << 2;
        connect_flags |= will_qos << 3;
        connect_flags |= will_retain << 5;
        connect_flags |= password << 6;
        connect_flags |= username << 7;
        connect_flags
    }

}