# Customization and Error Handling

Demonstrating theme customization, Unicode support, and error handling.

## Theme Configuration

Configure the Mermaid theme using the `%%{init: {'theme': 'forest'}}%%` directive.

```mermaid
%%{init: {'theme': 'forest'}}%%
graph TD
    A[Standard Forest Theme] --> B(Greenish Nodes)
    B --> C{Decision}
    C -->|Yes| D[Result 1]
    C -->|No| E[Result 2]
```

## Custom Theme Variables

Override specific theme variables for fine-grained control.

```mermaid
%%{init: { 'theme': 'base', 'themeVariables': { 'primaryColor': '#ffcccc', 'edgeLabelBackground':'#ffffff', 'tertiaryColor': '#fff0f0' } } }%%
graph LR
    Start --> Stop
```

## Unicode Support

`mdbook-mermaid-mmdr` supports Unicode characters, including Chinese text.

```mermaid
graph TD
    开始[开始] --> 过程{进行中}
    过程 -->|成功| 结束[结束]
    过程 -->|失败| 重试[重试]
    重试 --> 过程
```

## Error Handling

If a syntax error occurs, `mdbook-mermaid-mmdr` renders an error message with source code.

```mermaid
graph TD
    A --> B
    B -- Syntax Error! --- C
    C -->
```
