// this file creates a method of deserializing large arrays
// that, instead of loading the entirety into memory,
// will enqueue each item onto a channel, and return the receiver.
// then, the client code can spin up threads consume messages from the channel.

use std::sync::mpsc::Sender;

use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer,
};

struct ChannelVisitor<T> {
    pub sender: Sender<T>,
}

impl<'de, T> Visitor<'de> for ChannelVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = ();

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("any type that implements Deserialize")
    }

    // This method is called by the Deserializer with the deserialized value
    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize the value using the generic type parameter
        let value = T::deserialize(deserializer)?;

        // Send the value to the channel
        self.sender.send(value).map_err(Error::custom)?;

        // Return an empty tuple as the output of the visitor
        Ok(())
    }
}
