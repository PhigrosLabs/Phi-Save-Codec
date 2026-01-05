use crate::phi_base::*;
use shua_struct::BinaryStruct;

#[derive(Debug, Default, BinaryStruct)]
#[binary_struct(bit_order = shua_struct::Lsb0)]
pub struct User {
    #[binary_field(align = 8)]
    pub show_player_id: bool,
    pub self_intro: PhiString,
    pub avatar: PhiString,
    pub background: PhiString,
}
