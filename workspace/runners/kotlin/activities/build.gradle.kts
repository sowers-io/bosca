plugins {
    kotlin("jvm") version "2.1.0"
    kotlin("plugin.serialization") version "2.1.0"
}

group = "io.bosca"
version = "1.0-SNAPSHOT"

repositories {
    mavenLocal()
    mavenCentral()
    maven {
        url = uri("https://maven.pkg.github.com/sowers-io")
    }
}

dependencies {
    implementation("com.apollographql.apollo:apollo-runtime:4.1.0")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core-jvm:1.10.1")
    implementation("io.bosca.bible:shared-jvm:0.0.5")
    implementation("com.dashjoin:jsonata:0.9.8")
    implementation("com.fleeksoft.ksoup:ksoup:0.2.2")
    implementation(project(":engine"))

    testImplementation(kotlin("test"))
}

tasks.test {
    useJUnitPlatform()
}

kotlin {
    jvmToolchain(23)
}
