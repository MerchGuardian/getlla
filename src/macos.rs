use std::cell::{Cell, RefCell};

use objc2_core_location::{CLLocationManager, CLAuthorizationStatus, CLLocation, CLLocationManagerDelegate};
use objc2_foundation::{NSObject, NSObjectProtocol, MainThreadMarker, NSArray, NSError, NSRunLoop};
use objc2::{DefinedClass, rc::Retained, MainThreadOnly, define_class, msg_send, runtime::ProtocolObject};
use objc2_core_foundation::CFRunLoop;

use crate::Lla;

extern crate objc2_core_location;

/// Instance variables for the LocationManagerDelegate, and the primary method for communicating results back
#[derive(Default)]
struct DelegateIVars {
    location: RefCell<Option<Lla>>
}

define_class!{

    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - `Delegate` does not implement `Drop`.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = DelegateIVars]
    struct Delegate;

    unsafe impl NSObjectProtocol for Delegate { }
    unsafe impl CLLocationManagerDelegate for Delegate {
        #[unsafe(method(locationManager:didUpdateLocations:))]
        fn locationManager_didUpdateLocations(&self, lm: &CLLocationManager, locs: &NSArray<CLLocation>) {
            let lmLla = locs.firstObject().expect("locs has at least one coordinate");

            let coordinate = unsafe { lmLla.coordinate() };
            let altitude = unsafe { lmLla.altitude() };

            let lla = Lla {
                latitude_degs: coordinate.latitude,
                longitude_degs: coordinate.longitude,
                altitude_m_hae: altitude
            };            

            self.ivars().location.replace(Some(lla));

            // return control flow to run loop run so we can do something with the location
            CFRunLoop::stop(&CFRunLoop::current().expect("must be started from a runloop"));
        }

        #[unsafe(method(locationManager:didFailWithError:))]
        fn locationManager_didFailWithError(&self, lm: &CLLocationManager, error: &NSError) {
            dbg!(error);
            dbg!(lm);
        }

        #[unsafe(method(locationManagerDidChangeAuthorization:))]
        unsafe fn locationManagerDidChangeAuthorization(&self, manager: &CLLocationManager) {
            println!("location status changed");
            printStatus(manager)
        }
    }
}

impl Delegate {
    fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(DelegateIVars::default());
        unsafe { msg_send![super(this), init] }
    }
}

fn printStatus(lm: &CLLocationManager) {
    match unsafe {lm.authorizationStatus()} {
        CLAuthorizationStatus::NotDetermined => {
            println!("Undetermined authorization, requesting when in use");
        },
        CLAuthorizationStatus::Restricted => {
            println!("Restricted");
        },
        CLAuthorizationStatus::AuthorizedAlways => {
            println!("Always");
        },
        CLAuthorizationStatus::AuthorizedWhenInUse => {
            println!("When in use");
        }
        o => {
            println!("Other auth value {o:?}");
        }
    }
}

pub struct Macos {
    /// MacOS LocationManager
    lm: Retained<CLLocationManager>,

    /// This crate's delegate for the LocationManager
    delegate: Retained<Delegate>
}

impl Macos {
    /// Contract: This must be called on the main thread
    pub fn new() -> crate::Result<Self> {
        let lm = unsafe { CLLocationManager::new() };

        let mtm = MainThreadMarker::new().unwrap();
        let delegate = Delegate::new(mtm);
        unsafe {
            lm.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));
        }

        Ok(Self {
            lm,
            delegate
        })
    }
}

impl crate::Backend for Macos {
    fn get(&mut self) -> crate::Result<Lla> {
        unsafe {
            // println!("Location services enabled: {:?}", lm.locationServicesEnabled());
            // printStatus(&self.lm);

            self.lm.startUpdatingLocation();

            CFRunLoop::run();
            // NSRunLoop::mainRunLoop().run();
            println!("run loop actually stopped");

            if let Some(lla) = self.delegate.ivars().location.take() {
                println!("Got LLA {lla:?}!");
                return Ok(lla);
            } else {
                todo!("didn't get a location")
            }
        }
    }

    fn get_permissions(&mut self) -> crate::Result<()> {
        // On MacOS, getting permissions is the same as calling startUpdatingLocation
        unsafe { self.lm.requestWhenInUseAuthorization() };

        // run looper till we get a result!
        self.get()?;
        Ok(())
    }
}
