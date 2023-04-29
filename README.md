# Fastdustry

- Fastdustry is a total rewrite of mindustry in rust.
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
  The problem here is that if we have many inheritances the code for accessing the fields will be very long, eg: `meltdown.laserTurret.powerTurret.turret.reloadTurret.baseTurret.block.range`
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
- `@Nullable` in the original version are `Option`s here
- Try to implement the `Default` trait for all structs, because it's a good way to provide default values for the fields
- #### Comments
    - `//` is a single line comment, used for comments that are describe how something works or why something is done in a certain way, or todos
    - `/** */` becomes `///` here. It is a documentation comment, it is used to generate documentation for the code, it is used by the `rustdoc` tool
    - `@param` becomes `# Arguments` here
    - `@return` becomes `# Returns` here, but don't use it too often, cause most of the time its clear what the function returns after reading the description
    - `@see` becomes `# See also` here
    - `@throws` becomes `# Panics` here
    - `@deprecated` becomes `#[deprecated]` here
    - `@implNote` becomes `# Implementation notes` here
    - `@implSpec` becomes `# Implementation details` here
    - Sometimes its useful to include examples in the documentation, for that we use the `# Examples` section.
    - Please visit the [rust book](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments) for more information about documentation comments
    - the [`comment`](https://github.com/WMF-Industries/MindRustry/blob/Mods/mind/help_scripts/src/bin/comment.rs) script already uses the correct syntax most of the time, but still needs manual editing
- Always try to use `&str` instead of `String` if possible, for example the names of items or teams won;t change, so they
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
- There are utility scripts in the [`help_scripts`](https://github.com/WMF-Industries/MindRustry/tree/Mods/mind/help_scripts) folder, they are not used in the project, but they can be used to help
  with the development of the project


### Contact
Join our [discord server](http://discord.phoenix-network.dev) if you have any questions or suggestions
