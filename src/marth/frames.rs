//! To simulate a 100% accurate Melee Marth, make Marth absolutely nasty in the air.

use {
    smash::{app::lua_bind::*, hash40, lua2cpp::*},
    smashline::*,
};

use crate::vars::CancelFrame;

const CANCELLABLE_MOVES: &'static [CancelFrame] = &[
    CancelFrame {
        action: hash40("attack_air_f"),
        cancel_frame: 15.0,
    },
    CancelFrame {
        action: hash40("attack_air_lw"),
        cancel_frame: 30.0,
    },
    CancelFrame {
        action: hash40("attack_air_hi"),
        cancel_frame: 28.0,
    },
];

pub unsafe extern "C" fn melee_marth_frames(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let motion_kind = MotionModule::motion_kind(boma);
    let cm: Option<&CancelFrame> = CANCELLABLE_MOVES
        .into_iter()
        .filter(|&pair| pair.action == motion_kind)
        .next();
    match cm {
        Some(cf) => {
            if cf.is_cancellable(boma) {
                CancelModule::enable_cancel(boma);
            }
        }
        None => {}
    }
}

pub fn install() {
    Agent::new("marth")
        .on_line(Main, melee_marth_frames)
        .install();
}
