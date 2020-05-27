
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Specifies the kind of chat members to return in searchChatMembers
pub trait TDChatMembersFilter: Debug + RObject {}

/// Specifies the kind of chat members to return in searchChatMembers
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ChatMembersFilter {
  #[doc(hidden)] _Default(()),
  /// Returns the creator and administrators
  Administrators(ChatMembersFilterAdministrators),
  /// Returns users banned from the chat; can be used only by administrators in a supergroup or in a channel
  Banned(ChatMembersFilterBanned),
  /// Returns bot members of the chat
  Bots(ChatMembersFilterBots),
  /// Returns all chat members, including restricted chat members
  Members(ChatMembersFilterMembers),
  /// Returns users under certain restrictions in the chat; can be used only by administrators in a supergroup
  Restricted(ChatMembersFilterRestricted),

}

impl Default for ChatMembersFilter {
  fn default() -> Self { ChatMembersFilter::_Default(()) }
}

impl<'de> Deserialize<'de> for ChatMembersFilter {
  fn deserialize<D>(deserializer: D) -> Result<ChatMembersFilter, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ChatMembersFilter,
      (chatMembersFilterAdministrators, Administrators);
      (chatMembersFilterBanned, Banned);
      (chatMembersFilterBots, Bots);
      (chatMembersFilterMembers, Members);
      (chatMembersFilterRestricted, Restricted);

    )(deserializer)
  }
}

impl RObject for ChatMembersFilter {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      ChatMembersFilter::Administrators(t) => t.td_name(),
      ChatMembersFilter::Banned(t) => t.td_name(),
      ChatMembersFilter::Bots(t) => t.td_name(),
      ChatMembersFilter::Members(t) => t.td_name(),
      ChatMembersFilter::Restricted(t) => t.td_name(),

      _ => "-1",
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl ChatMembersFilter {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let ChatMembersFilter::_Default(_) = self { true } else { false } }

  pub fn is_administrators(&self) -> bool { if let ChatMembersFilter::Administrators(_) = self { true } else { false } }
  pub fn is_banned(&self) -> bool { if let ChatMembersFilter::Banned(_) = self { true } else { false } }
  pub fn is_bots(&self) -> bool { if let ChatMembersFilter::Bots(_) = self { true } else { false } }
  pub fn is_members(&self) -> bool { if let ChatMembersFilter::Members(_) = self { true } else { false } }
  pub fn is_restricted(&self) -> bool { if let ChatMembersFilter::Restricted(_) = self { true } else { false } }

  pub fn on_administrators<F: FnOnce(&ChatMembersFilterAdministrators)>(&self, fnc: F) -> &Self { if let ChatMembersFilter::Administrators(t) = self { fnc(t) }; self }
  pub fn on_banned<F: FnOnce(&ChatMembersFilterBanned)>(&self, fnc: F) -> &Self { if let ChatMembersFilter::Banned(t) = self { fnc(t) }; self }
  pub fn on_bots<F: FnOnce(&ChatMembersFilterBots)>(&self, fnc: F) -> &Self { if let ChatMembersFilter::Bots(t) = self { fnc(t) }; self }
  pub fn on_members<F: FnOnce(&ChatMembersFilterMembers)>(&self, fnc: F) -> &Self { if let ChatMembersFilter::Members(t) = self { fnc(t) }; self }
  pub fn on_restricted<F: FnOnce(&ChatMembersFilterRestricted)>(&self, fnc: F) -> &Self { if let ChatMembersFilter::Restricted(t) = self { fnc(t) }; self }

  pub fn as_administrators(&self) -> Option<&ChatMembersFilterAdministrators> { if let ChatMembersFilter::Administrators(t) = self { return Some(t) } None }
  pub fn as_banned(&self) -> Option<&ChatMembersFilterBanned> { if let ChatMembersFilter::Banned(t) = self { return Some(t) } None }
  pub fn as_bots(&self) -> Option<&ChatMembersFilterBots> { if let ChatMembersFilter::Bots(t) = self { return Some(t) } None }
  pub fn as_members(&self) -> Option<&ChatMembersFilterMembers> { if let ChatMembersFilter::Members(t) = self { return Some(t) } None }
  pub fn as_restricted(&self) -> Option<&ChatMembersFilterRestricted> { if let ChatMembersFilter::Restricted(t) = self { return Some(t) } None }



  pub fn administrators<T: AsRef<ChatMembersFilterAdministrators>>(t: T) -> Self { ChatMembersFilter::Administrators(t.as_ref().clone()) }

  pub fn banned<T: AsRef<ChatMembersFilterBanned>>(t: T) -> Self { ChatMembersFilter::Banned(t.as_ref().clone()) }

  pub fn bots<T: AsRef<ChatMembersFilterBots>>(t: T) -> Self { ChatMembersFilter::Bots(t.as_ref().clone()) }

  pub fn members<T: AsRef<ChatMembersFilterMembers>>(t: T) -> Self { ChatMembersFilter::Members(t.as_ref().clone()) }

  pub fn restricted<T: AsRef<ChatMembersFilterRestricted>>(t: T) -> Self { ChatMembersFilter::Restricted(t.as_ref().clone()) }

}

impl AsRef<ChatMembersFilter> for ChatMembersFilter {
  fn as_ref(&self) -> &ChatMembersFilter { self }
}







/// Returns the creator and administrators
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMembersFilterAdministrators {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for ChatMembersFilterAdministrators {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatMembersFilterAdministrators" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatMembersFilter for ChatMembersFilterAdministrators {}



impl ChatMembersFilterAdministrators {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatMembersFilterAdministratorsBuilder {
    let mut inner = ChatMembersFilterAdministrators::default();
    inner.td_name = "chatMembersFilterAdministrators".to_string();
    RTDChatMembersFilterAdministratorsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatMembersFilterAdministratorsBuilder {
  inner: ChatMembersFilterAdministrators
}

impl RTDChatMembersFilterAdministratorsBuilder {
  pub fn build(&self) -> ChatMembersFilterAdministrators { self.inner.clone() }

}

impl AsRef<ChatMembersFilterAdministrators> for ChatMembersFilterAdministrators {
  fn as_ref(&self) -> &ChatMembersFilterAdministrators { self }
}

impl AsRef<ChatMembersFilterAdministrators> for RTDChatMembersFilterAdministratorsBuilder {
  fn as_ref(&self) -> &ChatMembersFilterAdministrators { &self.inner }
}







/// Returns users banned from the chat; can be used only by administrators in a supergroup or in a channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMembersFilterBanned {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for ChatMembersFilterBanned {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatMembersFilterBanned" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatMembersFilter for ChatMembersFilterBanned {}



impl ChatMembersFilterBanned {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatMembersFilterBannedBuilder {
    let mut inner = ChatMembersFilterBanned::default();
    inner.td_name = "chatMembersFilterBanned".to_string();
    RTDChatMembersFilterBannedBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatMembersFilterBannedBuilder {
  inner: ChatMembersFilterBanned
}

impl RTDChatMembersFilterBannedBuilder {
  pub fn build(&self) -> ChatMembersFilterBanned { self.inner.clone() }

}

impl AsRef<ChatMembersFilterBanned> for ChatMembersFilterBanned {
  fn as_ref(&self) -> &ChatMembersFilterBanned { self }
}

impl AsRef<ChatMembersFilterBanned> for RTDChatMembersFilterBannedBuilder {
  fn as_ref(&self) -> &ChatMembersFilterBanned { &self.inner }
}







/// Returns bot members of the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMembersFilterBots {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for ChatMembersFilterBots {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatMembersFilterBots" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatMembersFilter for ChatMembersFilterBots {}



impl ChatMembersFilterBots {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatMembersFilterBotsBuilder {
    let mut inner = ChatMembersFilterBots::default();
    inner.td_name = "chatMembersFilterBots".to_string();
    RTDChatMembersFilterBotsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatMembersFilterBotsBuilder {
  inner: ChatMembersFilterBots
}

impl RTDChatMembersFilterBotsBuilder {
  pub fn build(&self) -> ChatMembersFilterBots { self.inner.clone() }

}

impl AsRef<ChatMembersFilterBots> for ChatMembersFilterBots {
  fn as_ref(&self) -> &ChatMembersFilterBots { self }
}

impl AsRef<ChatMembersFilterBots> for RTDChatMembersFilterBotsBuilder {
  fn as_ref(&self) -> &ChatMembersFilterBots { &self.inner }
}







/// Returns all chat members, including restricted chat members
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMembersFilterMembers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for ChatMembersFilterMembers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatMembersFilterMembers" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatMembersFilter for ChatMembersFilterMembers {}



impl ChatMembersFilterMembers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatMembersFilterMembersBuilder {
    let mut inner = ChatMembersFilterMembers::default();
    inner.td_name = "chatMembersFilterMembers".to_string();
    RTDChatMembersFilterMembersBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatMembersFilterMembersBuilder {
  inner: ChatMembersFilterMembers
}

impl RTDChatMembersFilterMembersBuilder {
  pub fn build(&self) -> ChatMembersFilterMembers { self.inner.clone() }

}

impl AsRef<ChatMembersFilterMembers> for ChatMembersFilterMembers {
  fn as_ref(&self) -> &ChatMembersFilterMembers { self }
}

impl AsRef<ChatMembersFilterMembers> for RTDChatMembersFilterMembersBuilder {
  fn as_ref(&self) -> &ChatMembersFilterMembers { &self.inner }
}







/// Returns users under certain restrictions in the chat; can be used only by administrators in a supergroup
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMembersFilterRestricted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for ChatMembersFilterRestricted {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatMembersFilterRestricted" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatMembersFilter for ChatMembersFilterRestricted {}



impl ChatMembersFilterRestricted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatMembersFilterRestrictedBuilder {
    let mut inner = ChatMembersFilterRestricted::default();
    inner.td_name = "chatMembersFilterRestricted".to_string();
    RTDChatMembersFilterRestrictedBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatMembersFilterRestrictedBuilder {
  inner: ChatMembersFilterRestricted
}

impl RTDChatMembersFilterRestrictedBuilder {
  pub fn build(&self) -> ChatMembersFilterRestricted { self.inner.clone() }

}

impl AsRef<ChatMembersFilterRestricted> for ChatMembersFilterRestricted {
  fn as_ref(&self) -> &ChatMembersFilterRestricted { self }
}

impl AsRef<ChatMembersFilterRestricted> for RTDChatMembersFilterRestrictedBuilder {
  fn as_ref(&self) -> &ChatMembersFilterRestricted { &self.inner }
}



