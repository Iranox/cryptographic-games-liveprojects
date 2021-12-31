use crate::utility::cards::Cards;
use crate::utility::cards::Cards::{KING, QUEEN};

pub struct Alice {
    pub want_dating: bool
}

impl Alice {
   pub fn encode(self)->Vec<Cards>{
        return match self.want_dating {
            true => vec![QUEEN, KING],
            _ => vec![KING, QUEEN]
        }
    }
}