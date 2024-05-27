#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Cluster CH%s, containing CMPR?_CFG, CMPR?_VALUE0, CMPR?_VALUE1, GEN?_CFG0, GEN?_FORCE, GEN?_A, GEN?_B, DB?_CFG, DB?_FED_CFG, DB?_RED_CFG, CHOPPER?_CFG, TZ?_CFG0, TZ?_CFG1, TZ?_STATUS
pub struct CH {
    cmpr_cfg: CMPR_CFG,
    cmpr_value0: CMPR_VALUE0,
    cmpr_value1: CMPR_VALUE1,
    gen_cfg0: GEN_CFG0,
    gen_force: GEN_FORCE,
    gen: [GEN; 2],
    db_cfg: DB_CFG,
    db_fed_cfg: DB_FED_CFG,
    db_red_cfg: DB_RED_CFG,
    chopper_cfg: CHOPPER_CFG,
    tz_cfg0: TZ_CFG0,
    tz_cfg1: TZ_CFG1,
    tz_status: TZ_STATUS,
}
impl CH {
    ///0x00 - Transfer status and update method for time stamp registers A and B
    #[inline(always)]
    pub const fn cmpr_cfg(&self) -> &CMPR_CFG {
        &self.cmpr_cfg
    }
    ///0x04 - Shadow register for register A.
    #[inline(always)]
    pub const fn cmpr_value0(&self) -> &CMPR_VALUE0 {
        &self.cmpr_value0
    }
    ///0x08 - Shadow register for register B.
    #[inline(always)]
    pub const fn cmpr_value1(&self) -> &CMPR_VALUE1 {
        &self.cmpr_value1
    }
    ///0x0c - Fault event T0 and T1 handling
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
    ///0x1c - dead time type selection and configuration
    #[inline(always)]
    pub const fn db_cfg(&self) -> &DB_CFG {
        &self.db_cfg
    }
    ///0x20 - Shadow register for falling edge delay (FED).
    #[inline(always)]
    pub const fn db_fed_cfg(&self) -> &DB_FED_CFG {
        &self.db_fed_cfg
    }
    ///0x24 - Shadow register for rising edge delay (RED).
    #[inline(always)]
    pub const fn db_red_cfg(&self) -> &DB_RED_CFG {
        &self.db_red_cfg
    }
    ///0x28 - Carrier enable and configuratoin
    #[inline(always)]
    pub const fn chopper_cfg(&self) -> &CHOPPER_CFG {
        &self.chopper_cfg
    }
    ///0x2c - Actions on PWMxA and PWMxB trip events
    #[inline(always)]
    pub const fn tz_cfg0(&self) -> &TZ_CFG0 {
        &self.tz_cfg0
    }
    ///0x30 - Software triggers for fault handler actions
    #[inline(always)]
    pub const fn tz_cfg1(&self) -> &TZ_CFG1 {
        &self.tz_cfg1
    }
    ///0x34 - Status of fault events.
    #[inline(always)]
    pub const fn tz_status(&self) -> &TZ_STATUS {
        &self.tz_status
    }
}
/**CMPR_CFG (rw) register accessor: Transfer status and update method for time stamp registers A and B

You can [`read`](crate::generic::Reg::read) this register and get [`cmpr_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpr_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cmpr_cfg`] module*/
pub type CMPR_CFG = crate::Reg<cmpr_cfg::CMPR_CFG_SPEC>;
///Transfer status and update method for time stamp registers A and B
pub mod cmpr_cfg;
/**CMPR_VALUE0 (rw) register accessor: Shadow register for register A.

You can [`read`](crate::generic::Reg::read) this register and get [`cmpr_value0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpr_value0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cmpr_value0`] module*/
pub type CMPR_VALUE0 = crate::Reg<cmpr_value0::CMPR_VALUE0_SPEC>;
///Shadow register for register A.
pub mod cmpr_value0;
/**CMPR_VALUE1 (rw) register accessor: Shadow register for register B.

You can [`read`](crate::generic::Reg::read) this register and get [`cmpr_value1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpr_value1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cmpr_value1`] module*/
pub type CMPR_VALUE1 = crate::Reg<cmpr_value1::CMPR_VALUE1_SPEC>;
///Shadow register for register B.
pub mod cmpr_value1;
/**GEN_CFG0 (rw) register accessor: Fault event T0 and T1 handling

You can [`read`](crate::generic::Reg::read) this register and get [`gen_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gen_cfg0`] module*/
pub type GEN_CFG0 = crate::Reg<gen_cfg0::GEN_CFG0_SPEC>;
///Fault event T0 and T1 handling
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
/**DB_CFG (rw) register accessor: dead time type selection and configuration

You can [`read`](crate::generic::Reg::read) this register and get [`db_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@db_cfg`] module*/
pub type DB_CFG = crate::Reg<db_cfg::DB_CFG_SPEC>;
///dead time type selection and configuration
pub mod db_cfg;
/**DB_FED_CFG (rw) register accessor: Shadow register for falling edge delay (FED).

You can [`read`](crate::generic::Reg::read) this register and get [`db_fed_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db_fed_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@db_fed_cfg`] module*/
pub type DB_FED_CFG = crate::Reg<db_fed_cfg::DB_FED_CFG_SPEC>;
///Shadow register for falling edge delay (FED).
pub mod db_fed_cfg;
/**DB_RED_CFG (rw) register accessor: Shadow register for rising edge delay (RED).

You can [`read`](crate::generic::Reg::read) this register and get [`db_red_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`db_red_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@db_red_cfg`] module*/
pub type DB_RED_CFG = crate::Reg<db_red_cfg::DB_RED_CFG_SPEC>;
///Shadow register for rising edge delay (RED).
pub mod db_red_cfg;
/**CHOPPER_CFG (rw) register accessor: Carrier enable and configuratoin

You can [`read`](crate::generic::Reg::read) this register and get [`chopper_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chopper_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@chopper_cfg`] module*/
pub type CHOPPER_CFG = crate::Reg<chopper_cfg::CHOPPER_CFG_SPEC>;
///Carrier enable and configuratoin
pub mod chopper_cfg;
/**TZ_CFG0 (rw) register accessor: Actions on PWMxA and PWMxB trip events

You can [`read`](crate::generic::Reg::read) this register and get [`tz_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tz_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tz_cfg0`] module*/
pub type TZ_CFG0 = crate::Reg<tz_cfg0::TZ_CFG0_SPEC>;
///Actions on PWMxA and PWMxB trip events
pub mod tz_cfg0;
/**TZ_CFG1 (rw) register accessor: Software triggers for fault handler actions

You can [`read`](crate::generic::Reg::read) this register and get [`tz_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tz_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tz_cfg1`] module*/
pub type TZ_CFG1 = crate::Reg<tz_cfg1::TZ_CFG1_SPEC>;
///Software triggers for fault handler actions
pub mod tz_cfg1;
/**TZ_STATUS (r) register accessor: Status of fault events.

You can [`read`](crate::generic::Reg::read) this register and get [`tz_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tz_status`] module*/
pub type TZ_STATUS = crate::Reg<tz_status::TZ_STATUS_SPEC>;
///Status of fault events.
pub mod tz_status;
