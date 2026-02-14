# Flowchart

A minimal example of a Mermaid flowchart.

```mermaid
graph TD
    A[Start] --> B{Is it working?}
    B -- Yes --> C[Great!]
    B -- No --> D[Check logs]
    D --> B
```
