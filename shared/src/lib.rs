pub mod models {
    pub mod cam_model {
        pub mod cam;
        pub mod cam_list;
    }

    pub mod drone_model {
        pub mod drone;
        pub mod drone_list;
    }
    pub mod inc_model {
        pub mod incident;
        pub mod incident_list;
    }
}

pub mod interfaces {
    pub mod cam_interface;
    pub mod drone_interface;
    pub mod global_interface;
    pub mod incident_interface;
    pub mod map_interface;
}

pub mod views {
    pub mod app_views {
        pub mod cams_views;
        pub mod drone_views;
        pub mod inc_views;
    }

    pub mod map_views {
        pub mod map;
        pub mod plugins;
        pub mod windows;
    }

    pub mod dialog_alert;
    pub mod icon;
}

pub mod controllers {
    pub mod incident;
}

pub mod utils;
pub mod will_message;
