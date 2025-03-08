
# Bosca Kotlin Runner

The Bosca Kotlin Runner is a native application built with GraalVM Native Image technology for optimal performance and reduced startup time.

## Requirements

- GraalVM with JDK 23
- Gradle (wrapper included)

## Building the Runner

### JVM Build

To build the runner for JVM execution:

```bash
./gradlew build
```

### Native Image Build

To build a native executable:

```bash
./gradlew nativeBuild
```

The native executable will be created at `runner/build/native/nativeCompile/bosca-runner`.

## Running the Runner

### Running on JVM

To run the application on the JVM:

```bash
./gradlew run
```

### Running with Native Image Agent

To run on JVM with the native-image-agent for collecting metadata:

```bash
./gradlew -Pagent run
```

### Copying Metadata for Native Image

To copy the metadata collected by the agent into the project sources:

```bash
./gradlew metadataCopy --task run --dir src/main/resources/META-INF/native-image
```

### Running the Native Executable

After building the native executable, you can run it directly:

```bash
./runner/build/native/nativeCompile/bosca-runner
```

## Docker Build

You can also build and run the runner using Docker:

```bash
docker build -f Dockerfile-runner -t bosca-runner .
docker run -p 8000:8000 bosca-runner
```

## Environment Variables

The runner uses the following environment variables:

- `BOSCA_USERNAME`: Username for authentication
- `BOSCA_PASSWORD`: Password for authentication
- `BOSCA_URL`: URL for the GraphQL endpoint
- `BOSCA_QUEUES`: Queue configuration

## Additional Information

For more detailed information about the Bosca project, refer to the documentation in the `/docs` directory.
