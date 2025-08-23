# Analytics GraphQL Query Examples

This document provides example queries for testing the analytics implementation.

## Available Predefined Queries

### Time Series Queries
- `page-views-7d` - Daily page views for last 7 days (Line graph)
- `user-sessions-30d` - Daily sessions for last 30 days (Area graph)
- `content-created-monthly` - Monthly content creation for last 12 months (Bar graph)

### Categorical Queries
- `events-by-type` - Event distribution by type (Bar graph)
- `content-by-category` - Content distribution by category (Bar graph)
- `events-by-type-stacked` - Event types over time (Stacked bar graph)

### Geographic Queries
- `users-by-country` - User distribution by country (Bar graph)
- `sessions-by-region` - Session distribution by region (Bar graph)

### Distribution Queries
- `device-types` - Device type distribution (Donut graph)
- `browser-distribution` - Browser distribution (Donut graph)

## Example Queries

### 1. Get Recent Events
```graphql
query GetRecentEvents {
  analytics {
    events(offset: 0, limit: 10) {
      type
      name
      clientId
      created
      element {
        id
        type
        extras
      }
      context {
        userId
        sessionId
        device {
          platform
          deviceType
        }
        geo {
          city
          country
        }
      }
    }
  }
}
```

### 2. Execute Line Graph Query (Page Views)
```graphql
query ExecutePageViews {
  analytics {
    execute(request: { id: "page-views-7d" }) {
      container {
        type
        scale
        graphs {
          type
        }
        axis {
          type
          label {
            label
            type
            color
          }
          tick {
            enabled
            format
          }
        }
        data {
          ... on AnalyticSingleDataRecord {
            x
            y
            attributes
          }
        }
      }
      annotations {
        content {
          text
        }
        x
        y
      }
    }
  }
}
```

### 3. Execute Donut Graph Query (Device Types)
```graphql
query ExecuteDeviceTypes {
  analytics {
    execute(request: { id: "device-types" }) {
      legend {
        type
        items {
          name
          color
          shape
          inactive
        }
      }
      container {
        type
        graphs {
          type
        }
        data {
          ... on AnalyticSingleDataRecord {
            x
            y
            attributes
          }
        }
      }
    }
  }
}
```

### 4. Execute Stacked Bar Graph Query
```graphql
query ExecuteStackedEvents {
  analytics {
    execute(request: { id: "events-by-type-stacked" }) {
      legend {
        type
        items {
          name
          color
          shape
        }
      }
      container {
        type
        scale
        graphs {
          type
        }
        axis {
          type
          label {
            label
            type
          }
        }
        data {
          ... on AnalyticMultiDataRecord {
            x
            y
            attributes
          }
          ... on AnalyticSingleDataRecord {
            x
            y
            attributes
          }
        }
      }
    }
  }
}
```

### 5. Geographic Distribution Query
```graphql
query ExecuteUsersByCountry {
  analytics {
    execute(request: { id: "users-by-country" }) {
      container {
        graphs {
          type
        }
        axis {
          type
          label {
            label
          }
        }
        data {
          ... on AnalyticSingleDataRecord {
            x
            y
            attributes
          }
        }
      }
    }
  }
}
```

### 6. Test Invalid Query ID
```graphql
query TestInvalidQuery {
  analytics {
    execute(request: { id: "invalid-query" }) {
      container {
        type
      }
    }
  }
}
```

## Expected Behavior

- All queries should return properly formatted data according to the GraphQL schema
- Time series data should have timestamps in milliseconds for x values
- Categorical data should have incremental x values (0, 1, 2, etc.) 
- Percentage data should sum to approximately 100%
- Multi-series data (stacked bars) should have arrays for y values
- Legends should be included for donut charts and stacked charts
- Invalid query IDs should return helpful error messages with available options

## Frontend Integration

These queries can be used directly with the admin dashboard widgets:

1. **LineChartWidget** expects data format: `{ x: number, y: number }`
2. **Charts with legends** can use the legend data for styling
3. **Multi-series data** supports stacked visualizations
4. **Attributes field** provides additional metadata for tooltips