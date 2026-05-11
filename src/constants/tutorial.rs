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

// PixelWorlds inventory keys are packed as `(inventory_type << 16) | block_type`.
// Block placement uses inventory_type=0, so the key collapses to the block id;
// seedlings/fertilizers use inventory_type=512 which yields e.g. 33557167 for
// soil (2735) and 33555502 for fertilizer (1070). Source: frida decode rec
// 346 (BUp Bi=2735), rec 454 (BUp Bi=33557167) and rec 480 (BUp Bi=33555502).
pub const fn inventory_key(inventory_type: u16, block_type: i32) -> i32 {
    ((inventory_type as i32) << 16) | block_type
}

// Soil block placement key (InventoryType=0 collapses to BlockType).
pub const SOIL_PLACEMENT_BI: i32 = inventory_key(0, SOIL_BLOCK_ID);
// Seedling/fertilizer inventory keys for the planting phase.
pub const SOIL_SEED_BI: i32 = inventory_key(SEED_INVENTORY_TYPE, SOIL_BLOCK_ID);
pub const FERTILIZER_SEED_BI: i32 = inventory_key(FERTILIZER_INVENTORY_TYPE, FERTILIZER_BLOCK_ID);

// Number of times the bot hits each soil block before it spawns a gem.
// Source: frida decode rec 374-440 — each of the four BUILD_TARGETS receives
// either four (66,39) or five (rest) HB packets in succession.
pub const HIT_COUNT_PER_TILE: usize = 5;

// Pause after PAoP before sending mp(65, 47) + PAiP.
// Source: frida decode rec 294 → 298 (~880ms, one ping interval).
pub fn portal_teleport_pause() -> Duration {
    Duration::from_millis(880)
}

// Pauses between climb steps (rec 298 → 300 → 302 → 304 → 306).
pub fn climb_step_short_pause() -> Duration {
    Duration::from_millis(260)
}

pub fn climb_step_long_pause() -> Duration {
    Duration::from_millis(450)
}

// Idle pause after reaching (65, 39) before the BUp(soil) packet.
// Source: frida decode rec 306 → 346 (~6.0s of idle ST/p packets).
pub fn climb_to_build_pause() -> Duration {
    Duration::from_millis(6_000)
}

// Pause after BUp(soil) before the first SB packet.
// Source: frida decode rec 346 → 354 (~2.4s).
pub fn build_select_pause() -> Duration {
    Duration::from_millis(2_400)
}

// Pause between consecutive SB packets.
// Source: frida decode rec 354 → 356 → 358 (~250-280ms).
pub fn build_step_pause() -> Duration {
    Duration::from_millis(280)
}

// Pause between the last SB and the first HB.
// Source: frida decode rec 362 → 374 (~1.4s).
pub fn build_to_hit_pause() -> Duration {
    Duration::from_millis(1_400)
}

// Pause between consecutive HB packets on the same tile.
// Source: frida decode rec 402-410 (~200ms between hits on (67,39)).
pub fn hit_step_pause() -> Duration {
    Duration::from_millis(200)
}

// Pause between hitting different soil tiles.
// Source: frida decode rec 410 → 424 (~1.4s gap between tile groups).
pub fn hit_tile_pause() -> Duration {
    Duration::from_millis(1_400)
}

// Pause after the last HB before the first plant BUp.
// Source: frida decode rec 442 → 454 (~3.0s including ST sync).
pub fn hit_to_plant_pause() -> Duration {
    Duration::from_millis(3_000)
}

// Pause between the soil seedling plant and the fertilizer plant.
// Source: frida decode rec 464 → 488 (~3.1s).
pub fn plant_fertilizer_pause() -> Duration {
    Duration::from_millis(3_100)
}

// Pause after the fertilizer plant before sending HB(64, 39) to harvest.
// Source: frida decode rec 488 → 510 (~2.6s while the seedling grows).
pub fn plant_to_harvest_pause() -> Duration {
    Duration::from_millis(2_600)
}

// Pause between HB(64, 39) and starting the collect walk.
// Source: frida decode rec 510 → 522 (~1.7s of idle mP packets).
pub fn harvest_to_collect_pause() -> Duration {
    Duration::from_millis(1_700)
}

// Pause between collect steps (walking and picking up gems).
// Source: frida decode rec 524 → 532 → 540 (~1.0-1.1s per gem).
pub fn collect_step_pause() -> Duration {
    Duration::from_millis(1_000)
}

// Pause after the last gem is collected before sending PSicU/BIPack.
// Source: frida decode rec 568 → 638 (~17s of idle while NPC dialog plays).
pub fn collect_to_npc_pause() -> Duration {
    Duration::from_millis(17_000)
}

// Pause between BIPack and A(AE=6).
// Source: frida decode rec 638 → 640 (~0.3s with a single ping interval).
pub fn bipack_to_action_pause() -> Duration {
    Duration::from_millis(300)
}

// Pause after A(AE=6) before sending the first WeOwC.
// Source: frida decode rec 640 → 774 (~ a long stretch of PSicU and ST keepalives).
pub fn action_to_equip_pause() -> Duration {
    Duration::from_millis(13_500)
}

// Pause between consecutive WeOwC packets.
// Source: frida decode rec 774 → 804 (~3s) and 804 → 818 (~1.4s).
pub fn equip_first_pause() -> Duration {
    Duration::from_millis(3_000)
}

pub fn equip_second_pause() -> Duration {
    Duration::from_millis(1_400)
}

// Pause after the last WeOwC before TState=7 + LW.
// Source: frida decode rec 818 → 842 (~2.4s).
pub fn equip_to_leave_pause() -> Duration {
    Duration::from_millis(2_400)
}

// Pause after LW before sending TTjW PIXELSTATION.
// Source: frida decode rec 842 → 862 (~ a couple of seconds for LW ack).
pub fn leave_to_join_pause() -> Duration {
    Duration::from_millis(2_000)
}

// Multi-tile climb groups. Each tuple is (tiles, world_y, anim) where `tiles`
// are emitted in a single mp packet and `world_y` is the visible y the bot
// reports in the accompanying mP packet. World x stays at 20.80 (tile 65) for
// the whole sequence. Source: frida decode rec 298-306 — the legitimate
// client sends five mp packets covering the (65, 47)→(65, 39) climb with
// animation transitions IDLE→LAUNCH(4)→JET(5)→JET(5)→IDLE(1).
pub const CLIMB_ENTRY_WORLD_Y: f64 = 14.88;
pub const CLIMB_STEPS: &[(&[(i32, i32)], f64, i32)] = &[
    (&[(65, 46)], 14.855, 4),
    (&[(65, 45)], 14.355, 5),
    (&[(65, 44), (65, 43), (65, 42), (65, 41)], 13.142, 5),
    (&[(65, 40), (65, 39)], 12.320, 1),
];
pub const CLIMB_TOP_WORLD_X: f64 = 20.80;
pub const CLIMB_TOP_WORLD_Y: f64 = 12.32;

// Soil placement order. Frida rec 354/356/362 groups the four SBs as
// (66,40) → (66,39)+(67,40) → (67,39). The two-SB packet keeps placements
// adjacent so the server validates them in one tick. The bot mirrors this
// grouping below.
pub const BUILD_GROUPS: &[&[(i32, i32)]] = &[
    &[(66, 40)],
    &[(66, 39), (67, 40)],
    &[(67, 39)],
];

// Build target order matched to the gem collect order observed in frida:
//   seedling (FARM_TARGET) → (66, 39) → (67, 39) → (67, 40) → (66, 40)
// Source: frida decode rec 524/532/540/562/566.
pub const COLLECT_ORDER: &[(i32, i32)] = &[
    (FARM_TARGET_X, FARM_TARGET_Y),
    (66, 39),
    (67, 39),
    (67, 40),
    (66, 40),
];

// Timeout for waiting for the harvested seedling drop and the four gems to
// appear in `state.collectables`. Source: frida decode shows all five
// collectables present within ~3s of the harvest hit.
pub fn full_collectable_timeout() -> Duration {
    Duration::from_secs(15)
}

// Timeout for waiting for the post-tutorial world (PIXELSTATION) to load.
pub fn post_tutorial_world_timeout() -> Duration {
    Duration::from_secs(30)
}

#[cfg(test)]
mod tests {
    use super::{
        FERTILIZER_SEED_BI, SOIL_PLACEMENT_BI, SOIL_SEED_BI, inventory_key,
    };

    /// Frida decode rec 346 / 454 / 480 show the legitimate client sending
    /// BUp Bi=2735 for the soil placement, Bi=33557167 for the soil seedling,
    /// and Bi=33555502 for the fertilizer. The packed inventory key formula
    /// must reproduce all three values exactly.
    #[test]
    fn inventory_keys_match_frida_decode() {
        assert_eq!(SOIL_PLACEMENT_BI, 2735);
        assert_eq!(SOIL_SEED_BI, 33_557_167);
        assert_eq!(FERTILIZER_SEED_BI, 33_555_502);
        assert_eq!(inventory_key(0, 0), 0);
        assert_eq!(inventory_key(512, 1), (512 << 16) | 1);
    }
}
