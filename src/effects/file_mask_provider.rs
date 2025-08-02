use crate::consts as CONSTS;
use crate::traits::const_lookup as CONST_LOOKUP;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ConstFileMasks;

impl CONST_LOOKUP::ConstFileMask for ConstFileMasks {
    fn not_a_file(&self) -> u64 {
        CONSTS::NOT_AB_FILE
    }

    fn not_h_file(&self) -> u64 {
        CONSTS::NOT_H_FILE
    }

    fn not_ab_file(&self) -> u64 {
        CONSTS::NOT_AB_FILE
    }

    fn not_hg_file(&self) -> u64 {
        CONSTS::NOT_HG_FILE
    }
}

