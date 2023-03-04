# MindRustry
Mindustry in rust project

### Notes
- `seq<>` in the original version are `HashSets` here

### Guide
- `class` translates to `struct` if no class inherits from it (aka no `extends` keyword)
- `class` translates to `trait` if it inherits from another class (aka `extends` keyword)
- `interface` translates to `trait`
- `arc_core` is the core of arc