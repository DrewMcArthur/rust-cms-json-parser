mod filtered_in_network_file;
mod in_network_file_dto;
mod node_filters;

use crate::{
    in_network_file_dto::{InNetworkFile, InNetworkRateObject},
};
fn main() {
    println!("Hello, world!");
    let _test = InNetworkFile {
        reporting_entity_name: "CMS".to_string(),
        reporting_entity_type: "FAKE".to_string(),
        version: "NEW".to_string(),
        last_updated_on: "NEVER".to_string(),
        plan_name: None,
        plan_id: None,
        plan_id_type: None,
        plan_market_type: None,
        in_network: vec![InNetworkRateObject {
            negotiation_arrangement: "CMS".to_string(),
            name: "CMS".to_string(),
            billing_code_type: "CMS".to_string(),
            billing_code_type_version: "CMS".to_string(),
            billing_code: "CMS".to_string(),
            negotiated_rates: vec![],
            description: "CMS".to_string(),
            bundled_codes: None,
            covered_services: None,
        }],
        provider_references: Some(vec![]),
    };

}
