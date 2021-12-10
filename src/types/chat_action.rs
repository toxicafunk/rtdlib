
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes the different types of activity in a chat
pub trait TDChatAction: Debug + RObject {}

/// Describes the different types of activity in a chat
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ChatAction {
  #[doc(hidden)] _Default(()),
  /// The user has canceled the previous action
  Cancel(ChatActionCancel),
  /// The user is picking a contact to send
  ChoosingContact(ChatActionChoosingContact),
  /// The user is picking a location or venue to send
  ChoosingLocation(ChatActionChoosingLocation),
  /// The user is picking a sticker to send
  ChoosingSticker(ChatActionChoosingSticker),
  /// The user is recording a video
  RecordingVideo(ChatActionRecordingVideo),
  /// The user is recording a video note
  RecordingVideoNote(ChatActionRecordingVideoNote),
  /// The user is recording a voice note
  RecordingVoiceNote(ChatActionRecordingVoiceNote),
  /// The user has started to play a game
  StartPlayingGame(ChatActionStartPlayingGame),
  /// The user is typing a message
  Typing(ChatActionTyping),
  /// The user is uploading a document
  UploadingDocument(ChatActionUploadingDocument),
  /// The user is uploading a photo
  UploadingPhoto(ChatActionUploadingPhoto),
  /// The user is uploading a video
  UploadingVideo(ChatActionUploadingVideo),
  /// The user is uploading a video note
  UploadingVideoNote(ChatActionUploadingVideoNote),
  /// The user is uploading a voice note
  UploadingVoiceNote(ChatActionUploadingVoiceNote),
  /// The user is watching animations sent by the other party by clicking on an animated emoji
  WatchingAnimations(ChatActionWatchingAnimations),

}

impl Default for ChatAction {
  fn default() -> Self { ChatAction::_Default(()) }
}

impl<'de> Deserialize<'de> for ChatAction {
  fn deserialize<D>(deserializer: D) -> Result<ChatAction, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ChatAction,
      (chatActionCancel, Cancel);
      (chatActionChoosingContact, ChoosingContact);
      (chatActionChoosingLocation, ChoosingLocation);
      (chatActionChoosingSticker, ChoosingSticker);
      (chatActionRecordingVideo, RecordingVideo);
      (chatActionRecordingVideoNote, RecordingVideoNote);
      (chatActionRecordingVoiceNote, RecordingVoiceNote);
      (chatActionStartPlayingGame, StartPlayingGame);
      (chatActionTyping, Typing);
      (chatActionUploadingDocument, UploadingDocument);
      (chatActionUploadingPhoto, UploadingPhoto);
      (chatActionUploadingVideo, UploadingVideo);
      (chatActionUploadingVideoNote, UploadingVideoNote);
      (chatActionUploadingVoiceNote, UploadingVoiceNote);
      (chatActionWatchingAnimations, WatchingAnimations);

    )(deserializer)
  }
}

impl RObject for ChatAction {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      ChatAction::Cancel(t) => t.td_name(),
      ChatAction::ChoosingContact(t) => t.td_name(),
      ChatAction::ChoosingLocation(t) => t.td_name(),
      ChatAction::ChoosingSticker(t) => t.td_name(),
      ChatAction::RecordingVideo(t) => t.td_name(),
      ChatAction::RecordingVideoNote(t) => t.td_name(),
      ChatAction::RecordingVoiceNote(t) => t.td_name(),
      ChatAction::StartPlayingGame(t) => t.td_name(),
      ChatAction::Typing(t) => t.td_name(),
      ChatAction::UploadingDocument(t) => t.td_name(),
      ChatAction::UploadingPhoto(t) => t.td_name(),
      ChatAction::UploadingVideo(t) => t.td_name(),
      ChatAction::UploadingVideoNote(t) => t.td_name(),
      ChatAction::UploadingVoiceNote(t) => t.td_name(),
      ChatAction::WatchingAnimations(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      ChatAction::Cancel(t) => t.extra(),
      ChatAction::ChoosingContact(t) => t.extra(),
      ChatAction::ChoosingLocation(t) => t.extra(),
      ChatAction::ChoosingSticker(t) => t.extra(),
      ChatAction::RecordingVideo(t) => t.extra(),
      ChatAction::RecordingVideoNote(t) => t.extra(),
      ChatAction::RecordingVoiceNote(t) => t.extra(),
      ChatAction::StartPlayingGame(t) => t.extra(),
      ChatAction::Typing(t) => t.extra(),
      ChatAction::UploadingDocument(t) => t.extra(),
      ChatAction::UploadingPhoto(t) => t.extra(),
      ChatAction::UploadingVideo(t) => t.extra(),
      ChatAction::UploadingVideoNote(t) => t.extra(),
      ChatAction::UploadingVoiceNote(t) => t.extra(),
      ChatAction::WatchingAnimations(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl ChatAction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let ChatAction::_Default(_) = self { true } else { false } }

  pub fn is_cancel(&self) -> bool { if let ChatAction::Cancel(_) = self { true } else { false } }
  pub fn is_choosing_contact(&self) -> bool { if let ChatAction::ChoosingContact(_) = self { true } else { false } }
  pub fn is_choosing_location(&self) -> bool { if let ChatAction::ChoosingLocation(_) = self { true } else { false } }
  pub fn is_choosing_sticker(&self) -> bool { if let ChatAction::ChoosingSticker(_) = self { true } else { false } }
  pub fn is_recording_video(&self) -> bool { if let ChatAction::RecordingVideo(_) = self { true } else { false } }
  pub fn is_recording_video_note(&self) -> bool { if let ChatAction::RecordingVideoNote(_) = self { true } else { false } }
  pub fn is_recording_voice_note(&self) -> bool { if let ChatAction::RecordingVoiceNote(_) = self { true } else { false } }
  pub fn is_start_playing_game(&self) -> bool { if let ChatAction::StartPlayingGame(_) = self { true } else { false } }
  pub fn is_typing(&self) -> bool { if let ChatAction::Typing(_) = self { true } else { false } }
  pub fn is_uploading_document(&self) -> bool { if let ChatAction::UploadingDocument(_) = self { true } else { false } }
  pub fn is_uploading_photo(&self) -> bool { if let ChatAction::UploadingPhoto(_) = self { true } else { false } }
  pub fn is_uploading_video(&self) -> bool { if let ChatAction::UploadingVideo(_) = self { true } else { false } }
  pub fn is_uploading_video_note(&self) -> bool { if let ChatAction::UploadingVideoNote(_) = self { true } else { false } }
  pub fn is_uploading_voice_note(&self) -> bool { if let ChatAction::UploadingVoiceNote(_) = self { true } else { false } }
  pub fn is_watching_animations(&self) -> bool { if let ChatAction::WatchingAnimations(_) = self { true } else { false } }

  pub fn on_cancel<F: FnOnce(&ChatActionCancel)>(&self, fnc: F) -> &Self { if let ChatAction::Cancel(t) = self { fnc(t) }; self }
  pub fn on_choosing_contact<F: FnOnce(&ChatActionChoosingContact)>(&self, fnc: F) -> &Self { if let ChatAction::ChoosingContact(t) = self { fnc(t) }; self }
  pub fn on_choosing_location<F: FnOnce(&ChatActionChoosingLocation)>(&self, fnc: F) -> &Self { if let ChatAction::ChoosingLocation(t) = self { fnc(t) }; self }
  pub fn on_choosing_sticker<F: FnOnce(&ChatActionChoosingSticker)>(&self, fnc: F) -> &Self { if let ChatAction::ChoosingSticker(t) = self { fnc(t) }; self }
  pub fn on_recording_video<F: FnOnce(&ChatActionRecordingVideo)>(&self, fnc: F) -> &Self { if let ChatAction::RecordingVideo(t) = self { fnc(t) }; self }
  pub fn on_recording_video_note<F: FnOnce(&ChatActionRecordingVideoNote)>(&self, fnc: F) -> &Self { if let ChatAction::RecordingVideoNote(t) = self { fnc(t) }; self }
  pub fn on_recording_voice_note<F: FnOnce(&ChatActionRecordingVoiceNote)>(&self, fnc: F) -> &Self { if let ChatAction::RecordingVoiceNote(t) = self { fnc(t) }; self }
  pub fn on_start_playing_game<F: FnOnce(&ChatActionStartPlayingGame)>(&self, fnc: F) -> &Self { if let ChatAction::StartPlayingGame(t) = self { fnc(t) }; self }
  pub fn on_typing<F: FnOnce(&ChatActionTyping)>(&self, fnc: F) -> &Self { if let ChatAction::Typing(t) = self { fnc(t) }; self }
  pub fn on_uploading_document<F: FnOnce(&ChatActionUploadingDocument)>(&self, fnc: F) -> &Self { if let ChatAction::UploadingDocument(t) = self { fnc(t) }; self }
  pub fn on_uploading_photo<F: FnOnce(&ChatActionUploadingPhoto)>(&self, fnc: F) -> &Self { if let ChatAction::UploadingPhoto(t) = self { fnc(t) }; self }
  pub fn on_uploading_video<F: FnOnce(&ChatActionUploadingVideo)>(&self, fnc: F) -> &Self { if let ChatAction::UploadingVideo(t) = self { fnc(t) }; self }
  pub fn on_uploading_video_note<F: FnOnce(&ChatActionUploadingVideoNote)>(&self, fnc: F) -> &Self { if let ChatAction::UploadingVideoNote(t) = self { fnc(t) }; self }
  pub fn on_uploading_voice_note<F: FnOnce(&ChatActionUploadingVoiceNote)>(&self, fnc: F) -> &Self { if let ChatAction::UploadingVoiceNote(t) = self { fnc(t) }; self }
  pub fn on_watching_animations<F: FnOnce(&ChatActionWatchingAnimations)>(&self, fnc: F) -> &Self { if let ChatAction::WatchingAnimations(t) = self { fnc(t) }; self }

  pub fn as_cancel(&self) -> Option<&ChatActionCancel> { if let ChatAction::Cancel(t) = self { return Some(t) } None }
  pub fn as_choosing_contact(&self) -> Option<&ChatActionChoosingContact> { if let ChatAction::ChoosingContact(t) = self { return Some(t) } None }
  pub fn as_choosing_location(&self) -> Option<&ChatActionChoosingLocation> { if let ChatAction::ChoosingLocation(t) = self { return Some(t) } None }
  pub fn as_choosing_sticker(&self) -> Option<&ChatActionChoosingSticker> { if let ChatAction::ChoosingSticker(t) = self { return Some(t) } None }
  pub fn as_recording_video(&self) -> Option<&ChatActionRecordingVideo> { if let ChatAction::RecordingVideo(t) = self { return Some(t) } None }
  pub fn as_recording_video_note(&self) -> Option<&ChatActionRecordingVideoNote> { if let ChatAction::RecordingVideoNote(t) = self { return Some(t) } None }
  pub fn as_recording_voice_note(&self) -> Option<&ChatActionRecordingVoiceNote> { if let ChatAction::RecordingVoiceNote(t) = self { return Some(t) } None }
  pub fn as_start_playing_game(&self) -> Option<&ChatActionStartPlayingGame> { if let ChatAction::StartPlayingGame(t) = self { return Some(t) } None }
  pub fn as_typing(&self) -> Option<&ChatActionTyping> { if let ChatAction::Typing(t) = self { return Some(t) } None }
  pub fn as_uploading_document(&self) -> Option<&ChatActionUploadingDocument> { if let ChatAction::UploadingDocument(t) = self { return Some(t) } None }
  pub fn as_uploading_photo(&self) -> Option<&ChatActionUploadingPhoto> { if let ChatAction::UploadingPhoto(t) = self { return Some(t) } None }
  pub fn as_uploading_video(&self) -> Option<&ChatActionUploadingVideo> { if let ChatAction::UploadingVideo(t) = self { return Some(t) } None }
  pub fn as_uploading_video_note(&self) -> Option<&ChatActionUploadingVideoNote> { if let ChatAction::UploadingVideoNote(t) = self { return Some(t) } None }
  pub fn as_uploading_voice_note(&self) -> Option<&ChatActionUploadingVoiceNote> { if let ChatAction::UploadingVoiceNote(t) = self { return Some(t) } None }
  pub fn as_watching_animations(&self) -> Option<&ChatActionWatchingAnimations> { if let ChatAction::WatchingAnimations(t) = self { return Some(t) } None }



  pub fn cancel<T: AsRef<ChatActionCancel>>(t: T) -> Self { ChatAction::Cancel(t.as_ref().clone()) }

  pub fn choosing_contact<T: AsRef<ChatActionChoosingContact>>(t: T) -> Self { ChatAction::ChoosingContact(t.as_ref().clone()) }

  pub fn choosing_location<T: AsRef<ChatActionChoosingLocation>>(t: T) -> Self { ChatAction::ChoosingLocation(t.as_ref().clone()) }

  pub fn choosing_sticker<T: AsRef<ChatActionChoosingSticker>>(t: T) -> Self { ChatAction::ChoosingSticker(t.as_ref().clone()) }

  pub fn recording_video<T: AsRef<ChatActionRecordingVideo>>(t: T) -> Self { ChatAction::RecordingVideo(t.as_ref().clone()) }

  pub fn recording_video_note<T: AsRef<ChatActionRecordingVideoNote>>(t: T) -> Self { ChatAction::RecordingVideoNote(t.as_ref().clone()) }

  pub fn recording_voice_note<T: AsRef<ChatActionRecordingVoiceNote>>(t: T) -> Self { ChatAction::RecordingVoiceNote(t.as_ref().clone()) }

  pub fn start_playing_game<T: AsRef<ChatActionStartPlayingGame>>(t: T) -> Self { ChatAction::StartPlayingGame(t.as_ref().clone()) }

  pub fn typing<T: AsRef<ChatActionTyping>>(t: T) -> Self { ChatAction::Typing(t.as_ref().clone()) }

  pub fn uploading_document<T: AsRef<ChatActionUploadingDocument>>(t: T) -> Self { ChatAction::UploadingDocument(t.as_ref().clone()) }

  pub fn uploading_photo<T: AsRef<ChatActionUploadingPhoto>>(t: T) -> Self { ChatAction::UploadingPhoto(t.as_ref().clone()) }

  pub fn uploading_video<T: AsRef<ChatActionUploadingVideo>>(t: T) -> Self { ChatAction::UploadingVideo(t.as_ref().clone()) }

  pub fn uploading_video_note<T: AsRef<ChatActionUploadingVideoNote>>(t: T) -> Self { ChatAction::UploadingVideoNote(t.as_ref().clone()) }

  pub fn uploading_voice_note<T: AsRef<ChatActionUploadingVoiceNote>>(t: T) -> Self { ChatAction::UploadingVoiceNote(t.as_ref().clone()) }

  pub fn watching_animations<T: AsRef<ChatActionWatchingAnimations>>(t: T) -> Self { ChatAction::WatchingAnimations(t.as_ref().clone()) }

}

impl AsRef<ChatAction> for ChatAction {
  fn as_ref(&self) -> &ChatAction { self }
}







/// The user has canceled the previous action
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionCancel {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ChatActionCancel {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionCancel" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatAction for ChatActionCancel {}



impl ChatActionCancel {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatActionCancelBuilder {
    let mut inner = ChatActionCancel::default();
    inner.td_name = "chatActionCancel".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatActionCancelBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatActionCancelBuilder {
  inner: ChatActionCancel
}

impl RTDChatActionCancelBuilder {
  pub fn build(&self) -> ChatActionCancel { self.inner.clone() }

}

impl AsRef<ChatActionCancel> for ChatActionCancel {
  fn as_ref(&self) -> &ChatActionCancel { self }
}

impl AsRef<ChatActionCancel> for RTDChatActionCancelBuilder {
  fn as_ref(&self) -> &ChatActionCancel { &self.inner }
}







/// The user is picking a contact to send
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionChoosingContact {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ChatActionChoosingContact {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionChoosingContact" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatAction for ChatActionChoosingContact {}



impl ChatActionChoosingContact {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatActionChoosingContactBuilder {
    let mut inner = ChatActionChoosingContact::default();
    inner.td_name = "chatActionChoosingContact".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatActionChoosingContactBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatActionChoosingContactBuilder {
  inner: ChatActionChoosingContact
}

impl RTDChatActionChoosingContactBuilder {
  pub fn build(&self) -> ChatActionChoosingContact { self.inner.clone() }

}

impl AsRef<ChatActionChoosingContact> for ChatActionChoosingContact {
  fn as_ref(&self) -> &ChatActionChoosingContact { self }
}

impl AsRef<ChatActionChoosingContact> for RTDChatActionChoosingContactBuilder {
  fn as_ref(&self) -> &ChatActionChoosingContact { &self.inner }
}







/// The user is picking a location or venue to send
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionChoosingLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ChatActionChoosingLocation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionChoosingLocation" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatAction for ChatActionChoosingLocation {}



impl ChatActionChoosingLocation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatActionChoosingLocationBuilder {
    let mut inner = ChatActionChoosingLocation::default();
    inner.td_name = "chatActionChoosingLocation".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatActionChoosingLocationBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatActionChoosingLocationBuilder {
  inner: ChatActionChoosingLocation
}

impl RTDChatActionChoosingLocationBuilder {
  pub fn build(&self) -> ChatActionChoosingLocation { self.inner.clone() }

}

impl AsRef<ChatActionChoosingLocation> for ChatActionChoosingLocation {
  fn as_ref(&self) -> &ChatActionChoosingLocation { self }
}

impl AsRef<ChatActionChoosingLocation> for RTDChatActionChoosingLocationBuilder {
  fn as_ref(&self) -> &ChatActionChoosingLocation { &self.inner }
}







/// The user is picking a sticker to send
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionChoosingSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ChatActionChoosingSticker {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionChoosingSticker" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatAction for ChatActionChoosingSticker {}



impl ChatActionChoosingSticker {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatActionChoosingStickerBuilder {
    let mut inner = ChatActionChoosingSticker::default();
    inner.td_name = "chatActionChoosingSticker".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatActionChoosingStickerBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatActionChoosingStickerBuilder {
  inner: ChatActionChoosingSticker
}

impl RTDChatActionChoosingStickerBuilder {
  pub fn build(&self) -> ChatActionChoosingSticker { self.inner.clone() }

}

impl AsRef<ChatActionChoosingSticker> for ChatActionChoosingSticker {
  fn as_ref(&self) -> &ChatActionChoosingSticker { self }
}

impl AsRef<ChatActionChoosingSticker> for RTDChatActionChoosingStickerBuilder {
  fn as_ref(&self) -> &ChatActionChoosingSticker { &self.inner }
}







/// The user is recording a video
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionRecordingVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ChatActionRecordingVideo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionRecordingVideo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatAction for ChatActionRecordingVideo {}



impl ChatActionRecordingVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatActionRecordingVideoBuilder {
    let mut inner = ChatActionRecordingVideo::default();
    inner.td_name = "chatActionRecordingVideo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatActionRecordingVideoBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatActionRecordingVideoBuilder {
  inner: ChatActionRecordingVideo
}

impl RTDChatActionRecordingVideoBuilder {
  pub fn build(&self) -> ChatActionRecordingVideo { self.inner.clone() }

}

impl AsRef<ChatActionRecordingVideo> for ChatActionRecordingVideo {
  fn as_ref(&self) -> &ChatActionRecordingVideo { self }
}

impl AsRef<ChatActionRecordingVideo> for RTDChatActionRecordingVideoBuilder {
  fn as_ref(&self) -> &ChatActionRecordingVideo { &self.inner }
}







/// The user is recording a video note
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionRecordingVideoNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ChatActionRecordingVideoNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionRecordingVideoNote" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatAction for ChatActionRecordingVideoNote {}



impl ChatActionRecordingVideoNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatActionRecordingVideoNoteBuilder {
    let mut inner = ChatActionRecordingVideoNote::default();
    inner.td_name = "chatActionRecordingVideoNote".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatActionRecordingVideoNoteBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatActionRecordingVideoNoteBuilder {
  inner: ChatActionRecordingVideoNote
}

impl RTDChatActionRecordingVideoNoteBuilder {
  pub fn build(&self) -> ChatActionRecordingVideoNote { self.inner.clone() }

}

impl AsRef<ChatActionRecordingVideoNote> for ChatActionRecordingVideoNote {
  fn as_ref(&self) -> &ChatActionRecordingVideoNote { self }
}

impl AsRef<ChatActionRecordingVideoNote> for RTDChatActionRecordingVideoNoteBuilder {
  fn as_ref(&self) -> &ChatActionRecordingVideoNote { &self.inner }
}







/// The user is recording a voice note
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionRecordingVoiceNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ChatActionRecordingVoiceNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionRecordingVoiceNote" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatAction for ChatActionRecordingVoiceNote {}



impl ChatActionRecordingVoiceNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatActionRecordingVoiceNoteBuilder {
    let mut inner = ChatActionRecordingVoiceNote::default();
    inner.td_name = "chatActionRecordingVoiceNote".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatActionRecordingVoiceNoteBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatActionRecordingVoiceNoteBuilder {
  inner: ChatActionRecordingVoiceNote
}

impl RTDChatActionRecordingVoiceNoteBuilder {
  pub fn build(&self) -> ChatActionRecordingVoiceNote { self.inner.clone() }

}

impl AsRef<ChatActionRecordingVoiceNote> for ChatActionRecordingVoiceNote {
  fn as_ref(&self) -> &ChatActionRecordingVoiceNote { self }
}

impl AsRef<ChatActionRecordingVoiceNote> for RTDChatActionRecordingVoiceNoteBuilder {
  fn as_ref(&self) -> &ChatActionRecordingVoiceNote { &self.inner }
}







/// The user has started to play a game
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionStartPlayingGame {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ChatActionStartPlayingGame {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionStartPlayingGame" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatAction for ChatActionStartPlayingGame {}



impl ChatActionStartPlayingGame {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatActionStartPlayingGameBuilder {
    let mut inner = ChatActionStartPlayingGame::default();
    inner.td_name = "chatActionStartPlayingGame".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatActionStartPlayingGameBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatActionStartPlayingGameBuilder {
  inner: ChatActionStartPlayingGame
}

impl RTDChatActionStartPlayingGameBuilder {
  pub fn build(&self) -> ChatActionStartPlayingGame { self.inner.clone() }

}

impl AsRef<ChatActionStartPlayingGame> for ChatActionStartPlayingGame {
  fn as_ref(&self) -> &ChatActionStartPlayingGame { self }
}

impl AsRef<ChatActionStartPlayingGame> for RTDChatActionStartPlayingGameBuilder {
  fn as_ref(&self) -> &ChatActionStartPlayingGame { &self.inner }
}







/// The user is typing a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionTyping {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for ChatActionTyping {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionTyping" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatAction for ChatActionTyping {}



impl ChatActionTyping {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatActionTypingBuilder {
    let mut inner = ChatActionTyping::default();
    inner.td_name = "chatActionTyping".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatActionTypingBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatActionTypingBuilder {
  inner: ChatActionTyping
}

impl RTDChatActionTypingBuilder {
  pub fn build(&self) -> ChatActionTyping { self.inner.clone() }

}

impl AsRef<ChatActionTyping> for ChatActionTyping {
  fn as_ref(&self) -> &ChatActionTyping { self }
}

impl AsRef<ChatActionTyping> for RTDChatActionTypingBuilder {
  fn as_ref(&self) -> &ChatActionTyping { &self.inner }
}







/// The user is uploading a document
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionUploadingDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Upload progress, as a percentage
  progress: i64,
  
}

impl RObject for ChatActionUploadingDocument {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionUploadingDocument" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatAction for ChatActionUploadingDocument {}



impl ChatActionUploadingDocument {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatActionUploadingDocumentBuilder {
    let mut inner = ChatActionUploadingDocument::default();
    inner.td_name = "chatActionUploadingDocument".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatActionUploadingDocumentBuilder { inner }
  }

  pub fn progress(&self) -> i64 { self.progress }

}

#[doc(hidden)]
pub struct RTDChatActionUploadingDocumentBuilder {
  inner: ChatActionUploadingDocument
}

impl RTDChatActionUploadingDocumentBuilder {
  pub fn build(&self) -> ChatActionUploadingDocument { self.inner.clone() }

   
  pub fn progress(&mut self, progress: i64) -> &mut Self {
    self.inner.progress = progress;
    self
  }

}

impl AsRef<ChatActionUploadingDocument> for ChatActionUploadingDocument {
  fn as_ref(&self) -> &ChatActionUploadingDocument { self }
}

impl AsRef<ChatActionUploadingDocument> for RTDChatActionUploadingDocumentBuilder {
  fn as_ref(&self) -> &ChatActionUploadingDocument { &self.inner }
}







/// The user is uploading a photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionUploadingPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Upload progress, as a percentage
  progress: i64,
  
}

impl RObject for ChatActionUploadingPhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionUploadingPhoto" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatAction for ChatActionUploadingPhoto {}



impl ChatActionUploadingPhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatActionUploadingPhotoBuilder {
    let mut inner = ChatActionUploadingPhoto::default();
    inner.td_name = "chatActionUploadingPhoto".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatActionUploadingPhotoBuilder { inner }
  }

  pub fn progress(&self) -> i64 { self.progress }

}

#[doc(hidden)]
pub struct RTDChatActionUploadingPhotoBuilder {
  inner: ChatActionUploadingPhoto
}

impl RTDChatActionUploadingPhotoBuilder {
  pub fn build(&self) -> ChatActionUploadingPhoto { self.inner.clone() }

   
  pub fn progress(&mut self, progress: i64) -> &mut Self {
    self.inner.progress = progress;
    self
  }

}

impl AsRef<ChatActionUploadingPhoto> for ChatActionUploadingPhoto {
  fn as_ref(&self) -> &ChatActionUploadingPhoto { self }
}

impl AsRef<ChatActionUploadingPhoto> for RTDChatActionUploadingPhotoBuilder {
  fn as_ref(&self) -> &ChatActionUploadingPhoto { &self.inner }
}







/// The user is uploading a video
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionUploadingVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Upload progress, as a percentage
  progress: i64,
  
}

impl RObject for ChatActionUploadingVideo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionUploadingVideo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatAction for ChatActionUploadingVideo {}



impl ChatActionUploadingVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatActionUploadingVideoBuilder {
    let mut inner = ChatActionUploadingVideo::default();
    inner.td_name = "chatActionUploadingVideo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatActionUploadingVideoBuilder { inner }
  }

  pub fn progress(&self) -> i64 { self.progress }

}

#[doc(hidden)]
pub struct RTDChatActionUploadingVideoBuilder {
  inner: ChatActionUploadingVideo
}

impl RTDChatActionUploadingVideoBuilder {
  pub fn build(&self) -> ChatActionUploadingVideo { self.inner.clone() }

   
  pub fn progress(&mut self, progress: i64) -> &mut Self {
    self.inner.progress = progress;
    self
  }

}

impl AsRef<ChatActionUploadingVideo> for ChatActionUploadingVideo {
  fn as_ref(&self) -> &ChatActionUploadingVideo { self }
}

impl AsRef<ChatActionUploadingVideo> for RTDChatActionUploadingVideoBuilder {
  fn as_ref(&self) -> &ChatActionUploadingVideo { &self.inner }
}







/// The user is uploading a video note
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionUploadingVideoNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Upload progress, as a percentage
  progress: i64,
  
}

impl RObject for ChatActionUploadingVideoNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionUploadingVideoNote" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatAction for ChatActionUploadingVideoNote {}



impl ChatActionUploadingVideoNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatActionUploadingVideoNoteBuilder {
    let mut inner = ChatActionUploadingVideoNote::default();
    inner.td_name = "chatActionUploadingVideoNote".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatActionUploadingVideoNoteBuilder { inner }
  }

  pub fn progress(&self) -> i64 { self.progress }

}

#[doc(hidden)]
pub struct RTDChatActionUploadingVideoNoteBuilder {
  inner: ChatActionUploadingVideoNote
}

impl RTDChatActionUploadingVideoNoteBuilder {
  pub fn build(&self) -> ChatActionUploadingVideoNote { self.inner.clone() }

   
  pub fn progress(&mut self, progress: i64) -> &mut Self {
    self.inner.progress = progress;
    self
  }

}

impl AsRef<ChatActionUploadingVideoNote> for ChatActionUploadingVideoNote {
  fn as_ref(&self) -> &ChatActionUploadingVideoNote { self }
}

impl AsRef<ChatActionUploadingVideoNote> for RTDChatActionUploadingVideoNoteBuilder {
  fn as_ref(&self) -> &ChatActionUploadingVideoNote { &self.inner }
}







/// The user is uploading a voice note
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionUploadingVoiceNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Upload progress, as a percentage
  progress: i64,
  
}

impl RObject for ChatActionUploadingVoiceNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionUploadingVoiceNote" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatAction for ChatActionUploadingVoiceNote {}



impl ChatActionUploadingVoiceNote {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatActionUploadingVoiceNoteBuilder {
    let mut inner = ChatActionUploadingVoiceNote::default();
    inner.td_name = "chatActionUploadingVoiceNote".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatActionUploadingVoiceNoteBuilder { inner }
  }

  pub fn progress(&self) -> i64 { self.progress }

}

#[doc(hidden)]
pub struct RTDChatActionUploadingVoiceNoteBuilder {
  inner: ChatActionUploadingVoiceNote
}

impl RTDChatActionUploadingVoiceNoteBuilder {
  pub fn build(&self) -> ChatActionUploadingVoiceNote { self.inner.clone() }

   
  pub fn progress(&mut self, progress: i64) -> &mut Self {
    self.inner.progress = progress;
    self
  }

}

impl AsRef<ChatActionUploadingVoiceNote> for ChatActionUploadingVoiceNote {
  fn as_ref(&self) -> &ChatActionUploadingVoiceNote { self }
}

impl AsRef<ChatActionUploadingVoiceNote> for RTDChatActionUploadingVoiceNoteBuilder {
  fn as_ref(&self) -> &ChatActionUploadingVoiceNote { &self.inner }
}







/// The user is watching animations sent by the other party by clicking on an animated emoji
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatActionWatchingAnimations {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The animated emoji
  emoji: String,
  
}

impl RObject for ChatActionWatchingAnimations {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionWatchingAnimations" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatAction for ChatActionWatchingAnimations {}



impl ChatActionWatchingAnimations {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatActionWatchingAnimationsBuilder {
    let mut inner = ChatActionWatchingAnimations::default();
    inner.td_name = "chatActionWatchingAnimations".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatActionWatchingAnimationsBuilder { inner }
  }

  pub fn emoji(&self) -> &String { &self.emoji }

}

#[doc(hidden)]
pub struct RTDChatActionWatchingAnimationsBuilder {
  inner: ChatActionWatchingAnimations
}

impl RTDChatActionWatchingAnimationsBuilder {
  pub fn build(&self) -> ChatActionWatchingAnimations { self.inner.clone() }

   
  pub fn emoji<T: AsRef<str>>(&mut self, emoji: T) -> &mut Self {
    self.inner.emoji = emoji.as_ref().to_string();
    self
  }

}

impl AsRef<ChatActionWatchingAnimations> for ChatActionWatchingAnimations {
  fn as_ref(&self) -> &ChatActionWatchingAnimations { self }
}

impl AsRef<ChatActionWatchingAnimations> for RTDChatActionWatchingAnimationsBuilder {
  fn as_ref(&self) -> &ChatActionWatchingAnimations { &self.inner }
}



