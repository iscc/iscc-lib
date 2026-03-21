plugins {
    kotlin("jvm") version "2.1.10"
}

group = "io.iscc"
version = providers.gradleProperty("version").get()

repositories {
    mavenCentral()
}

dependencies {
    implementation("net.java.dev.jna:jna:5.16.0")
}

tasks.withType<Test> {
    useJUnitPlatform()
    jvmArgs("-Djava.library.path=${rootProject.rootDir}/../../target/debug")
}
