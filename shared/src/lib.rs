pub mod model {
    pub mod cam;
    pub mod cam_list;
    pub mod coordenates;
    pub mod incident;
    pub mod incident_interface;
    pub mod incident_list;
}

pub mod views {
    pub mod cams {
        pub mod cams;
        pub mod cams_list;
    }
    pub mod incidents {
        pub mod incidents;
        pub mod incidents_editor;
        pub mod incidents_list;
    }
    pub mod dialog_alert;
}

pub mod controllers {
    pub mod incident;
}
