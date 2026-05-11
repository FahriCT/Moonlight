use std::time::Duration;

pub const TUTORIAL_WORLD: &str = "TUTORIAL2";
pub const POST_TUTORIAL_WORLD: &str = "PIXELSTATION";

pub const TUTORIAL_GENDER: i32 = 0;
pub const TUTORIAL_COUNTRY: i32 = 999;
pub const TUTORIAL_SKIN_COLOR: i32 = 7;

pub const PRE_CHARACTER_POD_SELECTION: [i32; 2] = [2, 20];
pub const STARTER_FACE_BLOCK: i32 = 527;
pub const STARTER_HAIR_BLOCK: i32 = 515;
pub const POST_CHARACTER_POD_CONFIRMATION: [i32; 2] = [10, 5];

// Sleeping pod spawn: map (39, 44) = world (12.48, 13.92).
// The GWC WorldStartPoint for TUTORIAL2 is (40, 30) which is the generic
// visitor spawn; new accounts ignore it and always spawn here instead.
// Source: frida decode — initial mP at x=12.48 y=13.92 tp=true on world enter.
pub const TUTORIAL_SPAWN_MAP_X: i32 = 39;
pub const TUTORIAL_SPAWN_MAP_Y: i32 = 44;

// Pod selection tile the player walks to after character creation.
// Source: frida decode rec 268 — mp pM=[(42,44),(43,44)] sent after TState=5.
pub const SPAWN_POT_MAP_X: i32 = 42;
pub const SPAWN_POT_MAP_Y: i32 = 44;
// Walk path the bot follows to the pod-selection tile (42,44).
// Frida decode shows the legitimate client sending two solo mp packets
// (rec 238 → (40,44); rec 242 → (41,44)) and then a combined mp
// pM=[(42,44),(43,44)] in rec 268 after TState=5. The bot serialises that
// combined packet as a separate (42,44) step before TState=5 because
// make_map_point only carries a single tile; the resulting flow still ends on
// the pod tile (42,44) at exactly the same moment TState=5 is sent.
pub const SPAWN_POD_CONFIRM_PATH: &[(i32, i32)] = &[(40, 44), (41, 44), (42, 44)];

// BIPack the bot purchases at the end of the tutorial NPC chat.
// Source: frida decode rec 638 — BIPack IPId="BasicClothes".
pub const CLOTHES_PACK_ID: &str = "BasicClothes";
// Action event sent alongside the purchase to equip the clothes.
// Source: frida decode rec 640 — A AE=6 (the server responds with APu=[6,5]).
pub const CLOTHES_PACK_AE: i32 = 6;
// Wearable block IDs the bot equips after the clothes purchase.
// Source: frida decode rec 774, 804, 818 — WeOwC hBlock=741, 355, 552.
pub const EQUIP_BLOCKS: [i32; 3] = [741, 355, 552];

// Build/farm phase block IDs.
// Source: frida decode rec 354/356/362 (SB BlockType=2735) and rec 488
// (SS BlockType=1070 planted on the seedling tile).
pub const SOIL_BLOCK_ID: i32 = 2735;
pub const FERTILIZER_BLOCK_ID: i32 = 1070;
pub const SEED_INVENTORY_TYPE: u16 = 512;
pub const FERTILIZER_INVENTORY_TYPE: u16 = 512;

// The four soil tiles the bot places and then hits to spawn the tutorial gems.
// Source: frida decode rec 354/356/362 (SB placement order) and rec 374-440
// (HB hit order); gems collected at rec 532-566 confirm the same positions.
pub const BUILD_TARGETS: [(i32, i32); 4] = [(66, 39), (67, 39), (66, 40), (67, 40)];
// Seedling tile the bot plants the soil + fertilizer on.
// Source: frida decode rec 464/488 — SS at (64, 39) BlockType=2735 / 1070.
pub const FARM_TARGET_X: i32 = 64;
pub const FARM_TARGET_Y: i32 = 39;

// Portal tile the bot stops on before sending TState=6 + PAoP.
// Source: frida decode rec 280 (mp pM=(46,45)) and rec 294 (PAoP x=46 y=45).
pub const PORTAL_APPROACH_X: i32 = 46;
pub const PORTAL_APPROACH_Y: i32 = 45;
// Other side of the portal where the bot teleports in.
// Source: frida decode rec 298 — mp pM=(65,47) + PAiP x=65 y=47.
pub const PORTAL_ENTRY_X: i32 = 65;
pub const PORTAL_ENTRY_Y: i32 = 47;
// Tile the bot walks up to after exiting the portal (top of the shaft).
// Source: frida decode rec 306 — mp pM=[(65,40),(65,39)].
pub const TUTORIAL_LANDING_X: i32 = 65;
pub const TUTORIAL_LANDING_Y: i32 = 39;

// Full pod-to-portal walk path, in the order the legitimate client visits each
// tile. Source: frida decode rec 238/242/268/270/276/278/280 — the bot lands
// at (39,44), walks east through (40-44, 44), drops down to (44,45), then
// finishes the approach at (45,45) and (46,45). The legitimate client never
// visits (43,45) or (44,46); those were artefacts of the old packets.bin
// capture and are removed here so the documented path matches reality.
pub const INTRO_PORTAL_WALK_PATH: &[(i32, i32)] = &[
    (40, 44),
    (41, 44),
    (42, 44),
    (43, 44),
    (44, 44),
    (44, 45),
    (45, 45),
    (46, 45),
];

pub fn short_pause() -> Duration {
    Duration::from_millis(350)
}

pub fn walk_step_pause() -> Duration {
    Duration::from_millis(180)
}

pub fn medium_pause() -> Duration {
    Duration::from_millis(750)
}

pub fn spawn_pod_confirm_timeout() -> Duration {
    Duration::from_secs(6)
}

pub fn spawn_pod_settle_pause() -> Duration {
    Duration::from_millis(2_500)
}

pub fn long_pause() -> Duration {
    Duration::from_millis(1_500)
}

pub fn world_join_timeout() -> Duration {
    Duration::from_secs(25)
}

pub fn initial_spawn_pause() -> Duration {
    Duration::from_secs(6)
}

pub fn post_spawn_tstate_pause() -> Duration {
    Duration::from_secs(5)
}

pub fn pre_charc_friends_list_pause() -> Duration {
    Duration::from_secs(8)
}

pub fn pre_charc_st_pause() -> Duration {
    Duration::from_millis(800)
}

pub fn pre_charc_create_pause() -> Duration {
    Duration::from_millis(267)
}

pub fn post_apu_first_step_pause() -> Duration {
    Duration::from_millis(2950)
}

pub fn post_apu_second_step_pause() -> Duration {
    Duration::from_millis(534)
}

pub fn post_apu_third_step_pause() -> Duration {
    Duration::from_millis(266)
}

pub fn post_apu_tstate5_pause() -> Duration {
    Duration::from_millis(1600)
}

pub fn portal_walk_start_pause() -> Duration {
    Duration::from_millis(3469)
}

pub fn portal_walk_step_pause() -> Duration {
    Duration::from_millis(266)
}

pub fn portal_walk_idle_pause() -> Duration {
    Duration::from_millis(1534)
}

pub fn portal_jump_pause() -> Duration {
    Duration::from_millis(2935)
}

pub fn portal_land_pause() -> Duration {
    Duration::from_millis(267)
}

pub fn portal_settle_pause() -> Duration {
    Duration::from_millis(800)
}

pub fn portal_ready_pause() -> Duration {
    Duration::from_millis(1868)
}

pub fn collectable_timeout() -> Duration {
    Duration::from_secs(8)
}

pub fn portal_transition_timeout() -> Duration {
    Duration::from_secs(6)
}
