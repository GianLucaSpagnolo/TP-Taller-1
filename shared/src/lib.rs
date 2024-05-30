pub mod models {
    pub mod cam_model {
        pub mod cam;
        pub mod cam_list;
    }
    pub mod coordenates;
    pub mod inc_model {
        pub mod incident;
        pub mod incident_list;
    }
}

pub mod interfaces {
    pub mod incident_interface;
}
pub mod views {
    pub mod cams_views {
        pub mod cams;
        pub mod cams_list;
    }
    pub mod incs_views {
        pub mod incidents;
        pub mod incidents_editor;
        pub mod incidents_list;
    }
    pub mod dialog_alert;
}

pub mod controllers {
    pub mod incident;
}
