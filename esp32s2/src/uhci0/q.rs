#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster Q%s, containing Q?_WORD0, Q?_WORD1"]
pub struct Q {
    word0: WORD0,
    word1: WORD1,
}
impl Q {
    #[doc = "0x00 - Q0_WORD0 quick_sent register"]
    #[inline(always)]
    pub const fn word0(&self) -> &WORD0 {
        &self.word0
    }
    #[doc = "0x04 - Q0_WORD1 quick_sent register"]
    #[inline(always)]
    pub const fn word1(&self) -> &WORD1 {
        &self.word1
    }
}
#[doc = "WORD0 (rw) register accessor: Q0_WORD0 quick_sent register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`word0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`word0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word0`] module"]
pub type WORD0 = crate::Reg<word0::WORD0_SPEC>;
#[doc = "Q0_WORD0 quick_sent register"]
pub mod word0;
#[doc = "WORD1 (rw) register accessor: Q0_WORD1 quick_sent register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`word1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`word1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@word1`] module"]
pub type WORD1 = crate::Reg<word1::WORD1_SPEC>;
#[doc = "Q0_WORD1 quick_sent register"]
pub mod word1;
