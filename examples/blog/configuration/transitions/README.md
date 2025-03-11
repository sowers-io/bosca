# Workflow Transitions

This directory contains YAML definitions for workflow transitions. Each file defines a transition between two states.

## Transition Definition Format

```yaml
fromState: SourceStateName    # Name of the source state
toState: DestinationStateName # Name of the destination state
description: Description      # Description of what this transition represents
```

Transitions define the allowed paths between states in a workflow. For example, content might transition from "Draft" to "Review" and then to "Published".

Note that transitions reference states by name, so the states must be installed before transitions can be processed. The installation process automatically handles this ordering.

Transitions are automatically installed when running the Kotlin runner.