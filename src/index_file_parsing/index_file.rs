use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct IndexFileMetadata {
    pub reporting_entity_name: String,
    pub reporting_entity_type: String,
}

#[derive(Deserialize, Debug)]
pub struct IndexFile {
    pub reporting_entity_name: String,
    pub reporting_entity_type: String,
    pub reporting_structure: Vec<ReportingStructure>,
}

#[derive(Deserialize, Debug)]
pub struct AsyncIndexFile {
    pub reporting_entity_name: String,
    pub reporting_entity_type: String,
    pub reporting_structure_processing_stats: ProcessingStats,
}

#[derive(Deserialize, Debug)]
pub struct ProcessingStats {
    pub num_reporting_structures: usize,
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
