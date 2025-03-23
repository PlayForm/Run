### 1. Main Execution Flow

```mermaid
graph LR
    subgraph main
        A[Start] --> B[Parse command-line arguments]
        B --> C[Create Option struct]
        C --> D[Process entries based on pattern]
        D --> E[Execute commands]
        E --> F[End]
    end

    subgraph Parse command-line arguments
        B --> G[Parse File flag]
        B --> H[Parse Parallel flag]
        B --> I[Parse Root directory]
        B --> J[Parse Exclude patterns]
        B --> K[Parse Command arguments]
    end

    subgraph Process entries based on pattern
        D --> L[Filter entries by pattern]
        L --> M[Prepare entries for processing]
    end

    subgraph Execute commands
        E --> N[Execute commands in parallel]
        N --> O[Execute commands sequentially]
    end
```

### 2. Command Processing Flow

```mermaid
graph LR
    subgraph Execute commands
        E[Execute commands] --> P[Check if parallel processing is enabled]
        P -->|Yes| Q[Execute commands in parallel]
        P -->|No| R[Execute commands sequentially]

        subgraph Execute commands in parallel
            Q --> S[Spawn worker threads]
            S --> T[Process each entry in parallel]
            T --> U[Collect and print outputs]
        end

        subgraph Execute commands sequentially
            R --> V[Process each entry one by one]
            V --> W[Execute each command for the entry]
            W --> X[Print output]
        end
    end
```

### Explanation

1. **Main Execution Flow**:

    - The main function starts by parsing command-line arguments.
    - It then creates an `Option` struct based on the parsed arguments.
    - The entries are processed based on the specified pattern.
    - Commands are executed either in parallel or sequentially.
    - The process ends after all commands are executed.

2. **Command Processing Flow**:
    - The command execution starts by checking if parallel processing is
      enabled.
    - If parallel processing is enabled, worker threads are spawned to process
      each entry in parallel.
    - If parallel processing is disabled, each entry is processed sequentially.
    - Outputs are collected and printed in both cases.
