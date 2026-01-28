plugins {
    id("com.android.library")
    id("org.jetbrains.kotlin.android") version "1.9.21"
    id("maven-publish")
    kotlin("plugin.serialization") version "1.9.21"
}

repositories {
    mavenCentral()
    google()
}

android {
    namespace = "org.fedimint.client.uniffi"
    compileSdk = 34

    defaultConfig {
        minSdk = 24
        consumerProguardFiles("consumer-rules.pro")
    }
    
    kotlinOptions {
        jvmTarget = "1.8"
    }

    buildTypes {
        getByName("release") {
            @Suppress("UnstableApiUsage")
            isMinifyEnabled = false
            proguardFiles(file("proguard-android-optimize.txt"), file("proguard-rules.pro"))
        }
    }
}

dependencies {
    implementation("com.squareup.okio:okio:3.6.0")
    implementation("net.java.dev.jna:jna:5.18.0@aar")
    implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk7")
    implementation("androidx.appcompat:appcompat:1.6.1")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.9.0")
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.6.3")
    implementation("org.jetbrains.kotlinx:atomicfu:0.23.1")
}

val libraryVersion: String by project

publishing {
    publications {
        create<MavenPublication>("maven") {
            groupId = "org.fedimint"
            artifactId = "fedimint-client-uniffi"
            version = libraryVersion

            afterEvaluate {
                from(components["release"])
            }
        }
    }
}
