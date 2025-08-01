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
    implementation("com.apollographql.apollo:apollo-runtime:4.1.1")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core-jvm:1.10.1")
    implementation("com.dashjoin:jsonata:0.9.8")
    implementation("com.fleeksoft.ksoup:ksoup:0.2.2")
    implementation("org.graalvm.polyglot:js-community:24.2.0")
    implementation("org.graalvm.polyglot:polyglot:24.2.0")
    implementation("com.fasterxml.jackson.datatype:jackson-datatype-jsr310:2.18.3")
    implementation("com.meilisearch.sdk:meilisearch-java:0.15.0")

    implementation("com.sun.mail:jakarta.mail:2.0.1")
    implementation("org.thymeleaf:thymeleaf:3.1.2.RELEASE")

    implementation(project(":engine"))

    testImplementation(kotlin("test"))
}

tasks.test {
    useJUnitPlatform()
}

kotlin {
    jvmToolchain(24)
}
