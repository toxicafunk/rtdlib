
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Options to be used when a message is sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageSendOptions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Pass true to disable notification for the message
  disable_notification: bool,
  /// Pass true if the message is sent from the background
  from_background: bool,
  /// Message scheduling state; pass null to send message immediately. Messages sent to a secret chat, live location messages and self-destructing messages can't be scheduled
  scheduling_state: MessageSchedulingState,
  
}

impl RObject for MessageSendOptions {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageSendOptions" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl MessageSendOptions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDMessageSendOptionsBuilder {
    let mut inner = MessageSendOptions::default();
    inner.td_name = "messageSendOptions".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDMessageSendOptionsBuilder { inner }
  }

  pub fn disable_notification(&self) -> bool { self.disable_notification }

  pub fn from_background(&self) -> bool { self.from_background }

  pub fn scheduling_state(&self) -> &MessageSchedulingState { &self.scheduling_state }

}

#[doc(hidden)]
pub struct RTDMessageSendOptionsBuilder {
  inner: MessageSendOptions
}

impl RTDMessageSendOptionsBuilder {
  pub fn build(&self) -> MessageSendOptions { self.inner.clone() }

   
  pub fn disable_notification(&mut self, disable_notification: bool) -> &mut Self {
    self.inner.disable_notification = disable_notification;
    self
  }

   
  pub fn from_background(&mut self, from_background: bool) -> &mut Self {
    self.inner.from_background = from_background;
    self
  }

   
  pub fn scheduling_state<T: AsRef<MessageSchedulingState>>(&mut self, scheduling_state: T) -> &mut Self {
    self.inner.scheduling_state = scheduling_state.as_ref().clone();
    self
  }

}

impl AsRef<MessageSendOptions> for MessageSendOptions {
  fn as_ref(&self) -> &MessageSendOptions { self }
}

impl AsRef<MessageSendOptions> for RTDMessageSendOptionsBuilder {
  fn as_ref(&self) -> &MessageSendOptions { &self.inner }
}



