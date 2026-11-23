// Resolve Gradle plugins from Maven Central first. The plugin portal redirects
// to Maven Central anyway, but Gradle then looks up a plugin POM's parent/BOM
// (kotlin-gradle-plugins-bom) only on the portal, which can fail; Maven Central
// serves the plugin markers, the plugins and their BOMs directly.
pluginManagement {
    repositories {
        mavenCentral()
        gradlePluginPortal()
    }
}

rootProject.name = "iscc-lib-kotlin"
