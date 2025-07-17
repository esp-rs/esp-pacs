#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
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
    #[doc = "0x00 - Configure system timer clock"]
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    #[doc = "0x04..0x0c - Read UNIT%s value to registers"]
    #[inline(always)]
    pub const fn unit_op(&self, n: usize) -> &UNIT_OP {
        &self.unit_op[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x0c - Read UNIT%s value to registers"]
    #[inline(always)]
    pub fn unit_op_iter(&self) -> impl Iterator<Item = &UNIT_OP> {
        self.unit_op.iter()
    }
    #[doc = "0x04 - Read UNIT0 value to registers"]
    #[inline(always)]
    pub const fn unit0_op(&self) -> &UNIT_OP {
        self.unit_op(0)
    }
    #[doc = "0x08 - Read UNIT1 value to registers"]
    #[inline(always)]
    pub const fn unit1_op(&self) -> &UNIT_OP {
        self.unit_op(1)
    }
    #[doc = "0x0c..0x1c - Cluster UNIT%sLOAD, containing UNIT?_LOAD_HI, UNIT?_LOAD_LO"]
    #[inline(always)]
    pub const fn unitload(&self, n: usize) -> &UNITLOAD {
        &self.unitload[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x1c - Cluster UNIT%sLOAD, containing UNIT?_LOAD_HI, UNIT?_LOAD_LO"]
    #[inline(always)]
    pub fn unitload_iter(&self) -> impl Iterator<Item = &UNITLOAD> {
        self.unitload.iter()
    }
    #[doc = "0x0c..0x14 - Cluster UNIT0LOAD, containing UNIT?_LOAD_HI, UNIT?_LOAD_LO"]
    #[inline(always)]
    pub const fn unit0load(&self) -> &UNITLOAD {
        self.unitload(0)
    }
    #[doc = "0x14..0x1c - Cluster UNIT1LOAD, containing UNIT?_LOAD_HI, UNIT?_LOAD_LO"]
    #[inline(always)]
    pub const fn unit1load(&self) -> &UNITLOAD {
        self.unitload(1)
    }
    #[doc = "0x1c..0x34 - Cluster TRGT%s, containing TARGET?_HI, TARGET?_LO"]
    #[inline(always)]
    pub const fn trgt(&self, n: usize) -> &TRGT {
        &self.trgt[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c..0x34 - Cluster TRGT%s, containing TARGET?_HI, TARGET?_LO"]
    #[inline(always)]
    pub fn trgt_iter(&self) -> impl Iterator<Item = &TRGT> {
        self.trgt.iter()
    }
    #[doc = "0x34..0x40 - Configure COMP%s alarm mode"]
    #[inline(always)]
    pub const fn target_conf(&self, n: usize) -> &TARGET_CONF {
        &self.target_conf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x34..0x40 - Configure COMP%s alarm mode"]
    #[inline(always)]
    pub fn target_conf_iter(&self) -> impl Iterator<Item = &TARGET_CONF> {
        self.target_conf.iter()
    }
    #[doc = "0x34 - Configure COMP0 alarm mode"]
    #[inline(always)]
    pub const fn target0_conf(&self) -> &TARGET_CONF {
        self.target_conf(0)
    }
    #[doc = "0x38 - Configure COMP1 alarm mode"]
    #[inline(always)]
    pub const fn target1_conf(&self) -> &TARGET_CONF {
        self.target_conf(1)
    }
    #[doc = "0x3c - Configure COMP2 alarm mode"]
    #[inline(always)]
    pub const fn target2_conf(&self) -> &TARGET_CONF {
        self.target_conf(2)
    }
    #[doc = "0x40..0x50 - Cluster UNIT%s_VALUE, containing UNIT?_VALUE_HI, UNIT?_VALUE_LO"]
    #[inline(always)]
    pub const fn unit_value(&self, n: usize) -> &UNIT_VALUE {
        &self.unit_value[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x50 - Cluster UNIT%s_VALUE, containing UNIT?_VALUE_HI, UNIT?_VALUE_LO"]
    #[inline(always)]
    pub fn unit_value_iter(&self) -> impl Iterator<Item = &UNIT_VALUE> {
        self.unit_value.iter()
    }
    #[doc = "0x40..0x48 - Cluster UNIT0_VALUE, containing UNIT?_VALUE_HI, UNIT?_VALUE_LO"]
    #[inline(always)]
    pub const fn unit0_value(&self) -> &UNIT_VALUE {
        self.unit_value(0)
    }
    #[doc = "0x48..0x50 - Cluster UNIT1_VALUE, containing UNIT?_VALUE_HI, UNIT?_VALUE_LO"]
    #[inline(always)]
    pub const fn unit1_value(&self) -> &UNIT_VALUE {
        self.unit_value(1)
    }
    #[doc = "0x50..0x5c - COMP%s synchronization register"]
    #[inline(always)]
    pub const fn comp_load(&self, n: usize) -> &COMP_LOAD {
        &self.comp_load[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x5c - COMP%s synchronization register"]
    #[inline(always)]
    pub fn comp_load_iter(&self) -> impl Iterator<Item = &COMP_LOAD> {
        self.comp_load.iter()
    }
    #[doc = "0x50 - COMP0 synchronization register"]
    #[inline(always)]
    pub const fn comp0_load(&self) -> &COMP_LOAD {
        self.comp_load(0)
    }
    #[doc = "0x54 - COMP1 synchronization register"]
    #[inline(always)]
    pub const fn comp1_load(&self) -> &COMP_LOAD {
        self.comp_load(1)
    }
    #[doc = "0x58 - COMP2 synchronization register"]
    #[inline(always)]
    pub const fn comp2_load(&self) -> &COMP_LOAD {
        self.comp_load(2)
    }
    #[doc = "0x5c..0x64 - UNIT%s synchronization register"]
    #[inline(always)]
    pub const fn unit_load(&self, n: usize) -> &UNIT_LOAD {
        &self.unit_load[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5c..0x64 - UNIT%s synchronization register"]
    #[inline(always)]
    pub fn unit_load_iter(&self) -> impl Iterator<Item = &UNIT_LOAD> {
        self.unit_load.iter()
    }
    #[doc = "0x5c - UNIT0 synchronization register"]
    #[inline(always)]
    pub const fn unit0_load(&self) -> &UNIT_LOAD {
        self.unit_load(0)
    }
    #[doc = "0x60 - UNIT1 synchronization register"]
    #[inline(always)]
    pub const fn unit1_load(&self) -> &UNIT_LOAD {
        self.unit_load(1)
    }
    #[doc = "0x64 - Interrupt enable register of system timer"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x68 - Interrupt raw register of system timer"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x6c - Interrupt clear register of system timer"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x70 - Interrupt status register of system timer"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x74..0x8c - Cluster REAL_TARGET%s, containing REAL_TARGET?_LO, REAL_TARGET?_HI"]
    #[inline(always)]
    pub const fn real_target(&self, n: usize) -> &REAL_TARGET {
        &self.real_target[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x74..0x8c - Cluster REAL_TARGET%s, containing REAL_TARGET?_LO, REAL_TARGET?_HI"]
    #[inline(always)]
    pub fn real_target_iter(&self) -> impl Iterator<Item = &REAL_TARGET> {
        self.real_target.iter()
    }
    #[doc = "0xfc - Version control register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CONF (rw) register accessor: Configure system timer clock\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`] module"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "Configure system timer clock"]
pub mod conf;
#[doc = "UNIT_OP (rw) register accessor: Read UNIT%s value to registers\n\nYou can [`read`](crate::Reg::read) this register and get [`unit_op::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unit_op::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unit_op`] module"]
pub type UNIT_OP = crate::Reg<unit_op::UNIT_OP_SPEC>;
#[doc = "Read UNIT%s value to registers"]
pub mod unit_op;
#[doc = "Cluster UNIT%sLOAD, containing UNIT?_LOAD_HI, UNIT?_LOAD_LO"]
pub use self::unitload::UNITLOAD;
#[doc = r"Cluster"]
#[doc = "Cluster UNIT%sLOAD, containing UNIT?_LOAD_HI, UNIT?_LOAD_LO"]
pub mod unitload;
#[doc = "Cluster TRGT%s, containing TARGET?_HI, TARGET?_LO"]
pub use self::trgt::TRGT;
#[doc = r"Cluster"]
#[doc = "Cluster TRGT%s, containing TARGET?_HI, TARGET?_LO"]
pub mod trgt;
#[doc = "TARGET_CONF (rw) register accessor: Configure COMP%s alarm mode\n\nYou can [`read`](crate::Reg::read) this register and get [`target_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`target_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target_conf`] module"]
pub type TARGET_CONF = crate::Reg<target_conf::TARGET_CONF_SPEC>;
#[doc = "Configure COMP%s alarm mode"]
pub mod target_conf;
#[doc = "Cluster UNIT%s_VALUE, containing UNIT?_VALUE_HI, UNIT?_VALUE_LO"]
pub use self::unit_value::UNIT_VALUE;
#[doc = r"Cluster"]
#[doc = "Cluster UNIT%s_VALUE, containing UNIT?_VALUE_HI, UNIT?_VALUE_LO"]
pub mod unit_value;
#[doc = "COMP_LOAD (w) register accessor: COMP%s synchronization register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp_load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_load`] module"]
pub type COMP_LOAD = crate::Reg<comp_load::COMP_LOAD_SPEC>;
#[doc = "COMP%s synchronization register"]
pub mod comp_load;
#[doc = "UNIT_LOAD (w) register accessor: UNIT%s synchronization register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unit_load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unit_load`] module"]
pub type UNIT_LOAD = crate::Reg<unit_load::UNIT_LOAD_SPEC>;
#[doc = "UNIT%s synchronization register"]
pub mod unit_load;
#[doc = "INT_ENA (rw) register accessor: Interrupt enable register of system timer\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable register of system timer"]
pub mod int_ena;
#[doc = "INT_RAW (rw) register accessor: Interrupt raw register of system timer\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Interrupt raw register of system timer"]
pub mod int_raw;
#[doc = "INT_CLR (w) register accessor: Interrupt clear register of system timer\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear register of system timer"]
pub mod int_clr;
#[doc = "INT_ST (r) register accessor: Interrupt status register of system timer\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Interrupt status register of system timer"]
pub mod int_st;
#[doc = "Cluster REAL_TARGET%s, containing REAL_TARGET?_LO, REAL_TARGET?_HI"]
pub use self::real_target::REAL_TARGET;
#[doc = r"Cluster"]
#[doc = "Cluster REAL_TARGET%s, containing REAL_TARGET?_LO, REAL_TARGET?_HI"]
pub mod real_target;
pub use crate::dma::date;
pub use crate::dma::DATE;
