# Modulübersicht

```
+-------------------------+
|        Client          |
|  (Bevy, Render, Input) |
+-----------+-------------+
            |
            v
+-----------+-------------+
|      Game-Logic         |
|  Klassen, Berufe, Quests|
|  Ethik & Fraktionen     |
+-----------+-------------+
            |
            v
+-----------+-------------+
|      SeedNet            |
|  Seed-Persistenz, DNA   |
+-----------+-------------+
            |
            v
+-----------+-------------+
|     Cluster-AI          |
|  Weltweite Koordination |
+-----------+-------------+
            |
            v
+-----------+-------------+
|    AeonCode / Puzzle    |
+-------------------------+
```

- **TTS & VoiceCloning** hängen am Localization-Modul und können optional aktiviert werden.
- Jeder Seed besitzt eigene NPCs, Biome und Ereignisse, gesteuert über die Cluster-AI.
- Neue Fraktionen und Erinnerungen entstehen dynamisch und fließen in alle Module ein.
