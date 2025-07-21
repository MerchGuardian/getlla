package dev.foxhunter.getlla

class Getlla {
    companion object {
        @JvmStatic external fun getlla(level: Int): Long;
        init {
            System.loadLibrary("getlla_jni")
        }
    }
}
