---
description: AI rules derived by SpecStory from the project AI interaction history
globs: *
---

## <headers/>

## PROJECT DOCUMENTATION & CONTEXT SYSTEM

## TECH STACK

## CODING STANDARDS

## WORKFLOW & RELEASE RULES

## DEBUGGING

### SurrealMCP Configuration Rules

- The SurrealMCP server supports running a single instance with multiple namespaces and databases.

### Startup Options

1.  **Recommended:** Start without pre-configured namespace/database to allow maximum flexibility in switching.
    ```bash
    surrealmcp start \
      --endpoint ws://localhost:8000/rpc \
      --user root \
      --pass root
    ```
    Or with Docker:
    ```bash
    docker run --rm -i --pull always \
      -e SURREALDB_URL="ws://localhost:8000/rpc" \
      -e SURREALDB_USER="root" \
      -e SURREALDB_PASS="root" \
      surrealdb/surrealmcp:latest start
    ```
2.  Optionally, start with a default namespace and database:
    ```bash
    surrealmcp start \
      --endpoint ws://localhost:8000/rpc \
      --ns default_namespace \
      --db default_database \
      --user root \
      --pass root
    ```

### Switching Between Namespaces and Databases

- Use the provided tools to switch contexts:
    -   `use_namespace`: Switch to a different namespace.
    -   `use_database`: Switch to a different database.
    -   `list_namespaces`: List all available namespaces.
    -   `list_databases`: List all available databases.

    Example workflow:

    ```
    1. "List all namespaces"
       → Uses list_namespaces tool

    2. "Switch to the 'production' namespace"
       → Uses use_namespace('production')

    3. "List databases in this namespace"
       → Uses list_databases tool

    4. "Switch to the 'analytics' database"
       → Uses use_database('analytics')

    5. "Query all users"
       → Uses select(["user"]) or query("SELECT * FROM user")
    ```

### Configuration for Multiple Agents

To support multiple agents working in parallel in different namespaces and databases, the following updates are required:

-   **Cloud Tooling**: Disable all cloud-hosted related tools by default to prevent accidental calls. A flag with a `true` value, likely in an environment variable, is required to enable them. If the flag is absent or set to `false`, the tools must not be used.
-   **HTTP Transport**: Ensure HTTP transport is enabled.
-   **Parameters**: Add parameters for:
    -   `project name`
    -   `storage type`: An enum with at least `AGENT_MEMORY`, `RECIPES` as values. Other storage types may be added later.
    -   The database should be the name of the workspace. This should be automatically configured, or exposed as a parameter.
-   **Tool Call Parameters**: Instead of setting namespace/database at startup, these must be required parameters on each tool call. Each tool call instance must target the correct namespace and database.