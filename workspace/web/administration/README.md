# Bosca Administration

Bosca is an AI-powered Content Management, Analytics, and Personalization
platform built to help organizations unlock the full potential of their content
strategies. With cutting-edge AI/ML workflows, Bosca simplifies content
management while delivering actionable insights and personalized user
experiences that drive meaningful engagement.

### Administration

This is the administration app for Bosca. For more details about Bosca, see:
https://docs.bosca.io/

_Status_: In-Progress

## Getting Started

Ensure Deno is Installed:

```bash
curl -fsSL https://deno.land/install.sh | sh
```

Ensure Bosca Server is Running. See the [Quick Start](https://docs.bosca.io/quickstart/).

You should be able to access it at http://localhost:8000

Setup Web Administration Dashboard:

```bash
deno install --allow-scripts=npm:maplibre-gl@2.4.0,npm:vue-demi@0.14.10,npm:@parcel/watcher@2.5.1
deno run install-queries  # Adds the persisted queries to the Bosca Server, without this nothing will work
deno run dev
```

You should then be able to access: http://localhost:3000/

Username / Password: `admin` / `password`

To seed some initial configurations, use the runner to install everything that's needed.
Documentation on this will come later.

If you add or change any graphql queries, you can regenerate the client library by:

```bash
deno run codegen
deno run install-queries
```

To format the project:

```bash
deno run format
```

To lint the project:

```bash
deno run lint
```
