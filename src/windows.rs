use windows::{Devices::Geolocation::Geolocator, core::Result};
use windows_future::IAsyncOperation;

pub fn go() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime
        .block_on(async move || -> Result<()> {
            let locator = Geolocator::new()?;
            let pos = locator.GetGeopositionAsync()?.await?;

            let coord = pos.Coordinate()?;
            let point = coord.Point()?;
            let pos = point.Position()?;

            println!("Latitude:  {}", pos.Latitude);
            println!("Longitude: {}", pos.Longitude);
            println!("Altitude:  {} meters", pos.Altitude);

            Ok(())
        }())
        .unwrap()
}
