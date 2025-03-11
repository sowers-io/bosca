# Workflow States

This directory contains YAML definitions for workflow states. Each file defines a state that can be used in workflows.

## State Definition Format

```yaml
name: StateName           # Name of the state
description: Description  # Description of what this state represents
type: StateType           # Type of state (pending, processing, draft, approval, etc.)
configuration:            # Optional configuration specific to this state type
  key1: value1
  key2: value2
workflowId: workflow.id   # Optional: ID of workflow to execute when entering this state
exitWorkflowId: wf.id     # Optional: ID of workflow to execute when exiting this state
entryWorkflowId: wf.id    # Optional: ID of workflow that can enter this state
```

## State Types

The system uses the following state types:

- `pending`: Content waiting to be processed
- `processing`: Content currently being processed by automation
- `draft`: Content in draft/editing mode
- `approval`: Content awaiting review/approval
- `approved`: Content that has been approved but not published
- `published`: Content that is live and accessible
- `failure`: Content that failed processing

These types match the workflow_state_type enum in the database.

States are automatically installed when running the Kotlin runner.