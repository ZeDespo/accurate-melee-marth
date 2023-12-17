mod attack_dair;
mod attack_fair;
mod attack_fsmash;
mod attack_uair;
mod grab;
mod special_side;
mod throw_high;
mod throw_low;

pub fn install() {
    attack_dair::install();
    attack_fair::install();
    attack_uair::install();
    attack_fsmash::install();
    grab::install();
    special_side::install();
    throw_high::install();
    throw_low::install();
}