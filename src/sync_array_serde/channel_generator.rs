use std::sync::mpsc::Receiver;

/// replicating the function of `yield` in python
/// returns a "generator" object, which is an iterator where
/// calling `next` pulls a message from the channel receiver the generator was instantiated with.

pub struct ChannelGenerator<T> {
    pub(crate) receiver: Receiver<T>,
}

impl<'f, T> Iterator for ChannelGenerator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.receiver.recv();
        res.map_or(None, |i| Some(i))
    }
}

// impl<'de, T> ChannelGenerator<T>
// where
//     T: Deserialize<'de> + Send + 'static,
// {
//     pub fn new(path: PathBuf) -> Self {
//         let (sender, receiver) = sync_channel::<T>(0);

//         thread::spawn(move || {
//             let reader = BufReader::new(File::open(path).unwrap()); //in real scenario may want to send error, instead of unwrapping
//             let mut deserializer = serde_json::Deserializer::from_reader(reader);
//             deserializer
//                 .deserialize_seq(ChannelVisitor::<T> {
//                     sender: sender.clone(),
//                     f: PhantomData,
//                 })
//                 .unwrap();
//         });

//         Self { receiver }
//     }
// }
