
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes the type of a chat
pub trait TDChatType: Debug + RObject {}

/// Describes the type of a chat
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ChatType {
  #[doc(hidden)] _Default(()),
  /// A basic group (a chat with 0-200 other users)
  BasicGroup(ChatTypeBasicGroup),
  /// An ordinary chat with a user
  Private(ChatTypePrivate),
  /// A secret chat with a user
  Secret(ChatTypeSecret),
  /// A supergroup or channel (with unlimited members)
  Supergroup(ChatTypeSupergroup),

}

impl Default for ChatType {
  fn default() -> Self { ChatType::_Default(()) }
}

impl<'de> Deserialize<'de> for ChatType {
  fn deserialize<D>(deserializer: D) -> Result<ChatType, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ChatType,
      (chatTypeBasicGroup, BasicGroup);
      (chatTypePrivate, Private);
      (chatTypeSecret, Secret);
      (chatTypeSupergroup, Supergroup);

    )(deserializer)
  }
}

impl RObject for ChatType {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      ChatType::BasicGroup(t) => t.td_name(),
      ChatType::Private(t) => t.td_name(),
      ChatType::Secret(t) => t.td_name(),
      ChatType::Supergroup(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      ChatType::BasicGroup(t) => t.extra(),
      ChatType::Private(t) => t.extra(),
      ChatType::Secret(t) => t.extra(),
      ChatType::Supergroup(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl ChatType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let ChatType::_Default(_) = self { true } else { false } }

  pub fn is_basic_group(&self) -> bool { if let ChatType::BasicGroup(_) = self { true } else { false } }
  pub fn is_private(&self) -> bool { if let ChatType::Private(_) = self { true } else { false } }
  pub fn is_secret(&self) -> bool { if let ChatType::Secret(_) = self { true } else { false } }
  pub fn is_supergroup(&self) -> bool { if let ChatType::Supergroup(_) = self { true } else { false } }

  pub fn on_basic_group<F: FnOnce(&ChatTypeBasicGroup)>(&self, fnc: F) -> &Self { if let ChatType::BasicGroup(t) = self { fnc(t) }; self }
  pub fn on_private<F: FnOnce(&ChatTypePrivate)>(&self, fnc: F) -> &Self { if let ChatType::Private(t) = self { fnc(t) }; self }
  pub fn on_secret<F: FnOnce(&ChatTypeSecret)>(&self, fnc: F) -> &Self { if let ChatType::Secret(t) = self { fnc(t) }; self }
  pub fn on_supergroup<F: FnOnce(&ChatTypeSupergroup)>(&self, fnc: F) -> &Self { if let ChatType::Supergroup(t) = self { fnc(t) }; self }

  pub fn as_basic_group(&self) -> Option<&ChatTypeBasicGroup> { if let ChatType::BasicGroup(t) = self { return Some(t) } None }
  pub fn as_private(&self) -> Option<&ChatTypePrivate> { if let ChatType::Private(t) = self { return Some(t) } None }
  pub fn as_secret(&self) -> Option<&ChatTypeSecret> { if let ChatType::Secret(t) = self { return Some(t) } None }
  pub fn as_supergroup(&self) -> Option<&ChatTypeSupergroup> { if let ChatType::Supergroup(t) = self { return Some(t) } None }



  pub fn basic_group<T: AsRef<ChatTypeBasicGroup>>(t: T) -> Self { ChatType::BasicGroup(t.as_ref().clone()) }

  pub fn private<T: AsRef<ChatTypePrivate>>(t: T) -> Self { ChatType::Private(t.as_ref().clone()) }

  pub fn secret<T: AsRef<ChatTypeSecret>>(t: T) -> Self { ChatType::Secret(t.as_ref().clone()) }

  pub fn supergroup<T: AsRef<ChatTypeSupergroup>>(t: T) -> Self { ChatType::Supergroup(t.as_ref().clone()) }

}

impl AsRef<ChatType> for ChatType {
  fn as_ref(&self) -> &ChatType { self }
}







/// A basic group (a chat with 0-200 other users)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatTypeBasicGroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Basic group identifier
  basic_group_id: i64,
  
}

impl RObject for ChatTypeBasicGroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatTypeBasicGroup" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatType for ChatTypeBasicGroup {}



impl ChatTypeBasicGroup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatTypeBasicGroupBuilder {
    let mut inner = ChatTypeBasicGroup::default();
    inner.td_name = "chatTypeBasicGroup".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatTypeBasicGroupBuilder { inner }
  }

  pub fn basic_group_id(&self) -> i64 { self.basic_group_id }

}

#[doc(hidden)]
pub struct RTDChatTypeBasicGroupBuilder {
  inner: ChatTypeBasicGroup
}

impl RTDChatTypeBasicGroupBuilder {
  pub fn build(&self) -> ChatTypeBasicGroup { self.inner.clone() }

   
  pub fn basic_group_id(&mut self, basic_group_id: i64) -> &mut Self {
    self.inner.basic_group_id = basic_group_id;
    self
  }

}

impl AsRef<ChatTypeBasicGroup> for ChatTypeBasicGroup {
  fn as_ref(&self) -> &ChatTypeBasicGroup { self }
}

impl AsRef<ChatTypeBasicGroup> for RTDChatTypeBasicGroupBuilder {
  fn as_ref(&self) -> &ChatTypeBasicGroup { &self.inner }
}







/// An ordinary chat with a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatTypePrivate {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier
  user_id: i64,
  
}

impl RObject for ChatTypePrivate {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatTypePrivate" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatType for ChatTypePrivate {}



impl ChatTypePrivate {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatTypePrivateBuilder {
    let mut inner = ChatTypePrivate::default();
    inner.td_name = "chatTypePrivate".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatTypePrivateBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

}

#[doc(hidden)]
pub struct RTDChatTypePrivateBuilder {
  inner: ChatTypePrivate
}

impl RTDChatTypePrivateBuilder {
  pub fn build(&self) -> ChatTypePrivate { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

}

impl AsRef<ChatTypePrivate> for ChatTypePrivate {
  fn as_ref(&self) -> &ChatTypePrivate { self }
}

impl AsRef<ChatTypePrivate> for RTDChatTypePrivateBuilder {
  fn as_ref(&self) -> &ChatTypePrivate { &self.inner }
}







/// A secret chat with a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatTypeSecret {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Secret chat identifier
  secret_chat_id: i64,
  /// User identifier of the secret chat peer
  user_id: i64,
  
}

impl RObject for ChatTypeSecret {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatTypeSecret" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatType for ChatTypeSecret {}



impl ChatTypeSecret {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatTypeSecretBuilder {
    let mut inner = ChatTypeSecret::default();
    inner.td_name = "chatTypeSecret".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatTypeSecretBuilder { inner }
  }

  pub fn secret_chat_id(&self) -> i64 { self.secret_chat_id }

  pub fn user_id(&self) -> i64 { self.user_id }

}

#[doc(hidden)]
pub struct RTDChatTypeSecretBuilder {
  inner: ChatTypeSecret
}

impl RTDChatTypeSecretBuilder {
  pub fn build(&self) -> ChatTypeSecret { self.inner.clone() }

   
  pub fn secret_chat_id(&mut self, secret_chat_id: i64) -> &mut Self {
    self.inner.secret_chat_id = secret_chat_id;
    self
  }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

}

impl AsRef<ChatTypeSecret> for ChatTypeSecret {
  fn as_ref(&self) -> &ChatTypeSecret { self }
}

impl AsRef<ChatTypeSecret> for RTDChatTypeSecretBuilder {
  fn as_ref(&self) -> &ChatTypeSecret { &self.inner }
}







/// A supergroup or channel (with unlimited members)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatTypeSupergroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Supergroup or channel identifier
  supergroup_id: i64,
  /// True, if the supergroup is a channel
  is_channel: bool,
  
}

impl RObject for ChatTypeSupergroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatTypeSupergroup" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatType for ChatTypeSupergroup {}



impl ChatTypeSupergroup {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatTypeSupergroupBuilder {
    let mut inner = ChatTypeSupergroup::default();
    inner.td_name = "chatTypeSupergroup".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatTypeSupergroupBuilder { inner }
  }

  pub fn supergroup_id(&self) -> i64 { self.supergroup_id }

  pub fn is_channel(&self) -> bool { self.is_channel }

}

#[doc(hidden)]
pub struct RTDChatTypeSupergroupBuilder {
  inner: ChatTypeSupergroup
}

impl RTDChatTypeSupergroupBuilder {
  pub fn build(&self) -> ChatTypeSupergroup { self.inner.clone() }

   
  pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
    self.inner.supergroup_id = supergroup_id;
    self
  }

   
  pub fn is_channel(&mut self, is_channel: bool) -> &mut Self {
    self.inner.is_channel = is_channel;
    self
  }

}

impl AsRef<ChatTypeSupergroup> for ChatTypeSupergroup {
  fn as_ref(&self) -> &ChatTypeSupergroup { self }
}

impl AsRef<ChatTypeSupergroup> for RTDChatTypeSupergroupBuilder {
  fn as_ref(&self) -> &ChatTypeSupergroup { &self.inner }
}



