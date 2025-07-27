use jni::sys::jint;
use jni::sys::jdouble;
use jni::objects::JClass;
use jni::JNIEnv;
use getlla::Android;
use getlla::Backend;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_dev_foxhunter_getlla_Getlla_getlla(
    mut env: JNIEnv,
    _: JClass,
    level: jint,
) -> jdouble {
    android_logger::init_once(android_logger::Config::default().with_tag("dev.foxhunter.getlla").with_max_level(log::LevelFilter::Trace));
    log::info!("test");
    let mut getter = Android::new(env.get_java_vm().expect("Get a JavaVM"));
    getter.get_permissions();
    let Ok(getlla::Lla { latitude_degs, longitude_degs, altitude_m_hae }) = getter.get() else { return -1.; };
    latitude_degs
}
