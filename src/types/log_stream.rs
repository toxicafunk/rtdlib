
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes a stream to which TDLib internal log is written
pub trait TDLogStream: Debug + RObject {}

/// Describes a stream to which TDLib internal log is written
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum LogStream {
  #[doc(hidden)] _Default(()),
  /// Returns information about currently used log stream for internal logging of TDLib. Can be called synchronously
  GetLogStream(GetLogStream),
  /// The log is written to stderr or an OS specific log
  Default(LogStreamDefault),
  /// The log is written nowhere
  Empty(LogStreamEmpty),
  /// The log is written to a file
  File(LogStreamFile),

}

impl Default for LogStream {
  fn default() -> Self { LogStream::_Default(()) }
}

impl<'de> Deserialize<'de> for LogStream {
  fn deserialize<D>(deserializer: D) -> Result<LogStream, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      LogStream,
      (getLogStream, GetLogStream);
      (logStreamDefault, Default);
      (logStreamEmpty, Empty);
      (logStreamFile, File);

    )(deserializer)
  }
}

impl RObject for LogStream {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      LogStream::GetLogStream(t) => t.td_name(),
      LogStream::Default(t) => t.td_name(),
      LogStream::Empty(t) => t.td_name(),
      LogStream::File(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      LogStream::GetLogStream(t) => t.extra(),
      LogStream::Default(t) => t.extra(),
      LogStream::Empty(t) => t.extra(),
      LogStream::File(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl LogStream {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let LogStream::_Default(_) = self { true } else { false } }

  pub fn is_get_log_stream(&self) -> bool { if let LogStream::GetLogStream(_) = self { true } else { false } }
  pub fn is_default(&self) -> bool { if let LogStream::Default(_) = self { true } else { false } }
  pub fn is_empty(&self) -> bool { if let LogStream::Empty(_) = self { true } else { false } }
  pub fn is_file(&self) -> bool { if let LogStream::File(_) = self { true } else { false } }

  pub fn on_get_log_stream<F: FnOnce(&GetLogStream)>(&self, fnc: F) -> &Self { if let LogStream::GetLogStream(t) = self { fnc(t) }; self }
  pub fn on_default<F: FnOnce(&LogStreamDefault)>(&self, fnc: F) -> &Self { if let LogStream::Default(t) = self { fnc(t) }; self }
  pub fn on_empty<F: FnOnce(&LogStreamEmpty)>(&self, fnc: F) -> &Self { if let LogStream::Empty(t) = self { fnc(t) }; self }
  pub fn on_file<F: FnOnce(&LogStreamFile)>(&self, fnc: F) -> &Self { if let LogStream::File(t) = self { fnc(t) }; self }

  pub fn as_get_log_stream(&self) -> Option<&GetLogStream> { if let LogStream::GetLogStream(t) = self { return Some(t) } None }
  pub fn as_default(&self) -> Option<&LogStreamDefault> { if let LogStream::Default(t) = self { return Some(t) } None }
  pub fn as_empty(&self) -> Option<&LogStreamEmpty> { if let LogStream::Empty(t) = self { return Some(t) } None }
  pub fn as_file(&self) -> Option<&LogStreamFile> { if let LogStream::File(t) = self { return Some(t) } None }



  pub fn get_log_stream<T: AsRef<GetLogStream>>(t: T) -> Self { LogStream::GetLogStream(t.as_ref().clone()) }

  pub fn default<T: AsRef<LogStreamDefault>>(t: T) -> Self { LogStream::Default(t.as_ref().clone()) }

  pub fn empty<T: AsRef<LogStreamEmpty>>(t: T) -> Self { LogStream::Empty(t.as_ref().clone()) }

  pub fn file<T: AsRef<LogStreamFile>>(t: T) -> Self { LogStream::File(t.as_ref().clone()) }

}

impl AsRef<LogStream> for LogStream {
  fn as_ref(&self) -> &LogStream { self }
}







/// The log is written to stderr or an OS specific log
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogStreamDefault {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for LogStreamDefault {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "logStreamDefault" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDLogStream for LogStreamDefault {}



impl LogStreamDefault {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDLogStreamDefaultBuilder {
    let mut inner = LogStreamDefault::default();
    inner.td_name = "logStreamDefault".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDLogStreamDefaultBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDLogStreamDefaultBuilder {
  inner: LogStreamDefault
}

impl RTDLogStreamDefaultBuilder {
  pub fn build(&self) -> LogStreamDefault { self.inner.clone() }

}

impl AsRef<LogStreamDefault> for LogStreamDefault {
  fn as_ref(&self) -> &LogStreamDefault { self }
}

impl AsRef<LogStreamDefault> for RTDLogStreamDefaultBuilder {
  fn as_ref(&self) -> &LogStreamDefault { &self.inner }
}







/// The log is written nowhere
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogStreamEmpty {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for LogStreamEmpty {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "logStreamEmpty" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDLogStream for LogStreamEmpty {}



impl LogStreamEmpty {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDLogStreamEmptyBuilder {
    let mut inner = LogStreamEmpty::default();
    inner.td_name = "logStreamEmpty".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDLogStreamEmptyBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDLogStreamEmptyBuilder {
  inner: LogStreamEmpty
}

impl RTDLogStreamEmptyBuilder {
  pub fn build(&self) -> LogStreamEmpty { self.inner.clone() }

}

impl AsRef<LogStreamEmpty> for LogStreamEmpty {
  fn as_ref(&self) -> &LogStreamEmpty { self }
}

impl AsRef<LogStreamEmpty> for RTDLogStreamEmptyBuilder {
  fn as_ref(&self) -> &LogStreamEmpty { &self.inner }
}







/// The log is written to a file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogStreamFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Path to the file to where the internal TDLib log will be written
  path: String,
  /// The maximum size of the file to where the internal TDLib log is written before the file will be auto-rotated, in bytes
  max_file_size: i64,
  /// Pass true to additionally redirect stderr to the log file. Ignored on Windows
  redirect_stderr: bool,
  
}

impl RObject for LogStreamFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "logStreamFile" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDLogStream for LogStreamFile {}



impl LogStreamFile {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDLogStreamFileBuilder {
    let mut inner = LogStreamFile::default();
    inner.td_name = "logStreamFile".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDLogStreamFileBuilder { inner }
  }

  pub fn path(&self) -> &String { &self.path }

  pub fn max_file_size(&self) -> i64 { self.max_file_size }

  pub fn redirect_stderr(&self) -> bool { self.redirect_stderr }

}

#[doc(hidden)]
pub struct RTDLogStreamFileBuilder {
  inner: LogStreamFile
}

impl RTDLogStreamFileBuilder {
  pub fn build(&self) -> LogStreamFile { self.inner.clone() }

   
  pub fn path<T: AsRef<str>>(&mut self, path: T) -> &mut Self {
    self.inner.path = path.as_ref().to_string();
    self
  }

   
  pub fn max_file_size(&mut self, max_file_size: i64) -> &mut Self {
    self.inner.max_file_size = max_file_size;
    self
  }

   
  pub fn redirect_stderr(&mut self, redirect_stderr: bool) -> &mut Self {
    self.inner.redirect_stderr = redirect_stderr;
    self
  }

}

impl AsRef<LogStreamFile> for LogStreamFile {
  fn as_ref(&self) -> &LogStreamFile { self }
}

impl AsRef<LogStreamFile> for RTDLogStreamFileBuilder {
  fn as_ref(&self) -> &LogStreamFile { &self.inner }
}



