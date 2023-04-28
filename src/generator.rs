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
