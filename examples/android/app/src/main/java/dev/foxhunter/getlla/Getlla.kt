package dev.foxhunter.getlla

class Getlla {
    companion object {
        @JvmStatic external fun getlla(level: Int): Double;
        init {
            System.loadLibrary("getlla_jni")
        }
    }
}
