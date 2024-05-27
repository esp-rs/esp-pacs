#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Cluster CH%s, containing CH?_CONF0, CH?_HPOINT, CH?_DUTY, CH?_CONF1, CH?_DUTY_R
pub struct CH {
    conf0: CONF0,
    hpoint: HPOINT,
    duty: DUTY,
    conf1: CONF1,
    duty_r: DUTY_R,
}
impl CH {
    ///0x00 - Configuration register 0 for channel 0
    #[inline(always)]
    pub const fn conf0(&self) -> &CONF0 {
        &self.conf0
    }
    ///0x04 - High point register for channel 0
    #[inline(always)]
    pub const fn hpoint(&self) -> &HPOINT {
        &self.hpoint
    }
    ///0x08 - Initial duty cycle register for channel 0
    #[inline(always)]
    pub const fn duty(&self) -> &DUTY {
        &self.duty
    }
    ///0x0c - Configuration register 1 for channel 0
    #[inline(always)]
    pub const fn conf1(&self) -> &CONF1 {
        &self.conf1
    }
    ///0x10 - Current duty cycle register for channel 0
    #[inline(always)]
    pub const fn duty_r(&self) -> &DUTY_R {
        &self.duty_r
    }
}
/**CONF0 (rw) register accessor: Configuration register 0 for channel 0

You can [`read`](crate::generic::Reg::read) this register and get [`conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@conf0`] module*/
pub type CONF0 = crate::Reg<conf0::CONF0_SPEC>;
///Configuration register 0 for channel 0
pub mod conf0;
/**HPOINT (rw) register accessor: High point register for channel 0

You can [`read`](crate::generic::Reg::read) this register and get [`hpoint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpoint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hpoint`] module*/
pub type HPOINT = crate::Reg<hpoint::HPOINT_SPEC>;
///High point register for channel 0
pub mod hpoint;
/**DUTY (rw) register accessor: Initial duty cycle register for channel 0

You can [`read`](crate::generic::Reg::read) this register and get [`duty::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`duty::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@duty`] module*/
pub type DUTY = crate::Reg<duty::DUTY_SPEC>;
///Initial duty cycle register for channel 0
pub mod duty;
/**CONF1 (rw) register accessor: Configuration register 1 for channel 0

You can [`read`](crate::generic::Reg::read) this register and get [`conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@conf1`] module*/
pub type CONF1 = crate::Reg<conf1::CONF1_SPEC>;
///Configuration register 1 for channel 0
pub mod conf1;
/**DUTY_R (r) register accessor: Current duty cycle register for channel 0

You can [`read`](crate::generic::Reg::read) this register and get [`duty_r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@duty_r`] module*/
pub type DUTY_R = crate::Reg<duty_r::DUTY_R_SPEC>;
///Current duty cycle register for channel 0
pub mod duty_r;
