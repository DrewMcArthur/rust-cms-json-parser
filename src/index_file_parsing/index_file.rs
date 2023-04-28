use serde::Deserialize;

use crate::{channel_deserializer::deserialize_to_channel, generator::ChannelGenerator};

#[derive(Deserialize)]
pub struct IndexFile {
    pub reporting_entity_name: String,
    pub reporting_entity_type: String,
    #[serde(deserialize_with = "deserialize_to_channel")]
    pub reporting_structure: ChannelGenerator<ReportingStructure>,
}

#[derive(Deserialize, Debug)]
pub struct ReportingStructure {
    pub reporting_plans: Vec<ReportingPlan>,
    pub in_network_files: Vec<LinkedFile>,
    pub allowed_amount_file: LinkedFile,
}

#[derive(Deserialize, Debug)]
pub struct ReportingPlan {
    pub plan_name: String,
    pub plan_id_type: String, // TODO: enum
    pub plan_id: String,
    pub plan_market_type: String, // TODO: enum
}

#[derive(Deserialize, Debug)]
pub struct LinkedFile {
    pub description: String,
    pub location: String, // URL
}
