use std::time::Duration;
use windows::Devices::Geolocation::Geolocator;

pub struct Windows {
    locator: Geolocator,
}

impl Windows {
    pub fn new() -> crate::Result<Self> {
        Ok(Self {
            locator: Geolocator::new()?,
        })
    }

    pub fn get_permissions(&mut self) -> crate::Result<()> {
        let status = Geolocator::RequestAccessAsync().and_then(|r| r.get());

        if status != Ok(windows::Devices::Geolocation::GeolocationAccessStatus::Allowed) {
            println!("Launching settings intent!");
            let uri = windows::Foundation::Uri::CreateUri(&"ms-settings:privacy-location".into())
                .unwrap();

            windows::System::Launcher::LaunchUriAsync(&uri).unwrap();
            // Wait a bit and then return failure since we dont know when (or even if) the user
            // will re-enable location to us
            std::thread::sleep(Duration::from_secs(2));
            return Err(crate::Error::PermissionsDenied);
        }

        Ok(())
    }

    pub fn get(&mut self) -> crate::Result<crate::Lla> {
        let pos = self.locator.GetGeopositionAsync()?.get()?;

        let coord = pos.Coordinate()?;
        let point = coord.Point()?;
        let pos = point.Position()?;

        Ok(crate::Lla {
            latitude_degs: pos.Latitude,
            longitude_degs: pos.Longitude,
            altitude_m_hae: pos.Altitude,
        })
    }
}
