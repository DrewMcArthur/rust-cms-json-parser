#[derive(Debug)]
pub struct IndexFileParsingResults {
    pub index_file_id: usize,
    pub reporting_entity_name: String,
    pub reporting_entity_type: String,
    pub num_reporting_structures: i32,
    pub num_plans: i32,
    pub num_rate_files: i32,
}
