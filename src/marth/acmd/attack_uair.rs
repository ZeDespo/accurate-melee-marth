//! Make the sword's active hitbox stay out just a bit longer.

use {
    smash::{
        app::{lua_bind::*, sv_animcmd::*},
        lib::lua_const::*,
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::*,
};

unsafe extern "C" fn marth_attackairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(
            agent.module_accessor,
            *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING,
        );
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(
            agent,
            0,
            0,
            Hash40::new("sword1"),
            9.5,
            80,
            80,
            0,
            40,
            3.5,
            1.0,
            0.0,
            0.5,
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
            1,
            0,
            Hash40::new("armr"),
            9.5,
            80,
            80,
            0,
            40,
            3.0,
            0.0,
            1.0,
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
            Hash40::new("colonells"),
            9.5,
            80,
            80,
            0,
            40,
            2.0,
            0.0,
            0.0,
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
            *ATTACK_SOUND_LEVEL_S,
            *COLLISION_SOUND_ATTR_CUTUP,
            *ATTACK_REGION_SWORD,
        );
        macros::ATTACK(
            agent,
            3,
            0,
            Hash40::new("sword1"),
            13.0,
            90,
            84,
            0,
            40,
            3.5,
            1.0,
            0.0,
            6.7,
            None,
            None,
            None,
            1.25,
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
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(
            agent.module_accessor,
            *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING,
        );
    }
}

pub fn install() {
    Agent::new("marth")
        .game_acmd("game_attackairhi", marth_attackairhi)
        .install();
}
