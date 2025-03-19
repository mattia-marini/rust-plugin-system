# Rust plugin system
This aims to be a sensible boilerplate for creating a rust plugin system, allowing plugins to be written in rust as cdylibs, or with any compiled program that supports the C abi

End goal would be to have a semantic versioning system for APIs
  - In `x.0.0` versions, breaking changes are allowed
  - In `0.x.0` versions, adding APIs is allowed, but old APIs are not allowed to change
  - In `0.0.x` versions, only fixes and transparent changes are allowed
