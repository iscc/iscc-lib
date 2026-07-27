plugins {
    kotlin("jvm") version "2.4.10"
    `maven-publish`
    signing
}

group = "io.iscc"
version = providers.gradleProperty("version").get()

repositories {
    mavenLocal()
    mavenCentral()
}

java {
    withSourcesJar()
    withJavadocJar()
}

dependencies {
    implementation("net.java.dev.jna:jna:5.19.1")
    testImplementation("org.junit.jupiter:junit-jupiter:6.1.2")
    // Gradle 9 no longer auto-injects a launcher — an explicit junit-platform-launcher
    // on the test runtime classpath is required, and pinning it here keeps the platform
    // jars aligned with junit-jupiter. JUnit 6 gives Platform and Jupiter a single
    // version number, so keep the launcher identical to the junit-jupiter version.
    testRuntimeOnly("org.junit.platform:junit-platform-launcher:6.1.2")
    testImplementation("com.google.code.gson:gson:2.14.0")
}

tasks.withType<Test> {
    useJUnitPlatform()
    val nativeLibDir = "${rootProject.rootDir}/../../target/debug"
    jvmArgs("-Djava.library.path=$nativeLibDir", "-Djna.library.path=$nativeLibDir")
    environment("LD_LIBRARY_PATH", nativeLibDir)
    val fixtureDir = "${rootProject.rootDir}/../../crates/iscc-lib/tests"
    systemProperty("iscc.fixtureDir", fixtureDir)
    // The canonical fixture lives outside this Gradle project, so declare it as a task
    // input; without this `./gradlew test` reports UP-TO-DATE and silently skips the
    // boundary vectors after the fixture changes. PathSensitivity.NONE hashes contents
    // only — the absolute path differs between checkouts and must not force a re-run.
    inputs.file("$fixtureDir/unicode_boundary.json").withPathSensitivity(PathSensitivity.NONE)
}

publishing {
    publications {
        create<MavenPublication>("maven") {
            groupId = "io.iscc"
            artifactId = "iscc-lib-kotlin"
            from(components["java"])
            pom {
                name.set("iscc-lib-kotlin")
                description.set("Kotlin bindings for iscc-lib - ISO 24138 ISCC")
                url.set("https://github.com/iscc/iscc-lib")
                licenses {
                    license {
                        name.set("Apache-2.0")
                        url.set("https://www.apache.org/licenses/LICENSE-2.0")
                    }
                }
                developers {
                    developer {
                        id.set("titusz")
                        name.set("Titusz Pan")
                    }
                }
                scm {
                    connection.set("scm:git:https://github.com/iscc/iscc-lib.git")
                    url.set("https://github.com/iscc/iscc-lib")
                }
            }
        }
    }
    repositories {
        maven {
            name = "staging"
            url = uri(layout.buildDirectory.dir("staging-deploy"))
        }
    }
}

signing {
    val passphrase: String? = System.getenv("MAVEN_GPG_PASSPHRASE")
    if (passphrase != null) {
        useGpgCmd()
    }
    sign(publishing.publications["maven"])
}

tasks.withType<Sign>().configureEach {
    onlyIf { System.getenv("MAVEN_GPG_PASSPHRASE") != null }
}
