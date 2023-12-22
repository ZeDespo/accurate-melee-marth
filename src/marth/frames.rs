//! To simulate a 100% accurate Melee Marth, make Marth absolutely nasty in the air.

use skyline_smash::{
    app::BattleObjectModuleAccessor,
    lib::lua_const::{
        FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK,
        FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE,
        FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO,
    },
};

use {
    smash::{app::lua_bind::*, hash40, lua2cpp::*},
    smashline::*,
};

unsafe extern "C" fn cancel_move_if_able(
    boma: *mut BattleObjectModuleAccessor,
    attack_flag: i32,
) -> bool {
    if WorkModule::is_flag(boma, attack_flag) {
        WorkModule::off_flag(boma, attack_flag);
        CancelModule::enable_cancel(boma);
        return true;
    }
    return false;
}
// Allow a cancel after Jab 1 like in Melee, Brawl, and Sm4sh.
// Allow a cancel after Dancing Blade 1 if and only if Marth is airborne
pub unsafe extern "C" fn melee_marth_frames(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let mk = MotionModule::motion_kind(boma);
    if mk == hash40("attack_11") {
        cancel_move_if_able(boma, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    } else if mk == hash40("special_air_s1") || mk == hash40("special_s1") {
        let cancelled = cancel_move_if_able(
            boma,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE,
        );
        if cancelled == true {
            WorkModule::off_flag(boma, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
        }
    }
}

pub fn install() {
    Agent::new("marth")
        .on_line(Main, melee_marth_frames)
        .install();
}
