
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains information about a country
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CountryInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A two-letter ISO 3166-1 alpha-2 country code
  country_code: String,
  /// Native name of the country
  name: String,
  /// English name of the country
  english_name: String,
  /// True, if the country must be hidden from the list of all countries
  is_hidden: bool,
  /// List of country calling codes
  calling_codes: Vec<String>,
  
}

impl RObject for CountryInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "countryInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl CountryInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDCountryInfoBuilder {
    let mut inner = CountryInfo::default();
    inner.td_name = "countryInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDCountryInfoBuilder { inner }
  }

  pub fn country_code(&self) -> &String { &self.country_code }

  pub fn name(&self) -> &String { &self.name }

  pub fn english_name(&self) -> &String { &self.english_name }

  pub fn is_hidden(&self) -> bool { self.is_hidden }

  pub fn calling_codes(&self) -> &Vec<String> { &self.calling_codes }

}

#[doc(hidden)]
pub struct RTDCountryInfoBuilder {
  inner: CountryInfo
}

impl RTDCountryInfoBuilder {
  pub fn build(&self) -> CountryInfo { self.inner.clone() }

   
  pub fn country_code<T: AsRef<str>>(&mut self, country_code: T) -> &mut Self {
    self.inner.country_code = country_code.as_ref().to_string();
    self
  }

   
  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

   
  pub fn english_name<T: AsRef<str>>(&mut self, english_name: T) -> &mut Self {
    self.inner.english_name = english_name.as_ref().to_string();
    self
  }

   
  pub fn is_hidden(&mut self, is_hidden: bool) -> &mut Self {
    self.inner.is_hidden = is_hidden;
    self
  }

   
  pub fn calling_codes(&mut self, calling_codes: Vec<String>) -> &mut Self {
    self.inner.calling_codes = calling_codes;
    self
  }

}

impl AsRef<CountryInfo> for CountryInfo {
  fn as_ref(&self) -> &CountryInfo { self }
}

impl AsRef<CountryInfo> for RTDCountryInfoBuilder {
  fn as_ref(&self) -> &CountryInfo { &self.inner }
}



