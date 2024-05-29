use super::incident_list::IncidentList;

#[derive(Default)]
pub struct IncidentInterface {
    pub historial: IncidentList,
    pub latitude_field: String,
    pub longitude_field: String,
    pub wrong_data: bool,
    pub show_data_alert: bool,
    pub editable: bool,
}
