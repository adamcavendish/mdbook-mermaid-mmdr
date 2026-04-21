# Diagram Types

mdbook-mermaid-mmdr supports all diagram types provided by `mermaid-rs-renderer`. Below are examples of the most commonly used types. Each section shows the Markdown source followed by the live rendered result.

## Flowchart

````markdown
```mermaid
flowchart LR
    A[Start] --> B{Decision}
    B -->|Yes| C[OK]
    B -->|No| D[Not OK]
```
````

```mermaid
flowchart LR
    A[Start] --> B{Decision}
    B -->|Yes| C[OK]
    B -->|No| D[Not OK]
```

## Sequence Diagram

````markdown
```mermaid
sequenceDiagram
    participant A as Alice
    participant B as Bob
    A->>B: Hello Bob
    B-->>A: Hi Alice
    A->>B: How are you?
    B-->>A: Great!
```
````

```mermaid
sequenceDiagram
    participant A as Alice
    participant B as Bob
    A->>B: Hello Bob
    B-->>A: Hi Alice
    A->>B: How are you?
    B-->>A: Great!
```

## Class Diagram

````markdown
```mermaid
classDiagram
    class Animal {
        +String name
        +int age
        +makeSound()
    }
    class Dog {
        +fetch()
    }
    Animal <|-- Dog
```
````

```mermaid
classDiagram
    class Animal {
        +String name
        +int age
        +makeSound()
    }
    class Dog {
        +fetch()
    }
    Animal <|-- Dog
```

## State Diagram

````markdown
```mermaid
stateDiagram-v2
    [*] --> Idle
    Idle --> Processing : start
    Processing --> Done : complete
    Processing --> Error : fail
    Error --> Idle : reset
    Done --> [*]
```
````

```mermaid
stateDiagram-v2
    [*] --> Idle
    Idle --> Processing : start
    Processing --> Done : complete
    Processing --> Error : fail
    Error --> Idle : reset
    Done --> [*]
```

## Entity Relationship Diagram

````markdown
```mermaid
erDiagram
    CUSTOMER ||--o{ ORDER : places
    ORDER ||--|{ LINE-ITEM : contains
    PRODUCT ||--o{ LINE-ITEM : "is in"
```
````

```mermaid
erDiagram
    CUSTOMER ||--o{ ORDER : places
    ORDER ||--|{ LINE-ITEM : contains
    PRODUCT ||--o{ LINE-ITEM : "is in"
```

## Gantt Chart

````markdown
```mermaid
gantt
    title Project Timeline
    dateFormat YYYY-MM-DD
    section Phase 1
    Research       :a1, 2024-01-01, 30d
    Design         :a2, after a1, 20d
    section Phase 2
    Implementation :b1, after a2, 40d
    Testing        :b2, after b1, 15d
```
````

```mermaid
gantt
    title Project Timeline
    dateFormat YYYY-MM-DD
    section Phase 1
    Research       :a1, 2024-01-01, 30d
    Design         :a2, after a1, 20d
    section Phase 2
    Implementation :b1, after a2, 40d
    Testing        :b2, after b1, 15d
```

## Pie Chart

````markdown
```mermaid
pie title Language Distribution
    "Rust" : 60
    "Python" : 25
    "Other" : 15
```
````

```mermaid
pie title Language Distribution
    "Rust" : 60
    "Python" : 25
    "Other" : 15
```

## Mindmap

````markdown
```mermaid
mindmap
    root((Project))
        Frontend
            React
            CSS
        Backend
            Rust
            Database
        DevOps
            CI/CD
            Monitoring
```
````

```mermaid
mindmap
    root((Project))
        Frontend
            React
            CSS
        Backend
            Rust
            Database
        DevOps
            CI/CD
            Monitoring
```

## Inline Theme Directives

You can override the theme on a per-diagram basis using Mermaid's `%%{init: ...}%%` directive:

````markdown
```mermaid
%%{init: {"theme": "forest"}}%%
graph LR
    A --> B --> C
```
````

```mermaid
%%{init: {"theme": "forest"}}%%
graph LR
    A --> B --> C
```

For the full list of supported diagram types, see the [mermaid-rs-renderer documentation](https://github.com/1jehuang/mermaid-rs-renderer).
