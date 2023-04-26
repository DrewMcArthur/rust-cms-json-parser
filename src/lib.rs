mod filtered_in_network_file;
pub mod in_network_file_dto;
pub mod index_file_parsing;
mod node_filters;

// use crate::node_filters::NodeFilters;
use crate::in_network_file_dto::InNetworkFile;

pub fn get_filtered_in_network_file(
    bytes: &[u8],
    // filters: &NodeFilters,
) -> String {
    serde_json::to_string(
        &serde_json::from_slice::<InNetworkFile>(bytes).expect("valid InNetworkFile json"),
    )
    .expect("validly deserialized InNetworkFile")
}
