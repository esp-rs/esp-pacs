#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    conf: CONF,
    unit_op: [UNIT_OP; 2],
    unitload: [UNITLOAD; 2],
    trgt: [TRGT; 3],
    target_conf: [TARGET_CONF; 3],
    unit_value: [UNIT_VALUE; 2],
    comp_load: [COMP_LOAD; 3],
    unit_load: [UNIT_LOAD; 2],
    int_ena: INT_ENA,
    int_raw: INT_RAW,
    int_clr: INT_CLR,
    int_st: INT_ST,
    real_target: [REAL_TARGET; 3],
    _reserved13: [u8; 0x70],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - Configure system timer clock
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    ///0x04..0x0c - system timer unit%s value update register
    #[inline(always)]
    pub const fn unit_op(&self, n: usize) -> &UNIT_OP {
        &self.unit_op[n]
    }
    ///Iterator for array of:
    ///0x04..0x0c - system timer unit%s value update register
    #[inline(always)]
    pub fn unit_op_iter(&self) -> impl Iterator<Item = &UNIT_OP> {
        self.unit_op.iter()
    }
    ///0x04 - system timer unit0 value update register
    #[inline(always)]
    pub const fn unit0_op(&self) -> &UNIT_OP {
        self.unit_op(0)
    }
    ///0x08 - system timer unit1 value update register
    #[inline(always)]
    pub const fn unit1_op(&self) -> &UNIT_OP {
        self.unit_op(1)
    }
    ///0x0c..0x1c - Cluster UNIT%sLOAD, containing UNIT?_LOAD_HI, UNIT?_LOAD_LO
    #[inline(always)]
    pub const fn unitload(&self, n: usize) -> &UNITLOAD {
        &self.unitload[n]
    }
    ///Iterator for array of:
    ///0x0c..0x1c - Cluster UNIT%sLOAD, containing UNIT?_LOAD_HI, UNIT?_LOAD_LO
    #[inline(always)]
    pub fn unitload_iter(&self) -> impl Iterator<Item = &UNITLOAD> {
        self.unitload.iter()
    }
    ///0x0c..0x14 - Cluster UNIT0LOAD, containing UNIT?_LOAD_HI, UNIT?_LOAD_LO
    #[inline(always)]
    pub const fn unit0load(&self) -> &UNITLOAD {
        self.unitload(0)
    }
    ///0x14..0x1c - Cluster UNIT1LOAD, containing UNIT?_LOAD_HI, UNIT?_LOAD_LO
    #[inline(always)]
    pub const fn unit1load(&self) -> &UNITLOAD {
        self.unitload(1)
    }
    ///0x1c..0x34 - Cluster TRGT%s, containing TARGET?_HI, TARGET?_LO
    #[inline(always)]
    pub const fn trgt(&self, n: usize) -> &TRGT {
        &self.trgt[n]
    }
    ///Iterator for array of:
    ///0x1c..0x34 - Cluster TRGT%s, containing TARGET?_HI, TARGET?_LO
    #[inline(always)]
    pub fn trgt_iter(&self) -> impl Iterator<Item = &TRGT> {
        self.trgt.iter()
    }
    ///0x34..0x40 - system timer comp%s target mode register
    #[inline(always)]
    pub const fn target_conf(&self, n: usize) -> &TARGET_CONF {
        &self.target_conf[n]
    }
    ///Iterator for array of:
    ///0x34..0x40 - system timer comp%s target mode register
    #[inline(always)]
    pub fn target_conf_iter(&self) -> impl Iterator<Item = &TARGET_CONF> {
        self.target_conf.iter()
    }
    ///0x34 - system timer comp0 target mode register
    #[inline(always)]
    pub const fn target0_conf(&self) -> &TARGET_CONF {
        self.target_conf(0)
    }
    ///0x38 - system timer comp1 target mode register
    #[inline(always)]
    pub const fn target1_conf(&self) -> &TARGET_CONF {
        self.target_conf(1)
    }
    ///0x3c - system timer comp2 target mode register
    #[inline(always)]
    pub const fn target2_conf(&self) -> &TARGET_CONF {
        self.target_conf(2)
    }
    ///0x40..0x50 - Cluster UNIT%s_VALUE, containing UNIT?_VALUE_HI, UNIT?_VALUE_LO
    #[inline(always)]
    pub const fn unit_value(&self, n: usize) -> &UNIT_VALUE {
        &self.unit_value[n]
    }
    ///Iterator for array of:
    ///0x40..0x50 - Cluster UNIT%s_VALUE, containing UNIT?_VALUE_HI, UNIT?_VALUE_LO
    #[inline(always)]
    pub fn unit_value_iter(&self) -> impl Iterator<Item = &UNIT_VALUE> {
        self.unit_value.iter()
    }
    ///0x40..0x48 - Cluster UNIT0_VALUE, containing UNIT?_VALUE_HI, UNIT?_VALUE_LO
    #[inline(always)]
    pub const fn unit0_value(&self) -> &UNIT_VALUE {
        self.unit_value(0)
    }
    ///0x48..0x50 - Cluster UNIT1_VALUE, containing UNIT?_VALUE_HI, UNIT?_VALUE_LO
    #[inline(always)]
    pub const fn unit1_value(&self) -> &UNIT_VALUE {
        self.unit_value(1)
    }
    ///0x50..0x5c - system timer comp%s conf sync register
    #[inline(always)]
    pub const fn comp_load(&self, n: usize) -> &COMP_LOAD {
        &self.comp_load[n]
    }
    ///Iterator for array of:
    ///0x50..0x5c - system timer comp%s conf sync register
    #[inline(always)]
    pub fn comp_load_iter(&self) -> impl Iterator<Item = &COMP_LOAD> {
        self.comp_load.iter()
    }
    ///0x50 - system timer comp0 conf sync register
    #[inline(always)]
    pub const fn comp0_load(&self) -> &COMP_LOAD {
        self.comp_load(0)
    }
    ///0x54 - system timer comp1 conf sync register
    #[inline(always)]
    pub const fn comp1_load(&self) -> &COMP_LOAD {
        self.comp_load(1)
    }
    ///0x58 - system timer comp2 conf sync register
    #[inline(always)]
    pub const fn comp2_load(&self) -> &COMP_LOAD {
        self.comp_load(2)
    }
    ///0x5c..0x64 - system timer unit%s conf sync register
    #[inline(always)]
    pub const fn unit_load(&self, n: usize) -> &UNIT_LOAD {
        &self.unit_load[n]
    }
    ///Iterator for array of:
    ///0x5c..0x64 - system timer unit%s conf sync register
    #[inline(always)]
    pub fn unit_load_iter(&self) -> impl Iterator<Item = &UNIT_LOAD> {
        self.unit_load.iter()
    }
    ///0x5c - system timer unit0 conf sync register
    #[inline(always)]
    pub const fn unit0_load(&self) -> &UNIT_LOAD {
        self.unit_load(0)
    }
    ///0x60 - system timer unit1 conf sync register
    #[inline(always)]
    pub const fn unit1_load(&self) -> &UNIT_LOAD {
        self.unit_load(1)
    }
    ///0x64 - systimer interrupt enable register
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x68 - systimer interrupt raw register
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x6c - systimer interrupt clear register
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x70 - systimer interrupt status register
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x74..0x8c - Cluster REAL_TARGET%s, containing REAL_TARGET?_LO, REAL_TARGET?_HI
    #[inline(always)]
    pub const fn real_target(&self, n: usize) -> &REAL_TARGET {
        &self.real_target[n]
    }
    ///Iterator for array of:
    ///0x74..0x8c - Cluster REAL_TARGET%s, containing REAL_TARGET?_LO, REAL_TARGET?_HI
    #[inline(always)]
    pub fn real_target_iter(&self) -> impl Iterator<Item = &REAL_TARGET> {
        self.real_target.iter()
    }
    ///0xfc - system timer version control register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**CONF (rw) register accessor: Configure system timer clock

You can [`read`](crate::generic::Reg::read) this register and get [`conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@conf`] module*/
pub type CONF = crate::Reg<conf::CONF_SPEC>;
///Configure system timer clock
pub mod conf;
/**UNIT_OP (rw) register accessor: system timer unit%s value update register

You can [`read`](crate::generic::Reg::read) this register and get [`unit_op::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unit_op::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@unit_op`] module*/
pub type UNIT_OP = crate::Reg<unit_op::UNIT_OP_SPEC>;
///system timer unit%s value update register
pub mod unit_op;
///Cluster UNIT%sLOAD, containing UNIT?_LOAD_HI, UNIT?_LOAD_LO
pub use self::unitload::UNITLOAD;
///Cluster
///Cluster UNIT%sLOAD, containing UNIT?_LOAD_HI, UNIT?_LOAD_LO
pub mod unitload;
///Cluster TRGT%s, containing TARGET?_HI, TARGET?_LO
pub use self::trgt::TRGT;
///Cluster
///Cluster TRGT%s, containing TARGET?_HI, TARGET?_LO
pub mod trgt;
/**TARGET_CONF (rw) register accessor: system timer comp%s target mode register

You can [`read`](crate::generic::Reg::read) this register and get [`target_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@target_conf`] module*/
pub type TARGET_CONF = crate::Reg<target_conf::TARGET_CONF_SPEC>;
///system timer comp%s target mode register
pub mod target_conf;
///Cluster UNIT%s_VALUE, containing UNIT?_VALUE_HI, UNIT?_VALUE_LO
pub use self::unit_value::UNIT_VALUE;
///Cluster
///Cluster UNIT%s_VALUE, containing UNIT?_VALUE_HI, UNIT?_VALUE_LO
pub mod unit_value;
/**COMP_LOAD (w) register accessor: system timer comp%s conf sync register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp_load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@comp_load`] module*/
pub type COMP_LOAD = crate::Reg<comp_load::COMP_LOAD_SPEC>;
///system timer comp%s conf sync register
pub mod comp_load;
/**UNIT_LOAD (w) register accessor: system timer unit%s conf sync register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unit_load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@unit_load`] module*/
pub type UNIT_LOAD = crate::Reg<unit_load::UNIT_LOAD_SPEC>;
///system timer unit%s conf sync register
pub mod unit_load;
/**INT_ENA (rw) register accessor: systimer interrupt enable register

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///systimer interrupt enable register
pub mod int_ena;
/**INT_RAW (rw) register accessor: systimer interrupt raw register

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///systimer interrupt raw register
pub mod int_raw;
/**INT_CLR (w) register accessor: systimer interrupt clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///systimer interrupt clear register
pub mod int_clr;
/**INT_ST (r) register accessor: systimer interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///systimer interrupt status register
pub mod int_st;
///Cluster REAL_TARGET%s, containing REAL_TARGET?_LO, REAL_TARGET?_HI
pub use self::real_target::REAL_TARGET;
///Cluster
///Cluster REAL_TARGET%s, containing REAL_TARGET?_LO, REAL_TARGET?_HI
pub mod real_target;
/**DATE (rw) register accessor: system timer version control register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///system timer version control register
pub mod date;
