
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Represents the current authorization state of the TDLib client
pub trait TDAuthorizationState: Debug + RObject {}

/// Represents the current authorization state of the TDLib client
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum AuthorizationState {
  #[doc(hidden)] _Default(()),
  /// TDLib client is in its final state. All databases are closed and all resources are released. No other updates will be received after this. All queries will be responded to with error code 500. To continue working, one must create a new instance of the TDLib client
  Closed(AuthorizationStateClosed),
  /// TDLib is closing, all subsequent queries will be answered with the error 500. Note that closing TDLib can take a while. All resources will be freed only after authorizationStateClosed has been received
  Closing(AuthorizationStateClosing),
  /// The user is currently logging out
  LoggingOut(AuthorizationStateLoggingOut),
  /// The user has been successfully authorized. TDLib is now ready to answer queries
  Ready(AuthorizationStateReady),
  /// TDLib needs the user's authentication code to authorize
  WaitCode(AuthorizationStateWaitCode),
  /// TDLib needs an encryption key to decrypt the local database
  WaitEncryptionKey(AuthorizationStateWaitEncryptionKey),
  /// The user needs to confirm authorization on another logged in device by scanning a QR code with the provided link
  WaitOtherDeviceConfirmation(AuthorizationStateWaitOtherDeviceConfirmation),
  /// The user has been authorized, but needs to enter a password to start using the application
  WaitPassword(AuthorizationStateWaitPassword),
  /// TDLib needs the user's phone number to authorize. Call `setAuthenticationPhoneNumber` to provide the phone number, or use `requestQrCodeAuthentication`, or `checkAuthenticationBotToken` for other authentication options
  WaitPhoneNumber(AuthorizationStateWaitPhoneNumber),
  /// The user is unregistered and need to accept terms of service and enter their first name and last name to finish registration
  WaitRegistration(AuthorizationStateWaitRegistration),
  /// TDLib needs TdlibParameters for initialization
  WaitTdlibParameters(AuthorizationStateWaitTdlibParameters),
  /// Returns the current authorization state; this is an offline request. For informational purposes only. Use updateAuthorizationState instead to maintain the current authorization state. Can be called before initialization
  GetAuthorizationState(GetAuthorizationState),

}

impl Default for AuthorizationState {
  fn default() -> Self { AuthorizationState::_Default(()) }
}

impl<'de> Deserialize<'de> for AuthorizationState {
  fn deserialize<D>(deserializer: D) -> Result<AuthorizationState, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      AuthorizationState,
      (authorizationStateClosed, Closed);
      (authorizationStateClosing, Closing);
      (authorizationStateLoggingOut, LoggingOut);
      (authorizationStateReady, Ready);
      (authorizationStateWaitCode, WaitCode);
      (authorizationStateWaitEncryptionKey, WaitEncryptionKey);
      (authorizationStateWaitOtherDeviceConfirmation, WaitOtherDeviceConfirmation);
      (authorizationStateWaitPassword, WaitPassword);
      (authorizationStateWaitPhoneNumber, WaitPhoneNumber);
      (authorizationStateWaitRegistration, WaitRegistration);
      (authorizationStateWaitTdlibParameters, WaitTdlibParameters);
      (getAuthorizationState, GetAuthorizationState);

    )(deserializer)
  }
}

impl RObject for AuthorizationState {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      AuthorizationState::Closed(t) => t.td_name(),
      AuthorizationState::Closing(t) => t.td_name(),
      AuthorizationState::LoggingOut(t) => t.td_name(),
      AuthorizationState::Ready(t) => t.td_name(),
      AuthorizationState::WaitCode(t) => t.td_name(),
      AuthorizationState::WaitEncryptionKey(t) => t.td_name(),
      AuthorizationState::WaitOtherDeviceConfirmation(t) => t.td_name(),
      AuthorizationState::WaitPassword(t) => t.td_name(),
      AuthorizationState::WaitPhoneNumber(t) => t.td_name(),
      AuthorizationState::WaitRegistration(t) => t.td_name(),
      AuthorizationState::WaitTdlibParameters(t) => t.td_name(),
      AuthorizationState::GetAuthorizationState(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      AuthorizationState::Closed(t) => t.extra(),
      AuthorizationState::Closing(t) => t.extra(),
      AuthorizationState::LoggingOut(t) => t.extra(),
      AuthorizationState::Ready(t) => t.extra(),
      AuthorizationState::WaitCode(t) => t.extra(),
      AuthorizationState::WaitEncryptionKey(t) => t.extra(),
      AuthorizationState::WaitOtherDeviceConfirmation(t) => t.extra(),
      AuthorizationState::WaitPassword(t) => t.extra(),
      AuthorizationState::WaitPhoneNumber(t) => t.extra(),
      AuthorizationState::WaitRegistration(t) => t.extra(),
      AuthorizationState::WaitTdlibParameters(t) => t.extra(),
      AuthorizationState::GetAuthorizationState(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl AuthorizationState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let AuthorizationState::_Default(_) = self { true } else { false } }

  pub fn is_closed(&self) -> bool { if let AuthorizationState::Closed(_) = self { true } else { false } }
  pub fn is_closing(&self) -> bool { if let AuthorizationState::Closing(_) = self { true } else { false } }
  pub fn is_logging_out(&self) -> bool { if let AuthorizationState::LoggingOut(_) = self { true } else { false } }
  pub fn is_ready(&self) -> bool { if let AuthorizationState::Ready(_) = self { true } else { false } }
  pub fn is_wait_code(&self) -> bool { if let AuthorizationState::WaitCode(_) = self { true } else { false } }
  pub fn is_wait_encryption_key(&self) -> bool { if let AuthorizationState::WaitEncryptionKey(_) = self { true } else { false } }
  pub fn is_wait_other_device_confirmation(&self) -> bool { if let AuthorizationState::WaitOtherDeviceConfirmation(_) = self { true } else { false } }
  pub fn is_wait_password(&self) -> bool { if let AuthorizationState::WaitPassword(_) = self { true } else { false } }
  pub fn is_wait_phone_number(&self) -> bool { if let AuthorizationState::WaitPhoneNumber(_) = self { true } else { false } }
  pub fn is_wait_registration(&self) -> bool { if let AuthorizationState::WaitRegistration(_) = self { true } else { false } }
  pub fn is_wait_tdlib_parameters(&self) -> bool { if let AuthorizationState::WaitTdlibParameters(_) = self { true } else { false } }
  pub fn is_get_authorization_state(&self) -> bool { if let AuthorizationState::GetAuthorizationState(_) = self { true } else { false } }

  pub fn on_closed<F: FnOnce(&AuthorizationStateClosed)>(&self, fnc: F) -> &Self { if let AuthorizationState::Closed(t) = self { fnc(t) }; self }
  pub fn on_closing<F: FnOnce(&AuthorizationStateClosing)>(&self, fnc: F) -> &Self { if let AuthorizationState::Closing(t) = self { fnc(t) }; self }
  pub fn on_logging_out<F: FnOnce(&AuthorizationStateLoggingOut)>(&self, fnc: F) -> &Self { if let AuthorizationState::LoggingOut(t) = self { fnc(t) }; self }
  pub fn on_ready<F: FnOnce(&AuthorizationStateReady)>(&self, fnc: F) -> &Self { if let AuthorizationState::Ready(t) = self { fnc(t) }; self }
  pub fn on_wait_code<F: FnOnce(&AuthorizationStateWaitCode)>(&self, fnc: F) -> &Self { if let AuthorizationState::WaitCode(t) = self { fnc(t) }; self }
  pub fn on_wait_encryption_key<F: FnOnce(&AuthorizationStateWaitEncryptionKey)>(&self, fnc: F) -> &Self { if let AuthorizationState::WaitEncryptionKey(t) = self { fnc(t) }; self }
  pub fn on_wait_other_device_confirmation<F: FnOnce(&AuthorizationStateWaitOtherDeviceConfirmation)>(&self, fnc: F) -> &Self { if let AuthorizationState::WaitOtherDeviceConfirmation(t) = self { fnc(t) }; self }
  pub fn on_wait_password<F: FnOnce(&AuthorizationStateWaitPassword)>(&self, fnc: F) -> &Self { if let AuthorizationState::WaitPassword(t) = self { fnc(t) }; self }
  pub fn on_wait_phone_number<F: FnOnce(&AuthorizationStateWaitPhoneNumber)>(&self, fnc: F) -> &Self { if let AuthorizationState::WaitPhoneNumber(t) = self { fnc(t) }; self }
  pub fn on_wait_registration<F: FnOnce(&AuthorizationStateWaitRegistration)>(&self, fnc: F) -> &Self { if let AuthorizationState::WaitRegistration(t) = self { fnc(t) }; self }
  pub fn on_wait_tdlib_parameters<F: FnOnce(&AuthorizationStateWaitTdlibParameters)>(&self, fnc: F) -> &Self { if let AuthorizationState::WaitTdlibParameters(t) = self { fnc(t) }; self }
  pub fn on_get_authorization_state<F: FnOnce(&GetAuthorizationState)>(&self, fnc: F) -> &Self { if let AuthorizationState::GetAuthorizationState(t) = self { fnc(t) }; self }

  pub fn as_closed(&self) -> Option<&AuthorizationStateClosed> { if let AuthorizationState::Closed(t) = self { return Some(t) } None }
  pub fn as_closing(&self) -> Option<&AuthorizationStateClosing> { if let AuthorizationState::Closing(t) = self { return Some(t) } None }
  pub fn as_logging_out(&self) -> Option<&AuthorizationStateLoggingOut> { if let AuthorizationState::LoggingOut(t) = self { return Some(t) } None }
  pub fn as_ready(&self) -> Option<&AuthorizationStateReady> { if let AuthorizationState::Ready(t) = self { return Some(t) } None }
  pub fn as_wait_code(&self) -> Option<&AuthorizationStateWaitCode> { if let AuthorizationState::WaitCode(t) = self { return Some(t) } None }
  pub fn as_wait_encryption_key(&self) -> Option<&AuthorizationStateWaitEncryptionKey> { if let AuthorizationState::WaitEncryptionKey(t) = self { return Some(t) } None }
  pub fn as_wait_other_device_confirmation(&self) -> Option<&AuthorizationStateWaitOtherDeviceConfirmation> { if let AuthorizationState::WaitOtherDeviceConfirmation(t) = self { return Some(t) } None }
  pub fn as_wait_password(&self) -> Option<&AuthorizationStateWaitPassword> { if let AuthorizationState::WaitPassword(t) = self { return Some(t) } None }
  pub fn as_wait_phone_number(&self) -> Option<&AuthorizationStateWaitPhoneNumber> { if let AuthorizationState::WaitPhoneNumber(t) = self { return Some(t) } None }
  pub fn as_wait_registration(&self) -> Option<&AuthorizationStateWaitRegistration> { if let AuthorizationState::WaitRegistration(t) = self { return Some(t) } None }
  pub fn as_wait_tdlib_parameters(&self) -> Option<&AuthorizationStateWaitTdlibParameters> { if let AuthorizationState::WaitTdlibParameters(t) = self { return Some(t) } None }
  pub fn as_get_authorization_state(&self) -> Option<&GetAuthorizationState> { if let AuthorizationState::GetAuthorizationState(t) = self { return Some(t) } None }



  pub fn closed<T: AsRef<AuthorizationStateClosed>>(t: T) -> Self { AuthorizationState::Closed(t.as_ref().clone()) }

  pub fn closing<T: AsRef<AuthorizationStateClosing>>(t: T) -> Self { AuthorizationState::Closing(t.as_ref().clone()) }

  pub fn logging_out<T: AsRef<AuthorizationStateLoggingOut>>(t: T) -> Self { AuthorizationState::LoggingOut(t.as_ref().clone()) }

  pub fn ready<T: AsRef<AuthorizationStateReady>>(t: T) -> Self { AuthorizationState::Ready(t.as_ref().clone()) }

  pub fn wait_code<T: AsRef<AuthorizationStateWaitCode>>(t: T) -> Self { AuthorizationState::WaitCode(t.as_ref().clone()) }

  pub fn wait_encryption_key<T: AsRef<AuthorizationStateWaitEncryptionKey>>(t: T) -> Self { AuthorizationState::WaitEncryptionKey(t.as_ref().clone()) }

  pub fn wait_other_device_confirmation<T: AsRef<AuthorizationStateWaitOtherDeviceConfirmation>>(t: T) -> Self { AuthorizationState::WaitOtherDeviceConfirmation(t.as_ref().clone()) }

  pub fn wait_password<T: AsRef<AuthorizationStateWaitPassword>>(t: T) -> Self { AuthorizationState::WaitPassword(t.as_ref().clone()) }

  pub fn wait_phone_number<T: AsRef<AuthorizationStateWaitPhoneNumber>>(t: T) -> Self { AuthorizationState::WaitPhoneNumber(t.as_ref().clone()) }

  pub fn wait_registration<T: AsRef<AuthorizationStateWaitRegistration>>(t: T) -> Self { AuthorizationState::WaitRegistration(t.as_ref().clone()) }

  pub fn wait_tdlib_parameters<T: AsRef<AuthorizationStateWaitTdlibParameters>>(t: T) -> Self { AuthorizationState::WaitTdlibParameters(t.as_ref().clone()) }

  pub fn get_authorization_state<T: AsRef<GetAuthorizationState>>(t: T) -> Self { AuthorizationState::GetAuthorizationState(t.as_ref().clone()) }

}

impl AsRef<AuthorizationState> for AuthorizationState {
  fn as_ref(&self) -> &AuthorizationState { self }
}







/// TDLib client is in its final state. All databases are closed and all resources are released. No other updates will be received after this. All queries will be responded to with error code 500. To continue working, one must create a new instance of the TDLib client
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateClosed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for AuthorizationStateClosed {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateClosed" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDAuthorizationState for AuthorizationStateClosed {}



impl AuthorizationStateClosed {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAuthorizationStateClosedBuilder {
    let mut inner = AuthorizationStateClosed::default();
    inner.td_name = "authorizationStateClosed".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAuthorizationStateClosedBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDAuthorizationStateClosedBuilder {
  inner: AuthorizationStateClosed
}

impl RTDAuthorizationStateClosedBuilder {
  pub fn build(&self) -> AuthorizationStateClosed { self.inner.clone() }

}

impl AsRef<AuthorizationStateClosed> for AuthorizationStateClosed {
  fn as_ref(&self) -> &AuthorizationStateClosed { self }
}

impl AsRef<AuthorizationStateClosed> for RTDAuthorizationStateClosedBuilder {
  fn as_ref(&self) -> &AuthorizationStateClosed { &self.inner }
}







/// TDLib is closing, all subsequent queries will be answered with the error 500. Note that closing TDLib can take a while. All resources will be freed only after authorizationStateClosed has been received
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateClosing {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for AuthorizationStateClosing {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateClosing" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDAuthorizationState for AuthorizationStateClosing {}



impl AuthorizationStateClosing {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAuthorizationStateClosingBuilder {
    let mut inner = AuthorizationStateClosing::default();
    inner.td_name = "authorizationStateClosing".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAuthorizationStateClosingBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDAuthorizationStateClosingBuilder {
  inner: AuthorizationStateClosing
}

impl RTDAuthorizationStateClosingBuilder {
  pub fn build(&self) -> AuthorizationStateClosing { self.inner.clone() }

}

impl AsRef<AuthorizationStateClosing> for AuthorizationStateClosing {
  fn as_ref(&self) -> &AuthorizationStateClosing { self }
}

impl AsRef<AuthorizationStateClosing> for RTDAuthorizationStateClosingBuilder {
  fn as_ref(&self) -> &AuthorizationStateClosing { &self.inner }
}







/// The user is currently logging out
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateLoggingOut {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for AuthorizationStateLoggingOut {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateLoggingOut" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDAuthorizationState for AuthorizationStateLoggingOut {}



impl AuthorizationStateLoggingOut {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAuthorizationStateLoggingOutBuilder {
    let mut inner = AuthorizationStateLoggingOut::default();
    inner.td_name = "authorizationStateLoggingOut".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAuthorizationStateLoggingOutBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDAuthorizationStateLoggingOutBuilder {
  inner: AuthorizationStateLoggingOut
}

impl RTDAuthorizationStateLoggingOutBuilder {
  pub fn build(&self) -> AuthorizationStateLoggingOut { self.inner.clone() }

}

impl AsRef<AuthorizationStateLoggingOut> for AuthorizationStateLoggingOut {
  fn as_ref(&self) -> &AuthorizationStateLoggingOut { self }
}

impl AsRef<AuthorizationStateLoggingOut> for RTDAuthorizationStateLoggingOutBuilder {
  fn as_ref(&self) -> &AuthorizationStateLoggingOut { &self.inner }
}







/// The user has been successfully authorized. TDLib is now ready to answer queries
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateReady {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for AuthorizationStateReady {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateReady" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDAuthorizationState for AuthorizationStateReady {}



impl AuthorizationStateReady {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAuthorizationStateReadyBuilder {
    let mut inner = AuthorizationStateReady::default();
    inner.td_name = "authorizationStateReady".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAuthorizationStateReadyBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDAuthorizationStateReadyBuilder {
  inner: AuthorizationStateReady
}

impl RTDAuthorizationStateReadyBuilder {
  pub fn build(&self) -> AuthorizationStateReady { self.inner.clone() }

}

impl AsRef<AuthorizationStateReady> for AuthorizationStateReady {
  fn as_ref(&self) -> &AuthorizationStateReady { self }
}

impl AsRef<AuthorizationStateReady> for RTDAuthorizationStateReadyBuilder {
  fn as_ref(&self) -> &AuthorizationStateReady { &self.inner }
}







/// TDLib needs the user's authentication code to authorize
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Information about the authorization code that was sent
  code_info: AuthenticationCodeInfo,
  
}

impl RObject for AuthorizationStateWaitCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateWaitCode" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDAuthorizationState for AuthorizationStateWaitCode {}



impl AuthorizationStateWaitCode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAuthorizationStateWaitCodeBuilder {
    let mut inner = AuthorizationStateWaitCode::default();
    inner.td_name = "authorizationStateWaitCode".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAuthorizationStateWaitCodeBuilder { inner }
  }

  pub fn code_info(&self) -> &AuthenticationCodeInfo { &self.code_info }

}

#[doc(hidden)]
pub struct RTDAuthorizationStateWaitCodeBuilder {
  inner: AuthorizationStateWaitCode
}

impl RTDAuthorizationStateWaitCodeBuilder {
  pub fn build(&self) -> AuthorizationStateWaitCode { self.inner.clone() }

   
  pub fn code_info<T: AsRef<AuthenticationCodeInfo>>(&mut self, code_info: T) -> &mut Self {
    self.inner.code_info = code_info.as_ref().clone();
    self
  }

}

impl AsRef<AuthorizationStateWaitCode> for AuthorizationStateWaitCode {
  fn as_ref(&self) -> &AuthorizationStateWaitCode { self }
}

impl AsRef<AuthorizationStateWaitCode> for RTDAuthorizationStateWaitCodeBuilder {
  fn as_ref(&self) -> &AuthorizationStateWaitCode { &self.inner }
}







/// TDLib needs an encryption key to decrypt the local database
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitEncryptionKey {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the database is currently encrypted
  is_encrypted: bool,
  
}

impl RObject for AuthorizationStateWaitEncryptionKey {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateWaitEncryptionKey" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDAuthorizationState for AuthorizationStateWaitEncryptionKey {}



impl AuthorizationStateWaitEncryptionKey {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAuthorizationStateWaitEncryptionKeyBuilder {
    let mut inner = AuthorizationStateWaitEncryptionKey::default();
    inner.td_name = "authorizationStateWaitEncryptionKey".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAuthorizationStateWaitEncryptionKeyBuilder { inner }
  }

  pub fn is_encrypted(&self) -> bool { self.is_encrypted }

}

#[doc(hidden)]
pub struct RTDAuthorizationStateWaitEncryptionKeyBuilder {
  inner: AuthorizationStateWaitEncryptionKey
}

impl RTDAuthorizationStateWaitEncryptionKeyBuilder {
  pub fn build(&self) -> AuthorizationStateWaitEncryptionKey { self.inner.clone() }

   
  pub fn is_encrypted(&mut self, is_encrypted: bool) -> &mut Self {
    self.inner.is_encrypted = is_encrypted;
    self
  }

}

impl AsRef<AuthorizationStateWaitEncryptionKey> for AuthorizationStateWaitEncryptionKey {
  fn as_ref(&self) -> &AuthorizationStateWaitEncryptionKey { self }
}

impl AsRef<AuthorizationStateWaitEncryptionKey> for RTDAuthorizationStateWaitEncryptionKeyBuilder {
  fn as_ref(&self) -> &AuthorizationStateWaitEncryptionKey { &self.inner }
}







/// The user needs to confirm authorization on another logged in device by scanning a QR code with the provided link
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitOtherDeviceConfirmation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A tg:// URL for the QR code. The link will be updated frequently
  link: String,
  
}

impl RObject for AuthorizationStateWaitOtherDeviceConfirmation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateWaitOtherDeviceConfirmation" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDAuthorizationState for AuthorizationStateWaitOtherDeviceConfirmation {}



impl AuthorizationStateWaitOtherDeviceConfirmation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAuthorizationStateWaitOtherDeviceConfirmationBuilder {
    let mut inner = AuthorizationStateWaitOtherDeviceConfirmation::default();
    inner.td_name = "authorizationStateWaitOtherDeviceConfirmation".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAuthorizationStateWaitOtherDeviceConfirmationBuilder { inner }
  }

  pub fn link(&self) -> &String { &self.link }

}

#[doc(hidden)]
pub struct RTDAuthorizationStateWaitOtherDeviceConfirmationBuilder {
  inner: AuthorizationStateWaitOtherDeviceConfirmation
}

impl RTDAuthorizationStateWaitOtherDeviceConfirmationBuilder {
  pub fn build(&self) -> AuthorizationStateWaitOtherDeviceConfirmation { self.inner.clone() }

   
  pub fn link<T: AsRef<str>>(&mut self, link: T) -> &mut Self {
    self.inner.link = link.as_ref().to_string();
    self
  }

}

impl AsRef<AuthorizationStateWaitOtherDeviceConfirmation> for AuthorizationStateWaitOtherDeviceConfirmation {
  fn as_ref(&self) -> &AuthorizationStateWaitOtherDeviceConfirmation { self }
}

impl AsRef<AuthorizationStateWaitOtherDeviceConfirmation> for RTDAuthorizationStateWaitOtherDeviceConfirmationBuilder {
  fn as_ref(&self) -> &AuthorizationStateWaitOtherDeviceConfirmation { &self.inner }
}







/// The user has been authorized, but needs to enter a password to start using the application
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitPassword {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Hint for the password; may be empty
  password_hint: String,
  /// True, if a recovery email address has been set up
  has_recovery_email_address: bool,
  /// Pattern of the email address to which the recovery email was sent; empty until a recovery email has been sent
  recovery_email_address_pattern: String,
  
}

impl RObject for AuthorizationStateWaitPassword {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateWaitPassword" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDAuthorizationState for AuthorizationStateWaitPassword {}



impl AuthorizationStateWaitPassword {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAuthorizationStateWaitPasswordBuilder {
    let mut inner = AuthorizationStateWaitPassword::default();
    inner.td_name = "authorizationStateWaitPassword".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAuthorizationStateWaitPasswordBuilder { inner }
  }

  pub fn password_hint(&self) -> &String { &self.password_hint }

  pub fn has_recovery_email_address(&self) -> bool { self.has_recovery_email_address }

  pub fn recovery_email_address_pattern(&self) -> &String { &self.recovery_email_address_pattern }

}

#[doc(hidden)]
pub struct RTDAuthorizationStateWaitPasswordBuilder {
  inner: AuthorizationStateWaitPassword
}

impl RTDAuthorizationStateWaitPasswordBuilder {
  pub fn build(&self) -> AuthorizationStateWaitPassword { self.inner.clone() }

   
  pub fn password_hint<T: AsRef<str>>(&mut self, password_hint: T) -> &mut Self {
    self.inner.password_hint = password_hint.as_ref().to_string();
    self
  }

   
  pub fn has_recovery_email_address(&mut self, has_recovery_email_address: bool) -> &mut Self {
    self.inner.has_recovery_email_address = has_recovery_email_address;
    self
  }

   
  pub fn recovery_email_address_pattern<T: AsRef<str>>(&mut self, recovery_email_address_pattern: T) -> &mut Self {
    self.inner.recovery_email_address_pattern = recovery_email_address_pattern.as_ref().to_string();
    self
  }

}

impl AsRef<AuthorizationStateWaitPassword> for AuthorizationStateWaitPassword {
  fn as_ref(&self) -> &AuthorizationStateWaitPassword { self }
}

impl AsRef<AuthorizationStateWaitPassword> for RTDAuthorizationStateWaitPasswordBuilder {
  fn as_ref(&self) -> &AuthorizationStateWaitPassword { &self.inner }
}







/// TDLib needs the user's phone number to authorize. Call `setAuthenticationPhoneNumber` to provide the phone number, or use `requestQrCodeAuthentication`, or `checkAuthenticationBotToken` for other authentication options
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitPhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for AuthorizationStateWaitPhoneNumber {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateWaitPhoneNumber" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDAuthorizationState for AuthorizationStateWaitPhoneNumber {}



impl AuthorizationStateWaitPhoneNumber {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAuthorizationStateWaitPhoneNumberBuilder {
    let mut inner = AuthorizationStateWaitPhoneNumber::default();
    inner.td_name = "authorizationStateWaitPhoneNumber".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAuthorizationStateWaitPhoneNumberBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDAuthorizationStateWaitPhoneNumberBuilder {
  inner: AuthorizationStateWaitPhoneNumber
}

impl RTDAuthorizationStateWaitPhoneNumberBuilder {
  pub fn build(&self) -> AuthorizationStateWaitPhoneNumber { self.inner.clone() }

}

impl AsRef<AuthorizationStateWaitPhoneNumber> for AuthorizationStateWaitPhoneNumber {
  fn as_ref(&self) -> &AuthorizationStateWaitPhoneNumber { self }
}

impl AsRef<AuthorizationStateWaitPhoneNumber> for RTDAuthorizationStateWaitPhoneNumberBuilder {
  fn as_ref(&self) -> &AuthorizationStateWaitPhoneNumber { &self.inner }
}







/// The user is unregistered and need to accept terms of service and enter their first name and last name to finish registration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitRegistration {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Telegram terms of service
  terms_of_service: TermsOfService,
  
}

impl RObject for AuthorizationStateWaitRegistration {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateWaitRegistration" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDAuthorizationState for AuthorizationStateWaitRegistration {}



impl AuthorizationStateWaitRegistration {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAuthorizationStateWaitRegistrationBuilder {
    let mut inner = AuthorizationStateWaitRegistration::default();
    inner.td_name = "authorizationStateWaitRegistration".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAuthorizationStateWaitRegistrationBuilder { inner }
  }

  pub fn terms_of_service(&self) -> &TermsOfService { &self.terms_of_service }

}

#[doc(hidden)]
pub struct RTDAuthorizationStateWaitRegistrationBuilder {
  inner: AuthorizationStateWaitRegistration
}

impl RTDAuthorizationStateWaitRegistrationBuilder {
  pub fn build(&self) -> AuthorizationStateWaitRegistration { self.inner.clone() }

   
  pub fn terms_of_service<T: AsRef<TermsOfService>>(&mut self, terms_of_service: T) -> &mut Self {
    self.inner.terms_of_service = terms_of_service.as_ref().clone();
    self
  }

}

impl AsRef<AuthorizationStateWaitRegistration> for AuthorizationStateWaitRegistration {
  fn as_ref(&self) -> &AuthorizationStateWaitRegistration { self }
}

impl AsRef<AuthorizationStateWaitRegistration> for RTDAuthorizationStateWaitRegistrationBuilder {
  fn as_ref(&self) -> &AuthorizationStateWaitRegistration { &self.inner }
}







/// TDLib needs TdlibParameters for initialization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuthorizationStateWaitTdlibParameters {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for AuthorizationStateWaitTdlibParameters {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authorizationStateWaitTdlibParameters" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDAuthorizationState for AuthorizationStateWaitTdlibParameters {}



impl AuthorizationStateWaitTdlibParameters {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDAuthorizationStateWaitTdlibParametersBuilder {
    let mut inner = AuthorizationStateWaitTdlibParameters::default();
    inner.td_name = "authorizationStateWaitTdlibParameters".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDAuthorizationStateWaitTdlibParametersBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDAuthorizationStateWaitTdlibParametersBuilder {
  inner: AuthorizationStateWaitTdlibParameters
}

impl RTDAuthorizationStateWaitTdlibParametersBuilder {
  pub fn build(&self) -> AuthorizationStateWaitTdlibParameters { self.inner.clone() }

}

impl AsRef<AuthorizationStateWaitTdlibParameters> for AuthorizationStateWaitTdlibParameters {
  fn as_ref(&self) -> &AuthorizationStateWaitTdlibParameters { self }
}

impl AsRef<AuthorizationStateWaitTdlibParameters> for RTDAuthorizationStateWaitTdlibParametersBuilder {
  fn as_ref(&self) -> &AuthorizationStateWaitTdlibParameters { &self.inner }
}



