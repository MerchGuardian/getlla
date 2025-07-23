use jni::{objects::{JObject, JValue, JValueGen}, JNIEnv};

/// Requests permissions on android
pub fn request_permission<'local>(mut env: JNIEnv<'local>) {
    // const REQUEST_PERM_HANDLE: &'static str = "dev.foxhunter.getlla.LOCATION";
    // let request_perm_handle = env.new_string(REQUEST_PERM_HANDLE).expect("Couldn't create JNI String");

    let JValueGen::Object(at) = env
        .call_static_method(
            "android/app/ActivityThread",
            "currentActivityThread",
            "()Landroid/app/ActivityThread;",
            &[],
        )
        .expect("Couldn't get activity thread method") else {
        log::info!("got non-object when expecting object");
        return;
    };
        // .get_object(env).expect("couldn't get activity thread");

    let JValueGen::Object(context) = env.call_method(at, "getApplication", "()Landroid/app/Application;", &[]).expect("coudn't get application method") else {
        log::info!("got non-object when expecting object");
        return;
    };

    log::info!("got context {context:?}");

    let fine_location = env.new_string("android.permission.ACCESS_FINE_LOCATION").expect("string");
    // let coarse_location = env.new_string("android.permission.ACCESS_COARSE_LOCATION").expect("string");
    let perms_to_get = env.new_object_array(1, "java/lang/String", fine_location).expect("alloc array");
    // env.set_object_array_element(&perms_to_get, 1, coarse_location);

    

    // //(Landroid/app/Activity;[Ljava/lang/String;I)V
    // let perm_res = env.get_method_id("android/app/Activity", "closeContextMenu", "()");
    let perm_res = env.call_static_method("androidx/core/app/ActivityCompat",
         "requestPermissions",
         "(Landroid/app/Activity;[Ljava/lang/String;I)V",
         &[(&context).into(), (&perms_to_get).into(), JValue::Int(420)]);

        // env.register_native_methods(``, methods)

    log::info!("asked for permissions: {perm_res:?}");

    let location_service_handle = env.new_string("location").expect("create JNI String");
    let lm = env.call_method(&context, "getSystemService", "(Ljava/lang/String;)Ljava/lang/Object;", &[(&location_service_handle).into()]).expect("get location manager");



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
