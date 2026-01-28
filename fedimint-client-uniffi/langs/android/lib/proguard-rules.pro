# for JNA
-dontwarn java.awt.*
-keep class com.sun.jna.** { *; }
-keep class org.fedimint.client.uniffi.** { *; }
-keepclassmembers class * extends org.fedimint.client.uniffi.** { public *; }
-keepclassmembers class * extends com.sun.jna.** { public *; }
