#[allow(dead_code)]
pub struct Worker;

#[derive(Debug)]
pub struct Message {
    data: String,
}

impl Message {
    pub fn new(data: String) -> Self {
        Message { data }
    }
}

impl Worker {
    #[tracing::instrument(skip_all)]
    pub async fn build_patch(&self, incoming_message: Message) -> Result<String, ()> {
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        Ok(format!("patch: {}", incoming_message.data))
    }

    #[tracing::instrument(skip_all, fields(message=incoming_message.data))]
    pub async fn process_incoming(&self, incoming_message: Message) {
        let _ = self.build_patch(incoming_message).await.unwrap();
    }
}
