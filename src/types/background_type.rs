
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes the type of a background
pub trait TDBackgroundType: Debug + RObject {}

/// Describes the type of a background
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum BackgroundType {
  #[doc(hidden)] _Default(()),
  /// A filled background
  Fill(BackgroundTypeFill),
  /// A PNG or TGV (gzipped subset of SVG with MIME type "application/x-tgwallpattern") pattern to be combined with the background fill chosen by the user
  Pattern(BackgroundTypePattern),
  /// A wallpaper in JPEG format
  Wallpaper(BackgroundTypeWallpaper),

}

impl Default for BackgroundType {
  fn default() -> Self { BackgroundType::_Default(()) }
}

impl<'de> Deserialize<'de> for BackgroundType {
  fn deserialize<D>(deserializer: D) -> Result<BackgroundType, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      BackgroundType,
      (backgroundTypeFill, Fill);
      (backgroundTypePattern, Pattern);
      (backgroundTypeWallpaper, Wallpaper);

    )(deserializer)
  }
}

impl RObject for BackgroundType {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      BackgroundType::Fill(t) => t.td_name(),
      BackgroundType::Pattern(t) => t.td_name(),
      BackgroundType::Wallpaper(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      BackgroundType::Fill(t) => t.extra(),
      BackgroundType::Pattern(t) => t.extra(),
      BackgroundType::Wallpaper(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl BackgroundType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let BackgroundType::_Default(_) = self { true } else { false } }

  pub fn is_fill(&self) -> bool { if let BackgroundType::Fill(_) = self { true } else { false } }
  pub fn is_pattern(&self) -> bool { if let BackgroundType::Pattern(_) = self { true } else { false } }
  pub fn is_wallpaper(&self) -> bool { if let BackgroundType::Wallpaper(_) = self { true } else { false } }

  pub fn on_fill<F: FnOnce(&BackgroundTypeFill)>(&self, fnc: F) -> &Self { if let BackgroundType::Fill(t) = self { fnc(t) }; self }
  pub fn on_pattern<F: FnOnce(&BackgroundTypePattern)>(&self, fnc: F) -> &Self { if let BackgroundType::Pattern(t) = self { fnc(t) }; self }
  pub fn on_wallpaper<F: FnOnce(&BackgroundTypeWallpaper)>(&self, fnc: F) -> &Self { if let BackgroundType::Wallpaper(t) = self { fnc(t) }; self }

  pub fn as_fill(&self) -> Option<&BackgroundTypeFill> { if let BackgroundType::Fill(t) = self { return Some(t) } None }
  pub fn as_pattern(&self) -> Option<&BackgroundTypePattern> { if let BackgroundType::Pattern(t) = self { return Some(t) } None }
  pub fn as_wallpaper(&self) -> Option<&BackgroundTypeWallpaper> { if let BackgroundType::Wallpaper(t) = self { return Some(t) } None }



  pub fn fill<T: AsRef<BackgroundTypeFill>>(t: T) -> Self { BackgroundType::Fill(t.as_ref().clone()) }

  pub fn pattern<T: AsRef<BackgroundTypePattern>>(t: T) -> Self { BackgroundType::Pattern(t.as_ref().clone()) }

  pub fn wallpaper<T: AsRef<BackgroundTypeWallpaper>>(t: T) -> Self { BackgroundType::Wallpaper(t.as_ref().clone()) }

}

impl AsRef<BackgroundType> for BackgroundType {
  fn as_ref(&self) -> &BackgroundType { self }
}







/// A filled background
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BackgroundTypeFill {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The background fill
  fill: BackgroundFill,
  
}

impl RObject for BackgroundTypeFill {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "backgroundTypeFill" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDBackgroundType for BackgroundTypeFill {}



impl BackgroundTypeFill {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDBackgroundTypeFillBuilder {
    let mut inner = BackgroundTypeFill::default();
    inner.td_name = "backgroundTypeFill".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDBackgroundTypeFillBuilder { inner }
  }

  pub fn fill(&self) -> &BackgroundFill { &self.fill }

}

#[doc(hidden)]
pub struct RTDBackgroundTypeFillBuilder {
  inner: BackgroundTypeFill
}

impl RTDBackgroundTypeFillBuilder {
  pub fn build(&self) -> BackgroundTypeFill { self.inner.clone() }

   
  pub fn fill<T: AsRef<BackgroundFill>>(&mut self, fill: T) -> &mut Self {
    self.inner.fill = fill.as_ref().clone();
    self
  }

}

impl AsRef<BackgroundTypeFill> for BackgroundTypeFill {
  fn as_ref(&self) -> &BackgroundTypeFill { self }
}

impl AsRef<BackgroundTypeFill> for RTDBackgroundTypeFillBuilder {
  fn as_ref(&self) -> &BackgroundTypeFill { &self.inner }
}







/// A PNG or TGV (gzipped subset of SVG with MIME type "application/x-tgwallpattern") pattern to be combined with the background fill chosen by the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BackgroundTypePattern {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Fill of the background
  fill: BackgroundFill,
  /// Intensity of the pattern when it is shown above the filled background; 0-100.
  intensity: i64,
  /// True, if the background fill must be applied only to the pattern itself. All other pixels are black in this case. For dark themes only
  is_inverted: bool,
  /// True, if the background needs to be slightly moved when device is tilted
  is_moving: bool,
  
}

impl RObject for BackgroundTypePattern {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "backgroundTypePattern" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDBackgroundType for BackgroundTypePattern {}



impl BackgroundTypePattern {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDBackgroundTypePatternBuilder {
    let mut inner = BackgroundTypePattern::default();
    inner.td_name = "backgroundTypePattern".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDBackgroundTypePatternBuilder { inner }
  }

  pub fn fill(&self) -> &BackgroundFill { &self.fill }

  pub fn intensity(&self) -> i64 { self.intensity }

  pub fn is_inverted(&self) -> bool { self.is_inverted }

  pub fn is_moving(&self) -> bool { self.is_moving }

}

#[doc(hidden)]
pub struct RTDBackgroundTypePatternBuilder {
  inner: BackgroundTypePattern
}

impl RTDBackgroundTypePatternBuilder {
  pub fn build(&self) -> BackgroundTypePattern { self.inner.clone() }

   
  pub fn fill<T: AsRef<BackgroundFill>>(&mut self, fill: T) -> &mut Self {
    self.inner.fill = fill.as_ref().clone();
    self
  }

   
  pub fn intensity(&mut self, intensity: i64) -> &mut Self {
    self.inner.intensity = intensity;
    self
  }

   
  pub fn is_inverted(&mut self, is_inverted: bool) -> &mut Self {
    self.inner.is_inverted = is_inverted;
    self
  }

   
  pub fn is_moving(&mut self, is_moving: bool) -> &mut Self {
    self.inner.is_moving = is_moving;
    self
  }

}

impl AsRef<BackgroundTypePattern> for BackgroundTypePattern {
  fn as_ref(&self) -> &BackgroundTypePattern { self }
}

impl AsRef<BackgroundTypePattern> for RTDBackgroundTypePatternBuilder {
  fn as_ref(&self) -> &BackgroundTypePattern { &self.inner }
}







/// A wallpaper in JPEG format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BackgroundTypeWallpaper {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the wallpaper must be downscaled to fit in 450x450 square and then box-blurred with radius 12
  is_blurred: bool,
  /// True, if the background needs to be slightly moved when device is tilted
  is_moving: bool,
  
}

impl RObject for BackgroundTypeWallpaper {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "backgroundTypeWallpaper" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDBackgroundType for BackgroundTypeWallpaper {}



impl BackgroundTypeWallpaper {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDBackgroundTypeWallpaperBuilder {
    let mut inner = BackgroundTypeWallpaper::default();
    inner.td_name = "backgroundTypeWallpaper".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDBackgroundTypeWallpaperBuilder { inner }
  }

  pub fn is_blurred(&self) -> bool { self.is_blurred }

  pub fn is_moving(&self) -> bool { self.is_moving }

}

#[doc(hidden)]
pub struct RTDBackgroundTypeWallpaperBuilder {
  inner: BackgroundTypeWallpaper
}

impl RTDBackgroundTypeWallpaperBuilder {
  pub fn build(&self) -> BackgroundTypeWallpaper { self.inner.clone() }

   
  pub fn is_blurred(&mut self, is_blurred: bool) -> &mut Self {
    self.inner.is_blurred = is_blurred;
    self
  }

   
  pub fn is_moving(&mut self, is_moving: bool) -> &mut Self {
    self.inner.is_moving = is_moving;
    self
  }

}

impl AsRef<BackgroundTypeWallpaper> for BackgroundTypeWallpaper {
  fn as_ref(&self) -> &BackgroundTypeWallpaper { self }
}

impl AsRef<BackgroundTypeWallpaper> for RTDBackgroundTypeWallpaperBuilder {
  fn as_ref(&self) -> &BackgroundTypeWallpaper { &self.inner }
}



