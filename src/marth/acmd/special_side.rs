//! Marth's side special, if used in the air, provides a boost to velocity to use as a recovery option.

use crate::vars::SIDE_SPECIAL_AIR_PHANTOM_HOP;
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

unsafe extern "C" fn marth_specialairs1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(
            agent.module_accessor,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK,
        );
        KineticModule::add_speed(agent.module_accessor, SIDE_SPECIAL_AIR_PHANTOM_HOP);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("top"),
            2.5,
            361,
            30,
            0,
            25,
            6.5,
            0.0,
            9.0,
            13.0,
            None,
            None,
            None,
            0.5,
            0.8,
            *ATTACK_SETOFF_KIND_OFF,
            *ATTACK_LR_CHECK_F,
            true,
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
            *ATTACK_SOUND_LEVEL_S,
            *COLLISION_SOUND_ATTR_CUTUP,
            *ATTACK_REGION_SWORD,
        );
        macros::ATTACK(
            agent,
            1,
            0,
            Hash40::new("top"),
            3.0,
            90,
            30,
            0,
            25,
            5.5,
            0.0,
            9.0,
            16.0,
            None,
            None,
            None,
            0.5,
            0.8,
            *ATTACK_SETOFF_KIND_OFF,
            *ATTACK_LR_CHECK_F,
            true,
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
        macros::ATTACK(
            agent,
            2,
            0,
            Hash40::new("top"),
            2.5,
            361,
            30,
            0,
            25,
            5.5,
            0.0,
            9.0,
            8.0,
            None,
            None,
            None,
            0.5,
            0.8,
            *ATTACK_SETOFF_KIND_OFF,
            *ATTACK_LR_CHECK_F,
            true,
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
            *ATTACK_SOUND_LEVEL_S,
            *COLLISION_SOUND_ATTR_CUTUP,
            *ATTACK_REGION_SWORD,
        );
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(
            agent.module_accessor,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE,
        );
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(
            agent.module_accessor,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK,
        );
    }
}

pub fn install() {
    Agent::new("marth")
        .game_acmd("game_specialairs1", marth_specialairs1)
        .install();
}
