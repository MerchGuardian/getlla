use std::marker::PhantomData;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "android")]
mod android;

#[cfg(target_os = "windows")]
mod windows;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unsupported platform")]
    Unsupported,
    /// Permissions to access location / device are not allowed, and cannot be accessed until the
    /// user manually intervenes.
    #[error("Permissions denied")]
    PermissionsDenied,
    #[cfg(target_os = "windows")]
    #[error("{0}")]
    Windows(#[from] windows_result::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Getter {
    inner: Inner,
    _phantom: PhantomData<*mut ()>,
}

enum Inner {
    #[cfg(target_os = "android")]
    Android(),
    #[cfg(target_os = "macos")]
    Macos(),
    #[cfg(target_os = "windows")]
    Windows(crate::windows::Windows),
}

impl Getter {
    /// Attempts to return the default getter which can be used to obtain GPS location.
    pub fn system_default() -> Result<Self> {
        #[cfg(target_os = "android")]
        unimplemented!();
        #[cfg(target_os = "macos")]
        unimplemented!();
        #[cfg(target_os = "windows")]
        return Ok(crate::windows::Windows::new()?.into());
        #[cfg(all(
            not(target_os = "android"),
            not(target_os = "macos"),
            not(target_os = "windows")
        ))]
        return Err(Error::Unsupported);
    }

    /// Ensures this application has permissions to access GPS location.
    /// If permissions are already granted, this is a nop.
    pub fn get_permissions(&mut self) -> Result<()> {
        match &mut self.inner {
            #[cfg(target_os = "macos")]
            Inner::Macos(macos) => macos.get_permissions(),
            #[cfg(target_os = "android")]
            Inner::Android(android) => android.get_permissions(),
            #[cfg(target_os = "windows")]
            Inner::Windows(windows) => windows.get_permissions(),
            _ => Err(Error::Unsupported),
        }
    }

    /// Gets the current GPS position, blocking until it becomes available.
    pub fn get(&mut self) -> Result<Lla> {
        match &mut self.inner {
            #[cfg(target_os = "macos")]
            Inner::Macos(macos) => macos.get(),
            #[cfg(target_os = "android")]
            Inner::Android(android) => android.get(),
            #[cfg(target_os = "windows")]
            Inner::Windows(windows) => windows.get(),
            _ => Err(Error::Unsupported),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Lla {
    pub latitude_degs: f64,
    pub longitude_degs: f64,
    /// Altitude (m) above WGS84 ellipsoid.
    pub altitude_m_hae: f64,
}

#[cfg(target_os = "windows")]
impl From<crate::windows::Windows> for Getter {
    fn from(inner: crate::windows::Windows) -> Self {
        Self {
            inner: Inner::Windows(inner),
            _phantom: PhantomData,
        }
    }
}
