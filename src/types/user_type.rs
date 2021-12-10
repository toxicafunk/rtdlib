
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Represents the type of a user. The following types are possible: regular users, deleted users and bots
pub trait TDUserType: Debug + RObject {}

/// Represents the type of a user. The following types are possible: regular users, deleted users and bots
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum UserType {
  #[doc(hidden)] _Default(()),
  /// A bot (see https://core.telegram.org/bots)
  Bot(UserTypeBot),
  /// A deleted user or deleted bot. No information on the user besides the user identifier is available. It is not possible to perform any active actions on this type of user
  Deleted(UserTypeDeleted),
  /// A regular user
  Regular(UserTypeRegular),
  /// No information on the user besides the user identifier is available, yet this user has not been deleted. This object is extremely rare and must be handled like a deleted user. It is not possible to perform any actions on users of this type
  Unknown(UserTypeUnknown),

}

impl Default for UserType {
  fn default() -> Self { UserType::_Default(()) }
}

impl<'de> Deserialize<'de> for UserType {
  fn deserialize<D>(deserializer: D) -> Result<UserType, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      UserType,
      (userTypeBot, Bot);
      (userTypeDeleted, Deleted);
      (userTypeRegular, Regular);
      (userTypeUnknown, Unknown);

    )(deserializer)
  }
}

impl RObject for UserType {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      UserType::Bot(t) => t.td_name(),
      UserType::Deleted(t) => t.td_name(),
      UserType::Regular(t) => t.td_name(),
      UserType::Unknown(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      UserType::Bot(t) => t.extra(),
      UserType::Deleted(t) => t.extra(),
      UserType::Regular(t) => t.extra(),
      UserType::Unknown(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl UserType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let UserType::_Default(_) = self { true } else { false } }

  pub fn is_bot(&self) -> bool { if let UserType::Bot(_) = self { true } else { false } }
  pub fn is_deleted(&self) -> bool { if let UserType::Deleted(_) = self { true } else { false } }
  pub fn is_regular(&self) -> bool { if let UserType::Regular(_) = self { true } else { false } }
  pub fn is_unknown(&self) -> bool { if let UserType::Unknown(_) = self { true } else { false } }

  pub fn on_bot<F: FnOnce(&UserTypeBot)>(&self, fnc: F) -> &Self { if let UserType::Bot(t) = self { fnc(t) }; self }
  pub fn on_deleted<F: FnOnce(&UserTypeDeleted)>(&self, fnc: F) -> &Self { if let UserType::Deleted(t) = self { fnc(t) }; self }
  pub fn on_regular<F: FnOnce(&UserTypeRegular)>(&self, fnc: F) -> &Self { if let UserType::Regular(t) = self { fnc(t) }; self }
  pub fn on_unknown<F: FnOnce(&UserTypeUnknown)>(&self, fnc: F) -> &Self { if let UserType::Unknown(t) = self { fnc(t) }; self }

  pub fn as_bot(&self) -> Option<&UserTypeBot> { if let UserType::Bot(t) = self { return Some(t) } None }
  pub fn as_deleted(&self) -> Option<&UserTypeDeleted> { if let UserType::Deleted(t) = self { return Some(t) } None }
  pub fn as_regular(&self) -> Option<&UserTypeRegular> { if let UserType::Regular(t) = self { return Some(t) } None }
  pub fn as_unknown(&self) -> Option<&UserTypeUnknown> { if let UserType::Unknown(t) = self { return Some(t) } None }



  pub fn bot<T: AsRef<UserTypeBot>>(t: T) -> Self { UserType::Bot(t.as_ref().clone()) }

  pub fn deleted<T: AsRef<UserTypeDeleted>>(t: T) -> Self { UserType::Deleted(t.as_ref().clone()) }

  pub fn regular<T: AsRef<UserTypeRegular>>(t: T) -> Self { UserType::Regular(t.as_ref().clone()) }

  pub fn unknown<T: AsRef<UserTypeUnknown>>(t: T) -> Self { UserType::Unknown(t.as_ref().clone()) }

}

impl AsRef<UserType> for UserType {
  fn as_ref(&self) -> &UserType { self }
}







/// A bot (see https://core.telegram.org/bots)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserTypeBot {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the bot can be invited to basic group and supergroup chats
  can_join_groups: bool,
  /// True, if the bot can read all messages in basic group or supergroup chats and not just those addressed to the bot. In private and channel chats a bot can always read all messages
  can_read_all_group_messages: bool,
  /// True, if the bot supports inline queries
  is_inline: bool,
  /// Placeholder for inline queries (displayed on the application input field)
  inline_query_placeholder: String,
  /// True, if the location of the user is expected to be sent with every inline query to this bot
  need_location: bool,
  
}

impl RObject for UserTypeBot {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userTypeBot" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUserType for UserTypeBot {}



impl UserTypeBot {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUserTypeBotBuilder {
    let mut inner = UserTypeBot::default();
    inner.td_name = "userTypeBot".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUserTypeBotBuilder { inner }
  }

  pub fn can_join_groups(&self) -> bool { self.can_join_groups }

  pub fn can_read_all_group_messages(&self) -> bool { self.can_read_all_group_messages }

  pub fn is_inline(&self) -> bool { self.is_inline }

  pub fn inline_query_placeholder(&self) -> &String { &self.inline_query_placeholder }

  pub fn need_location(&self) -> bool { self.need_location }

}

#[doc(hidden)]
pub struct RTDUserTypeBotBuilder {
  inner: UserTypeBot
}

impl RTDUserTypeBotBuilder {
  pub fn build(&self) -> UserTypeBot { self.inner.clone() }

   
  pub fn can_join_groups(&mut self, can_join_groups: bool) -> &mut Self {
    self.inner.can_join_groups = can_join_groups;
    self
  }

   
  pub fn can_read_all_group_messages(&mut self, can_read_all_group_messages: bool) -> &mut Self {
    self.inner.can_read_all_group_messages = can_read_all_group_messages;
    self
  }

   
  pub fn is_inline(&mut self, is_inline: bool) -> &mut Self {
    self.inner.is_inline = is_inline;
    self
  }

   
  pub fn inline_query_placeholder<T: AsRef<str>>(&mut self, inline_query_placeholder: T) -> &mut Self {
    self.inner.inline_query_placeholder = inline_query_placeholder.as_ref().to_string();
    self
  }

   
  pub fn need_location(&mut self, need_location: bool) -> &mut Self {
    self.inner.need_location = need_location;
    self
  }

}

impl AsRef<UserTypeBot> for UserTypeBot {
  fn as_ref(&self) -> &UserTypeBot { self }
}

impl AsRef<UserTypeBot> for RTDUserTypeBotBuilder {
  fn as_ref(&self) -> &UserTypeBot { &self.inner }
}







/// A deleted user or deleted bot. No information on the user besides the user identifier is available. It is not possible to perform any active actions on this type of user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserTypeDeleted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for UserTypeDeleted {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userTypeDeleted" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUserType for UserTypeDeleted {}



impl UserTypeDeleted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUserTypeDeletedBuilder {
    let mut inner = UserTypeDeleted::default();
    inner.td_name = "userTypeDeleted".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUserTypeDeletedBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDUserTypeDeletedBuilder {
  inner: UserTypeDeleted
}

impl RTDUserTypeDeletedBuilder {
  pub fn build(&self) -> UserTypeDeleted { self.inner.clone() }

}

impl AsRef<UserTypeDeleted> for UserTypeDeleted {
  fn as_ref(&self) -> &UserTypeDeleted { self }
}

impl AsRef<UserTypeDeleted> for RTDUserTypeDeletedBuilder {
  fn as_ref(&self) -> &UserTypeDeleted { &self.inner }
}







/// A regular user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserTypeRegular {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for UserTypeRegular {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userTypeRegular" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUserType for UserTypeRegular {}



impl UserTypeRegular {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUserTypeRegularBuilder {
    let mut inner = UserTypeRegular::default();
    inner.td_name = "userTypeRegular".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUserTypeRegularBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDUserTypeRegularBuilder {
  inner: UserTypeRegular
}

impl RTDUserTypeRegularBuilder {
  pub fn build(&self) -> UserTypeRegular { self.inner.clone() }

}

impl AsRef<UserTypeRegular> for UserTypeRegular {
  fn as_ref(&self) -> &UserTypeRegular { self }
}

impl AsRef<UserTypeRegular> for RTDUserTypeRegularBuilder {
  fn as_ref(&self) -> &UserTypeRegular { &self.inner }
}







/// No information on the user besides the user identifier is available, yet this user has not been deleted. This object is extremely rare and must be handled like a deleted user. It is not possible to perform any actions on users of this type
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserTypeUnknown {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for UserTypeUnknown {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "userTypeUnknown" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDUserType for UserTypeUnknown {}



impl UserTypeUnknown {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDUserTypeUnknownBuilder {
    let mut inner = UserTypeUnknown::default();
    inner.td_name = "userTypeUnknown".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDUserTypeUnknownBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDUserTypeUnknownBuilder {
  inner: UserTypeUnknown
}

impl RTDUserTypeUnknownBuilder {
  pub fn build(&self) -> UserTypeUnknown { self.inner.clone() }

}

impl AsRef<UserTypeUnknown> for UserTypeUnknown {
  fn as_ref(&self) -> &UserTypeUnknown { self }
}

impl AsRef<UserTypeUnknown> for RTDUserTypeUnknownBuilder {
  fn as_ref(&self) -> &UserTypeUnknown { &self.inner }
}



