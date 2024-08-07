pub enum AppTopics {
    CamTopic,
    DroneTopic,
    IncTopic,
}

impl AppTopics {
    pub fn get_topic(&self) -> String {
        match self {
            AppTopics::CamTopic => "camaras".to_string(),
            AppTopics::DroneTopic => "drone".to_string(),
            AppTopics::IncTopic => "inc".to_string(),
        }
    }
}
