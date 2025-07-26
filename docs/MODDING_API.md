# Modding API

AeonSeed exposes a lightweight API for custom content. Mods can define new
classes, items or quests using JSON files loaded at startup.

- Use `modding::api::ModdingRegistry` to register custom definitions
- AI related systems are available through the `seed_ai` re-export
- Example:
  ```json
  { "classes": [ { "name": "Chronomancer" } ] }
  ```

The API is designed for expansion and will grow with future versions.
