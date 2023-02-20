use serde::{Deserialize, Deserializer};
use serde_json::Number;

use crate::filtered_in_network_file::filter_nodes;
use crate::node_filters::NodeFilters;

fn filter_nodes_by_billing_codes<'de, D>(
    deserializer: D,
) -> Result<Vec<InNetworkRateObject>, D::Error>
where
    D: Deserializer<'de>,
{
    let billing_codes = vec!["945".to_string()];
    let filters = NodeFilters::new(billing_codes);
    filter_nodes(deserializer, filters)
}

#[derive(Deserialize, Debug)]
pub struct InNetworkFile {
    pub reporting_entity_name: String,
    pub reporting_entity_type: String,
    pub version: String,
    pub last_updated_on: String,

    pub plan_name: Option<String>,
    pub plan_id: Option<String>,
    pub plan_id_type: Option<String>,
    pub plan_market_type: Option<String>,

    #[serde(deserialize_with = "filter_nodes_by_billing_codes")]
    pub in_network: Vec<InNetworkRateObject>,
    pub provider_references: Option<Vec<ProviderReference>>,
}

#[derive(Deserialize, Debug)]
pub struct InNetworkRateObject {
    pub negotiation_arrangement: String,
    pub name: String,
    pub billing_code_type: String,
    pub billing_code_type_version: String,
    pub billing_code: String,
    pub negotiated_rates: Vec<NegotiatedRate>,
    pub description: String,
    pub bundled_codes: Option<Vec<CoveredService>>,
    pub covered_services: Option<Vec<CoveredService>>,
}

#[derive(Deserialize, Debug)]
pub struct NegotiatedRate {
    pub negotiated_prices: Vec<NegotiatedPrice>,
    pub provider_groups: Option<Vec<ProviderGroup>>,
    pub provider_references: Option<Vec<Number>>,
}

#[derive(Deserialize, Debug)]
pub struct NegotiatedPrice {
    pub negotiated_rate: Number,
    pub negotiated_type: String,
    pub expiration_date: String,
    pub service_code: Option<Vec<String>>,
    pub billing_class: BillingClass,
    pub billing_code_modifier: Option<Vec<String>>,
    pub additional_information: Option<String>,
}

#[derive(Deserialize, Debug)]
pub enum BillingClass {
    #[serde(rename = "professional")]
    Professional,
    #[serde(rename = "institutional")]
    Institutional,
}

#[derive(Deserialize, Debug)]
pub struct ProviderGroup {
    pub npi: Vec<Number>,
    pub tin: Tin,
}

#[derive(Deserialize, Debug)]
pub struct Tin {
    #[serde(rename = "type")]
    pub type_: TinType,
    pub value: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TinType {
    #[serde(rename = "ein")]
    Ein,
    #[serde(rename = "npi")]
    Npi,
}

#[derive(Deserialize, Debug)]
pub struct CoveredService {
    pub billing_code: String,
    pub billing_code_type: String,
    pub billing_code_type_version: String,
    pub description: String,
}

#[derive(Deserialize, Debug)]
pub struct ProviderReference {
    pub provider_group_id: Number,
    pub provider_groups: Option<Vec<ProviderGroup>>,
    pub location: Option<String>,
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self, read},
        path::{Path, PathBuf},
    };

    use super::*;

    #[test]
    fn test_deserialization() {
        let examples_dir = Path::new("price-transparency-guide")
            .join("examples")
            .join("in-network-rates");

        let paths = fs::read_dir(&examples_dir).expect("examples dir to exist");

        for path in paths {
            let path = path.expect("an existing path").path();
            if file_name_is_json(&path) {
                println!("testing with filename {}", path.to_string_lossy());
                let file_bytes = read(path).expect("bytes from files");
                let file_obj: InNetworkFile =
                    serde_json::from_slice(file_bytes.as_slice()).unwrap();
                assert!(
                    file_obj.reporting_entity_name == "cms".to_string()
                        || file_obj.reporting_entity_name == "medicare".to_string()
                );
            }
        }
    }

    fn file_name_is_json(path: &PathBuf) -> bool {
        match path.extension() {
            Some(ext) => ext.eq("json"),
            None => false,
        }
    }
}
