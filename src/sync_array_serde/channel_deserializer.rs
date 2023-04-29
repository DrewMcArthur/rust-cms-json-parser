use std::{
    error::Error,
    fmt,
    marker::PhantomData,
    sync::mpsc::{sync_channel, SyncSender},
};

use serde::{
    de::{SeqAccess, Visitor},
    Deserialize, Deserializer,
};

use super::channel_generator::ChannelGenerator;

// an implementation of a serde deserializer that returns a channel receiver immediately,
// and in a background thread starts deserializing objects and sending them to a channel,
// up to a max amount of memory.
// the function that calls the deserializer then can receive deserialized objects from the channel and process them
pub struct ChannelVisitor<T> {
    pub sender: SyncSender<T>,
    pub f: PhantomData<fn() -> T>,
}

impl<'de, T> Visitor<'de> for ChannelVisitor<T>
where
    T: Deserialize<'de> + Send,
{
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an array of objects of type T")
    }

    fn visit_seq<S>(self, mut seq: S) -> Result<Self::Value, S::Error>
    where
        S: SeqAccess<'de>,
    {
        while let Some(n) = seq.next_element::<T>()? {
            if self.sender.send(n).is_err() {
                break;
            }
        }
        Ok(())
    }
}

pub fn deserialize_to_channel<'de, T, D, E>(deserializer: D) -> Result<ChannelGenerator<T>, E>
where
    T: Deserialize<'de> + Send,
    D: Deserializer<'de>,
    E: Error,
{
    let (node_sender, node_receiver) = sync_channel::<T>(0);
    let visitor = ChannelVisitor {
        sender: node_sender,
        f: PhantomData,
    };
    deserializer.deserialize_seq(visitor).unwrap();
    Ok(ChannelGenerator {
        receiver: node_receiver,
    })
}
