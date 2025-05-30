plugins {
    kotlin("jvm") version "2.1.20"
    kotlin("plugin.serialization") version "2.1.20"
    id("org.graalvm.buildtools.native") version "0.10.4"
    application
}

group = "io.bosca"
version = "1.0-SNAPSHOT"


repositories {
    mavenCentral()
}

dependencies {
    implementation("com.github.ajalt.clikt:clikt:5.0.2")
    implementation(project(":engine"))
    implementation(project(":installers"))
    implementation(project(":activities"))

    compileOnly("org.graalvm.nativeimage:svm:24.2.0")

    testImplementation(kotlin("test"))
}

tasks.test {
    useJUnitPlatform()
}

kotlin {
    jvmToolchain(24)
}

application {
    mainClass.set("io.bosca.MainKt")
    applicationDefaultJvmArgs = listOf("-XX:+ExitOnOutOfMemoryError")
}

tasks.register<JavaExec>("runMain") {
    group = "application"
    description = "Run MainKt"
    classpath = sourceSets["main"].runtimeClasspath
    environment("BOSCA_USERNAME", "admin")
    environment("BOSCA_PASSWORD", "password")
    environment("BOSCA_QUEUES", "profiles,10;video,4;media,2;default,10;bible,20;bible-ai,10;bible-book,20;bible-chapter,20;bible-verse,10;media-transcription,1;media-upload,5;metadata,50;search-index,100;traits,100;transition,100;")
    environment("BOSCA_URL", "http://127.0.0.1:8000/graphql")
    mainClass.set("io.bosca.MainKt")
    jvmArgs = listOf("-XX:UseSVE=0")
    args = listOf("run")
}

//graalvmNative {
//    binaries {
//        named("main") {
//            imageName.set("bosca-runner")
//            mainClass.set("io.bosca.MainKt")
//            val args = mutableListOf(
//                "-H:+StaticExecutableWithDynamicLibC",
//                "-H:+AllowDeprecatedBuilderClassesOnImageClasspath",
//                "-H:+UnlockExperimentalVMOptions",
//                "-O1",
//                "--enable-url-protocols=http",
//                "--initialize-at-run-time=io.grpc.netty.shaded.io.netty.handler.ssl.BouncyCastleAlpnSslUtils",
//                "--initialize-at-run-time=ai.onnxruntime.OrtEnvironment",
//                "--initialize-at-run-time=ai.onnxruntime.OnnxRuntime",
//                "--initialize-at-build-time=com.oracle.truffle.trufflenode",
//                "--install-exit-handlers",
//                "--features=io.bosca.graalvm.BoscaFeature",
//                "--enable-preview",
//                "-D=java.io.tmpdir=/tmp/bosca"
//            )
//            if (System.getenv("MARCH") != null) {
//                args.add("-march=${System.getenv("MARCH")}")
//            }
//            buildArgs.addAll(args)
//        }
//    }
//    toolchainDetection.set(true)
//
//    // Configure reachability metadata
//    metadataRepository {
//        // We're now using a Feature class instead of JSON files
//        enabled.set(false)
//    }
//}