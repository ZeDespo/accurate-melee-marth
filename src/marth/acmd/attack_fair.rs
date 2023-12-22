//! Marth's forward aerial in melee was THE facilitator for the
//! Ken Combo. Restore FAIR to its former glory.
//!
//! And by that, I mean make it unnecessarily broken.

use crate::vars::{FAIR_TIPPER_SIZE, KEN_COMBO_FAIR_ANGLE};

use {
    smash::{
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::lua_const::*,
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::*,
};

unsafe extern "C" fn marth_attackairf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(
            agent.module_accessor,
            *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING,
        );
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.66); // Active Frame on or about 4
    frame(agent.lua_state_agent, 4.0);
    macros::FT_MOTION_RATE(agent, 1.3);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("sword1"),
            8.0,
            361,
            70,
            0,
            40,
            3.0,
            1.0,
            0.0,
            1.5,
            None,
            None,
            None,
            0.7,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_cutup"),
            *ATTACK_SOUND_LEVEL_M,
            *COLLISION_SOUND_ATTR_CUTUP,
            *ATTACK_REGION_SWORD,
        );
        macros::ATTACK(
            agent,
            1,
            0,
            Hash40::new("armr"),
            8.0,
            361,
            70,
            0,
            40,
            3.8,
            1.0,
            0.0,
            0.0,
            None,
            None,
            None,
            0.7,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_cutup"),
            *ATTACK_SOUND_LEVEL_M,
            *COLLISION_SOUND_ATTR_CUTUP,
            *ATTACK_REGION_SWORD,
        );
        macros::ATTACK(
            agent,
            2,
            0,
            Hash40::new("sword1"),
            13.0,
            KEN_COMBO_FAIR_ANGLE,
            70,
            0,
            42,
            FAIR_TIPPER_SIZE,
            1.0,
            0.0,
            7.5,
            None,
            None,
            None,
            2.0,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_F,
            false,
            0,
            0.0,
            0,
            false,
            false,
            false,
            false,
            true,
            *COLLISION_SITUATION_MASK_GA,
            *COLLISION_CATEGORY_MASK_ALL,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_cutup"),
            *ATTACK_SOUND_LEVEL_L,
            *COLLISION_SOUND_ATTR_MARTH_SWORD,
            *ATTACK_REGION_SWORD,
        );
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(
            agent.module_accessor,
            *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING,
        );
    }
}

pub fn install() {
    Agent::new("marth")
        .game_acmd("game_attackairf", marth_attackairf)
        .install();
}
