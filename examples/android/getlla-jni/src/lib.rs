use jni::sys::jint;
use jni::sys::jlong;
use jni::objects::JClass;
use jni::JNIEnv;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn Java_dev_foxhunter_getlla_Getlla_getlla(
    _: JNIEnv,
    _: JClass,
    level: jint,
) -> jlong {
    69
}
