#[derive(Clone, Debug)]
/// Cada Topic Filter debe ser seguido por el Subscriptions Options Byte
pub struct TopicFilter {
    pub topic_filter: String,
    pub subscription_options: u8,
}
