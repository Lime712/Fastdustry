# MindRustry

- Mindustry in rust project
- This version does not compile yet, because it is still in development.
- We try to write all the code first, and then we will start to compile it.
- The reason for that is, that it is very difficult to create a version with fewer features that compiles.

### Guide

- `class` translates to `struct` if no class inherits from it (aka no `extends` keyword)
- `class` translates to `trait` if it inherits from another class (aka `extends` keyword)
- sometimes the super class has fields, in that case there are 2 options:
    - store a reference to the "super struct" in a field, eg:
    ```rust
    pub struct Entity {
        pub id: i32,
        pub x: f32,
        pub y: f32,
    }

    pub struct Player {
        pub entity: Entity,
        pub name: String,
    }
    ```
    - create a new `trait` that contains a getter and setter for all the fields of the super class, eg:
    ```rust
    pub trait Entity {
        fn set_id(&mut self, id: i32);
        fn get_id(&self) -> i32;
        fn set_x(&mut self, x: f32);
        fn get_x(&self) -> f32;
        fn set_y(&mut self, y: f32);
        fn get_y(&self) -> f32;
    }
    
    pub struct Player {
        id: i32,
        x: f32,
        y: f32,
        pub name: String,
    }
    
    impl Entity for Player {
        fn set_id(&mut self, id: i32) { self.id = id; }
        fn get_id(&self) -> i32 { self.id }
        fn set_x(&mut self, x: f32) { self.x = x; }
        fn get_x(&self) -> f32 { self.x }
        fn set_y(&mut self, y: f32) { self.y = y; }
        fn get_y(&self) -> f32 { self.y }
    }
  ```
  but that creates a lot of boilerplate code

- `interface` translates to `trait`
- `arc_core` is the core of arc

### Progress

- Official [trello board](https://trello.com/b/b9KlBgIu/mindrustry)

### TODO

- everything

### Notes

- `seq<>` in the original version are `HashSets` here
- Always try to use `&str` instead of `String` if possible, for example the names of items or teams wont change, so they
  can be `&str` instead of `String`
- Always try to use references in general instead of cloning the value, because that uses less ram and cpu usage, eg:

```rust
pub struct Player {
    pub team: &'static Team,
}

impl Player {
    pub fn new(team: &'static Team) -> Self {
        Self { team }
    }
}
```

- Try to use consts instead of statics, because they get interpreted on compile time, so they code is faster, eg:

```rust
pub const VERSION: &str = "143.0";
```
