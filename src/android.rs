use jni::{objects::{JObject, JValue, JValueGen, JValueOwned}, JNIEnv};

/// Requests permissions on android
/// CONTRACT: MUST be called from an Activity.
/// If there is no Activity on this thread, we will not be able to produce a permission popup
pub fn request_permission<'local>(mut env: JNIEnv<'local>) {
    // const REQUEST_PERM_HANDLE: &'static str = "dev.foxhunter.getlla.LOCATION";
    // let request_perm_handle = env.new_string(REQUEST_PERM_HANDLE).expect("Couldn't create JNI String");
    //


    // A major drawback of this approach is that the method used to get the activity uses 100% "unsupported" APIs
    // see requestPermissions in Activity.java (ActivityCompat is just a wrapper around this)
    // It's possible that we can manually construct and send the intent (packageManager.buildRequestPermissionsIntent)
    // This approach would also require unsupported APIs (like startActivityForResult, which still needs an Activity)

    let Ok(JValueGen::Object(at)) = env
        .call_static_method(
            "android/app/ActivityThread",
            "currentActivityThread",
            "()Landroid/app/ActivityThread;",
            &[],
        ) else {
        log::info!("Unable to get activity thread");
        return;
    };

    
    let Ok(JValueOwned::Object(activities)) = env.get_field(&at, "mActivities", "Landroid/util/ArrayMap;") else {
        log::warn!("couldn't reach into mActivities");
        return;
    };
    log::info!("Got activities {activities:?}");

    let Ok(JValueOwned::Int(size)) = env.call_method(&activities, "size", "()I", &[]) else {
        log::warn!("couldn't get mActivities size");
        return;
    };
    log::info!("activities is {size:?} long");

    let Ok(JValueOwned::Object(activity_record)) = env.call_method(&activities, "valueAt", "(I)Ljava/lang/Object;", &[(size - 1).into()]) else {
        log::warn!("couldn't get mActivities valueAt");
        return;
    };
    log::info!("got activity record {activity_record:?} ");

    let Ok(JValueOwned::Object(context)) = env.get_field(&activity_record, "activity", "Landroid/app/Activity;") else {
        log::warn!("couldn't reach into activity_record");
        return;
    };
    log::info!("got activity {context:?} ");

    let fine_location = env.new_string("android.permission.ACCESS_FINE_LOCATION").expect("string");
    let perms_to_get = env.new_object_array(1, "java/lang/String", fine_location).expect("alloc array");

    

    // //(Landroid/app/Activity;[Ljava/lang/String;I)V
    // let perm_res = env.get_method_id("android/app/Activity", "closeContextMenu", "()");
    let perm_res = env.call_static_method("androidx/core/app/ActivityCompat",
         "requestPermissions",
         "(Landroid/app/Activity;[Ljava/lang/String;I)V",
         &[(&context).into(), (&perms_to_get).into(), JValue::Int(420)]);

        // env.register_native_methods(``, methods)

    log::info!("asked for permissions: {perm_res:?}");

    //TODO: separate following code into another function
    let location_service_handle = env.new_string("location").expect("create JNI String");
    let Ok(JValueOwned::Object(lm)) = env.call_method(&context, "getSystemService", "(Ljava/lang/String;)Ljava/lang/Object;", &[(&location_service_handle).into()]) else {
        log::warn!("Unable to get location manager");
        return;
    };

    let Ok(JValueOwned::Object(providers)) = env.call_method(&lm, "getProviders", "(Z)Ljava/util/List;", &[true.into()]) else {
        log::warn!("Unable to get location providers");
        return;
    };
    log::info!("got providers {providers:?} ");

    let Ok(JValueOwned::Object(provider)) = env.call_method(&providers, "getFirst", "()Ljava/lang/Object;", &[]) else {
        log::warn!("couldn't get first provider");
        return;
    };
    log::info!("got provider {provider:?} ");
    
    let Ok(JValueOwned::Object(location)) = env.call_method(&lm, "getLastKnownLocation", "(Ljava/lang/String;)Landroid/location/Location;", &[(&provider).into()]) else {
        log::warn!("Unable to get location providers");
        return;
    };
    log::info!("got location {location:?} ");

    let Ok(JValueOwned::Double(latitude)) = env.call_method(&location, "getLatitude", "()D", &[]) else {
        log::warn!("Unable to get lat");
        return;
    };
    let Ok(JValueOwned::Double(longitude)) = env.call_method(&location, "getLongitude", "()D", &[]) else {
        log::warn!("Unable to get lat");
        return;
    };
    //verified that its HAE in the docs
    let Ok(JValueOwned::Double(alt_hae)) = env.call_method(&location, "getAltitude", "()D", &[]) else {
        log::warn!("Unable to get alt");
        return;
    };
    log::info!("got LLA {latitude}, {longitude}, {alt_hae}");

    // env.call_method(
    //     usb_man,
    //     "requestPermission",
    //     "(Landroid/hardware/usb/UsbDevice;Landroid/app/PendingIntent;)V",
    //     &[(&self.devinst.as_obj()).into(), (&pending).into()],
    // )
    // .clear_ex()?;

    // env.
    // env.call_static_method(class, name, sig, args)
}

/*
@RequiresApi(Build.VERSION_CODES.N)
fun requestPermissions() {
    val locationPermissionRequest = registerForActivityResult(
        ActivityResultContracts.RequestMultiplePermissions()
    ) { permissions ->
        when {
            permissions.getOrDefault(Manifest.permission.ACCESS_FINE_LOCATION, false) -> {
                // Precise location access granted.
            }
            permissions.getOrDefault(Manifest.permission.ACCESS_COARSE_LOCATION, false) -> {
                // Only approximate location access granted.
            }
            else -> {
                // No location access granted.
            }
        }
    }

    // Before you perform the actual permission request, check whether your app
    // already has the permissions, and whether your app needs to show a permission
    // rationale dialog. For more details, see Request permissions:
    // https://developer.android.com/training/permissions/requesting#request-permission
    locationPermissionRequest.launch(
        arrayOf(
            Manifest.permission.ACCESS_FINE_LOCATION,
            Manifest.permission.ACCESS_COARSE_LOCATION
        )
    )
}
*/
