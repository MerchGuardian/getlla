use std::time::Duration;

use windows::{Devices::Geolocation::Geolocator, core::Result};
use windows_future::IAsyncOperation;

pub fn go() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime
        .block_on(async move || -> Result<()> {
            let locator = Geolocator::new()?;

            println!("Requesting access...");
            if let Err(e) = Geolocator::RequestAccessAsync() {
                println!("failed to get permissions: {e:?}");
            }

            let pos = locator.GetGeopositionAsync()?.await?;

            let coord = pos.Coordinate()?;
            let point = coord.Point()?;
            let pos = point.Position()?;

            println!("Latitude:  {}", pos.Latitude);
            println!("Longitude: {}", pos.Longitude);
            println!("Altitude:  {} meters", pos.Altitude);

            tokio::time::sleep(Duration::from_secs(5));

            println!("Launching settings intent!");
            let uri = windows::Foundation::Uri::CreateUri(&"ms-settings:privacy-location".into())?;

            tokio::time::sleep(Duration::from_secs(1)).await;
            windows::System::Launcher::LaunchUriAsync(&uri)?;
            tokio::time::sleep(Duration::from_secs(1)).await;

            Ok(())
        }())
        .unwrap()
}
