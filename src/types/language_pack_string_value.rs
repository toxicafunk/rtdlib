
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Represents the value of a string in a language pack
pub trait TDLanguagePackStringValue: Debug + RObject {}

/// Represents the value of a string in a language pack
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum LanguagePackStringValue {
  #[doc(hidden)] _Default(()),
  /// Returns a string stored in the local database from the specified localization target and language pack by its key. Returns a 404 error if the string is not found. Can be called synchronously
  GetLanguagePackString(GetLanguagePackString),
  /// A deleted language pack string, the value must be taken from the built-in English language pack
  Deleted(LanguagePackStringValueDeleted),
  /// An ordinary language pack string
  Ordinary(LanguagePackStringValueOrdinary),
  /// A language pack string which has different forms based on the number of some object it mentions. See https://www.unicode.org/cldr/charts/latest/supplemental/language_plural_rules.html for more info
  Pluralized(LanguagePackStringValuePluralized),

}

impl Default for LanguagePackStringValue {
  fn default() -> Self { LanguagePackStringValue::_Default(()) }
}

impl<'de> Deserialize<'de> for LanguagePackStringValue {
  fn deserialize<D>(deserializer: D) -> Result<LanguagePackStringValue, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      LanguagePackStringValue,
      (getLanguagePackString, GetLanguagePackString);
      (languagePackStringValueDeleted, Deleted);
      (languagePackStringValueOrdinary, Ordinary);
      (languagePackStringValuePluralized, Pluralized);

    )(deserializer)
  }
}

impl RObject for LanguagePackStringValue {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      LanguagePackStringValue::GetLanguagePackString(t) => t.td_name(),
      LanguagePackStringValue::Deleted(t) => t.td_name(),
      LanguagePackStringValue::Ordinary(t) => t.td_name(),
      LanguagePackStringValue::Pluralized(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      LanguagePackStringValue::GetLanguagePackString(t) => t.extra(),
      LanguagePackStringValue::Deleted(t) => t.extra(),
      LanguagePackStringValue::Ordinary(t) => t.extra(),
      LanguagePackStringValue::Pluralized(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl LanguagePackStringValue {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let LanguagePackStringValue::_Default(_) = self { true } else { false } }

  pub fn is_get_language_pack_string(&self) -> bool { if let LanguagePackStringValue::GetLanguagePackString(_) = self { true } else { false } }
  pub fn is_deleted(&self) -> bool { if let LanguagePackStringValue::Deleted(_) = self { true } else { false } }
  pub fn is_ordinary(&self) -> bool { if let LanguagePackStringValue::Ordinary(_) = self { true } else { false } }
  pub fn is_pluralized(&self) -> bool { if let LanguagePackStringValue::Pluralized(_) = self { true } else { false } }

  pub fn on_get_language_pack_string<F: FnOnce(&GetLanguagePackString)>(&self, fnc: F) -> &Self { if let LanguagePackStringValue::GetLanguagePackString(t) = self { fnc(t) }; self }
  pub fn on_deleted<F: FnOnce(&LanguagePackStringValueDeleted)>(&self, fnc: F) -> &Self { if let LanguagePackStringValue::Deleted(t) = self { fnc(t) }; self }
  pub fn on_ordinary<F: FnOnce(&LanguagePackStringValueOrdinary)>(&self, fnc: F) -> &Self { if let LanguagePackStringValue::Ordinary(t) = self { fnc(t) }; self }
  pub fn on_pluralized<F: FnOnce(&LanguagePackStringValuePluralized)>(&self, fnc: F) -> &Self { if let LanguagePackStringValue::Pluralized(t) = self { fnc(t) }; self }

  pub fn as_get_language_pack_string(&self) -> Option<&GetLanguagePackString> { if let LanguagePackStringValue::GetLanguagePackString(t) = self { return Some(t) } None }
  pub fn as_deleted(&self) -> Option<&LanguagePackStringValueDeleted> { if let LanguagePackStringValue::Deleted(t) = self { return Some(t) } None }
  pub fn as_ordinary(&self) -> Option<&LanguagePackStringValueOrdinary> { if let LanguagePackStringValue::Ordinary(t) = self { return Some(t) } None }
  pub fn as_pluralized(&self) -> Option<&LanguagePackStringValuePluralized> { if let LanguagePackStringValue::Pluralized(t) = self { return Some(t) } None }



  pub fn get_language_pack_string<T: AsRef<GetLanguagePackString>>(t: T) -> Self { LanguagePackStringValue::GetLanguagePackString(t.as_ref().clone()) }

  pub fn deleted<T: AsRef<LanguagePackStringValueDeleted>>(t: T) -> Self { LanguagePackStringValue::Deleted(t.as_ref().clone()) }

  pub fn ordinary<T: AsRef<LanguagePackStringValueOrdinary>>(t: T) -> Self { LanguagePackStringValue::Ordinary(t.as_ref().clone()) }

  pub fn pluralized<T: AsRef<LanguagePackStringValuePluralized>>(t: T) -> Self { LanguagePackStringValue::Pluralized(t.as_ref().clone()) }

}

impl AsRef<LanguagePackStringValue> for LanguagePackStringValue {
  fn as_ref(&self) -> &LanguagePackStringValue { self }
}







/// A deleted language pack string, the value must be taken from the built-in English language pack
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LanguagePackStringValueDeleted {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for LanguagePackStringValueDeleted {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "languagePackStringValueDeleted" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDLanguagePackStringValue for LanguagePackStringValueDeleted {}



impl LanguagePackStringValueDeleted {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDLanguagePackStringValueDeletedBuilder {
    let mut inner = LanguagePackStringValueDeleted::default();
    inner.td_name = "languagePackStringValueDeleted".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDLanguagePackStringValueDeletedBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDLanguagePackStringValueDeletedBuilder {
  inner: LanguagePackStringValueDeleted
}

impl RTDLanguagePackStringValueDeletedBuilder {
  pub fn build(&self) -> LanguagePackStringValueDeleted { self.inner.clone() }

}

impl AsRef<LanguagePackStringValueDeleted> for LanguagePackStringValueDeleted {
  fn as_ref(&self) -> &LanguagePackStringValueDeleted { self }
}

impl AsRef<LanguagePackStringValueDeleted> for RTDLanguagePackStringValueDeletedBuilder {
  fn as_ref(&self) -> &LanguagePackStringValueDeleted { &self.inner }
}







/// An ordinary language pack string
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LanguagePackStringValueOrdinary {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// String value
  value: String,
  
}

impl RObject for LanguagePackStringValueOrdinary {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "languagePackStringValueOrdinary" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDLanguagePackStringValue for LanguagePackStringValueOrdinary {}



impl LanguagePackStringValueOrdinary {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDLanguagePackStringValueOrdinaryBuilder {
    let mut inner = LanguagePackStringValueOrdinary::default();
    inner.td_name = "languagePackStringValueOrdinary".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDLanguagePackStringValueOrdinaryBuilder { inner }
  }

  pub fn value(&self) -> &String { &self.value }

}

#[doc(hidden)]
pub struct RTDLanguagePackStringValueOrdinaryBuilder {
  inner: LanguagePackStringValueOrdinary
}

impl RTDLanguagePackStringValueOrdinaryBuilder {
  pub fn build(&self) -> LanguagePackStringValueOrdinary { self.inner.clone() }

   
  pub fn value<T: AsRef<str>>(&mut self, value: T) -> &mut Self {
    self.inner.value = value.as_ref().to_string();
    self
  }

}

impl AsRef<LanguagePackStringValueOrdinary> for LanguagePackStringValueOrdinary {
  fn as_ref(&self) -> &LanguagePackStringValueOrdinary { self }
}

impl AsRef<LanguagePackStringValueOrdinary> for RTDLanguagePackStringValueOrdinaryBuilder {
  fn as_ref(&self) -> &LanguagePackStringValueOrdinary { &self.inner }
}







/// A language pack string which has different forms based on the number of some object it mentions. See https://www.unicode.org/cldr/charts/latest/supplemental/language_plural_rules.html for more info
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LanguagePackStringValuePluralized {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Value for zero objects
  zero_value: String,
  /// Value for one object
  one_value: String,
  /// Value for two objects
  two_value: String,
  /// Value for few objects
  few_value: String,
  /// Value for many objects
  many_value: String,
  /// Default value
  other_value: String,
  
}

impl RObject for LanguagePackStringValuePluralized {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "languagePackStringValuePluralized" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDLanguagePackStringValue for LanguagePackStringValuePluralized {}



impl LanguagePackStringValuePluralized {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDLanguagePackStringValuePluralizedBuilder {
    let mut inner = LanguagePackStringValuePluralized::default();
    inner.td_name = "languagePackStringValuePluralized".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDLanguagePackStringValuePluralizedBuilder { inner }
  }

  pub fn zero_value(&self) -> &String { &self.zero_value }

  pub fn one_value(&self) -> &String { &self.one_value }

  pub fn two_value(&self) -> &String { &self.two_value }

  pub fn few_value(&self) -> &String { &self.few_value }

  pub fn many_value(&self) -> &String { &self.many_value }

  pub fn other_value(&self) -> &String { &self.other_value }

}

#[doc(hidden)]
pub struct RTDLanguagePackStringValuePluralizedBuilder {
  inner: LanguagePackStringValuePluralized
}

impl RTDLanguagePackStringValuePluralizedBuilder {
  pub fn build(&self) -> LanguagePackStringValuePluralized { self.inner.clone() }

   
  pub fn zero_value<T: AsRef<str>>(&mut self, zero_value: T) -> &mut Self {
    self.inner.zero_value = zero_value.as_ref().to_string();
    self
  }

   
  pub fn one_value<T: AsRef<str>>(&mut self, one_value: T) -> &mut Self {
    self.inner.one_value = one_value.as_ref().to_string();
    self
  }

   
  pub fn two_value<T: AsRef<str>>(&mut self, two_value: T) -> &mut Self {
    self.inner.two_value = two_value.as_ref().to_string();
    self
  }

   
  pub fn few_value<T: AsRef<str>>(&mut self, few_value: T) -> &mut Self {
    self.inner.few_value = few_value.as_ref().to_string();
    self
  }

   
  pub fn many_value<T: AsRef<str>>(&mut self, many_value: T) -> &mut Self {
    self.inner.many_value = many_value.as_ref().to_string();
    self
  }

   
  pub fn other_value<T: AsRef<str>>(&mut self, other_value: T) -> &mut Self {
    self.inner.other_value = other_value.as_ref().to_string();
    self
  }

}

impl AsRef<LanguagePackStringValuePluralized> for LanguagePackStringValuePluralized {
  fn as_ref(&self) -> &LanguagePackStringValuePluralized { self }
}

impl AsRef<LanguagePackStringValuePluralized> for RTDLanguagePackStringValuePluralizedBuilder {
  fn as_ref(&self) -> &LanguagePackStringValuePluralized { &self.inner }
}



