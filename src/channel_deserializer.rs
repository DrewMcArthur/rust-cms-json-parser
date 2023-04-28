use std::{fmt, marker::PhantomData, sync::mpsc::channel};

use serde::{
    de::{SeqAccess, Visitor},
    Deserialize, Deserializer,
};

use crate::generator::ChannelGenerator;

// an implementation of a serde deserializer that returns a channel receiver immediately,
// and in a background thread starts deserializing objects and sending them to a channel,
// up to a max amount of memory.
// the function that calls the deserializer then can receive deserialized objects from the channel and process them
pub fn deserialize_to_channel<'de, T, D>(deserializer: D) -> Result<ChannelGenerator<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    struct ChannelVisitor<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for ChannelVisitor<T>
    where
        T: Deserialize<'de>,
    {
        type Value = ChannelGenerator<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an array of objects of type T")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<ChannelGenerator<T>, S::Error>
        where
            S: SeqAccess<'de>,
        {
            let (sender, receiver) = channel::<T>();

            let mut i = 0;
            // todo: this should happen in the background
            while let Some(value) = seq.next_element()? {
                println!("sending item {i}");
                i += 1;
                sender.send(value).unwrap();
            }

            Ok(ChannelGenerator { receiver })
        }
    }

    let visitor = ChannelVisitor(PhantomData);
    deserializer.deserialize_seq(visitor)
}
