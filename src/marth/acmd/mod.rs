mod attack_bair;
mod attack_dair;
mod attack_fair;
mod attack_fsmash;
mod attack_jab;
mod attack_nair;
mod attack_uair;
mod grab;
mod special_side;
mod throw_high;
mod throw_low;

pub fn install() {
    attack_dair::install();
    attack_fair::install();
    attack_uair::install();
    attack_jab::install();
    attack_fsmash::install();
    attack_nair::install();
    attack_bair::install();
    grab::install();
    special_side::install();
    throw_high::install();
    throw_low::install();
}
