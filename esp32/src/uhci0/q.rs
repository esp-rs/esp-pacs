#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Cluster Q%s, containing Q?_WORD0, Q?_WORD1
pub struct Q {
    word0: WORD0,
    word1: WORD1,
}
impl Q {
    ///0x00 -
    #[inline(always)]
    pub const fn word0(&self) -> &WORD0 {
        &self.word0
    }
    ///0x04 -
    #[inline(always)]
    pub const fn word1(&self) -> &WORD1 {
        &self.word1
    }
}
/**WORD0 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`word0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`word0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@word0`] module*/
pub type WORD0 = crate::Reg<word0::WORD0_SPEC>;
///
pub mod word0;
/**WORD1 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`word1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`word1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@word1`] module*/
pub type WORD1 = crate::Reg<word1::WORD1_SPEC>;
///
pub mod word1;
