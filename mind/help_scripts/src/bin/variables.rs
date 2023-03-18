use convert_case::Casing;

use help_scripts::scanner::{advance_to_next_string, convert_to_rust_type};

fn main() {
    let s = "public static final EventType.BuildDamageEvent BULLET_DAMAGE_EVENT = new BuildDamageEvent();

  public static final float HIT_DURATION = 9.0F;

  public static final float RECENT_DAMAGE_TIME = 60.0F * 5.0F;

  public static int SLEEPING_ENTITIES = 0;

  public static final EventType.BuildTeamChangeEvent TEAM_CHANGE_EVENT = new BuildTeamChangeEvent();

  public static final Seq<Building> TEMP_BUILDS = new Seq<>();

  public static final float TIME_TO_SLEEP = 60.0F * 1;

  public static final ObjectSet<Building> TMP_TILES = new ObjectSet<>();

  protected transient boolean added;

  public transient Block block;

  public transient int cdump;

  public transient boolean dead;

  protected transient float dumpAccum;

  public transient float efficiency;

  public transient boolean enabled = true;

  public transient float heal_suppression_time = -1.0F;

  public float health;

  public transient float hit_time;

  public transient int id = EntityGroup.nextId();

  protected transient int index__all = -1;

  protected transient int index__build = -1;

  protected transient boolean initialized;

  @Nullable
  public ItemModule items;

  public transient String last_accessed;

  protected transient float lastDamageTime = -RECENT_DAMAGE_TIME;

  @Nullable
  public transient Building last_disabler;

  public transient float last_heal_time = -120.0F * 10.0F;

  @Nullable
  public LiquidModule liquids;

  public transient float max_health = 1.0F;

  public transient float optional_efficiency;

  public transient float payload_rotation;

  public transient float potential_efficiency;

  @Nullable
  public PowerModule power;

  public transient Seq<Building> proximity = new Seq<>(6);

  public transient int rotation;

  protected transient float sleepTime;

  protected transient boolean sleeping;

  @Nullable
  protected transient SoundLoop sound;

  public Team team = Team.derelict;

  public transient Tile tile;

  protected transient float timeScale = 1.0F;

  protected transient float timeScaleDuration;

  public transient Interval timer = new Interval(6);

  public transient long visible_flags;

  public transient float visual_liquid;

  public transient boolean was_damaged;

  public transient boolean was_visible;

  @Annotations.SyncField(true)
  @Annotations.SyncLocal
  public float x;

  @Annotations.SyncField(true)
  @Annotations.SyncLocal
  public float y;";

    let mut statics = "".to_string();
    let mut struct_vars = "".to_string();
    let mut struct_init = "".to_string();

    for line in s.lines() {
        // ignore empty lines
        if line == "" {
            continue;
        }
        let mut line = &*line.replace("final", "");
        let mut line = &*line.replace("transient", "");
        let mut line = &*line.replace("protected", "");
        line = line.trim();
        if line.starts_with("@") {
            continue;
        }
        let mut code = "".to_string();
        // replace public with pub
        if let Some(start) = advance_to_next_string(&line, "public") {
            code.push_str("pub");
            line = &line[start + 6..];
        }
        line = line.trim();
        // replace static with static
        if let Some(start) = advance_to_next_string(&line, "static") {
            // create a static variable
            code.push_str(" static");
            line = &line[start + 6..];
            line = line.trim();
            // get type
            let line: Vec<&str> = line.split(" ").collect();
            let type_ = convert_to_rust_type(line[0]);
            // get name
            let name = line[1];
            // get value
            let value = line[3..].join(" ");
            code.push_str(&format!(" {}: {} = {}\n", name, type_, value));
            statics.push_str(&code);
        } else {
            // create a variable for the struct
            let line: Vec<&str> = line.split(" ").collect();
            let type_ = convert_to_rust_type(line[0]);
            let name = line[1].replace(";", "");
            if line.len() > 2 {
                let value = line[3..].join(" ").replace(";", "");
                code.push_str(&format!(" {}: {},\n", name.to_case(convert_case::Case::Camel), type_));
                struct_init.push_str(&format!("{}: {},\n", name.to_case(convert_case::Case::Camel), value));
            } else {
                code.push_str(&format!(" {}: {},\n", name.to_case(convert_case::Case::Camel), type_));
                struct_init.push_str(&format!("{}: {}::default(),\n", name.to_case(convert_case::Case::Camel), type_));
            }
            struct_vars.push_str(&code);
        }
    }
    println!("statics: \n{}", statics);
    println!("struct_vars: \n{}", struct_vars);
    println!("struct_init: \n{}", struct_init);
}