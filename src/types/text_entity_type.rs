
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Represents a part of the text which must be formatted differently
pub trait TDTextEntityType: Debug + RObject {}

/// Represents a part of the text which must be formatted differently
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TextEntityType {
  #[doc(hidden)] _Default(()),
  /// A bank card number. The getBankCardInfo method can be used to get information about the bank card
  BankCardNumber(TextEntityTypeBankCardNumber),
  /// A bold text
  Bold(TextEntityTypeBold),
  /// A bot command, beginning with "/"
  BotCommand(TextEntityTypeBotCommand),
  /// A cashtag text, beginning with "$" and consisting of capital English letters (e.g., "$USD")
  Cashtag(TextEntityTypeCashtag),
  /// Text that must be formatted as if inside a code HTML tag
  Code(TextEntityTypeCode),
  /// An email address
  EmailAddress(TextEntityTypeEmailAddress),
  /// A hashtag text, beginning with "#"
  Hashtag(TextEntityTypeHashtag),
  /// An italic text
  Italic(TextEntityTypeItalic),
  /// A media timestamp
  MediaTimestamp(TextEntityTypeMediaTimestamp),
  /// A mention of a user by their username
  Mention(TextEntityTypeMention),
  /// A text shows instead of a raw mention of the user (e.g., when the user has no username)
  MentionName(TextEntityTypeMentionName),
  /// A phone number
  PhoneNumber(TextEntityTypePhoneNumber),
  /// Text that must be formatted as if inside a pre HTML tag
  Pre(TextEntityTypePre),
  /// Text that must be formatted as if inside pre, and code HTML tags
  PreCode(TextEntityTypePreCode),
  /// A strikethrough text
  Strikethrough(TextEntityTypeStrikethrough),
  /// A text description shown instead of a raw URL
  TextUrl(TextEntityTypeTextUrl),
  /// An underlined text
  Underline(TextEntityTypeUnderline),
  /// An HTTP URL
  Url(TextEntityTypeUrl),

}

impl Default for TextEntityType {
  fn default() -> Self { TextEntityType::_Default(()) }
}

impl<'de> Deserialize<'de> for TextEntityType {
  fn deserialize<D>(deserializer: D) -> Result<TextEntityType, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      TextEntityType,
      (textEntityTypeBankCardNumber, BankCardNumber);
      (textEntityTypeBold, Bold);
      (textEntityTypeBotCommand, BotCommand);
      (textEntityTypeCashtag, Cashtag);
      (textEntityTypeCode, Code);
      (textEntityTypeEmailAddress, EmailAddress);
      (textEntityTypeHashtag, Hashtag);
      (textEntityTypeItalic, Italic);
      (textEntityTypeMediaTimestamp, MediaTimestamp);
      (textEntityTypeMention, Mention);
      (textEntityTypeMentionName, MentionName);
      (textEntityTypePhoneNumber, PhoneNumber);
      (textEntityTypePre, Pre);
      (textEntityTypePreCode, PreCode);
      (textEntityTypeStrikethrough, Strikethrough);
      (textEntityTypeTextUrl, TextUrl);
      (textEntityTypeUnderline, Underline);
      (textEntityTypeUrl, Url);

    )(deserializer)
  }
}

impl RObject for TextEntityType {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      TextEntityType::BankCardNumber(t) => t.td_name(),
      TextEntityType::Bold(t) => t.td_name(),
      TextEntityType::BotCommand(t) => t.td_name(),
      TextEntityType::Cashtag(t) => t.td_name(),
      TextEntityType::Code(t) => t.td_name(),
      TextEntityType::EmailAddress(t) => t.td_name(),
      TextEntityType::Hashtag(t) => t.td_name(),
      TextEntityType::Italic(t) => t.td_name(),
      TextEntityType::MediaTimestamp(t) => t.td_name(),
      TextEntityType::Mention(t) => t.td_name(),
      TextEntityType::MentionName(t) => t.td_name(),
      TextEntityType::PhoneNumber(t) => t.td_name(),
      TextEntityType::Pre(t) => t.td_name(),
      TextEntityType::PreCode(t) => t.td_name(),
      TextEntityType::Strikethrough(t) => t.td_name(),
      TextEntityType::TextUrl(t) => t.td_name(),
      TextEntityType::Underline(t) => t.td_name(),
      TextEntityType::Url(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      TextEntityType::BankCardNumber(t) => t.extra(),
      TextEntityType::Bold(t) => t.extra(),
      TextEntityType::BotCommand(t) => t.extra(),
      TextEntityType::Cashtag(t) => t.extra(),
      TextEntityType::Code(t) => t.extra(),
      TextEntityType::EmailAddress(t) => t.extra(),
      TextEntityType::Hashtag(t) => t.extra(),
      TextEntityType::Italic(t) => t.extra(),
      TextEntityType::MediaTimestamp(t) => t.extra(),
      TextEntityType::Mention(t) => t.extra(),
      TextEntityType::MentionName(t) => t.extra(),
      TextEntityType::PhoneNumber(t) => t.extra(),
      TextEntityType::Pre(t) => t.extra(),
      TextEntityType::PreCode(t) => t.extra(),
      TextEntityType::Strikethrough(t) => t.extra(),
      TextEntityType::TextUrl(t) => t.extra(),
      TextEntityType::Underline(t) => t.extra(),
      TextEntityType::Url(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl TextEntityType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let TextEntityType::_Default(_) = self { true } else { false } }

  pub fn is_bank_card_number(&self) -> bool { if let TextEntityType::BankCardNumber(_) = self { true } else { false } }
  pub fn is_bold(&self) -> bool { if let TextEntityType::Bold(_) = self { true } else { false } }
  pub fn is_bot_command(&self) -> bool { if let TextEntityType::BotCommand(_) = self { true } else { false } }
  pub fn is_cashtag(&self) -> bool { if let TextEntityType::Cashtag(_) = self { true } else { false } }
  pub fn is_code(&self) -> bool { if let TextEntityType::Code(_) = self { true } else { false } }
  pub fn is_email_address(&self) -> bool { if let TextEntityType::EmailAddress(_) = self { true } else { false } }
  pub fn is_hashtag(&self) -> bool { if let TextEntityType::Hashtag(_) = self { true } else { false } }
  pub fn is_italic(&self) -> bool { if let TextEntityType::Italic(_) = self { true } else { false } }
  pub fn is_media_timestamp(&self) -> bool { if let TextEntityType::MediaTimestamp(_) = self { true } else { false } }
  pub fn is_mention(&self) -> bool { if let TextEntityType::Mention(_) = self { true } else { false } }
  pub fn is_mention_name(&self) -> bool { if let TextEntityType::MentionName(_) = self { true } else { false } }
  pub fn is_phone_number(&self) -> bool { if let TextEntityType::PhoneNumber(_) = self { true } else { false } }
  pub fn is_pre(&self) -> bool { if let TextEntityType::Pre(_) = self { true } else { false } }
  pub fn is_pre_code(&self) -> bool { if let TextEntityType::PreCode(_) = self { true } else { false } }
  pub fn is_strikethrough(&self) -> bool { if let TextEntityType::Strikethrough(_) = self { true } else { false } }
  pub fn is_text_url(&self) -> bool { if let TextEntityType::TextUrl(_) = self { true } else { false } }
  pub fn is_underline(&self) -> bool { if let TextEntityType::Underline(_) = self { true } else { false } }
  pub fn is_url(&self) -> bool { if let TextEntityType::Url(_) = self { true } else { false } }

  pub fn on_bank_card_number<F: FnOnce(&TextEntityTypeBankCardNumber)>(&self, fnc: F) -> &Self { if let TextEntityType::BankCardNumber(t) = self { fnc(t) }; self }
  pub fn on_bold<F: FnOnce(&TextEntityTypeBold)>(&self, fnc: F) -> &Self { if let TextEntityType::Bold(t) = self { fnc(t) }; self }
  pub fn on_bot_command<F: FnOnce(&TextEntityTypeBotCommand)>(&self, fnc: F) -> &Self { if let TextEntityType::BotCommand(t) = self { fnc(t) }; self }
  pub fn on_cashtag<F: FnOnce(&TextEntityTypeCashtag)>(&self, fnc: F) -> &Self { if let TextEntityType::Cashtag(t) = self { fnc(t) }; self }
  pub fn on_code<F: FnOnce(&TextEntityTypeCode)>(&self, fnc: F) -> &Self { if let TextEntityType::Code(t) = self { fnc(t) }; self }
  pub fn on_email_address<F: FnOnce(&TextEntityTypeEmailAddress)>(&self, fnc: F) -> &Self { if let TextEntityType::EmailAddress(t) = self { fnc(t) }; self }
  pub fn on_hashtag<F: FnOnce(&TextEntityTypeHashtag)>(&self, fnc: F) -> &Self { if let TextEntityType::Hashtag(t) = self { fnc(t) }; self }
  pub fn on_italic<F: FnOnce(&TextEntityTypeItalic)>(&self, fnc: F) -> &Self { if let TextEntityType::Italic(t) = self { fnc(t) }; self }
  pub fn on_media_timestamp<F: FnOnce(&TextEntityTypeMediaTimestamp)>(&self, fnc: F) -> &Self { if let TextEntityType::MediaTimestamp(t) = self { fnc(t) }; self }
  pub fn on_mention<F: FnOnce(&TextEntityTypeMention)>(&self, fnc: F) -> &Self { if let TextEntityType::Mention(t) = self { fnc(t) }; self }
  pub fn on_mention_name<F: FnOnce(&TextEntityTypeMentionName)>(&self, fnc: F) -> &Self { if let TextEntityType::MentionName(t) = self { fnc(t) }; self }
  pub fn on_phone_number<F: FnOnce(&TextEntityTypePhoneNumber)>(&self, fnc: F) -> &Self { if let TextEntityType::PhoneNumber(t) = self { fnc(t) }; self }
  pub fn on_pre<F: FnOnce(&TextEntityTypePre)>(&self, fnc: F) -> &Self { if let TextEntityType::Pre(t) = self { fnc(t) }; self }
  pub fn on_pre_code<F: FnOnce(&TextEntityTypePreCode)>(&self, fnc: F) -> &Self { if let TextEntityType::PreCode(t) = self { fnc(t) }; self }
  pub fn on_strikethrough<F: FnOnce(&TextEntityTypeStrikethrough)>(&self, fnc: F) -> &Self { if let TextEntityType::Strikethrough(t) = self { fnc(t) }; self }
  pub fn on_text_url<F: FnOnce(&TextEntityTypeTextUrl)>(&self, fnc: F) -> &Self { if let TextEntityType::TextUrl(t) = self { fnc(t) }; self }
  pub fn on_underline<F: FnOnce(&TextEntityTypeUnderline)>(&self, fnc: F) -> &Self { if let TextEntityType::Underline(t) = self { fnc(t) }; self }
  pub fn on_url<F: FnOnce(&TextEntityTypeUrl)>(&self, fnc: F) -> &Self { if let TextEntityType::Url(t) = self { fnc(t) }; self }

  pub fn as_bank_card_number(&self) -> Option<&TextEntityTypeBankCardNumber> { if let TextEntityType::BankCardNumber(t) = self { return Some(t) } None }
  pub fn as_bold(&self) -> Option<&TextEntityTypeBold> { if let TextEntityType::Bold(t) = self { return Some(t) } None }
  pub fn as_bot_command(&self) -> Option<&TextEntityTypeBotCommand> { if let TextEntityType::BotCommand(t) = self { return Some(t) } None }
  pub fn as_cashtag(&self) -> Option<&TextEntityTypeCashtag> { if let TextEntityType::Cashtag(t) = self { return Some(t) } None }
  pub fn as_code(&self) -> Option<&TextEntityTypeCode> { if let TextEntityType::Code(t) = self { return Some(t) } None }
  pub fn as_email_address(&self) -> Option<&TextEntityTypeEmailAddress> { if let TextEntityType::EmailAddress(t) = self { return Some(t) } None }
  pub fn as_hashtag(&self) -> Option<&TextEntityTypeHashtag> { if let TextEntityType::Hashtag(t) = self { return Some(t) } None }
  pub fn as_italic(&self) -> Option<&TextEntityTypeItalic> { if let TextEntityType::Italic(t) = self { return Some(t) } None }
  pub fn as_media_timestamp(&self) -> Option<&TextEntityTypeMediaTimestamp> { if let TextEntityType::MediaTimestamp(t) = self { return Some(t) } None }
  pub fn as_mention(&self) -> Option<&TextEntityTypeMention> { if let TextEntityType::Mention(t) = self { return Some(t) } None }
  pub fn as_mention_name(&self) -> Option<&TextEntityTypeMentionName> { if let TextEntityType::MentionName(t) = self { return Some(t) } None }
  pub fn as_phone_number(&self) -> Option<&TextEntityTypePhoneNumber> { if let TextEntityType::PhoneNumber(t) = self { return Some(t) } None }
  pub fn as_pre(&self) -> Option<&TextEntityTypePre> { if let TextEntityType::Pre(t) = self { return Some(t) } None }
  pub fn as_pre_code(&self) -> Option<&TextEntityTypePreCode> { if let TextEntityType::PreCode(t) = self { return Some(t) } None }
  pub fn as_strikethrough(&self) -> Option<&TextEntityTypeStrikethrough> { if let TextEntityType::Strikethrough(t) = self { return Some(t) } None }
  pub fn as_text_url(&self) -> Option<&TextEntityTypeTextUrl> { if let TextEntityType::TextUrl(t) = self { return Some(t) } None }
  pub fn as_underline(&self) -> Option<&TextEntityTypeUnderline> { if let TextEntityType::Underline(t) = self { return Some(t) } None }
  pub fn as_url(&self) -> Option<&TextEntityTypeUrl> { if let TextEntityType::Url(t) = self { return Some(t) } None }



  pub fn bank_card_number<T: AsRef<TextEntityTypeBankCardNumber>>(t: T) -> Self { TextEntityType::BankCardNumber(t.as_ref().clone()) }

  pub fn bold<T: AsRef<TextEntityTypeBold>>(t: T) -> Self { TextEntityType::Bold(t.as_ref().clone()) }

  pub fn bot_command<T: AsRef<TextEntityTypeBotCommand>>(t: T) -> Self { TextEntityType::BotCommand(t.as_ref().clone()) }

  pub fn cashtag<T: AsRef<TextEntityTypeCashtag>>(t: T) -> Self { TextEntityType::Cashtag(t.as_ref().clone()) }

  pub fn code<T: AsRef<TextEntityTypeCode>>(t: T) -> Self { TextEntityType::Code(t.as_ref().clone()) }

  pub fn email_address<T: AsRef<TextEntityTypeEmailAddress>>(t: T) -> Self { TextEntityType::EmailAddress(t.as_ref().clone()) }

  pub fn hashtag<T: AsRef<TextEntityTypeHashtag>>(t: T) -> Self { TextEntityType::Hashtag(t.as_ref().clone()) }

  pub fn italic<T: AsRef<TextEntityTypeItalic>>(t: T) -> Self { TextEntityType::Italic(t.as_ref().clone()) }

  pub fn media_timestamp<T: AsRef<TextEntityTypeMediaTimestamp>>(t: T) -> Self { TextEntityType::MediaTimestamp(t.as_ref().clone()) }

  pub fn mention<T: AsRef<TextEntityTypeMention>>(t: T) -> Self { TextEntityType::Mention(t.as_ref().clone()) }

  pub fn mention_name<T: AsRef<TextEntityTypeMentionName>>(t: T) -> Self { TextEntityType::MentionName(t.as_ref().clone()) }

  pub fn phone_number<T: AsRef<TextEntityTypePhoneNumber>>(t: T) -> Self { TextEntityType::PhoneNumber(t.as_ref().clone()) }

  pub fn pre<T: AsRef<TextEntityTypePre>>(t: T) -> Self { TextEntityType::Pre(t.as_ref().clone()) }

  pub fn pre_code<T: AsRef<TextEntityTypePreCode>>(t: T) -> Self { TextEntityType::PreCode(t.as_ref().clone()) }

  pub fn strikethrough<T: AsRef<TextEntityTypeStrikethrough>>(t: T) -> Self { TextEntityType::Strikethrough(t.as_ref().clone()) }

  pub fn text_url<T: AsRef<TextEntityTypeTextUrl>>(t: T) -> Self { TextEntityType::TextUrl(t.as_ref().clone()) }

  pub fn underline<T: AsRef<TextEntityTypeUnderline>>(t: T) -> Self { TextEntityType::Underline(t.as_ref().clone()) }

  pub fn url<T: AsRef<TextEntityTypeUrl>>(t: T) -> Self { TextEntityType::Url(t.as_ref().clone()) }

}

impl AsRef<TextEntityType> for TextEntityType {
  fn as_ref(&self) -> &TextEntityType { self }
}







/// A bank card number. The getBankCardInfo method can be used to get information about the bank card
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeBankCardNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for TextEntityTypeBankCardNumber {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypeBankCardNumber" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTextEntityType for TextEntityTypeBankCardNumber {}



impl TextEntityTypeBankCardNumber {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTextEntityTypeBankCardNumberBuilder {
    let mut inner = TextEntityTypeBankCardNumber::default();
    inner.td_name = "textEntityTypeBankCardNumber".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTextEntityTypeBankCardNumberBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDTextEntityTypeBankCardNumberBuilder {
  inner: TextEntityTypeBankCardNumber
}

impl RTDTextEntityTypeBankCardNumberBuilder {
  pub fn build(&self) -> TextEntityTypeBankCardNumber { self.inner.clone() }

}

impl AsRef<TextEntityTypeBankCardNumber> for TextEntityTypeBankCardNumber {
  fn as_ref(&self) -> &TextEntityTypeBankCardNumber { self }
}

impl AsRef<TextEntityTypeBankCardNumber> for RTDTextEntityTypeBankCardNumberBuilder {
  fn as_ref(&self) -> &TextEntityTypeBankCardNumber { &self.inner }
}







/// A bold text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeBold {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for TextEntityTypeBold {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypeBold" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTextEntityType for TextEntityTypeBold {}



impl TextEntityTypeBold {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTextEntityTypeBoldBuilder {
    let mut inner = TextEntityTypeBold::default();
    inner.td_name = "textEntityTypeBold".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTextEntityTypeBoldBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDTextEntityTypeBoldBuilder {
  inner: TextEntityTypeBold
}

impl RTDTextEntityTypeBoldBuilder {
  pub fn build(&self) -> TextEntityTypeBold { self.inner.clone() }

}

impl AsRef<TextEntityTypeBold> for TextEntityTypeBold {
  fn as_ref(&self) -> &TextEntityTypeBold { self }
}

impl AsRef<TextEntityTypeBold> for RTDTextEntityTypeBoldBuilder {
  fn as_ref(&self) -> &TextEntityTypeBold { &self.inner }
}







/// A bot command, beginning with "/"
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeBotCommand {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for TextEntityTypeBotCommand {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypeBotCommand" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTextEntityType for TextEntityTypeBotCommand {}



impl TextEntityTypeBotCommand {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTextEntityTypeBotCommandBuilder {
    let mut inner = TextEntityTypeBotCommand::default();
    inner.td_name = "textEntityTypeBotCommand".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTextEntityTypeBotCommandBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDTextEntityTypeBotCommandBuilder {
  inner: TextEntityTypeBotCommand
}

impl RTDTextEntityTypeBotCommandBuilder {
  pub fn build(&self) -> TextEntityTypeBotCommand { self.inner.clone() }

}

impl AsRef<TextEntityTypeBotCommand> for TextEntityTypeBotCommand {
  fn as_ref(&self) -> &TextEntityTypeBotCommand { self }
}

impl AsRef<TextEntityTypeBotCommand> for RTDTextEntityTypeBotCommandBuilder {
  fn as_ref(&self) -> &TextEntityTypeBotCommand { &self.inner }
}







/// A cashtag text, beginning with "$" and consisting of capital English letters (e.g., "$USD")
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeCashtag {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for TextEntityTypeCashtag {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypeCashtag" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTextEntityType for TextEntityTypeCashtag {}



impl TextEntityTypeCashtag {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTextEntityTypeCashtagBuilder {
    let mut inner = TextEntityTypeCashtag::default();
    inner.td_name = "textEntityTypeCashtag".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTextEntityTypeCashtagBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDTextEntityTypeCashtagBuilder {
  inner: TextEntityTypeCashtag
}

impl RTDTextEntityTypeCashtagBuilder {
  pub fn build(&self) -> TextEntityTypeCashtag { self.inner.clone() }

}

impl AsRef<TextEntityTypeCashtag> for TextEntityTypeCashtag {
  fn as_ref(&self) -> &TextEntityTypeCashtag { self }
}

impl AsRef<TextEntityTypeCashtag> for RTDTextEntityTypeCashtagBuilder {
  fn as_ref(&self) -> &TextEntityTypeCashtag { &self.inner }
}







/// Text that must be formatted as if inside a code HTML tag
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for TextEntityTypeCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypeCode" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTextEntityType for TextEntityTypeCode {}



impl TextEntityTypeCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTextEntityTypeCodeBuilder {
    let mut inner = TextEntityTypeCode::default();
    inner.td_name = "textEntityTypeCode".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTextEntityTypeCodeBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDTextEntityTypeCodeBuilder {
  inner: TextEntityTypeCode
}

impl RTDTextEntityTypeCodeBuilder {
  pub fn build(&self) -> TextEntityTypeCode { self.inner.clone() }

}

impl AsRef<TextEntityTypeCode> for TextEntityTypeCode {
  fn as_ref(&self) -> &TextEntityTypeCode { self }
}

impl AsRef<TextEntityTypeCode> for RTDTextEntityTypeCodeBuilder {
  fn as_ref(&self) -> &TextEntityTypeCode { &self.inner }
}







/// An email address
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeEmailAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for TextEntityTypeEmailAddress {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypeEmailAddress" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTextEntityType for TextEntityTypeEmailAddress {}



impl TextEntityTypeEmailAddress {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTextEntityTypeEmailAddressBuilder {
    let mut inner = TextEntityTypeEmailAddress::default();
    inner.td_name = "textEntityTypeEmailAddress".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTextEntityTypeEmailAddressBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDTextEntityTypeEmailAddressBuilder {
  inner: TextEntityTypeEmailAddress
}

impl RTDTextEntityTypeEmailAddressBuilder {
  pub fn build(&self) -> TextEntityTypeEmailAddress { self.inner.clone() }

}

impl AsRef<TextEntityTypeEmailAddress> for TextEntityTypeEmailAddress {
  fn as_ref(&self) -> &TextEntityTypeEmailAddress { self }
}

impl AsRef<TextEntityTypeEmailAddress> for RTDTextEntityTypeEmailAddressBuilder {
  fn as_ref(&self) -> &TextEntityTypeEmailAddress { &self.inner }
}







/// A hashtag text, beginning with "#"
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeHashtag {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for TextEntityTypeHashtag {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypeHashtag" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTextEntityType for TextEntityTypeHashtag {}



impl TextEntityTypeHashtag {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTextEntityTypeHashtagBuilder {
    let mut inner = TextEntityTypeHashtag::default();
    inner.td_name = "textEntityTypeHashtag".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTextEntityTypeHashtagBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDTextEntityTypeHashtagBuilder {
  inner: TextEntityTypeHashtag
}

impl RTDTextEntityTypeHashtagBuilder {
  pub fn build(&self) -> TextEntityTypeHashtag { self.inner.clone() }

}

impl AsRef<TextEntityTypeHashtag> for TextEntityTypeHashtag {
  fn as_ref(&self) -> &TextEntityTypeHashtag { self }
}

impl AsRef<TextEntityTypeHashtag> for RTDTextEntityTypeHashtagBuilder {
  fn as_ref(&self) -> &TextEntityTypeHashtag { &self.inner }
}







/// An italic text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeItalic {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for TextEntityTypeItalic {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypeItalic" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTextEntityType for TextEntityTypeItalic {}



impl TextEntityTypeItalic {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTextEntityTypeItalicBuilder {
    let mut inner = TextEntityTypeItalic::default();
    inner.td_name = "textEntityTypeItalic".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTextEntityTypeItalicBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDTextEntityTypeItalicBuilder {
  inner: TextEntityTypeItalic
}

impl RTDTextEntityTypeItalicBuilder {
  pub fn build(&self) -> TextEntityTypeItalic { self.inner.clone() }

}

impl AsRef<TextEntityTypeItalic> for TextEntityTypeItalic {
  fn as_ref(&self) -> &TextEntityTypeItalic { self }
}

impl AsRef<TextEntityTypeItalic> for RTDTextEntityTypeItalicBuilder {
  fn as_ref(&self) -> &TextEntityTypeItalic { &self.inner }
}







/// A media timestamp
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeMediaTimestamp {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Timestamp from which a video/audio/video note/voice note playing must start, in seconds. The media can be in the content or the web page preview of the current message, or in the same places in the replied message
  media_timestamp: i64,
  
}

impl RObject for TextEntityTypeMediaTimestamp {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypeMediaTimestamp" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTextEntityType for TextEntityTypeMediaTimestamp {}



impl TextEntityTypeMediaTimestamp {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTextEntityTypeMediaTimestampBuilder {
    let mut inner = TextEntityTypeMediaTimestamp::default();
    inner.td_name = "textEntityTypeMediaTimestamp".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTextEntityTypeMediaTimestampBuilder { inner }
  }

  pub fn media_timestamp(&self) -> i64 { self.media_timestamp }

}

#[doc(hidden)]
pub struct RTDTextEntityTypeMediaTimestampBuilder {
  inner: TextEntityTypeMediaTimestamp
}

impl RTDTextEntityTypeMediaTimestampBuilder {
  pub fn build(&self) -> TextEntityTypeMediaTimestamp { self.inner.clone() }

   
  pub fn media_timestamp(&mut self, media_timestamp: i64) -> &mut Self {
    self.inner.media_timestamp = media_timestamp;
    self
  }

}

impl AsRef<TextEntityTypeMediaTimestamp> for TextEntityTypeMediaTimestamp {
  fn as_ref(&self) -> &TextEntityTypeMediaTimestamp { self }
}

impl AsRef<TextEntityTypeMediaTimestamp> for RTDTextEntityTypeMediaTimestampBuilder {
  fn as_ref(&self) -> &TextEntityTypeMediaTimestamp { &self.inner }
}







/// A mention of a user by their username
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeMention {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for TextEntityTypeMention {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypeMention" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTextEntityType for TextEntityTypeMention {}



impl TextEntityTypeMention {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTextEntityTypeMentionBuilder {
    let mut inner = TextEntityTypeMention::default();
    inner.td_name = "textEntityTypeMention".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTextEntityTypeMentionBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDTextEntityTypeMentionBuilder {
  inner: TextEntityTypeMention
}

impl RTDTextEntityTypeMentionBuilder {
  pub fn build(&self) -> TextEntityTypeMention { self.inner.clone() }

}

impl AsRef<TextEntityTypeMention> for TextEntityTypeMention {
  fn as_ref(&self) -> &TextEntityTypeMention { self }
}

impl AsRef<TextEntityTypeMention> for RTDTextEntityTypeMentionBuilder {
  fn as_ref(&self) -> &TextEntityTypeMention { &self.inner }
}







/// A text shows instead of a raw mention of the user (e.g., when the user has no username)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeMentionName {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the mentioned user
  user_id: i64,
  
}

impl RObject for TextEntityTypeMentionName {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypeMentionName" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTextEntityType for TextEntityTypeMentionName {}



impl TextEntityTypeMentionName {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTextEntityTypeMentionNameBuilder {
    let mut inner = TextEntityTypeMentionName::default();
    inner.td_name = "textEntityTypeMentionName".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTextEntityTypeMentionNameBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

}

#[doc(hidden)]
pub struct RTDTextEntityTypeMentionNameBuilder {
  inner: TextEntityTypeMentionName
}

impl RTDTextEntityTypeMentionNameBuilder {
  pub fn build(&self) -> TextEntityTypeMentionName { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

}

impl AsRef<TextEntityTypeMentionName> for TextEntityTypeMentionName {
  fn as_ref(&self) -> &TextEntityTypeMentionName { self }
}

impl AsRef<TextEntityTypeMentionName> for RTDTextEntityTypeMentionNameBuilder {
  fn as_ref(&self) -> &TextEntityTypeMentionName { &self.inner }
}







/// A phone number
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypePhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for TextEntityTypePhoneNumber {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypePhoneNumber" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTextEntityType for TextEntityTypePhoneNumber {}



impl TextEntityTypePhoneNumber {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTextEntityTypePhoneNumberBuilder {
    let mut inner = TextEntityTypePhoneNumber::default();
    inner.td_name = "textEntityTypePhoneNumber".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTextEntityTypePhoneNumberBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDTextEntityTypePhoneNumberBuilder {
  inner: TextEntityTypePhoneNumber
}

impl RTDTextEntityTypePhoneNumberBuilder {
  pub fn build(&self) -> TextEntityTypePhoneNumber { self.inner.clone() }

}

impl AsRef<TextEntityTypePhoneNumber> for TextEntityTypePhoneNumber {
  fn as_ref(&self) -> &TextEntityTypePhoneNumber { self }
}

impl AsRef<TextEntityTypePhoneNumber> for RTDTextEntityTypePhoneNumberBuilder {
  fn as_ref(&self) -> &TextEntityTypePhoneNumber { &self.inner }
}







/// Text that must be formatted as if inside a pre HTML tag
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypePre {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for TextEntityTypePre {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypePre" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTextEntityType for TextEntityTypePre {}



impl TextEntityTypePre {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTextEntityTypePreBuilder {
    let mut inner = TextEntityTypePre::default();
    inner.td_name = "textEntityTypePre".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTextEntityTypePreBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDTextEntityTypePreBuilder {
  inner: TextEntityTypePre
}

impl RTDTextEntityTypePreBuilder {
  pub fn build(&self) -> TextEntityTypePre { self.inner.clone() }

}

impl AsRef<TextEntityTypePre> for TextEntityTypePre {
  fn as_ref(&self) -> &TextEntityTypePre { self }
}

impl AsRef<TextEntityTypePre> for RTDTextEntityTypePreBuilder {
  fn as_ref(&self) -> &TextEntityTypePre { &self.inner }
}







/// Text that must be formatted as if inside pre, and code HTML tags
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypePreCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Programming language of the code; as defined by the sender
  language: String,
  
}

impl RObject for TextEntityTypePreCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypePreCode" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTextEntityType for TextEntityTypePreCode {}



impl TextEntityTypePreCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTextEntityTypePreCodeBuilder {
    let mut inner = TextEntityTypePreCode::default();
    inner.td_name = "textEntityTypePreCode".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTextEntityTypePreCodeBuilder { inner }
  }

  pub fn language(&self) -> &String { &self.language }

}

#[doc(hidden)]
pub struct RTDTextEntityTypePreCodeBuilder {
  inner: TextEntityTypePreCode
}

impl RTDTextEntityTypePreCodeBuilder {
  pub fn build(&self) -> TextEntityTypePreCode { self.inner.clone() }

   
  pub fn language<T: AsRef<str>>(&mut self, language: T) -> &mut Self {
    self.inner.language = language.as_ref().to_string();
    self
  }

}

impl AsRef<TextEntityTypePreCode> for TextEntityTypePreCode {
  fn as_ref(&self) -> &TextEntityTypePreCode { self }
}

impl AsRef<TextEntityTypePreCode> for RTDTextEntityTypePreCodeBuilder {
  fn as_ref(&self) -> &TextEntityTypePreCode { &self.inner }
}







/// A strikethrough text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeStrikethrough {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for TextEntityTypeStrikethrough {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypeStrikethrough" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTextEntityType for TextEntityTypeStrikethrough {}



impl TextEntityTypeStrikethrough {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTextEntityTypeStrikethroughBuilder {
    let mut inner = TextEntityTypeStrikethrough::default();
    inner.td_name = "textEntityTypeStrikethrough".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTextEntityTypeStrikethroughBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDTextEntityTypeStrikethroughBuilder {
  inner: TextEntityTypeStrikethrough
}

impl RTDTextEntityTypeStrikethroughBuilder {
  pub fn build(&self) -> TextEntityTypeStrikethrough { self.inner.clone() }

}

impl AsRef<TextEntityTypeStrikethrough> for TextEntityTypeStrikethrough {
  fn as_ref(&self) -> &TextEntityTypeStrikethrough { self }
}

impl AsRef<TextEntityTypeStrikethrough> for RTDTextEntityTypeStrikethroughBuilder {
  fn as_ref(&self) -> &TextEntityTypeStrikethrough { &self.inner }
}







/// A text description shown instead of a raw URL
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeTextUrl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// HTTP or tg:// URL to be opened when the link is clicked
  url: String,
  
}

impl RObject for TextEntityTypeTextUrl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypeTextUrl" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTextEntityType for TextEntityTypeTextUrl {}



impl TextEntityTypeTextUrl {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTextEntityTypeTextUrlBuilder {
    let mut inner = TextEntityTypeTextUrl::default();
    inner.td_name = "textEntityTypeTextUrl".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTextEntityTypeTextUrlBuilder { inner }
  }

  pub fn url(&self) -> &String { &self.url }

}

#[doc(hidden)]
pub struct RTDTextEntityTypeTextUrlBuilder {
  inner: TextEntityTypeTextUrl
}

impl RTDTextEntityTypeTextUrlBuilder {
  pub fn build(&self) -> TextEntityTypeTextUrl { self.inner.clone() }

   
  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

}

impl AsRef<TextEntityTypeTextUrl> for TextEntityTypeTextUrl {
  fn as_ref(&self) -> &TextEntityTypeTextUrl { self }
}

impl AsRef<TextEntityTypeTextUrl> for RTDTextEntityTypeTextUrlBuilder {
  fn as_ref(&self) -> &TextEntityTypeTextUrl { &self.inner }
}







/// An underlined text
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeUnderline {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for TextEntityTypeUnderline {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypeUnderline" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTextEntityType for TextEntityTypeUnderline {}



impl TextEntityTypeUnderline {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTextEntityTypeUnderlineBuilder {
    let mut inner = TextEntityTypeUnderline::default();
    inner.td_name = "textEntityTypeUnderline".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTextEntityTypeUnderlineBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDTextEntityTypeUnderlineBuilder {
  inner: TextEntityTypeUnderline
}

impl RTDTextEntityTypeUnderlineBuilder {
  pub fn build(&self) -> TextEntityTypeUnderline { self.inner.clone() }

}

impl AsRef<TextEntityTypeUnderline> for TextEntityTypeUnderline {
  fn as_ref(&self) -> &TextEntityTypeUnderline { self }
}

impl AsRef<TextEntityTypeUnderline> for RTDTextEntityTypeUnderlineBuilder {
  fn as_ref(&self) -> &TextEntityTypeUnderline { &self.inner }
}







/// An HTTP URL
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntityTypeUrl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for TextEntityTypeUrl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "textEntityTypeUrl" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDTextEntityType for TextEntityTypeUrl {}



impl TextEntityTypeUrl {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDTextEntityTypeUrlBuilder {
    let mut inner = TextEntityTypeUrl::default();
    inner.td_name = "textEntityTypeUrl".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDTextEntityTypeUrlBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDTextEntityTypeUrlBuilder {
  inner: TextEntityTypeUrl
}

impl RTDTextEntityTypeUrlBuilder {
  pub fn build(&self) -> TextEntityTypeUrl { self.inner.clone() }

}

impl AsRef<TextEntityTypeUrl> for TextEntityTypeUrl {
  fn as_ref(&self) -> &TextEntityTypeUrl { self }
}

impl AsRef<TextEntityTypeUrl> for RTDTextEntityTypeUrlBuilder {
  fn as_ref(&self) -> &TextEntityTypeUrl { &self.inner }
}



