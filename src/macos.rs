use objc2_core_location::{CLLocationManager, CLAuthorizationStatus, CLLocation, CLLocationManagerDelegate};
use objc2_foundation::{NSObject, NSObjectProtocol, MainThreadMarker, NSArray, NSError, NSRunLoop};
use objc2::{rc::Retained, MainThreadOnly, define_class, msg_send, runtime::ProtocolObject};

extern crate objc2_core_location;

define_class!{

    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - `Delegate` does not implement `Drop`.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    // #[ivars = AppDelegateIvars]
    struct Delegate;

    unsafe impl NSObjectProtocol for Delegate { }
    unsafe impl CLLocationManagerDelegate for Delegate {
        #[unsafe(method(locationManager:didUpdateLocations:))]
        fn locationManager_didUpdateLocations(&self, lm: &CLLocationManager, locs: &NSArray<CLLocation>) {
            dbg!(locs);
            dbg!(lm);
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
        let this = Self::alloc(mtm).set_ivars(());
        unsafe { msg_send![super(this), init] }
    }
}

fn printStatus(lm: &CLLocationManager) {
    match unsafe {lm.authorizationStatus()} {
        CLAuthorizationStatus::NotDetermined => {
            println!("Undetermined authorization, requesting when in use");
            // unsafe {lm.requestWhenInUseAuthorization()};
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

pub fn go() {
    unsafe {
        let lm = CLLocationManager::new();
        println!("Location services enabled: {:?}", lm.locationServicesEnabled());
        printStatus(&lm);

        let mtm = MainThreadMarker::new().unwrap();
        let delegate = Delegate::new(mtm);
        lm.setDelegate(Some(ProtocolObject::from_ref(&*delegate)));

        lm.startUpdatingLocation();
        loop {
            NSRunLoop::mainRunLoop().run();
        }
    }
}
