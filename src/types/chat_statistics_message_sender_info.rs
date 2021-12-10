
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains statistics about messages sent by a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatStatisticsMessageSenderInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier
  user_id: i64,
  /// Number of sent messages
  sent_message_count: i64,
  /// Average number of characters in sent messages; 0 if unknown
  average_character_count: i64,
  
}

impl RObject for ChatStatisticsMessageSenderInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatStatisticsMessageSenderInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatStatisticsMessageSenderInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatStatisticsMessageSenderInfoBuilder {
    let mut inner = ChatStatisticsMessageSenderInfo::default();
    inner.td_name = "chatStatisticsMessageSenderInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatStatisticsMessageSenderInfoBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn sent_message_count(&self) -> i64 { self.sent_message_count }

  pub fn average_character_count(&self) -> i64 { self.average_character_count }

}

#[doc(hidden)]
pub struct RTDChatStatisticsMessageSenderInfoBuilder {
  inner: ChatStatisticsMessageSenderInfo
}

impl RTDChatStatisticsMessageSenderInfoBuilder {
  pub fn build(&self) -> ChatStatisticsMessageSenderInfo { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn sent_message_count(&mut self, sent_message_count: i64) -> &mut Self {
    self.inner.sent_message_count = sent_message_count;
    self
  }

   
  pub fn average_character_count(&mut self, average_character_count: i64) -> &mut Self {
    self.inner.average_character_count = average_character_count;
    self
  }

}

impl AsRef<ChatStatisticsMessageSenderInfo> for ChatStatisticsMessageSenderInfo {
  fn as_ref(&self) -> &ChatStatisticsMessageSenderInfo { self }
}

impl AsRef<ChatStatisticsMessageSenderInfo> for RTDChatStatisticsMessageSenderInfoBuilder {
  fn as_ref(&self) -> &ChatStatisticsMessageSenderInfo { &self.inner }
}



