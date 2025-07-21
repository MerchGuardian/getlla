use jni::sys::jint;
use jni::sys::jlong;
use jni::objects::JClass;
use jni::JNIEnv;
use getlla::android::request_permission;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_dev_foxhunter_getlla_Getlla_getlla(
    env: JNIEnv,
    _: JClass,
    level: jint,
) -> jlong {
    android_logger::init_once(android_logger::Config::default().with_tag("dev.foxhunter.getlla").with_max_level(log::LevelFilter::Trace));
    log::info!("test");
    request_permission(env);
    12
}
