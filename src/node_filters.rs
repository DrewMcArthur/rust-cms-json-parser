use crate::in_network_file_dto::InNetworkRateObject;

pub struct NodeFilters {
    billing_codes: Vec<String>,
}

impl NodeFilters {
    pub fn new(billing_codes: Vec<String>) -> Self {
        NodeFilters { billing_codes }
    }

    pub fn matches(&self, o: &InNetworkRateObject) -> bool {
        self.billing_codes.len() == 0 || self.billing_codes.contains(&o.billing_code)
    }
}

#[cfg(test)]
mod tests {
    use crate::in_network_file_dto::InNetworkRateObject;

    pub struct FakeInNetworkRateObjectForTesting {
        obj: InNetworkRateObject,
    }
    impl FakeInNetworkRateObjectForTesting {
        fn new() -> Self {
            Self {
                obj: InNetworkRateObject {
                    negotiation_arrangement: "".to_string(),
                    name: "".to_string(),
                    billing_code_type: "".to_string(),
                    billing_code_type_version: "".to_string(),
                    billing_code: "1".to_string(),
                    negotiated_rates: vec![],
                    description: "".to_string(),
                    bundled_codes: None,
                    covered_services: None,
                },
            }
        }

        pub fn get() -> InNetworkRateObject {
            Self::new().obj
        }
    }

    #[test]
    fn matches_when_billing_codes_are_empty() {
        let filters = super::NodeFilters::new(vec![]);
        let o = FakeInNetworkRateObjectForTesting::get();
        assert!(filters.matches(&o));
    }

    #[test]
    fn matches_matching_billing_code() {
        let filters = super::NodeFilters::new(vec!["1".to_string()]);
        let o = FakeInNetworkRateObjectForTesting::get();
        assert!(filters.matches(&o));
    }

    #[test]
    fn no_match_on_different_billing_codes() {
        let filters = super::NodeFilters::new(vec!["3".to_string(), "2".to_string()]);
        let o = FakeInNetworkRateObjectForTesting::get();
        assert!(!filters.matches(&o));
    }
}
