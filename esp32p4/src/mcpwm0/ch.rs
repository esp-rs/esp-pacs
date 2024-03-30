#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Cluster CH%s, containing GEN?_STMP_CFG, GEN?_TSTMP_A, GEN?_TSTMP_B, GEN?_CFG0, GEN?_FORCE, GEN?_A, GEN?_B, DT?_CFG, DT?_FED_CFG, DT?_RED_CFG, CARRIER?_CFG, FH?_CFG0, FH?_CFG1, FH?_STATUS"]
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
    #[doc = "0x00 - Generator0 time stamp registers A and B transfer status and update method register"]
    #[inline(always)]
    pub const fn gen_stmp_cfg(&self) -> &GEN_STMP_CFG {
        &self.gen_stmp_cfg
    }
    #[doc = "0x04 - Generator0 time stamp A's shadow register"]
    #[inline(always)]
    pub const fn gen_tstmp_a(&self) -> &GEN_TSTMP_A {
        &self.gen_tstmp_a
    }
    #[doc = "0x08 - Generator0 time stamp B's shadow register"]
    #[inline(always)]
    pub const fn gen_tstmp_b(&self) -> &GEN_TSTMP_B {
        &self.gen_tstmp_b
    }
    #[doc = "0x0c - Generator0 fault event T0 and T1 configuration register"]
    #[inline(always)]
    pub const fn gen_cfg0(&self) -> &GEN_CFG0 {
        &self.gen_cfg0
    }
    #[doc = "0x10 - Generator0 output signal force mode register."]
    #[inline(always)]
    pub const fn gen_force(&self) -> &GEN_FORCE {
        &self.gen_force
    }
    #[doc = "0x14..0x1c - Actions triggered by events on PWMx%s"]
    #[inline(always)]
    pub const fn gen(&self, n: usize) -> &GEN {
        &self.gen[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x14..0x1c - Actions triggered by events on PWMx%s"]
    #[inline(always)]
    pub fn gen_iter(&self) -> impl Iterator<Item = &GEN> {
        self.gen.iter()
    }
    #[doc = "0x14 - Actions triggered by events on PWMxA"]
    #[inline(always)]
    pub const fn gena(&self) -> &GEN {
        self.gen(0)
    }
    #[doc = "0x18 - Actions triggered by events on PWMxB"]
    #[inline(always)]
    pub const fn genb(&self) -> &GEN {
        self.gen(1)
    }
    #[doc = "0x1c - Dead time configuration register"]
    #[inline(always)]
    pub const fn dt_cfg(&self) -> &DT_CFG {
        &self.dt_cfg
    }
    #[doc = "0x20 - Falling edge delay (FED) shadow register"]
    #[inline(always)]
    pub const fn dt_fed_cfg(&self) -> &DT_FED_CFG {
        &self.dt_fed_cfg
    }
    #[doc = "0x24 - Rising edge delay (RED) shadow register"]
    #[inline(always)]
    pub const fn dt_red_cfg(&self) -> &DT_RED_CFG {
        &self.dt_red_cfg
    }
    #[doc = "0x28 - Carrier0 configuration register"]
    #[inline(always)]
    pub const fn carrier_cfg(&self) -> &CARRIER_CFG {
        &self.carrier_cfg
    }
    #[doc = "0x2c - PWM0 A and PWM0 B trip events actions configuration register"]
    #[inline(always)]
    pub const fn fh_cfg0(&self) -> &FH_CFG0 {
        &self.fh_cfg0
    }
    #[doc = "0x30 - Software triggers for fault handler actions configuration register"]
    #[inline(always)]
    pub const fn fh_cfg1(&self) -> &FH_CFG1 {
        &self.fh_cfg1
    }
    #[doc = "0x34 - Fault events status register"]
    #[inline(always)]
    pub const fn fh_status(&self) -> &FH_STATUS {
        &self.fh_status
    }
}
#[doc = "GEN_STMP_CFG (rw) register accessor: Generator0 time stamp registers A and B transfer status and update method register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen_stmp_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_stmp_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen_stmp_cfg`] module"]
pub type GEN_STMP_CFG = crate::Reg<gen_stmp_cfg::GEN_STMP_CFG_SPEC>;
#[doc = "Generator0 time stamp registers A and B transfer status and update method register"]
pub mod gen_stmp_cfg;
#[doc = "GEN_TSTMP_A (rw) register accessor: Generator0 time stamp A's shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen_tstmp_a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_tstmp_a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen_tstmp_a`] module"]
pub type GEN_TSTMP_A = crate::Reg<gen_tstmp_a::GEN_TSTMP_A_SPEC>;
#[doc = "Generator0 time stamp A's shadow register"]
pub mod gen_tstmp_a;
#[doc = "GEN_TSTMP_B (rw) register accessor: Generator0 time stamp B's shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen_tstmp_b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_tstmp_b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen_tstmp_b`] module"]
pub type GEN_TSTMP_B = crate::Reg<gen_tstmp_b::GEN_TSTMP_B_SPEC>;
#[doc = "Generator0 time stamp B's shadow register"]
pub mod gen_tstmp_b;
#[doc = "GEN_CFG0 (rw) register accessor: Generator0 fault event T0 and T1 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen_cfg0`] module"]
pub type GEN_CFG0 = crate::Reg<gen_cfg0::GEN_CFG0_SPEC>;
#[doc = "Generator0 fault event T0 and T1 configuration register"]
pub mod gen_cfg0;
#[doc = "GEN_FORCE (rw) register accessor: Generator0 output signal force mode register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen_force::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_force::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen_force`] module"]
pub type GEN_FORCE = crate::Reg<gen_force::GEN_FORCE_SPEC>;
#[doc = "Generator0 output signal force mode register."]
pub mod gen_force;
#[doc = "GEN (rw) register accessor: Actions triggered by events on PWMx%s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen`] module"]
pub type GEN = crate::Reg<gen::GEN_SPEC>;
#[doc = "Actions triggered by events on PWMx%s"]
pub mod gen;
#[doc = "DT_CFG (rw) register accessor: Dead time configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt_cfg`] module"]
pub type DT_CFG = crate::Reg<dt_cfg::DT_CFG_SPEC>;
#[doc = "Dead time configuration register"]
pub mod dt_cfg;
#[doc = "DT_FED_CFG (rw) register accessor: Falling edge delay (FED) shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt_fed_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt_fed_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt_fed_cfg`] module"]
pub type DT_FED_CFG = crate::Reg<dt_fed_cfg::DT_FED_CFG_SPEC>;
#[doc = "Falling edge delay (FED) shadow register"]
pub mod dt_fed_cfg;
#[doc = "DT_RED_CFG (rw) register accessor: Rising edge delay (RED) shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt_red_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt_red_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt_red_cfg`] module"]
pub type DT_RED_CFG = crate::Reg<dt_red_cfg::DT_RED_CFG_SPEC>;
#[doc = "Rising edge delay (RED) shadow register"]
pub mod dt_red_cfg;
#[doc = "CARRIER_CFG (rw) register accessor: Carrier0 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`carrier_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`carrier_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@carrier_cfg`] module"]
pub type CARRIER_CFG = crate::Reg<carrier_cfg::CARRIER_CFG_SPEC>;
#[doc = "Carrier0 configuration register"]
pub mod carrier_cfg;
#[doc = "FH_CFG0 (rw) register accessor: PWM0 A and PWM0 B trip events actions configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fh_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fh_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fh_cfg0`] module"]
pub type FH_CFG0 = crate::Reg<fh_cfg0::FH_CFG0_SPEC>;
#[doc = "PWM0 A and PWM0 B trip events actions configuration register"]
pub mod fh_cfg0;
#[doc = "FH_CFG1 (rw) register accessor: Software triggers for fault handler actions configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fh_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fh_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fh_cfg1`] module"]
pub type FH_CFG1 = crate::Reg<fh_cfg1::FH_CFG1_SPEC>;
#[doc = "Software triggers for fault handler actions configuration register"]
pub mod fh_cfg1;
#[doc = "FH_STATUS (r) register accessor: Fault events status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fh_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fh_status`] module"]
pub type FH_STATUS = crate::Reg<fh_status::FH_STATUS_SPEC>;
#[doc = "Fault events status register"]
pub mod fh_status;
