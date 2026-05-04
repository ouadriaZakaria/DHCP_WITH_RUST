## Project structure
```
src/
├── main.rs
├── network/
│   ├── mod.rs
│   ├── listener.rs
│   └── responder.rs
├── protocol/
│   ├── mod.rs
│   ├── packet.rs      # Binary format definition
│   ├── options.rs     # TLV (Type-Length-Value) logic
│   └── dora.rs        # State machine logic
└── storage/
    ├── mod.rs
    ├── manager.rs     # CRUD for leases
    ├── pool.rs        # IP range logic
    └── lease.rs       # Lease data structure
```
