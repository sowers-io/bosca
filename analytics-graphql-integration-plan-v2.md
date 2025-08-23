# Analytics GraphQL Integration Plan v3
*Updated based on senior dev's schema design and codebase alignment*

## Overview
This plan outlines the implementation of GraphQL queries for the analytics system, following the senior dev's provided schema design. The system will enable querying raw events and executing flexible analytical queries with rich visualization metadata.

## Key Architecture Changes from v2
- **DataStore Layer Integration** - Follow established service architecture patterns
- **Context Integration** - Proper dependency injection via BoscaContext
- **Permission System** - Analytics-specific permissions following existing patterns
- **Configuration Integration** - Use existing ConfigurationDataStore
- **Cache Integration** - Leverage existing BoscaCacheManager
- **Widget System Integration** - Seamless Vue dashboard integration

## Architecture Decision Record (ADR)

### Context
- Analytics data is stored in Parquet files via a separate analytics service
- Senior dev has provided a complete GraphQL schema focusing on:
  - Raw event access
  - Flexible query execution system
  - Visualization-ready response format
  - Support for multiple chart types
- We need to support both raw event queries and analytical aggregations

### Decision
Implement the provided GraphQL schema following Bosca's established architectural patterns:
1. **DataStore Layer**: AnalyticsDataStore with proper business logic separation
2. **Context Integration**: Inject analytics services via BoscaContext
3. **Permission System**: Use existing permission checking patterns
4. **Configuration Management**: Leverage ConfigurationDataStore for settings
5. **Caching**: Use BoscaCacheManager for query result caching
6. **Widget Integration**: Extend existing Vue widget system

### Consequences
- **Pros**: Flexible query system, visualization-ready responses, follows established schema
- **Cons**: More complex implementation, requires query language design, visualization logic

## Phase 1: Foundation Setup

### Dependencies & Infrastructure
- [ ] Add DuckDB dependency to `workspace/core/server/Cargo.toml`
  - [ ] Research and select appropriate DuckDB Rust crate version
  - [ ] Ensure compatibility with async runtime (tokio)
  - [ ] Add feature flags if needed for optional analytics support

### Module Structure (Following Bosca Patterns)
- [ ] Create DataStore layer: `workspace/core/server/src/datastores/analytics/`
  - [ ] `mod.rs` - Module exports and AnalyticsDataStore
  - [ ] `events.rs` - EventsDataStore for event streaming
  - [ ] `queries.rs` - QueryDataStore for saved queries
  - [ ] `duckdb.rs` - DuckDB connection management
- [ ] Create GraphQL layer: `workspace/core/server/src/graphql/analytics/`
  - [ ] `mod.rs` - GraphQL module exports
  - [ ] `types/` - GraphQL type definitions only
  - [ ] `queries.rs` - GraphQL resolvers (thin layer)
- [ ] Update initialization: `workspace/core/server/src/initialization/`
  - [ ] Add `analytics.rs` for DuckDB setup
  - [ ] Update `mod.rs` to include analytics initialization

### Context Integration
- [ ] Update `BoscaContext` in `workspace/core/server/src/context.rs`
  ```rust
  #[derive(Clone)]
  pub struct BoscaContext {
      // existing fields...
      pub analytics: AnalyticsDataStore,
  }
  ```
- [ ] Add context initialization in server startup

### Configuration Integration
- [ ] Use existing `ConfigurationDataStore` for analytics settings:
  - [ ] `analytics.parquet_path` - Parquet file storage location
  - [ ] `analytics.duckdb_pool_size` - Connection pool settings
  - [ ] `analytics.query_timeout_ms` - Query timeout limits
  - [ ] `analytics.max_events_limit` - Maximum event streaming limits
  - [ ] `analytics.enabled` - Feature toggle

### Permission System
- [ ] Define analytics permissions in existing security system
  - [ ] `analytics.events.read` - Read raw events
  - [ ] `analytics.queries.execute` - Execute analytical queries
  - [ ] `analytics.queries.manage` - CRUD operations on saved queries
  - [ ] `analytics.admin` - Administrative access

## Phase 2: GraphQL Schema Implementation

### Event Types (`types/event.rs`)
- [ ] Implement `AnalyticEventType` enum (SESSION, INTERACTION, IMPRESSION, COMPLETION)
- [ ] Implement `AnalyticContent` type
  ```rust
  #[derive(SimpleObject)]
  struct AnalyticContent {
      id: String,
      #[graphql(name = "type")]
      content_type: String,
      index: Option<i32>,
      percent: Option<f64>,
  }
  ```
- [ ] Implement `AnalyticElement` type with JSON extras field
- [ ] Implement `AnalyticContext` type with all nested types:
  - [ ] `AnalyticDevice`
  - [ ] `AnalyticBrowser`
  - [ ] `AnalyticGeo`
- [ ] Implement `AnalyticEvent` main type

### Query System Types (`types/query.rs`)
- [ ] Implement `AnalyticQuery` type for saved queries
  ```rust
  #[derive(SimpleObject)]
  struct AnalyticQuery {
      id: UUID,
      name: String,
      description: String,
      // TODO: Add query definition fields
  }
  ```
- [ ] Implement `AnalyticQueryRequest` input type
- [ ] Design query definition language/structure (TODO item)

### Visualization Types (`types/visualization.rs`)
- [ ] Implement graph type enums:
  - [ ] `AnalyticQueryGraphType` (LINE, BAR, STACKED_BAR, AREA, DONUT)
  - [ ] `AnalyticQueryGraphCurveType` (BASIS)
- [ ] Implement axis types:
  - [ ] `AnalyticDataAxis` with position, type, label, tick
  - [ ] `AnalyticDataAxisLabel`
  - [ ] `AnalyticDataAxisTick` with formatting options
- [ ] Implement data record types:
  - [ ] `AnalyticDataRecord` union type
  - [ ] `AnalyticSingleDataRecord`
  - [ ] `AnalyticMultiDataRecord`
- [ ] Implement annotation types:
  - [ ] `AnalyticDataAnnotation`
  - [ ] `AnalyticDataAnnotationContent`
- [ ] Implement container types:
  - [ ] `AnalyticDataContainer` with graphs, axes, data
  - [ ] `AnalyticDataContainerGraph`
- [ ] Implement legend types:
  - [ ] `AnalyticDataLegend`
  - [ ] `AnalyticDataLegendItem`
- [ ] Implement `AnalyticQueryResponse` combining all visualization elements

### Query Resolvers (`queries.rs`)
- [ ] Implement `AnalyticQueries` object following Bosca patterns
  ```rust
  pub struct AnalyticQueries;
  
  #[Object]
  impl AnalyticQueries {
      #[tracing::instrument(skip(self, ctx))]
      async fn events(&self, ctx: &Context<'_>, offset: Option<i32>, limit: Option<i32>) -> Result<Vec<AnalyticEvent>, Error> {
          let bosca_ctx = ctx.data::<BoscaContext>()?;
          
          // Permission check following established pattern
          bosca_ctx.analytics_permission_check(PermissionCheck::Read).await?;
          
          // Delegate to DataStore
          bosca_ctx.analytics.events.find(offset, limit).await
      }
      
      #[tracing::instrument(skip(self, ctx))]
      async fn execute(&self, ctx: &Context<'_>, request: AnalyticQueryRequest) -> Result<AnalyticQueryResponse, Error> {
          let bosca_ctx = ctx.data::<BoscaContext>()?;
          
          // Permission check
          bosca_ctx.analytics_permission_check(PermissionCheck::Execute).await?;
          
          // Delegate to DataStore
          bosca_ctx.analytics.queries.execute(request).await
      }
  }
  ```

## Phase 3: DataStore Implementation

### AnalyticsDataStore (`datastores/analytics/mod.rs`)
- [ ] Implement main DataStore following established patterns
  ```rust
  #[derive(Clone)]
  pub struct AnalyticsDataStore {
      pub events: EventsDataStore,
      pub queries: QueryDataStore,
      pub pool: TracingPool<PostgresConnectionManager<MakeTlsConnector>>,
      pub cache: BoscaCacheManager,
      pub duck_pool: Arc<DuckDBConnectionPool>,
  }
  
  impl AnalyticsDataStore {
      pub fn new(pool: TracingPool<PostgresConnectionManager<MakeTlsConnector>>, 
                 cache: BoscaCacheManager,
                 duck_pool: Arc<DuckDBConnectionPool>) -> Self {
          Self {
              events: EventsDataStore::new(duck_pool.clone(), cache.clone()),
              queries: QueryDataStore::new(pool.clone(), cache.clone()),
              pool,
              cache,
              duck_pool,
          }
      }
  }
  ```

### EventsDataStore (`datastores/analytics/events.rs`)
- [ ] Implement event streaming following DataStore patterns
  ```rust
  #[derive(Clone)]
  pub struct EventsDataStore {
      duck_pool: Arc<DuckDBConnectionPool>,
      cache: BoscaCacheManager,
  }
  
  impl EventsDataStore {
      #[tracing::instrument(skip(self))]
      pub async fn find(&self, offset: Option<i32>, limit: Option<i32>) -> Result<Vec<AnalyticEvent>, Error> {
          // Cache key generation
          let cache_key = format!("analytics:events:{}:{}", offset.unwrap_or(0), limit.unwrap_or(100));
          
          // Check cache first
          if let Some(cached) = self.cache.get(&cache_key).await? {
              return Ok(cached);
          }
          
          // Query Parquet files via DuckDB
          let connection = self.duck_pool.get().await?;
          // Implementation...
      }
  }
  ```

### QueryDataStore (`datastores/analytics/queries.rs`)
- [ ] Implement query execution following DataStore patterns
  ```rust
  #[derive(Clone)]
  pub struct QueryDataStore {
      pool: TracingPool<PostgresConnectionManager<MakeTlsConnector>>,
      cache: BoscaCacheManager,
      duck_pool: Arc<DuckDBConnectionPool>,
  }
  
  impl QueryDataStore {
      #[tracing::instrument(skip(self))]
      pub async fn execute(&self, request: AnalyticQueryRequest) -> Result<AnalyticQueryResponse, Error> {
          // Query definition retrieval from PostgreSQL
          // Query execution via DuckDB
          // Result transformation for visualization
          // Cache result
      }
      
      #[tracing::instrument(skip(self))]
      pub async fn find_query(&self, id: Uuid) -> Result<Option<AnalyticQuery>, Error> {
          // Standard DataStore pattern for query retrieval
      }
  }
  ```

### DuckDB Integration (`datastores/analytics/duckdb.rs`)
- [ ] Implement DuckDB connection management following pool patterns
  - [ ] Connection pool with proper async support
  - [ ] Parquet file registration and management
  - [ ] Query timeout handling
  - [ ] Transaction management
  - [ ] Health check implementation

## Phase 4: Query Definition System

### Database Schema Integration
- [ ] Add analytics tables to existing PostgreSQL schema
  ```sql
  CREATE TABLE analytic_queries (
      id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
      name VARCHAR NOT NULL,
      description TEXT,
      definition JSONB NOT NULL, -- Query definition structure
      graph_type VARCHAR NOT NULL,
      created TIMESTAMPTZ DEFAULT NOW(),
      modified TIMESTAMPTZ DEFAULT NOW(),
      created_by UUID REFERENCES principals(id),
      tenant_id UUID, -- For multi-tenancy
      attributes JSONB -- Extensible metadata
  );
  ```

### Query CRUD Operations (Following DataStore Patterns)
- [ ] Implement standard DataStore query methods
  ```rust
  impl QueryDataStore {
      pub async fn create(&self, query: &CreateAnalyticQueryInput) -> Result<AnalyticQuery, Error>
      pub async fn find(&self, query: &FindQueryInput) -> Result<Vec<AnalyticQuery>, Error>
      pub async fn find_by_id(&self, id: Uuid) -> Result<Option<AnalyticQuery>, Error>
      pub async fn update(&self, id: Uuid, input: &UpdateAnalyticQueryInput) -> Result<AnalyticQuery, Error>
      pub async fn delete(&self, id: Uuid) -> Result<(), Error>
      pub async fn count(&self, query: &FindQueryInput) -> Result<i64, Error>
  }
  ```

### Query Definition Structure
- [ ] Design JSON schema for query definitions
  ```rust
  #[derive(Serialize, Deserialize)]
  struct QueryDefinition {
      metrics: Vec<Metric>,
      dimensions: Vec<Dimension>,
      filters: Vec<Filter>,
      time_range: TimeRange,
      graph_config: GraphConfiguration,
      cache_ttl: Option<i32>,
  }
  ```

### Common Queries
- [ ] Create preset queries for common use cases:
  - [ ] Daily Active Users (DAU)
  - [ ] User Session Duration
  - [ ] Content Engagement Rates
  - [ ] Funnel Analysis
  - [ ] Retention Cohorts

## Phase 5: Performance Optimization

### Query Optimization (Using Existing Cache)
- [ ] Integrate with existing `BoscaCacheManager`
  ```rust
  impl QueryDataStore {
      async fn execute_with_cache(&self, request: AnalyticQueryRequest) -> Result<AnalyticQueryResponse, Error> {
          let cache_key = format!("analytics:query:{}:{}", request.id, request.filters_hash());
          
          // Check cache using existing manager
          if let Some(cached) = self.cache.get(&cache_key).await? {
              return Ok(cached);
          }
          
          // Execute query
          let result = self.execute_query(request).await?;
          
          // Cache with TTL
          self.cache.set(&cache_key, &result, Duration::from_secs(300)).await?;
          
          Ok(result)
      }
  }
  ```
- [ ] Use existing Redis infrastructure for distributed caching
- [ ] Implement cache warming for common queries
- [ ] Add cache invalidation hooks

### Streaming Optimizations
- [ ] Implement efficient event streaming
  - [ ] Use Arrow for zero-copy reads
  - [ ] Implement server-side filtering
  - [ ] Add response compression
- [ ] Optimize Parquet file organization
  - [ ] Partition by date/hour
  - [ ] Implement file compaction
  - [ ] Add bloom filters for user/session lookups

## Phase 6: Integration & Testing

### GraphQL Integration
- [ ] Update `workspace/core/server/src/graphql/query.rs`
  ```rust
  #[Object(name = "Query")]
  impl QueryObject {
      // existing methods...
      
      async fn analytics(&self) -> AnalyticQueries {
          AnalyticQueries
      }
  }
  ```
- [ ] Add analytics module to GraphQL exports in `workspace/core/server/src/graphql/mod.rs`
- [ ] Update context initialization to include analytics DataStore
- [ ] Add feature flag checking using existing configuration system
- [ ] Ensure proper dependency injection follows established patterns

### Testing Strategy
- [ ] Unit tests for type conversions
  - [ ] Event parsing from Parquet
  - [ ] Visualization data transformation
  - [ ] Query definition validation
  
- [ ] Integration tests
  - [ ] End-to-end event query tests
  - [ ] Query execution with various filters
  - [ ] Chart data generation tests
  - [ ] Large dataset handling

- [ ] Performance benchmarks
  - [ ] Event streaming throughput
  - [ ] Query execution times
  - [ ] Memory usage under load
  - [ ] Concurrent query handling

### Example Queries for Testing
- [ ] Raw event access:
  ```graphql
  query GetRecentEvents {
    analytics {
      events(limit: 100) {
        type
        name
        created
        context {
          userId
          sessionId
        }
      }
    }
  }
  ```

- [ ] Analytical query execution:
  ```graphql
  query ExecuteDailyActiveUsers {
    analytics {
      execute(request: { id: "dau-query-uuid" }) {
        container {
          type
          data {
            ... on AnalyticSingleDataRecord {
              x
              y
            }
          }
        }
        legend {
          items {
            name
            color
          }
        }
      }
    }
  }
  ```

## Phase 7: Security & Monitoring

### Access Control (Following Existing Patterns)
- [ ] Implement analytics permission checks following existing security patterns
  ```rust
  impl BoscaContext {
      pub async fn analytics_permission_check(&self, check: PermissionCheck) -> Result<(), Error> {
          // Use existing permission checking infrastructure
          self.security.permission_check(&self.principal, &check).await
      }
  }
  ```
- [ ] Define analytics permissions in existing security system
- [ ] Add tenant isolation using existing multi-tenancy patterns
- [ ] Integrate with existing audit logging system
- [ ] Implement PII filtering based on user roles

### Monitoring
- [ ] Add metrics collection
  - [ ] Query execution times
  - [ ] Event streaming volume
  - [ ] Cache hit rates
  - [ ] Error rates by query type
- [ ] Implement health checks
  - [ ] Parquet file accessibility
  - [ ] DuckDB connection health
  - [ ] Query queue depth

## Phase 8: Vue Widget Integration

### Analytics Widgets (Following Existing Widget Patterns)
- [ ] Create analytics widget definitions in `workspace/web/administration/app/components/widgets/analytics/`
  ```typescript
  // AnalyticsQueryWidget.definition.ts
  export const analyticsQueryWidgetDefinition: WidgetDefinition = {
    type: 'analytics-query',
    name: 'Analytics Query',
    component: AnalyticsQueryWidget,
    defaultSettings: {
      queryId: '',
      refreshInterval: 300000, // 5 minutes
    },
    settingsSchema: {
      type: 'object',
      properties: {
        queryId: { type: 'string', title: 'Query ID' },
        refreshInterval: { type: 'number', title: 'Refresh Interval (ms)' },
      }
    },
    minSize: { w: 4, h: 3 },
    resizable: true
  }
  ```

### GraphQL Integration in Vue
- [ ] Create analytics GraphQL queries following existing patterns
  ```typescript
  // composables/useAnalytics.ts
  export const useAnalytics = () => {
    const client = useBoscaClient()
    
    const executeQuery = async (queryId: string) => {
      return await client.query({
        query: gql`
          query ExecuteAnalyticsQuery($request: AnalyticQueryRequest!) {
            analytics {
              execute(request: $request) {
                container { /* ... */ }
                legend { /* ... */ }
              }
            }
          }
        `,
        variables: { request: { id: queryId } }
      })
    }
    
    return { executeQuery }
  }
  ```

### Widget Registration
- [ ] Update widget registry to include analytics widgets
- [ ] Register widgets in `widget-registry.client.ts`
- [ ] Update analytics page to use new widgets

## Phase 9: Documentation & Rollout

### Documentation
- [ ] Document query definition language
- [ ] Create cookbook of common queries
- [ ] Document performance characteristics
- [ ] Add troubleshooting guide

### Migration Path
- [ ] Create migration tool for existing analytics
- [ ] Implement backwards compatibility layer
- [ ] Plan gradual rollout strategy

## Success Metrics

- [ ] Event query response time < 100ms for recent events
- [ ] Analytical query response time < 500ms for common queries
- [ ] Support for 1M+ events per day
- [ ] 99.9% query success rate
- [ ] Visualization data correctly formatted for all chart types

## Key Architectural Improvements in v3

1. **DataStore Architecture**: Follows established Bosca patterns with proper separation of concerns
2. **Context Integration**: Proper dependency injection via BoscaContext
3. **Permission System**: Uses existing security infrastructure
4. **Configuration Management**: Leverages ConfigurationDataStore
5. **Cache Integration**: Uses existing BoscaCacheManager and Redis
6. **Widget System**: Seamless integration with existing Vue dashboard
7. **Database Integration**: Uses existing PostgreSQL for query storage
8. **Error Handling**: Follows established error patterns
9. **Tracing**: Includes proper instrumentation
10. **Testing**: Aligns with existing testing patterns

## Implementation Checklist

### Prerequisites
- [ ] Confirm DuckDB integration approach with senior dev
- [ ] Define analytics permissions in security system
- [ ] Set up Parquet file storage location
- [ ] Plan database migration for analytics tables

### Phase Priority
1. **Phase 1-2**: Foundation and types (enables GraphQL schema)
2. **Phase 3**: DataStore implementation (core functionality)
3. **Phase 4**: Query system (enables saved queries)
4. **Phase 5**: Performance optimization (production readiness)
5. **Phase 6-7**: Integration and security (complete feature)
6. **Phase 8-9**: Vue integration and documentation (user-facing)

### Critical Dependencies
- DuckDB Rust crate selection and async compatibility
- Parquet file format and schema definition
- Query definition language design
- Permission model for analytics access

## Alignment with Existing Codebase ✅

This plan now follows all established Bosca patterns:
- ✅ DataStore architecture
- ✅ Context-based dependency injection
- ✅ Permission system integration
- ✅ Configuration management
- ✅ Cache integration
- ✅ GraphQL patterns
- ✅ Vue widget system
- ✅ Database integration
- ✅ Error handling
- ✅ Tracing instrumentation