use jni::{objects::{JObject, JValue, JValueGen, JValueOwned}, JNIEnv};

fn check_permission<'local>(env: &mut JNIEnv<'local>) -> Result<bool, ()> {
    let activity = get_current_activity(env).expect("must be called from an activitiy");
    let fine_location = env.new_string("android.permission.ACCESS_FINE_LOCATION").expect("string");

    let Ok(JValueGen::Int(perm)) = env.call_static_method("androidx/core/content/ContextCompat", "checkSelfPermission", "(Landroid/content/Context;Ljava/lang/String;)I", &[(&activity).into(), (&fine_location).into()]) else {
        return Err(());
    };

    Ok(perm == 0)
}

// cobbled together with undocumented APIs - when AOSP master we can replace the mActivities with similar logic already implemented there
// contract: we are inside of an activity
// TODO: check contract and err 
fn get_current_activity<'local>(env: &mut JNIEnv<'local>) -> Result<JObject<'local>, ()> {
    let Ok(JValueGen::Object(at)) = env
        .call_static_method(
            "android/app/ActivityThread",
            "currentActivityThread",
            "()Landroid/app/ActivityThread;",
            &[],
        ) else {
        log::info!("Unable to get activity thread");
        return Err(());
    };

    let Ok(JValueOwned::Object(activities)) = env.get_field(&at, "mActivities", "Landroid/util/ArrayMap;") else {
        log::warn!("couldn't reach into mActivities");
        return Err(());
    };
    log::info!("Got activities {activities:?}");

    let Ok(JValueOwned::Int(size)) = env.call_method(&activities, "size", "()I", &[]) else {
        log::warn!("couldn't get mActivities size");
        return Err(());
    };
    log::info!("activities is {size:?} long");

    if size == 0 {
        // ERROR: we are not called from a thread with an activity!
        return Err(());
    }

    let Ok(JValueOwned::Object(activity_record)) = env.call_method(&activities, "valueAt", "(I)Ljava/lang/Object;", &[(size - 1).into()]) else {
        log::warn!("couldn't get mActivities valueAt");
        return Err(());
    };
    log::info!("got activity record {activity_record:?} ");

    let Ok(JValueOwned::Object(activity)) = env.get_field(&activity_record, "activity", "Landroid/app/Activity;") else {
        log::warn!("couldn't reach into activity_record");
        return Err(());
    };
    log::info!("got activity {activity:?} ");

    Ok(activity)
}

/// Requests permissions on android
/// CONTRACT: MUST be called from an Activity.
/// If there is no Activity on this thread, we will not be able to produce a permission popup
pub fn request_permission<'local>(env: &mut JNIEnv<'local>) {
    // A major drawback of this approach is that the method used to get the activity uses 100% "unsupported" APIs
    // see requestPermissions in Activity.java (ActivityCompat is just a wrapper around this)
    // It's possible that we can manually construct and send the intent (packageManager.buildRequestPermissionsIntent)
    // This approach would also require unsupported APIs (like startActivityForResult, which still needs an Activity)

    let activity = get_current_activity(env).expect("must have a current activity");

    let fine_location = env.new_string("android.permission.ACCESS_FINE_LOCATION").expect("string");
    let perms_to_get = env.new_object_array(1, "java/lang/String", fine_location).expect("alloc array");

    let perm_res = env.call_static_method("androidx/core/app/ActivityCompat",
         "requestPermissions",
         "(Landroid/app/Activity;[Ljava/lang/String;I)V",
         &[(&activity).into(), (&perms_to_get).into(), JValue::Int(420)]);

    log::info!("asked for permissions: {perm_res:?}");

    while let Ok(false) = check_permission(env) {
        // busy loop waiting for perm.
        //  we could try to play some callback games, but we cant create classes so this gets very weird.
        //  we may be able to register functions, but haven't tried this yet.
    }
}

pub fn get_lla<'local>(env: &mut JNIEnv<'local>) -> Result<(f64, f64, f64), ()> {
    let context = get_current_activity(env).expect("must have a current activity");

    let location_service_handle = env.new_string("location").expect("create JNI String");
    let Ok(JValueOwned::Object(lm)) = env.call_method(&context, "getSystemService", "(Ljava/lang/String;)Ljava/lang/Object;", &[(&location_service_handle).into()]) else {
        log::warn!("Unable to get location manager");
        return Err(());
    };

    let Ok(JValueOwned::Object(providers)) = env.call_method(&lm, "getProviders", "(Z)Ljava/util/List;", &[true.into()]) else {
        log::warn!("Unable to get location providers");
        return Err(());
    };
    log::info!("got providers {providers:?} ");

    let Ok(JValueOwned::Object(provider)) = env.call_method(&providers, "getFirst", "()Ljava/lang/Object;", &[]) else {
        log::warn!("couldn't get first provider");
        return Err(());
    };
    log::info!("got provider {provider:?} ");
    
    let Ok(JValueOwned::Object(location)) = env.call_method(&lm, "getLastKnownLocation", "(Ljava/lang/String;)Landroid/location/Location;", &[(&provider).into()]) else {
        log::warn!("Unable to get location providers");
        return Err(());
    };
    log::info!("got location {location:?} ");

    let Ok(JValueOwned::Double(latitude)) = env.call_method(&location, "getLatitude", "()D", &[]) else {
        log::warn!("Unable to get lat");
        return Err(());
    };
    let Ok(JValueOwned::Double(longitude)) = env.call_method(&location, "getLongitude", "()D", &[]) else {
        log::warn!("Unable to get lat");
        return Err(());
    };
    //verified that its HAE in the docs
    let Ok(JValueOwned::Double(alt_hae)) = env.call_method(&location, "getAltitude", "()D", &[]) else {
        log::warn!("Unable to get alt");
        return Err(());
    };
    log::info!("got LLA {latitude}, {longitude}, {alt_hae}");

    return Ok((latitude, longitude, alt_hae));
}
