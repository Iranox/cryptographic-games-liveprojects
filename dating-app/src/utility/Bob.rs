use crate::utility::cards::Cards;
use crate::utility::cards::Cards::{KING, QUEEN};

pub struct Bob {
    pub want_dating: bool
}

impl Bob {
    pub fn encode(self)->Vec<Cards>{
        return match self.want_dating {
            true => vec![KING, QUEEN],
            _ => vec![QUEEN, KING]
        }
    }
}