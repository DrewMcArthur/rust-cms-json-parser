use crate::in_network_file_dto::InNetworkRateObject;

pub struct NodeFilters {
    billing_codes: Vec<String>,
}

impl NodeFilters {
  pub fn new(billing_codes: Vec<String>) -> Self {
    NodeFilters {
      billing_codes
    }
  }

  pub fn matches(&self, o: &InNetworkRateObject) -> bool{
    self.billing_codes.contains(&o.billing_code)
  }
}
