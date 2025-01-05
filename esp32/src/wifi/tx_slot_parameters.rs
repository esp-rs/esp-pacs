#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Used to set transmission parameters for the slot"]
pub struct TX_SLOT_PARAMETERS {
    dummy: DUMMY,
    plcp2: PLCP2,
    ht_sig: HT_SIG,
    ht_unknown: HT_UNKNOWN,
    duration: DURATION,
    _reserved5: [u8; 0x04],
    pmd: PMD,
    _reserved_end: [u8; 0x20],
}
impl TX_SLOT_PARAMETERS {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn dummy(&self) -> &DUMMY {
        &self.dummy
    }
    #[doc = "0x04 - PLCP2"]
    #[inline(always)]
    pub const fn plcp2(&self) -> &PLCP2 {
        &self.plcp2
    }
    #[doc = "0x08 - HT-SIG field in HT preamble"]
    #[inline(always)]
    pub const fn ht_sig(&self) -> &HT_SIG {
        &self.ht_sig
    }
    #[doc = "0x0c - exact meaning and name unknown, related to HT"]
    #[inline(always)]
    pub const fn ht_unknown(&self) -> &HT_UNKNOWN {
        &self.ht_unknown
    }
    #[doc = "0x10 - duration of the frame exchange"]
    #[inline(always)]
    pub const fn duration(&self) -> &DURATION {
        &self.duration
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn pmd(&self) -> &PMD {
        &self.pmd
    }
}
#[doc = "DUMMY (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`dummy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dummy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dummy`] module"]
pub type DUMMY = crate::Reg<dummy::DUMMY_SPEC>;
#[doc = ""]
pub mod dummy;
#[doc = "PLCP2 (rw) register accessor: PLCP2\n\nYou can [`read`](crate::Reg::read) this register and get [`plcp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plcp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plcp2`] module"]
pub type PLCP2 = crate::Reg<plcp2::PLCP2_SPEC>;
#[doc = "PLCP2"]
pub mod plcp2;
#[doc = "HT_SIG (rw) register accessor: HT-SIG field in HT preamble\n\nYou can [`read`](crate::Reg::read) this register and get [`ht_sig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ht_sig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ht_sig`] module"]
pub type HT_SIG = crate::Reg<ht_sig::HT_SIG_SPEC>;
#[doc = "HT-SIG field in HT preamble"]
pub mod ht_sig;
#[doc = "HT_UNKNOWN (rw) register accessor: exact meaning and name unknown, related to HT\n\nYou can [`read`](crate::Reg::read) this register and get [`ht_unknown::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ht_unknown::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ht_unknown`] module"]
pub type HT_UNKNOWN = crate::Reg<ht_unknown::HT_UNKNOWN_SPEC>;
#[doc = "exact meaning and name unknown, related to HT"]
pub mod ht_unknown;
#[doc = "DURATION (rw) register accessor: duration of the frame exchange\n\nYou can [`read`](crate::Reg::read) this register and get [`duration::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`duration::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@duration`] module"]
pub type DURATION = crate::Reg<duration::DURATION_SPEC>;
#[doc = "duration of the frame exchange"]
pub mod duration;
#[doc = "PMD (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`pmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmd`] module"]
pub type PMD = crate::Reg<pmd::PMD_SPEC>;
#[doc = ""]
pub mod pmd;
