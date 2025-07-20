# getlla
A cross-platform location source abstraction that polls available GPS APIs or hardware for a position

## WIP!
This README is closer to a braindump. There are no docs yet. This crate is a proof-of-concept stage.

## Goals
- Cover the platforms listed below
  - MacOS
    - Should we make the calls to main loop optional?
  - Linux
  - Windows
  - Android
  - iOS
- To the extent possible, support cross-compiling
  - Windows to MacOS is unlikely due to XCode licensing.
  - MacOS should be able to cross compile to everybody else
- Avoid spawning subprocesses
- Provide a generic error type
- To the extent possible, handle permission requests
  - What is a reasonable abstraction for triggering such permission requests?
- Platform-specific logging with approrpiate log creates
- Probably do futures, since the underlying implementations tend to be async/callback-oriented


## Caveats & Permissions
### MacOS 
MacOS requires code-signing and an "app" directory structure, with an "Info.plist" that declares the reasoning for the location sevices permission. Without this file, even explictly requesting the permission will not
If you can find a good workaround for this (that communicates the same info.plist information through other framework APIs), contributions ar

A script to do this is currently in build.sh

### Android 
TODO. There will be likely be some android permissions and manifest.xml entries 

### Windows 
TODO (no addl. permission should be necessary, based on existing powershell samplecode)

### Linux 
Unsure of the canonical way for accessing built-in GPS recievers for laptops running linux - this may be highly vendor specific, or it may be as simple as opening a serial device.
Will investigate. libgps/gpsd seem to cover some of these bases

### iOS 
TODO: Haven't tried this on iOS. Probably similar to MacOS CoreLocation with some extra steps

### External GPS Devices (serial) 
TODO. Add support for generic NMEA serial devices, and maybe the ublox binary protocol.

### External GPS Devices (userspace serial)
IIRC there is an existing open-source effort to implement userspace USB serial drivers using nusb/rusb. These drivers would enable external serial GPS recievers on Android.

## Support tiers

MacOS, Android, Windows, Linux, NMEA serial should be tier 0
external serial devices other than NMEA (like ublox's own protocol) tier 1 
iOS tier 2
Android with an external userspace serial device tier 3 (for Android, we can enable JVM-land implemenations of userspace serial to pass us raw serial via FFI, which we then parse. messy imo)
