# Architecture Documentation
## Zero-Copy Implementation
This toolkit utilizes Anchor's `zero_copy` attribute to minimize the overhead of account serialization. By mapping the account directly to memory, we achieve significantly higher throughput for large state updates.
