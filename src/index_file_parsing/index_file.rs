use serde::Deserialize;

#[derive(Deserialize)]
pub struct IndexFile {
    pub reporting_entity_name: String,
    pub reporting_entity_type: String,
    pub reporting_structure: Vec<ReportingStructure>,
}

#[derive(Deserialize)]
pub struct ReportingStructure {
    pub reporting_plans: Vec<ReportingPlan>,
    pub in_network_files: Vec<LinkedFile>,
    pub allowed_amount_file: LinkedFile,
}

#[derive(Deserialize)]
pub struct ReportingPlan {
    pub plan_name: String,
    pub plan_id_type: String, // TODO: enum
    pub plan_id: String,
    pub plan_market_type: String, // TODO: enum
}

#[derive(Deserialize)]
pub struct LinkedFile {
    pub description: String,
    pub location: String, // URL
}
