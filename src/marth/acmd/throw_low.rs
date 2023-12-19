//! Create an untechable downthrow that the opponent will need to adapt to.

use crate::vars::DOWN_THROW_ANGLE;

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

unsafe extern "C" fn marth_throwlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FT_LEAVE_NEAR_OTTOTTO(agent, 2, 2);
        macros::ATTACK_ABS(
            agent,
            *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,
            0,
            5.0,
            DOWN_THROW_ANGLE,
            50,
            0,
            65,
            0.0,
            1.0,
            *ATTACK_LR_CHECK_F,
            0.0,
            true,
            Hash40::new("collision_attr_normal"),
            *ATTACK_SOUND_LEVEL_S,
            *COLLISION_SOUND_ATTR_NONE,
            *ATTACK_REGION_THROW,
        );
        macros::ATTACK_ABS(
            agent,
            *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH,
            0,
            3.0,
            361,
            100,
            0,
            40,
            0.0,
            1.0,
            *ATTACK_LR_CHECK_F,
            0.0,
            true,
            Hash40::new("collision_attr_normal"),
            *ATTACK_SOUND_LEVEL_S,
            *COLLISION_SOUND_ATTR_NONE,
            *ATTACK_REGION_THROW,
        );
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::FT_CATCH_STOP(agent, 5, 1);
        macros::CHECK_FINISH_CAMERA(agent, 1, 0);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(
            agent.module_accessor,
            *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT,
        );
        let target_group = WorkModule::get_int64(
            agent.module_accessor,
            *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP,
        );
        let target_no = WorkModule::get_int64(
            agent.module_accessor,
            *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO,
        );
        macros::ATK_HIT_ABS(
            agent,
            *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW,
            Hash40::new("throw"),
            target,
            target_group,
            target_no,
        );
    }
}

pub fn install() {
    Agent::new("marth")
        .game_acmd("game_throwlw", marth_throwlw)
        .install();
}
