plugins {
    kotlin("jvm") version "2.1.20"
    kotlin("plugin.serialization") version "2.1.20"
}

group = "io.bosca"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    implementation("com.apollographql.apollo:apollo-runtime:4.1.0")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core-jvm:1.10.1")
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-core:1.6.0")
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.6.0")
    implementation("com.charleskorn.kaml:kaml:0.55.0")
    implementation(project(":engine"))
    implementation(project(":activities"))
    if (file("../../enterprise/kotlin/enterprise").exists()) {
        implementation(project(":enterprise"))
    }
    testImplementation(kotlin("test"))
    testImplementation("io.mockk:mockk:1.13.8")
}

tasks.test {
    useJUnitPlatform()
}

kotlin {
    jvmToolchain(23)

    if (file("../../enterprise/kotlin/enterprise").exists()) {
        sourceSets.main {
            kotlin.srcDir("src/main/enterprise")
        }
    } else {
        sourceSets.main {
            kotlin.srcDir("src/main/community")
        }
    }
}
