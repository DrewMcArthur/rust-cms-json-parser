/// creating an instance of this class will create 
/// an object that repeatedly fetches messages from 
/// an sqs queue, and passes the message to a callback function.
/// uses exponential backoff
/// 
/// TODO:
/// implement fetchMessages and sendMessageToDLQ

struct SqsQueueListener {
  queue_url: String,
  callback: fn(msg) -> Result<(), Err>,
  _backoff: int
}

impl SqsQueueListener {
  pub fn listen(self) {
    // fetch a batch of messages
    batch = self.fetchMessages();
    match(batch.length) {
      0 => {
        self.backoff();
      }
      _ => {
        self.handleMessages(batch);
      }
    }
  }

  fn fetchMessages() -> Vec<SqsMessage> {
    println("WARNING: fetchMessages not implemented");
    return Vec();
  }

  fn backoff(self) {
    // wait backoff seconds
    self._backoff *= 2;
  }

  fn handleMessages(self, msgs: Vec<SqsMessage>) {
    for msg in msgs {
      match self.callback(msg) {
        Ok() => println("handled message"),
        Err() => println("error handling message") // should send to DLQ here
      }
    }
    self._backoff = 1;
  }
}
