//! Give Jab 1 more hitstun to enable cancel, if and only if you hit with the sword.

use crate::vars::{JAB_1_HITSTUN, JAB_1_HITSTUN_TIPPER};

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

unsafe extern "C" fn marth_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("armr"),
            3.0,
            361,
            15,
            0,
            30,
            3.0,
            0.0,
            0.0,
            0.0,
            None,
            None,
            None,
            1.3,
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
            *ATTACK_SOUND_LEVEL_S,
            *COLLISION_SOUND_ATTR_CUTUP,
            *ATTACK_REGION_SWORD,
        );
        macros::ATTACK(
            agent,
            1,
            0,
            Hash40::new("sword1"),
            3.0,
            361,
            12,
            0,
            30,
            3.0,
            0.5,
            0.0,
            1.5,
            None,
            None,
            None,
            JAB_1_HITSTUN,
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
            *ATTACK_SOUND_LEVEL_S,
            *COLLISION_SOUND_ATTR_CUTUP,
            *ATTACK_REGION_SWORD,
        );
        macros::ATTACK(
            agent,
            2,
            0,
            Hash40::new("sword1"),
            5.0,
            180,
            12,
            0,
            30,
            3.0,
            1.0,
            0.0,
            7.0,
            None,
            None,
            None,
            JAB_1_HITSTUN_TIPPER,
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
            *COLLISION_CATEGORY_MASK_FIGHTER,
            *COLLISION_PART_MASK_ALL,
            false,
            Hash40::new("collision_attr_cutup"),
            *ATTACK_SOUND_LEVEL_M,
            *COLLISION_SOUND_ATTR_MARTH_SWORD,
            *ATTACK_REGION_SWORD,
        );
        macros::ATTACK(
            agent,
            3,
            0,
            Hash40::new("sword1"),
            5.0,
            361,
            12,
            0,
            30,
            3.0,
            1.0,
            0.0,
            7.0,
            None,
            None,
            None,
            JAB_1_HITSTUN_TIPPER,
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
            *COLLISION_SOUND_ATTR_MARTH_SWORD,
            *ATTACK_REGION_SWORD,
        );
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(
            agent.module_accessor,
            *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO,
        );
    }
}

pub fn install() {
    Agent::new("marth")
        .game_acmd("game_attack11", marth_attack11)
        .install();
}
