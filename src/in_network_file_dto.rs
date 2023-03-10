use serde::{Deserialize, Deserializer, Serialize};
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

#[derive(Deserialize, Debug, Serialize)]
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

#[derive(Deserialize, Debug, Serialize)]
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

#[derive(Deserialize, Debug, Serialize)]
pub struct NegotiatedRate {
    pub negotiated_prices: Vec<NegotiatedPrice>,
    pub provider_groups: Option<Vec<ProviderGroup>>,
    pub provider_references: Option<Vec<Number>>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct NegotiatedPrice {
    pub negotiated_rate: Number,
    pub negotiated_type: String,
    pub expiration_date: String,
    pub service_code: Option<Vec<String>>,
    pub billing_class: BillingClass,
    pub billing_code_modifier: Option<Vec<String>>,
    pub additional_information: Option<String>,
}

#[derive(Deserialize, Debug, Serialize)]
pub enum BillingClass {
    #[serde(rename = "professional")]
    Professional,
    #[serde(rename = "institutional")]
    Institutional,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ProviderGroup {
    pub npi: Vec<Number>,
    pub tin: Tin,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Tin {
    #[serde(rename = "type")]
    pub type_: TinType,
    pub value: String,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TinType {
    #[serde(rename = "ein")]
    Ein,
    #[serde(rename = "npi")]
    Npi,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct CoveredService {
    pub billing_code: String,
    pub billing_code_type: String,
    pub billing_code_type_version: String,
    pub description: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ProviderReference {
    pub provider_group_id: Number,
    pub provider_groups: Option<Vec<ProviderGroup>>,
    pub location: Option<String>,
}

#[cfg(test)]
mod tests {
}
