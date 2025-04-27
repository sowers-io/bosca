# Bosca Distributed Cache (DC)

A distributed cache service for the Bosca platform, providing fast, reliable, and consistent caching across a cluster of nodes.

## Features

- **Distributed Consensus**: Uses the Raft protocol (via raft-rs) to ensure consistency across the cluster.
- **In-Memory Caching**: Fast in-memory cache implementation using moka.
- **HTTP API**: RESTful API for interacting with the cache service using axum.
- **gRPC API**: High-performance gRPC API using protobuf and tonic.
- **Notifications**: Real-time notifications for cache updates.
- **Auto-Discovery**: Servers automatically discover and connect to other nodes in the cluster.
- **Async**: Built with async/await using tokio for high performance.

## Operations

The DC service supports the following operations:

- **Create Cache**: Create a new cache with a specified name and capacity.
- **Get**: Retrieve a value from the cache by key.
- **Put**: Store a value in the cache with a specified key.
- **Delete**: Remove a value from the cache by key.
- **Clear**: Remove all values from a cache.

## Notifications

The DC service provides real-time notifications for cache updates. Clients can subscribe to notifications for specific caches and receive events when values are updated, deleted, or when the cache is cleared.

## API

### HTTP API

The HTTP API is available at the following endpoints:

- `POST /caches`: Create a new cache
- `GET /caches/:cache_id/values/:key`: Get a value from the cache
- `POST /caches/:cache_id/values/:key`: Put a value in the cache
- `DELETE /caches/:cache_id/values/:key`: Delete a value from the cache
- `DELETE /caches/:cache_id`: Clear a cache
- `GET /caches/:cache_id/notifications`: Subscribe to notifications for a cache (Server-Sent Events)

### gRPC API

The gRPC API is defined in the `cache.proto` file and provides the same operations as the HTTP API.

## Usage

### Starting the Service

```bash
cargo run --bin bosca-dc
```

### Configuration

The service can be configured using environment variables:

- `DC_HOST`: Host to bind to (default: 0.0.0.0)
- `DC_PORT`: Port to bind to (default: 3000)
- `DC_NODE_ID`: (Optional) ID of this node in the cluster (auto-generated if not specified)
- `DC_PEERS`: (Optional) Comma-separated list of peer node IDs (auto-discovered if not specified)
- `DC_SERVICE_NAME`: (Optional) Service name for DNS discovery (default: "dc")

#### Auto-Discovery Configuration

The DC service now supports fully automatic node ID generation and peer discovery. No manual configuration is required in most cases.

**Automatic Node ID Generation:**
- By default, the service generates a unique node ID based on the hostname
- If hostname is not available, it falls back to using a UUID
- You can still manually set the node ID using the `DC_NODE_ID` environment variable

**Automatic Peer Discovery:**
- By default, the service discovers peers using DNS lookups for the service name
- The service name defaults to "dc" but can be configured using the `DC_SERVICE_NAME` environment variable
- You can still manually specify peers using the `DC_PEERS` environment variable

**Environment Variables:**
- `DC_NODE_ID`: (Optional) Manually set the node ID
- `DC_PEERS`: (Optional) Manually specify comma-separated list of peer node IDs
- `DC_SERVICE_NAME`: (Optional) Service name for DNS discovery (default: "dc")

**Example: Fully Automatic Configuration**
```
# No environment variables needed - everything is auto-discovered
```

**Example: Mixed Configuration**
```
# Manually set node ID but auto-discover peers
DC_NODE_ID=1
```

**Example: Manual Configuration**
```
# Manually set both node ID and peers
DC_NODE_ID=1
DC_PEERS=1,2,3
```

When a node starts up, it will automatically generate a node ID (if not specified) and discover peers (if not specified). The nodes will then use the Raft protocol to elect a leader and maintain consensus.

For Docker deployments, you can use Docker Compose to set up multiple nodes with minimal or no configuration. See the `docker-compose.yaml` file for an example.

## Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Generating Protobuf Code

The protobuf code is automatically generated during the build process using the `build.rs` script.

## Architecture

The DC service is built with the following components:

- **Cluster**: Manages the Raft consensus protocol for distributed consistency and handles auto-discovery of nodes in the cluster.
- **Cache**: Provides the in-memory cache implementation using moka.
- **API**: Exposes the HTTP and gRPC APIs for interacting with the cache.
- **Notification**: Handles real-time notifications for cache updates.

## License

This project is licensed under the same license as the Bosca platform.
