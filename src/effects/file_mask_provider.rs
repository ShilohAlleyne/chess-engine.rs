use crate::consts;
use crate::traits::const_lookup;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ConstFileMasks;

impl const_lookup::ConstFileMask for ConstFileMasks {
    fn not_a_file(&self) -> u64 {
        consts::NOT_A_FILE
    }

    fn not_h_file(&self) -> u64 {
        consts::NOT_H_FILE
    }

    fn not_ab_file(&self) -> u64 {
        consts::NOT_AB_FILE
    }

    fn not_hg_file(&self) -> u64 {
        consts::NOT_HG_FILE
    }
}

