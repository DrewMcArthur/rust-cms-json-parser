use std::{marker::PhantomData, fmt};

use serde::{Deserializer, de::{Visitor, SeqAccess}};

use crate::{node_filters::NodeFilters, in_network_file_dto::InNetworkRateObject};

pub fn filter_nodes<'de, D>(deserializer: D, filter: NodeFilters) -> Result<Vec<InNetworkRateObject>, D::Error>
where
    D: Deserializer<'de>,
{
    struct FilteredRateObjectVisitor { 
        data: PhantomData<fn() -> InNetworkRateObject>, 
        filter: NodeFilters 
    }

    impl<'de> Visitor<'de> for FilteredRateObjectVisitor
    {
        // return value of visitor.  will return a vector of 
        // only the RateObjects matching the given NodeFilter.
        type Value = Vec<InNetworkRateObject>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an in network rate object")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Vec<InNetworkRateObject>, S::Error>
        where
            S: SeqAccess<'de>,
        {
            // Start with an empty Vec
            let mut filtered_nodes = vec!();

            // only keep nodes that match our filter
            while let Some(value) = seq.next_element()? {
                if self.filter.matches(&value) {
                    filtered_nodes.push(value);
                }
            }

            Ok(filtered_nodes)
        }


    }

    // Create the visitor and ask the deserializer to drive it. The
    // deserializer will call visitor.visit_seq() if a seq is present in
    // the input data.
    let visitor = FilteredRateObjectVisitor { data: PhantomData, filter };
    deserializer.deserialize_seq(visitor)
}
