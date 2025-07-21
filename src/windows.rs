use anyhow::Context;
use windows::{
    Win32::{
        Devices::Sensors::{
            ISensor, ISensorCollection, ISensorDataReport, ISensorManager,
            SENSOR_DATA_TYPE_ALTITUDE_SEALEVEL_METERS, SENSOR_DATA_TYPE_LATITUDE_DEGREES,
            SENSOR_DATA_TYPE_LONGITUDE_DEGREES, SENSOR_TYPE_LOCATION_GPS,
        },
        Foundation::PROPERTYKEY,
        System::{
            Com::StructuredStorage::PropVariantClear,
            Com::{
                CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx,
                CoUninitialize,
            },
        },
    },
    core::{GUID, Interface, Result},
};

pub fn go() -> anyhow::Result<()> {
    unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };

    println!("A");

    // Fails if sensor service not running
    let sensor_manager: ISensorManager =
        unsafe { CoCreateInstance(&ISensorManager::IID, None, CLSCTX_INPROC_SERVER)? };

    println!("B");

    // Query all GPS sensors
    let mut sensors = unsafe { sensor_manager.GetSensorsByType(&SENSOR_TYPE_LOCATION_GPS) }
        .context("No sensor collection returned")?;

    println!("C");

    let mut count = unsafe { sensors.GetCount() }?;

    println!("D");

    if count == 0 {
        println!("No GPS sensors found.");
    } else {
        let mut sensor = unsafe { sensors.GetAt(0) }.context("Failed to get GPS sensor")?;

        println!("E");

        let mut report = unsafe { sensor.GetData() }.context("Failed to get data report")?;

        println!("F");

        // Helper closure to read a field
        unsafe fn read_value(report: &ISensorDataReport, field: &PROPERTYKEY, label: &str) {
            if let Ok(var) = report.GetSensorValue(field as *const _) {
                if unsafe { var.Anonymous.Anonymous.vt } == windows::Win32::System::Variant::VT_R8 {
                    let val = unsafe { var.Anonymous.Anonymous.Anonymous.dblVal };
                    println!("{label}: {val}");
                }
            }
            // PropVariantClear(&mut var as *mut _);
        }

        unsafe {
            read_value(&report, &SENSOR_DATA_TYPE_LATITUDE_DEGREES, "Latitude");
            read_value(&report, &SENSOR_DATA_TYPE_LONGITUDE_DEGREES, "Longitude");
            read_value(
                &report,
                &SENSOR_DATA_TYPE_ALTITUDE_SEALEVEL_METERS,
                "Altitude (meters)",
            );
        }

        println!("G");
    }

    unsafe { CoUninitialize() };

    Ok(())
}
