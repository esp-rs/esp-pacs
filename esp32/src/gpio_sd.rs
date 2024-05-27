#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    sigmadelta: [SIGMADELTA; 8],
    clock_gate: CLOCK_GATE,
    sigmadelta_misc: SIGMADELTA_MISC,
    version: VERSION,
}
impl RegisterBlock {
    ///0x00..0x20 -
    #[inline(always)]
    pub const fn sigmadelta(&self, n: usize) -> &SIGMADELTA {
        &self.sigmadelta[n]
    }
    ///Iterator for array of:
    ///0x00..0x20 -
    #[inline(always)]
    pub fn sigmadelta_iter(&self) -> impl Iterator<Item = &SIGMADELTA> {
        self.sigmadelta.iter()
    }
    ///0x20 -
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    ///0x24 -
    #[inline(always)]
    pub const fn sigmadelta_misc(&self) -> &SIGMADELTA_MISC {
        &self.sigmadelta_misc
    }
    ///0x28 -
    #[inline(always)]
    pub const fn version(&self) -> &VERSION {
        &self.version
    }
}
/**SIGMADELTA (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sigmadelta`] module*/
pub type SIGMADELTA = crate::Reg<sigmadelta::SIGMADELTA_SPEC>;
///
pub mod sigmadelta;
/**CLOCK_GATE (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock_gate`] module*/
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
///
pub mod clock_gate;
/**SIGMADELTA_MISC (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta_misc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta_misc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sigmadelta_misc`] module*/
pub type SIGMADELTA_MISC = crate::Reg<sigmadelta_misc::SIGMADELTA_MISC_SPEC>;
///
pub mod sigmadelta_misc;
/**VERSION (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`version::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`version::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@version`] module*/
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
///
pub mod version;
