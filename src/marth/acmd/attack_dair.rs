//! Ken Combo code to get the spike just right in terms of its hitboxes
//! and knockback angles.
//!
//! Also let's make the tipper more pronounced, as it's not easy getting a parody tipper

use crate::vars::{DAIR_BASE_KNOCKBACK, DAIR_TIPPER_ANGLE, DAIR_TIPPER_HITSTUN, SWORD_HITBOX_SIZE};

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

// 1) All tippers are spikes, no matter what.
unsafe extern "C" fn marth_attackairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(
            agent.module_accessor,
            *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING,
        );
        ControlModule::set_attack_air_kind(
            agent.module_accessor,
            *FIGHTER_COMMAND_ATTACK_AIR_KIND_B,
        );
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            1,
            0,
            Hash40::new("armr"),
            12.0,
            361,
            70,
            0,
            30,
            2.0,
            0.0,
            0.5,
            0.0,
            None,
            None,
            None,
            0.7,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_POS,
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
            12.0,
            DAIR_TIPPER_ANGLE,
            70,
            0,
            DAIR_BASE_KNOCKBACK,
            SWORD_HITBOX_SIZE - 0.8,
            1.0,
            0.0,
            0.0,
            None,
            None,
            None,
            DAIR_TIPPER_HITSTUN,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_POS,
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
            3,
            0,
            Hash40::new("sword1"),
            14.0,
            DAIR_TIPPER_ANGLE,
            80,
            0,
            DAIR_BASE_KNOCKBACK * 2,
            SWORD_HITBOX_SIZE + 0.5,
            1.0,
            0.0,
            6.7,
            None,
            None,
            None,
            DAIR_TIPPER_HITSTUN,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_POS,
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
            *COLLISION_SOUND_ATTR_MARTH_SWORD,
            *ATTACK_REGION_SWORD,
        );
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            3,
            0,
            Hash40::new("sword1"),
            14.0,
            361,
            80,
            0,
            DAIR_BASE_KNOCKBACK * 2,
            SWORD_HITBOX_SIZE,
            1.0,
            0.0,
            6.7,
            None,
            None,
            None,
            DAIR_TIPPER_HITSTUN,
            1.0,
            *ATTACK_SETOFF_KIND_ON,
            *ATTACK_LR_CHECK_POS,
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
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(
            agent.module_accessor,
            *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING,
        );
    }
}

pub fn install() {
    Agent::new("marth")
        .game_acmd("game_attackairlw", marth_attackairlw)
        .install();
}
