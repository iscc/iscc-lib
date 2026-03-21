plugins {
    kotlin("jvm") version "2.1.10"
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
                    connection.set("scm:git:git://github.com/iscc/iscc-lib.git")
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
    val signingKey: String? = System.getenv("SIGNING_KEY")
    val signingPassword: String? = System.getenv("SIGNING_PASSWORD")
    if (signingKey != null) {
        useInMemoryPgpKeys(signingKey, signingPassword)
    }
    sign(publishing.publications["maven"])
}

tasks.withType<Sign>().configureEach {
    onlyIf { System.getenv("SIGNING_KEY") != null }
}
