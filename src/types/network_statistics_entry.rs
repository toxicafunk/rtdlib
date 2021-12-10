
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Contains statistics about network usage
pub trait TDNetworkStatisticsEntry: Debug + RObject {}

/// Contains statistics about network usage
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum NetworkStatisticsEntry {
  #[doc(hidden)] _Default(()),
  /// Contains information about the total amount of data that was used for calls
  Call(NetworkStatisticsEntryCall),
  /// Contains information about the total amount of data that was used to send and receive files
  File(NetworkStatisticsEntryFile),

}

impl Default for NetworkStatisticsEntry {
  fn default() -> Self { NetworkStatisticsEntry::_Default(()) }
}

impl<'de> Deserialize<'de> for NetworkStatisticsEntry {
  fn deserialize<D>(deserializer: D) -> Result<NetworkStatisticsEntry, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      NetworkStatisticsEntry,
      (networkStatisticsEntryCall, Call);
      (networkStatisticsEntryFile, File);

    )(deserializer)
  }
}

impl RObject for NetworkStatisticsEntry {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      NetworkStatisticsEntry::Call(t) => t.td_name(),
      NetworkStatisticsEntry::File(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      NetworkStatisticsEntry::Call(t) => t.extra(),
      NetworkStatisticsEntry::File(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl NetworkStatisticsEntry {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let NetworkStatisticsEntry::_Default(_) = self { true } else { false } }

  pub fn is_call(&self) -> bool { if let NetworkStatisticsEntry::Call(_) = self { true } else { false } }
  pub fn is_file(&self) -> bool { if let NetworkStatisticsEntry::File(_) = self { true } else { false } }

  pub fn on_call<F: FnOnce(&NetworkStatisticsEntryCall)>(&self, fnc: F) -> &Self { if let NetworkStatisticsEntry::Call(t) = self { fnc(t) }; self }
  pub fn on_file<F: FnOnce(&NetworkStatisticsEntryFile)>(&self, fnc: F) -> &Self { if let NetworkStatisticsEntry::File(t) = self { fnc(t) }; self }

  pub fn as_call(&self) -> Option<&NetworkStatisticsEntryCall> { if let NetworkStatisticsEntry::Call(t) = self { return Some(t) } None }
  pub fn as_file(&self) -> Option<&NetworkStatisticsEntryFile> { if let NetworkStatisticsEntry::File(t) = self { return Some(t) } None }



  pub fn call<T: AsRef<NetworkStatisticsEntryCall>>(t: T) -> Self { NetworkStatisticsEntry::Call(t.as_ref().clone()) }

  pub fn file<T: AsRef<NetworkStatisticsEntryFile>>(t: T) -> Self { NetworkStatisticsEntry::File(t.as_ref().clone()) }

}

impl AsRef<NetworkStatisticsEntry> for NetworkStatisticsEntry {
  fn as_ref(&self) -> &NetworkStatisticsEntry { self }
}







/// Contains information about the total amount of data that was used for calls
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkStatisticsEntryCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Type of the network the data was sent through. Call setNetworkType to maintain the actual network type
  network_type: NetworkType,
  /// Total number of bytes sent
  sent_bytes: i64,
  /// Total number of bytes received
  received_bytes: i64,
  /// Total call duration, in seconds
  duration: f32,
  
}

impl RObject for NetworkStatisticsEntryCall {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "networkStatisticsEntryCall" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDNetworkStatisticsEntry for NetworkStatisticsEntryCall {}



impl NetworkStatisticsEntryCall {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDNetworkStatisticsEntryCallBuilder {
    let mut inner = NetworkStatisticsEntryCall::default();
    inner.td_name = "networkStatisticsEntryCall".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDNetworkStatisticsEntryCallBuilder { inner }
  }

  pub fn network_type(&self) -> &NetworkType { &self.network_type }

  pub fn sent_bytes(&self) -> i64 { self.sent_bytes }

  pub fn received_bytes(&self) -> i64 { self.received_bytes }

  pub fn duration(&self) -> f32 { self.duration }

}

#[doc(hidden)]
pub struct RTDNetworkStatisticsEntryCallBuilder {
  inner: NetworkStatisticsEntryCall
}

impl RTDNetworkStatisticsEntryCallBuilder {
  pub fn build(&self) -> NetworkStatisticsEntryCall { self.inner.clone() }

   
  pub fn network_type<T: AsRef<NetworkType>>(&mut self, network_type: T) -> &mut Self {
    self.inner.network_type = network_type.as_ref().clone();
    self
  }

   
  pub fn sent_bytes(&mut self, sent_bytes: i64) -> &mut Self {
    self.inner.sent_bytes = sent_bytes;
    self
  }

   
  pub fn received_bytes(&mut self, received_bytes: i64) -> &mut Self {
    self.inner.received_bytes = received_bytes;
    self
  }

   
  pub fn duration(&mut self, duration: f32) -> &mut Self {
    self.inner.duration = duration;
    self
  }

}

impl AsRef<NetworkStatisticsEntryCall> for NetworkStatisticsEntryCall {
  fn as_ref(&self) -> &NetworkStatisticsEntryCall { self }
}

impl AsRef<NetworkStatisticsEntryCall> for RTDNetworkStatisticsEntryCallBuilder {
  fn as_ref(&self) -> &NetworkStatisticsEntryCall { &self.inner }
}







/// Contains information about the total amount of data that was used to send and receive files
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NetworkStatisticsEntryFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Type of the file the data is part of; pass null if the data isn't related to files
  file_type: FileType,
  /// Type of the network the data was sent through. Call setNetworkType to maintain the actual network type
  network_type: NetworkType,
  /// Total number of bytes sent
  sent_bytes: i64,
  /// Total number of bytes received
  received_bytes: i64,
  
}

impl RObject for NetworkStatisticsEntryFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "networkStatisticsEntryFile" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDNetworkStatisticsEntry for NetworkStatisticsEntryFile {}



impl NetworkStatisticsEntryFile {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDNetworkStatisticsEntryFileBuilder {
    let mut inner = NetworkStatisticsEntryFile::default();
    inner.td_name = "networkStatisticsEntryFile".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDNetworkStatisticsEntryFileBuilder { inner }
  }

  pub fn file_type(&self) -> &FileType { &self.file_type }

  pub fn network_type(&self) -> &NetworkType { &self.network_type }

  pub fn sent_bytes(&self) -> i64 { self.sent_bytes }

  pub fn received_bytes(&self) -> i64 { self.received_bytes }

}

#[doc(hidden)]
pub struct RTDNetworkStatisticsEntryFileBuilder {
  inner: NetworkStatisticsEntryFile
}

impl RTDNetworkStatisticsEntryFileBuilder {
  pub fn build(&self) -> NetworkStatisticsEntryFile { self.inner.clone() }

   
  pub fn file_type<T: AsRef<FileType>>(&mut self, file_type: T) -> &mut Self {
    self.inner.file_type = file_type.as_ref().clone();
    self
  }

   
  pub fn network_type<T: AsRef<NetworkType>>(&mut self, network_type: T) -> &mut Self {
    self.inner.network_type = network_type.as_ref().clone();
    self
  }

   
  pub fn sent_bytes(&mut self, sent_bytes: i64) -> &mut Self {
    self.inner.sent_bytes = sent_bytes;
    self
  }

   
  pub fn received_bytes(&mut self, received_bytes: i64) -> &mut Self {
    self.inner.received_bytes = received_bytes;
    self
  }

}

impl AsRef<NetworkStatisticsEntryFile> for NetworkStatisticsEntryFile {
  fn as_ref(&self) -> &NetworkStatisticsEntryFile { self }
}

impl AsRef<NetworkStatisticsEntryFile> for RTDNetworkStatisticsEntryFileBuilder {
  fn as_ref(&self) -> &NetworkStatisticsEntryFile { &self.inner }
}



