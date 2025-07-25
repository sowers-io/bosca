plugins {
    kotlin("jvm") version "2.1.20"
    kotlin("plugin.serialization") version "2.1.20"
    id("com.apollographql.apollo") version "4.1.0"
}

group = "io.bosca"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    api("com.apollographql.apollo:apollo-runtime:4.1.0")
    api("org.jetbrains.kotlinx:kotlinx-serialization-json:1.8.0")

    api("dev.langchain4j:langchain4j:1.0.0-beta1")
    api("dev.langchain4j:langchain4j-open-ai:1.0.0-beta1")
    api("dev.langchain4j:langchain4j-google-ai-gemini:1.0.0-beta1")
    api("dev.langchain4j:langchain4j-ollama:1.0.0-beta1")
    api("dev.langchain4j:langchain4j-qdrant:1.0.0-beta1")
    api("dev.langchain4j:langchain4j-embeddings-all-minilm-l6-v2:1.0.0-beta1")

    api("io.github.smiley4:schema-kenerator-core:2.2.0")
    api("io.github.smiley4:schema-kenerator-serialization:2.2.0")
    api("io.github.smiley4:schema-kenerator-jsonschema:2.2.0")

    api("com.charleskorn.kaml:kaml:0.55.0")

    api("net.thisptr:jackson-jq:1.2.0")

    testImplementation(kotlin("test"))
}

tasks.test {
    useJUnitPlatform()
}

kotlin {
    jvmToolchain(24)
}

apollo {
    service("service") {
        packageName.set("io.bosca.graphql")
        mapScalarToUpload("Upload")
        mapScalar("DateTime", "java.time.ZonedDateTime")
    }
}
