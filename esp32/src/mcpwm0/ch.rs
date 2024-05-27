#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Cluster CH%s, containing GEN?_STMP_CFG, GEN?_TSTMP_A, GEN?_TSTMP_B, GEN?_CFG0, GEN?_FORCE, GEN?_A, GEN?_B, DT?_CFG, DT?_FED_CFG, DT?_RED_CFG, CARRIER?_CFG, FH?_CFG0, FH?_CFG1, FH?_STATUS
pub struct CH {
    gen_stmp_cfg: GEN_STMP_CFG,
    gen_tstmp_a: GEN_TSTMP_A,
    gen_tstmp_b: GEN_TSTMP_B,
    gen_cfg0: GEN_CFG0,
    gen_force: GEN_FORCE,
    gen: [GEN; 2],
    dt_cfg: DT_CFG,
    dt_fed_cfg: DT_FED_CFG,
    dt_red_cfg: DT_RED_CFG,
    carrier_cfg: CARRIER_CFG,
    fh_cfg0: FH_CFG0,
    fh_cfg1: FH_CFG1,
    fh_status: FH_STATUS,
}
impl CH {
    ///0x00 - Transfer status and update method for time stamp registers A and B
    #[inline(always)]
    pub const fn gen_stmp_cfg(&self) -> &GEN_STMP_CFG {
        &self.gen_stmp_cfg
    }
    ///0x04 - Shadow register for register A.
    #[inline(always)]
    pub const fn gen_tstmp_a(&self) -> &GEN_TSTMP_A {
        &self.gen_tstmp_a
    }
    ///0x08 - Shadow register for register B.
    #[inline(always)]
    pub const fn gen_tstmp_b(&self) -> &GEN_TSTMP_B {
        &self.gen_tstmp_b
    }
    ///0x0c -
    #[inline(always)]
    pub const fn gen_cfg0(&self) -> &GEN_CFG0 {
        &self.gen_cfg0
    }
    ///0x10 - Permissives to force PWMxA and PWMxB outputs by software
    #[inline(always)]
    pub const fn gen_force(&self) -> &GEN_FORCE {
        &self.gen_force
    }
    ///0x14..0x1c - Actions triggered by events on PWMx%s
    #[inline(always)]
    pub const fn gen(&self, n: usize) -> &GEN {
        &self.gen[n]
    }
    ///Iterator for array of:
    ///0x14..0x1c - Actions triggered by events on PWMx%s
    #[inline(always)]
    pub fn gen_iter(&self) -> impl Iterator<Item = &GEN> {
        self.gen.iter()
    }
    ///0x14 - Actions triggered by events on PWMxA
    #[inline(always)]
    pub const fn gena(&self) -> &GEN {
        self.gen(0)
    }
    ///0x18 - Actions triggered by events on PWMxB
    #[inline(always)]
    pub const fn genb(&self) -> &GEN {
        self.gen(1)
    }
    ///0x1c - Dead time type selection and configuration
    #[inline(always)]
    pub const fn dt_cfg(&self) -> &DT_CFG {
        &self.dt_cfg
    }
    ///0x20 - Shadow register for falling edge delay (FED).
    #[inline(always)]
    pub const fn dt_fed_cfg(&self) -> &DT_FED_CFG {
        &self.dt_fed_cfg
    }
    ///0x24 - Shadow register for rising edge delay (RED).
    #[inline(always)]
    pub const fn dt_red_cfg(&self) -> &DT_RED_CFG {
        &self.dt_red_cfg
    }
    ///0x28 - Carrier enable and configuratoin
    #[inline(always)]
    pub const fn carrier_cfg(&self) -> &CARRIER_CFG {
        &self.carrier_cfg
    }
    ///0x2c - Actions on PWM0A and PWM0B trip events
    #[inline(always)]
    pub const fn fh_cfg0(&self) -> &FH_CFG0 {
        &self.fh_cfg0
    }
    ///0x30 - Software triggers for fault handler actions
    #[inline(always)]
    pub const fn fh_cfg1(&self) -> &FH_CFG1 {
        &self.fh_cfg1
    }
    ///0x34 - Status of fault events.
    #[inline(always)]
    pub const fn fh_status(&self) -> &FH_STATUS {
        &self.fh_status
    }
}
/**GEN_STMP_CFG (rw) register accessor: Transfer status and update method for time stamp registers A and B

You can [`read`](crate::generic::Reg::read) this register and get [`gen_stmp_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_stmp_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gen_stmp_cfg`] module*/
pub type GEN_STMP_CFG = crate::Reg<gen_stmp_cfg::GEN_STMP_CFG_SPEC>;
///Transfer status and update method for time stamp registers A and B
pub mod gen_stmp_cfg;
/**GEN_TSTMP_A (rw) register accessor: Shadow register for register A.

You can [`read`](crate::generic::Reg::read) this register and get [`gen_tstmp_a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_tstmp_a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gen_tstmp_a`] module*/
pub type GEN_TSTMP_A = crate::Reg<gen_tstmp_a::GEN_TSTMP_A_SPEC>;
///Shadow register for register A.
pub mod gen_tstmp_a;
/**GEN_TSTMP_B (rw) register accessor: Shadow register for register B.

You can [`read`](crate::generic::Reg::read) this register and get [`gen_tstmp_b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_tstmp_b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gen_tstmp_b`] module*/
pub type GEN_TSTMP_B = crate::Reg<gen_tstmp_b::GEN_TSTMP_B_SPEC>;
///Shadow register for register B.
pub mod gen_tstmp_b;
/**GEN_CFG0 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`gen_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gen_cfg0`] module*/
pub type GEN_CFG0 = crate::Reg<gen_cfg0::GEN_CFG0_SPEC>;
///
pub mod gen_cfg0;
/**GEN_FORCE (rw) register accessor: Permissives to force PWMxA and PWMxB outputs by software

You can [`read`](crate::generic::Reg::read) this register and get [`gen_force::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_force::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gen_force`] module*/
pub type GEN_FORCE = crate::Reg<gen_force::GEN_FORCE_SPEC>;
///Permissives to force PWMxA and PWMxB outputs by software
pub mod gen_force;
/**GEN (rw) register accessor: Actions triggered by events on PWMx%s

You can [`read`](crate::generic::Reg::read) this register and get [`gen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gen`] module*/
pub type GEN = crate::Reg<gen::GEN_SPEC>;
///Actions triggered by events on PWMx%s
pub mod gen;
/**DT_CFG (rw) register accessor: Dead time type selection and configuration

You can [`read`](crate::generic::Reg::read) this register and get [`dt_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dt_cfg`] module*/
pub type DT_CFG = crate::Reg<dt_cfg::DT_CFG_SPEC>;
///Dead time type selection and configuration
pub mod dt_cfg;
/**DT_FED_CFG (rw) register accessor: Shadow register for falling edge delay (FED).

You can [`read`](crate::generic::Reg::read) this register and get [`dt_fed_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt_fed_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dt_fed_cfg`] module*/
pub type DT_FED_CFG = crate::Reg<dt_fed_cfg::DT_FED_CFG_SPEC>;
///Shadow register for falling edge delay (FED).
pub mod dt_fed_cfg;
/**DT_RED_CFG (rw) register accessor: Shadow register for rising edge delay (RED).

You can [`read`](crate::generic::Reg::read) this register and get [`dt_red_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt_red_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dt_red_cfg`] module*/
pub type DT_RED_CFG = crate::Reg<dt_red_cfg::DT_RED_CFG_SPEC>;
///Shadow register for rising edge delay (RED).
pub mod dt_red_cfg;
/**CARRIER_CFG (rw) register accessor: Carrier enable and configuratoin

You can [`read`](crate::generic::Reg::read) this register and get [`carrier_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`carrier_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@carrier_cfg`] module*/
pub type CARRIER_CFG = crate::Reg<carrier_cfg::CARRIER_CFG_SPEC>;
///Carrier enable and configuratoin
pub mod carrier_cfg;
/**FH_CFG0 (rw) register accessor: Actions on PWM0A and PWM0B trip events

You can [`read`](crate::generic::Reg::read) this register and get [`fh_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fh_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fh_cfg0`] module*/
pub type FH_CFG0 = crate::Reg<fh_cfg0::FH_CFG0_SPEC>;
///Actions on PWM0A and PWM0B trip events
pub mod fh_cfg0;
/**FH_CFG1 (rw) register accessor: Software triggers for fault handler actions

You can [`read`](crate::generic::Reg::read) this register and get [`fh_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fh_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fh_cfg1`] module*/
pub type FH_CFG1 = crate::Reg<fh_cfg1::FH_CFG1_SPEC>;
///Software triggers for fault handler actions
pub mod fh_cfg1;
/**FH_STATUS (r) register accessor: Status of fault events.

You can [`read`](crate::generic::Reg::read) this register and get [`fh_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fh_status`] module*/
pub type FH_STATUS = crate::Reg<fh_status::FH_STATUS_SPEC>;
///Status of fault events.
pub mod fh_status;
