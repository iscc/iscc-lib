plugins {
    kotlin("jvm") version "2.1.10"
}

group = "io.iscc"
version = providers.gradleProperty("version").get()

repositories {
    mavenLocal()
    mavenCentral()
}

dependencies {
    implementation("net.java.dev.jna:jna:5.16.0")
    testImplementation("org.junit.jupiter:junit-jupiter:5.11.4")
    testImplementation("com.google.code.gson:gson:2.11.0")
}

tasks.withType<Test> {
    useJUnitPlatform()
    val nativeLibDir = "${rootProject.rootDir}/../../target/debug"
    jvmArgs("-Djava.library.path=$nativeLibDir", "-Djna.library.path=$nativeLibDir")
    environment("LD_LIBRARY_PATH", nativeLibDir)
}
