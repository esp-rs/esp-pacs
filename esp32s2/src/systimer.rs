#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    conf: CONF,
    load: LOAD,
    load_hi: LOAD_HI,
    load_lo: LOAD_LO,
    step: STEP,
    trgt: [TRGT; 3],
    target_conf: [TARGET_CONF; 3],
    unit_op: [UNIT_OP; 1],
    unit_value: [UNIT_VALUE; 1],
    int_ena: INT_ENA,
    int_raw: INT_RAW,
    int_clr: INT_CLR,
    _reserved12: [u8; 0xac],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - Configure system timer clock
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    ///0x04 - Load value to system timer
    #[inline(always)]
    pub const fn load(&self) -> &LOAD {
        &self.load
    }
    ///0x08 - High 32 bits to be loaded to system timer
    #[inline(always)]
    pub const fn load_hi(&self) -> &LOAD_HI {
        &self.load_hi
    }
    ///0x0c - Low 32 bits to be loaded to system timer
    #[inline(always)]
    pub const fn load_lo(&self) -> &LOAD_LO {
        &self.load_lo
    }
    ///0x10 - System timer accumulation step
    #[inline(always)]
    pub const fn step(&self) -> &STEP {
        &self.step
    }
    ///0x14..0x2c - Cluster TRGT%s, containing TARGET?_HI, TARGET?_LO
    #[inline(always)]
    pub const fn trgt(&self, n: usize) -> &TRGT {
        &self.trgt[n]
    }
    ///Iterator for array of:
    ///0x14..0x2c - Cluster TRGT%s, containing TARGET?_HI, TARGET?_LO
    #[inline(always)]
    pub fn trgt_iter(&self) -> impl Iterator<Item = &TRGT> {
        self.trgt.iter()
    }
    ///0x2c..0x38 - Configure work mode for system timer target %s
    #[inline(always)]
    pub const fn target_conf(&self, n: usize) -> &TARGET_CONF {
        &self.target_conf[n]
    }
    ///Iterator for array of:
    ///0x2c..0x38 - Configure work mode for system timer target %s
    #[inline(always)]
    pub fn target_conf_iter(&self) -> impl Iterator<Item = &TARGET_CONF> {
        self.target_conf.iter()
    }
    ///0x2c - Configure work mode for system timer target 0
    #[inline(always)]
    pub const fn target0_conf(&self) -> &TARGET_CONF {
        self.target_conf(0)
    }
    ///0x30 - Configure work mode for system timer target 1
    #[inline(always)]
    pub const fn target1_conf(&self) -> &TARGET_CONF {
        self.target_conf(1)
    }
    ///0x34 - Configure work mode for system timer target 2
    #[inline(always)]
    pub const fn target2_conf(&self) -> &TARGET_CONF {
        self.target_conf(2)
    }
    ///0x38 - Read out system timer value
    #[inline(always)]
    pub const fn unit_op(&self, n: usize) -> &UNIT_OP {
        &self.unit_op[n]
    }
    ///Iterator for array of:
    ///0x38 - Read out system timer value
    #[inline(always)]
    pub fn unit_op_iter(&self) -> impl Iterator<Item = &UNIT_OP> {
        self.unit_op.iter()
    }
    ///0x38 - Read out system timer value
    #[inline(always)]
    pub const fn unit0_op(&self) -> &UNIT_OP {
        self.unit_op(0)
    }
    ///0x3c..0x44 - Cluster UNIT%s_VALUE, containing UNIT?_VALUE_HI, UNIT?_VALUE_LO
    #[inline(always)]
    pub const fn unit_value(&self, n: usize) -> &UNIT_VALUE {
        &self.unit_value[n]
    }
    ///Iterator for array of:
    ///0x3c..0x44 - Cluster UNIT%s_VALUE, containing UNIT?_VALUE_HI, UNIT?_VALUE_LO
    #[inline(always)]
    pub fn unit_value_iter(&self) -> impl Iterator<Item = &UNIT_VALUE> {
        self.unit_value.iter()
    }
    ///0x3c..0x44 - Cluster UNIT0_VALUE, containing UNIT?_VALUE_HI, UNIT?_VALUE_LO
    #[inline(always)]
    pub const fn unit0_value(&self) -> &UNIT_VALUE {
        self.unit_value(0)
    }
    ///0x44 - System timer interrupt enable
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x48 - System timer interrupt raw
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x4c - System timer interrupt clear
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0xfc - Version control register
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
/**LOAD (w) register accessor: Load value to system timer

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@load`] module*/
pub type LOAD = crate::Reg<load::LOAD_SPEC>;
///Load value to system timer
pub mod load;
/**LOAD_HI (rw) register accessor: High 32 bits to be loaded to system timer

You can [`read`](crate::generic::Reg::read) this register and get [`load_hi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`load_hi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@load_hi`] module*/
pub type LOAD_HI = crate::Reg<load_hi::LOAD_HI_SPEC>;
///High 32 bits to be loaded to system timer
pub mod load_hi;
/**LOAD_LO (rw) register accessor: Low 32 bits to be loaded to system timer

You can [`read`](crate::generic::Reg::read) this register and get [`load_lo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`load_lo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@load_lo`] module*/
pub type LOAD_LO = crate::Reg<load_lo::LOAD_LO_SPEC>;
///Low 32 bits to be loaded to system timer
pub mod load_lo;
/**STEP (rw) register accessor: System timer accumulation step

You can [`read`](crate::generic::Reg::read) this register and get [`step::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`step::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@step`] module*/
pub type STEP = crate::Reg<step::STEP_SPEC>;
///System timer accumulation step
pub mod step;
///Cluster TRGT%s, containing TARGET?_HI, TARGET?_LO
pub use self::trgt::TRGT;
///Cluster
///Cluster TRGT%s, containing TARGET?_HI, TARGET?_LO
pub mod trgt;
/**TARGET_CONF (rw) register accessor: Configure work mode for system timer target %s

You can [`read`](crate::generic::Reg::read) this register and get [`target_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@target_conf`] module*/
pub type TARGET_CONF = crate::Reg<target_conf::TARGET_CONF_SPEC>;
///Configure work mode for system timer target %s
pub mod target_conf;
/**UNIT_OP (rw) register accessor: Read out system timer value

You can [`read`](crate::generic::Reg::read) this register and get [`unit_op::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unit_op::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@unit_op`] module*/
pub type UNIT_OP = crate::Reg<unit_op::UNIT_OP_SPEC>;
///Read out system timer value
pub mod unit_op;
///Cluster UNIT%s_VALUE, containing UNIT?_VALUE_HI, UNIT?_VALUE_LO
pub use self::unit_value::UNIT_VALUE;
///Cluster
///Cluster UNIT%s_VALUE, containing UNIT?_VALUE_HI, UNIT?_VALUE_LO
pub mod unit_value;
/**INT_ENA (rw) register accessor: System timer interrupt enable

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///System timer interrupt enable
pub mod int_ena;
/**INT_RAW (r) register accessor: System timer interrupt raw

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///System timer interrupt raw
pub mod int_raw;
/**INT_CLR (w) register accessor: System timer interrupt clear

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///System timer interrupt clear
pub mod int_clr;
/**DATE (rw) register accessor: Version control register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Version control register
pub mod date;
