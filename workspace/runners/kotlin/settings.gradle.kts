pluginManagement {
    repositories {
        gradlePluginPortal()
        mavenCentral()
    }
}

rootProject.name = "bosca-runner"

include(":engine")
include(":activities")
include(":runner")
include(":installers")

if (file("../enterprise").exists()) {
    include(":enterprise")
    project(":enterprise").projectDir = File("../enterprise/kotlin/enterprise")
}
