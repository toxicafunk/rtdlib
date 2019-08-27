
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Provides information about the status of a member in a chat
pub trait TDChatMemberStatus: Debug + RObject {}

/// Provides information about the status of a member in a chat
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ChatMemberStatus {
  #[doc(hidden)] _Default(()),
  /// The user is the creator of a chat and has all the administrator privileges
  Creator(ChatMemberStatusCreator),
  /// The user is a member of a chat and has some additional privileges. In basic groups, administrators can edit and delete messages sent by others, add new members, and ban unprivileged members. In supergroups and channels, there are more detailed options for administrator privileges
  Administrator(ChatMemberStatusAdministrator),
  /// The user is a member of a chat, without any additional privileges or restrictions
  Member(ChatMemberStatusMember),
  /// The user is under certain restrictions in the chat. Not supported in basic groups and channels
  Restricted(ChatMemberStatusRestricted),
  /// The user is not a chat member
  Left(ChatMemberStatusLeft),
  /// The user was banned (and hence is not a member of the chat). Implies the user can't return to the chat or view messages
  Banned(ChatMemberStatusBanned),

}

impl Default for ChatMemberStatus {
  fn default() -> Self { ChatMemberStatus::_Default(()) }
}

impl<'de> Deserialize<'de> for ChatMemberStatus {
  fn deserialize<D>(deserializer: D) -> Result<ChatMemberStatus, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ChatMemberStatus,
      (chatMemberStatusCreator, Creator);
      (chatMemberStatusAdministrator, Administrator);
      (chatMemberStatusMember, Member);
      (chatMemberStatusRestricted, Restricted);
      (chatMemberStatusLeft, Left);
      (chatMemberStatusBanned, Banned);

    )(deserializer)
  }
}

impl RObject for ChatMemberStatus {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      ChatMemberStatus::Creator(t) => t.td_name(),
      ChatMemberStatus::Administrator(t) => t.td_name(),
      ChatMemberStatus::Member(t) => t.td_name(),
      ChatMemberStatus::Restricted(t) => t.td_name(),
      ChatMemberStatus::Left(t) => t.td_name(),
      ChatMemberStatus::Banned(t) => t.td_name(),

      _ => "-1",
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl ChatMemberStatus {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let ChatMemberStatus::_Default(_) = self { true } else { false } }

  pub fn is_creator(&self) -> bool { if let ChatMemberStatus::Creator(_) = self { true } else { false } }
  pub fn is_administrator(&self) -> bool { if let ChatMemberStatus::Administrator(_) = self { true } else { false } }
  pub fn is_member(&self) -> bool { if let ChatMemberStatus::Member(_) = self { true } else { false } }
  pub fn is_restricted(&self) -> bool { if let ChatMemberStatus::Restricted(_) = self { true } else { false } }
  pub fn is_left(&self) -> bool { if let ChatMemberStatus::Left(_) = self { true } else { false } }
  pub fn is_banned(&self) -> bool { if let ChatMemberStatus::Banned(_) = self { true } else { false } }

  pub fn on_creator<F: FnOnce(&ChatMemberStatusCreator)>(&self, fnc: F) -> &Self { if let ChatMemberStatus::Creator(t) = self { fnc(t) }; self }
  pub fn on_administrator<F: FnOnce(&ChatMemberStatusAdministrator)>(&self, fnc: F) -> &Self { if let ChatMemberStatus::Administrator(t) = self { fnc(t) }; self }
  pub fn on_member<F: FnOnce(&ChatMemberStatusMember)>(&self, fnc: F) -> &Self { if let ChatMemberStatus::Member(t) = self { fnc(t) }; self }
  pub fn on_restricted<F: FnOnce(&ChatMemberStatusRestricted)>(&self, fnc: F) -> &Self { if let ChatMemberStatus::Restricted(t) = self { fnc(t) }; self }
  pub fn on_left<F: FnOnce(&ChatMemberStatusLeft)>(&self, fnc: F) -> &Self { if let ChatMemberStatus::Left(t) = self { fnc(t) }; self }
  pub fn on_banned<F: FnOnce(&ChatMemberStatusBanned)>(&self, fnc: F) -> &Self { if let ChatMemberStatus::Banned(t) = self { fnc(t) }; self }

  pub fn as_creator(&self) -> Option<&ChatMemberStatusCreator> { if let ChatMemberStatus::Creator(t) = self { return Some(t) } None }
  pub fn as_administrator(&self) -> Option<&ChatMemberStatusAdministrator> { if let ChatMemberStatus::Administrator(t) = self { return Some(t) } None }
  pub fn as_member(&self) -> Option<&ChatMemberStatusMember> { if let ChatMemberStatus::Member(t) = self { return Some(t) } None }
  pub fn as_restricted(&self) -> Option<&ChatMemberStatusRestricted> { if let ChatMemberStatus::Restricted(t) = self { return Some(t) } None }
  pub fn as_left(&self) -> Option<&ChatMemberStatusLeft> { if let ChatMemberStatus::Left(t) = self { return Some(t) } None }
  pub fn as_banned(&self) -> Option<&ChatMemberStatusBanned> { if let ChatMemberStatus::Banned(t) = self { return Some(t) } None }



  pub fn creator<T: AsRef<ChatMemberStatusCreator>>(t: T) -> Self { ChatMemberStatus::Creator(t.as_ref().clone()) }

  pub fn administrator<T: AsRef<ChatMemberStatusAdministrator>>(t: T) -> Self { ChatMemberStatus::Administrator(t.as_ref().clone()) }

  pub fn member<T: AsRef<ChatMemberStatusMember>>(t: T) -> Self { ChatMemberStatus::Member(t.as_ref().clone()) }

  pub fn restricted<T: AsRef<ChatMemberStatusRestricted>>(t: T) -> Self { ChatMemberStatus::Restricted(t.as_ref().clone()) }

  pub fn left<T: AsRef<ChatMemberStatusLeft>>(t: T) -> Self { ChatMemberStatus::Left(t.as_ref().clone()) }

  pub fn banned<T: AsRef<ChatMemberStatusBanned>>(t: T) -> Self { ChatMemberStatus::Banned(t.as_ref().clone()) }

}

impl AsRef<ChatMemberStatus> for ChatMemberStatus {
  fn as_ref(&self) -> &ChatMemberStatus { self }
}







/// The user is the creator of a chat and has all the administrator privileges
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMemberStatusCreator {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// True, if the user is a member of the chat
  is_member: bool,
  
}

impl RObject for ChatMemberStatusCreator {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatMemberStatusCreator" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatMemberStatus for ChatMemberStatusCreator {}



impl ChatMemberStatusCreator {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatMemberStatusCreatorBuilder {
    let mut inner = ChatMemberStatusCreator::default();
    inner.td_name = "chatMemberStatusCreator".to_string();
    RTDChatMemberStatusCreatorBuilder { inner }
  }

  pub fn is_member(&self) -> bool { self.is_member }

}

#[doc(hidden)]
pub struct RTDChatMemberStatusCreatorBuilder {
  inner: ChatMemberStatusCreator
}

impl RTDChatMemberStatusCreatorBuilder {
  pub fn build(&self) -> ChatMemberStatusCreator { self.inner.clone() }

   
  pub fn is_member(&mut self, is_member: bool) -> &mut Self {
    self.inner.is_member = is_member;
    self
  }

}

impl AsRef<ChatMemberStatusCreator> for ChatMemberStatusCreator {
  fn as_ref(&self) -> &ChatMemberStatusCreator { self }
}

impl AsRef<ChatMemberStatusCreator> for RTDChatMemberStatusCreatorBuilder {
  fn as_ref(&self) -> &ChatMemberStatusCreator { &self.inner }
}







/// The user is a member of a chat and has some additional privileges. In basic groups, administrators can edit and delete messages sent by others, add new members, and ban unprivileged members. In supergroups and channels, there are more detailed options for administrator privileges
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMemberStatusAdministrator {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// True, if the current user can edit the administrator privileges for the called user
  can_be_edited: bool,
  /// True, if the administrator can change the chat title, photo, and other settings
  can_change_info: bool,
  /// True, if the administrator can create channel posts; applicable to channels only
  can_post_messages: bool,
  /// True, if the administrator can edit messages of other users and pin messages; applicable to channels only
  can_edit_messages: bool,
  /// True, if the administrator can delete messages of other users
  can_delete_messages: bool,
  /// True, if the administrator can invite new users to the chat
  can_invite_users: bool,
  /// True, if the administrator can restrict, ban, or unban chat members
  can_restrict_members: bool,
  /// True, if the administrator can pin messages; applicable to supergroups only
  can_pin_messages: bool,
  /// True, if the administrator can add new administrators with a subset of his own privileges or demote administrators that were directly or indirectly promoted by him
  can_promote_members: bool,
  
}

impl RObject for ChatMemberStatusAdministrator {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatMemberStatusAdministrator" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatMemberStatus for ChatMemberStatusAdministrator {}



impl ChatMemberStatusAdministrator {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatMemberStatusAdministratorBuilder {
    let mut inner = ChatMemberStatusAdministrator::default();
    inner.td_name = "chatMemberStatusAdministrator".to_string();
    RTDChatMemberStatusAdministratorBuilder { inner }
  }

  pub fn can_be_edited(&self) -> bool { self.can_be_edited }

  pub fn can_change_info(&self) -> bool { self.can_change_info }

  pub fn can_post_messages(&self) -> bool { self.can_post_messages }

  pub fn can_edit_messages(&self) -> bool { self.can_edit_messages }

  pub fn can_delete_messages(&self) -> bool { self.can_delete_messages }

  pub fn can_invite_users(&self) -> bool { self.can_invite_users }

  pub fn can_restrict_members(&self) -> bool { self.can_restrict_members }

  pub fn can_pin_messages(&self) -> bool { self.can_pin_messages }

  pub fn can_promote_members(&self) -> bool { self.can_promote_members }

}

#[doc(hidden)]
pub struct RTDChatMemberStatusAdministratorBuilder {
  inner: ChatMemberStatusAdministrator
}

impl RTDChatMemberStatusAdministratorBuilder {
  pub fn build(&self) -> ChatMemberStatusAdministrator { self.inner.clone() }

   
  pub fn can_be_edited(&mut self, can_be_edited: bool) -> &mut Self {
    self.inner.can_be_edited = can_be_edited;
    self
  }

   
  pub fn can_change_info(&mut self, can_change_info: bool) -> &mut Self {
    self.inner.can_change_info = can_change_info;
    self
  }

   
  pub fn can_post_messages(&mut self, can_post_messages: bool) -> &mut Self {
    self.inner.can_post_messages = can_post_messages;
    self
  }

   
  pub fn can_edit_messages(&mut self, can_edit_messages: bool) -> &mut Self {
    self.inner.can_edit_messages = can_edit_messages;
    self
  }

   
  pub fn can_delete_messages(&mut self, can_delete_messages: bool) -> &mut Self {
    self.inner.can_delete_messages = can_delete_messages;
    self
  }

   
  pub fn can_invite_users(&mut self, can_invite_users: bool) -> &mut Self {
    self.inner.can_invite_users = can_invite_users;
    self
  }

   
  pub fn can_restrict_members(&mut self, can_restrict_members: bool) -> &mut Self {
    self.inner.can_restrict_members = can_restrict_members;
    self
  }

   
  pub fn can_pin_messages(&mut self, can_pin_messages: bool) -> &mut Self {
    self.inner.can_pin_messages = can_pin_messages;
    self
  }

   
  pub fn can_promote_members(&mut self, can_promote_members: bool) -> &mut Self {
    self.inner.can_promote_members = can_promote_members;
    self
  }

}

impl AsRef<ChatMemberStatusAdministrator> for ChatMemberStatusAdministrator {
  fn as_ref(&self) -> &ChatMemberStatusAdministrator { self }
}

impl AsRef<ChatMemberStatusAdministrator> for RTDChatMemberStatusAdministratorBuilder {
  fn as_ref(&self) -> &ChatMemberStatusAdministrator { &self.inner }
}







/// The user is a member of a chat, without any additional privileges or restrictions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMemberStatusMember {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for ChatMemberStatusMember {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatMemberStatusMember" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatMemberStatus for ChatMemberStatusMember {}



impl ChatMemberStatusMember {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatMemberStatusMemberBuilder {
    let mut inner = ChatMemberStatusMember::default();
    inner.td_name = "chatMemberStatusMember".to_string();
    RTDChatMemberStatusMemberBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatMemberStatusMemberBuilder {
  inner: ChatMemberStatusMember
}

impl RTDChatMemberStatusMemberBuilder {
  pub fn build(&self) -> ChatMemberStatusMember { self.inner.clone() }

}

impl AsRef<ChatMemberStatusMember> for ChatMemberStatusMember {
  fn as_ref(&self) -> &ChatMemberStatusMember { self }
}

impl AsRef<ChatMemberStatusMember> for RTDChatMemberStatusMemberBuilder {
  fn as_ref(&self) -> &ChatMemberStatusMember { &self.inner }
}







/// The user is under certain restrictions in the chat. Not supported in basic groups and channels
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMemberStatusRestricted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// True, if the user is a member of the chat
  is_member: bool,
  /// Point in time (Unix timestamp) when restrictions will be lifted from the user; 0 if never. If the user is restricted for more than 366 days or for less than 30 seconds from the current time, the user is considered to be restricted forever
  restricted_until_date: i64,
  /// True, if the user can send text messages, contacts, locations, and venues
  can_send_messages: bool,
  /// True, if the user can send audio files, documents, photos, videos, video notes, and voice notes. Implies can_send_messages permissions
  can_send_media_messages: bool,
  /// True, if the user can send animations, games, and stickers and use inline bots. Implies can_send_media_messages permissions
  can_send_other_messages: bool,
  /// True, if the user may add a web page preview to his messages. Implies can_send_messages permissions
  can_add_web_page_previews: bool,
  
}

impl RObject for ChatMemberStatusRestricted {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatMemberStatusRestricted" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatMemberStatus for ChatMemberStatusRestricted {}



impl ChatMemberStatusRestricted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatMemberStatusRestrictedBuilder {
    let mut inner = ChatMemberStatusRestricted::default();
    inner.td_name = "chatMemberStatusRestricted".to_string();
    RTDChatMemberStatusRestrictedBuilder { inner }
  }

  pub fn is_member(&self) -> bool { self.is_member }

  pub fn restricted_until_date(&self) -> i64 { self.restricted_until_date }

  pub fn can_send_messages(&self) -> bool { self.can_send_messages }

  pub fn can_send_media_messages(&self) -> bool { self.can_send_media_messages }

  pub fn can_send_other_messages(&self) -> bool { self.can_send_other_messages }

  pub fn can_add_web_page_previews(&self) -> bool { self.can_add_web_page_previews }

}

#[doc(hidden)]
pub struct RTDChatMemberStatusRestrictedBuilder {
  inner: ChatMemberStatusRestricted
}

impl RTDChatMemberStatusRestrictedBuilder {
  pub fn build(&self) -> ChatMemberStatusRestricted { self.inner.clone() }

   
  pub fn is_member(&mut self, is_member: bool) -> &mut Self {
    self.inner.is_member = is_member;
    self
  }

   
  pub fn restricted_until_date(&mut self, restricted_until_date: i64) -> &mut Self {
    self.inner.restricted_until_date = restricted_until_date;
    self
  }

   
  pub fn can_send_messages(&mut self, can_send_messages: bool) -> &mut Self {
    self.inner.can_send_messages = can_send_messages;
    self
  }

   
  pub fn can_send_media_messages(&mut self, can_send_media_messages: bool) -> &mut Self {
    self.inner.can_send_media_messages = can_send_media_messages;
    self
  }

   
  pub fn can_send_other_messages(&mut self, can_send_other_messages: bool) -> &mut Self {
    self.inner.can_send_other_messages = can_send_other_messages;
    self
  }

   
  pub fn can_add_web_page_previews(&mut self, can_add_web_page_previews: bool) -> &mut Self {
    self.inner.can_add_web_page_previews = can_add_web_page_previews;
    self
  }

}

impl AsRef<ChatMemberStatusRestricted> for ChatMemberStatusRestricted {
  fn as_ref(&self) -> &ChatMemberStatusRestricted { self }
}

impl AsRef<ChatMemberStatusRestricted> for RTDChatMemberStatusRestrictedBuilder {
  fn as_ref(&self) -> &ChatMemberStatusRestricted { &self.inner }
}







/// The user is not a chat member
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMemberStatusLeft {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for ChatMemberStatusLeft {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatMemberStatusLeft" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatMemberStatus for ChatMemberStatusLeft {}



impl ChatMemberStatusLeft {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatMemberStatusLeftBuilder {
    let mut inner = ChatMemberStatusLeft::default();
    inner.td_name = "chatMemberStatusLeft".to_string();
    RTDChatMemberStatusLeftBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatMemberStatusLeftBuilder {
  inner: ChatMemberStatusLeft
}

impl RTDChatMemberStatusLeftBuilder {
  pub fn build(&self) -> ChatMemberStatusLeft { self.inner.clone() }

}

impl AsRef<ChatMemberStatusLeft> for ChatMemberStatusLeft {
  fn as_ref(&self) -> &ChatMemberStatusLeft { self }
}

impl AsRef<ChatMemberStatusLeft> for RTDChatMemberStatusLeftBuilder {
  fn as_ref(&self) -> &ChatMemberStatusLeft { &self.inner }
}







/// The user was banned (and hence is not a member of the chat). Implies the user can't return to the chat or view messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMemberStatusBanned {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Point in time (Unix timestamp) when the user will be unbanned; 0 if never. If the user is banned for more than 366 days or for less than 30 seconds from the current time, the user is considered to be banned forever
  banned_until_date: i64,
  
}

impl RObject for ChatMemberStatusBanned {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatMemberStatusBanned" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatMemberStatus for ChatMemberStatusBanned {}



impl ChatMemberStatusBanned {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatMemberStatusBannedBuilder {
    let mut inner = ChatMemberStatusBanned::default();
    inner.td_name = "chatMemberStatusBanned".to_string();
    RTDChatMemberStatusBannedBuilder { inner }
  }

  pub fn banned_until_date(&self) -> i64 { self.banned_until_date }

}

#[doc(hidden)]
pub struct RTDChatMemberStatusBannedBuilder {
  inner: ChatMemberStatusBanned
}

impl RTDChatMemberStatusBannedBuilder {
  pub fn build(&self) -> ChatMemberStatusBanned { self.inner.clone() }

   
  pub fn banned_until_date(&mut self, banned_until_date: i64) -> &mut Self {
    self.inner.banned_until_date = banned_until_date;
    self
  }

}

impl AsRef<ChatMemberStatusBanned> for ChatMemberStatusBanned {
  fn as_ref(&self) -> &ChatMemberStatusBanned { self }
}

impl AsRef<ChatMemberStatusBanned> for RTDChatMemberStatusBannedBuilder {
  fn as_ref(&self) -> &ChatMemberStatusBanned { &self.inner }
}



