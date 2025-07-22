// pub use getlla::macos;

use std::time::Duration;

fn main() {
    let mut getter = getlla::Getter::system_default().expect("Failed to get system default getter");
    getter.get_permissions().expect("Failed to get permissions");

    loop {
        std::thread::sleep(Duration::from_secs(5));
        let lla = getter.get().expect("Failed to get lla");
        dbg!(lla);
    }
}
