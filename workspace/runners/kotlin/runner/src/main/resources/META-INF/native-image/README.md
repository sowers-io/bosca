# GraalVM Native Image Configuration

This directory contains the configuration files needed for GraalVM native-image to properly build the Bosca Runner application.

## Feature-based Configuration

The primary configuration approach used is a GraalVM Feature class (`io.bosca.graalvm.BoscaFeature`), which programmatically registers classes, methods, and fields that need to be accessible via reflection at runtime.

The Feature class replaces the previous JSON-based `reachability-metadata.json` approach, providing a more maintainable and type-safe way to configure the native image build.

## Configuration Files

1. **resources-config.json**: Defines which resources should be included in the native image
2. **jni-config.json**: Configures JNI access for native libraries

## Building the Native Image

The native image is built using the Gradle native image plugin. The configuration in `build.gradle.kts` includes these key parameters:

```kotlin
graalvmNative {
    binaries {
        named("main") {
            // ...other settings...
            buildArgs.add("--features=io.bosca.graalvm.BoscaFeature")
        }
    }
    
    metadataRepository {
        enabled.set(false) // Disabled since we're using a Feature class
    }
}
```

## Feature Structure

The `BoscaFeature` class:

1. Implements the GraalVM `Feature` interface
2. Overrides the `beforeAnalysis` method to register reflection metadata
3. Contains helper methods for registering different types of classes, methods, and fields

## Common Registration Categories

The feature registers:

- Kotlin coroutines classes and volatile fields
- GraalJS and Truffle classes/fields
- Security providers
- Meilisearch classes
- Jsonata classes
- JNI proxy interfaces