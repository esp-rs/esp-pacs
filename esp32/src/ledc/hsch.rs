#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Cluster HSCH%s, containing HSCH?_CONF0, HSCH?_HPOINT, HSCH?_DUTY, HSCH?_CONF1, HSCH?_DUTY_R
pub struct HSCH {
    conf0: CONF0,
    hpoint: HPOINT,
    duty: DUTY,
    conf1: CONF1,
    duty_r: DUTY_R,
}
impl HSCH {
    ///0x00 -
    #[inline(always)]
    pub const fn conf0(&self) -> &CONF0 {
        &self.conf0
    }
    ///0x04 -
    #[inline(always)]
    pub const fn hpoint(&self) -> &HPOINT {
        &self.hpoint
    }
    ///0x08 -
    #[inline(always)]
    pub const fn duty(&self) -> &DUTY {
        &self.duty
    }
    ///0x0c -
    #[inline(always)]
    pub const fn conf1(&self) -> &CONF1 {
        &self.conf1
    }
    ///0x10 -
    #[inline(always)]
    pub const fn duty_r(&self) -> &DUTY_R {
        &self.duty_r
    }
}
/**CONF0 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@conf0`] module*/
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
///
pub mod conf0;
/**HPOINT (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`hpoint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpoint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hpoint`] module*/
pub type HPOINT = crate::Reg<hpoint::HPOINT_SPEC>;
///
pub mod hpoint;
/**DUTY (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`duty::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`duty::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@duty`] module*/
pub type DUTY = crate::Reg<duty::DUTY_SPEC>;
///
pub mod duty;
/**CONF1 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@conf1`] module*/
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
///
pub mod conf1;
/**DUTY_R (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`duty_r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@duty_r`] module*/
pub type DUTY_R = crate::Reg<duty_r::DUTY_R_SPEC>;
///
pub mod duty_r;
