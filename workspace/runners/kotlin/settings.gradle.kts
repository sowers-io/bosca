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

if (file("../enterprise/bible").exists()) {
    includeBuild("../enterprise/bible") {
        dependencySubstitution {
            // Substitute a dependency with a project in the included build
            substitute(module("io.bosca.bible:shared-jvm")).using(project(":shared"))
        }
    }
}
