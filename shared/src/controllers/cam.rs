pub mod cam_system_controllers {
    use std::io::Error;

    use logger::logger_handler::Logger;
    use mqtt::client::mqtt_client::MqttClient;
    use walkers::Position;

    use crate::models::cam_model::cam_list::CamList;


    pub fn send_cam(
        client: &mut MqttClient,
        cam_list: &CamList,
        id: &u8,
        logger: &Logger,
        db_path: &str,
    ) -> Result<(), Error> {
        let cam = cam_list.get_cam(id).unwrap().clone();
        client
            .publish(cam.as_bytes().clone(), "cam".to_string(), logger)?;
        cam_list.save(db_path)
    }


    pub fn delete_cam(
        client: &mut MqttClient,
        cam_list: &mut CamList,
        id: &u8,
        logger: &Logger,
        db_path: &str,
    ) -> Result<(), Error> {
        cam_list.delete_cam(id);
        send_cam(client, cam_list, id, logger, db_path)
    }

    pub fn add_cam(
        client: &mut MqttClient,
        cam_list: &mut CamList,
        pos: Position,
        logger: &Logger,
        db_path: &str,
    ) -> Result<(), Error> {
        let id = cam_list.add_cam(pos);
        send_cam(client, cam_list, &id, logger, db_path)
    }

    pub fn edit_cam(
        client: &mut MqttClient,
        cam_list: &mut CamList,
        id: &u8,
        pos: Position,
        logger: &Logger,
        db_path: &str,
    ) -> Result<(), Error> {
        cam_list.edit_cam(id, pos);
        send_cam(client, cam_list, id, logger, db_path)
    }
}