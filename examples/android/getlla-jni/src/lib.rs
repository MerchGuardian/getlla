use jni::sys::jint;
use jni::sys::jdouble;
use jni::objects::JClass;
use jni::JNIEnv;
use getlla::android::request_permission;
use getlla::android::get_lla;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_dev_foxhunter_getlla_Getlla_getlla(
    mut env: JNIEnv,
    _: JClass,
    level: jint,
) -> jdouble {
    android_logger::init_once(android_logger::Config::default().with_tag("dev.foxhunter.getlla").with_max_level(log::LevelFilter::Trace));
    log::info!("test");
    request_permission(&mut env);
    let Ok((latitude, longitude, altitude)) = get_lla(&mut env) else { return -1.; };
    latitude
}
