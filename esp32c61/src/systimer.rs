#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    conf: CONF,
    unit0_op: UNIT0_OP,
    unit1_op: UNIT1_OP,
    unit0_load_hi: UNIT0_LOAD_HI,
    unit0_load_lo: UNIT0_LOAD_LO,
    unit1_load_hi: UNIT1_LOAD_HI,
    unit1_load_lo: UNIT1_LOAD_LO,
    target0_hi: TARGET0_HI,
    target0_lo: TARGET0_LO,
    target1_hi: TARGET1_HI,
    target1_lo: TARGET1_LO,
    target2_hi: TARGET2_HI,
    target2_lo: TARGET2_LO,
    target0_conf: TARGET0_CONF,
    target1_conf: TARGET1_CONF,
    target2_conf: TARGET2_CONF,
    unit0_value_hi: UNIT0_VALUE_HI,
    unit0_value_lo: UNIT0_VALUE_LO,
    unit1_value_hi: UNIT1_VALUE_HI,
    unit1_value_lo: UNIT1_VALUE_LO,
    comp0_load: COMP0_LOAD,
    comp1_load: COMP1_LOAD,
    comp2_load: COMP2_LOAD,
    unit0_load: UNIT0_LOAD,
    unit1_load: UNIT1_LOAD,
    int_ena: INT_ENA,
    int_raw: INT_RAW,
    int_clr: INT_CLR,
    int_st: INT_ST,
    real_target0_lo: REAL_TARGET0_LO,
    real_target0_hi: REAL_TARGET0_HI,
    real_target1_lo: REAL_TARGET1_LO,
    real_target1_hi: REAL_TARGET1_HI,
    real_target2_lo: REAL_TARGET2_LO,
    real_target2_hi: REAL_TARGET2_HI,
    _reserved35: [u8; 0x70],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - Configure system timer clock"]
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    #[doc = "0x04 - Read UNIT0 value to registers"]
    #[inline(always)]
    pub const fn unit0_op(&self) -> &UNIT0_OP {
        &self.unit0_op
    }
    #[doc = "0x08 - Read UNIT1 value to registers"]
    #[inline(always)]
    pub const fn unit1_op(&self) -> &UNIT1_OP {
        &self.unit1_op
    }
    #[doc = "0x0c - High 20 bits to be loaded to UNIT0"]
    #[inline(always)]
    pub const fn unit0_load_hi(&self) -> &UNIT0_LOAD_HI {
        &self.unit0_load_hi
    }
    #[doc = "0x10 - Low 32 bits to be loaded to UNIT0"]
    #[inline(always)]
    pub const fn unit0_load_lo(&self) -> &UNIT0_LOAD_LO {
        &self.unit0_load_lo
    }
    #[doc = "0x14 - High 20 bits to be loaded to UNIT1"]
    #[inline(always)]
    pub const fn unit1_load_hi(&self) -> &UNIT1_LOAD_HI {
        &self.unit1_load_hi
    }
    #[doc = "0x18 - Low 32 bits to be loaded to UNIT1"]
    #[inline(always)]
    pub const fn unit1_load_lo(&self) -> &UNIT1_LOAD_LO {
        &self.unit1_load_lo
    }
    #[doc = "0x1c - Alarm value to be loaded to COMP0, high 20 bits"]
    #[inline(always)]
    pub const fn target0_hi(&self) -> &TARGET0_HI {
        &self.target0_hi
    }
    #[doc = "0x20 - Alarm value to be loaded to COMP0, low 32 bits"]
    #[inline(always)]
    pub const fn target0_lo(&self) -> &TARGET0_LO {
        &self.target0_lo
    }
    #[doc = "0x24 - Alarm value to be loaded to COMP1, high 20 bits"]
    #[inline(always)]
    pub const fn target1_hi(&self) -> &TARGET1_HI {
        &self.target1_hi
    }
    #[doc = "0x28 - Alarm value to be loaded to COMP1, low 32 bits"]
    #[inline(always)]
    pub const fn target1_lo(&self) -> &TARGET1_LO {
        &self.target1_lo
    }
    #[doc = "0x2c - Alarm value to be loaded to COMP2, high 20 bits"]
    #[inline(always)]
    pub const fn target2_hi(&self) -> &TARGET2_HI {
        &self.target2_hi
    }
    #[doc = "0x30 - Alarm value to be loaded to COMP2, low 32 bits"]
    #[inline(always)]
    pub const fn target2_lo(&self) -> &TARGET2_LO {
        &self.target2_lo
    }
    #[doc = "0x34 - Configure COMP0 alarm mode"]
    #[inline(always)]
    pub const fn target0_conf(&self) -> &TARGET0_CONF {
        &self.target0_conf
    }
    #[doc = "0x38 - Configure COMP1 alarm mode"]
    #[inline(always)]
    pub const fn target1_conf(&self) -> &TARGET1_CONF {
        &self.target1_conf
    }
    #[doc = "0x3c - Configure COMP2 alarm mode"]
    #[inline(always)]
    pub const fn target2_conf(&self) -> &TARGET2_CONF {
        &self.target2_conf
    }
    #[doc = "0x40 - UNIT0 value, high 20 bits"]
    #[inline(always)]
    pub const fn unit0_value_hi(&self) -> &UNIT0_VALUE_HI {
        &self.unit0_value_hi
    }
    #[doc = "0x44 - UNIT0 value, low 32 bits"]
    #[inline(always)]
    pub const fn unit0_value_lo(&self) -> &UNIT0_VALUE_LO {
        &self.unit0_value_lo
    }
    #[doc = "0x48 - UNIT1 value, high 20 bits"]
    #[inline(always)]
    pub const fn unit1_value_hi(&self) -> &UNIT1_VALUE_HI {
        &self.unit1_value_hi
    }
    #[doc = "0x4c - UNIT1 value, low 32 bits"]
    #[inline(always)]
    pub const fn unit1_value_lo(&self) -> &UNIT1_VALUE_LO {
        &self.unit1_value_lo
    }
    #[doc = "0x50 - COMP0 synchronization register"]
    #[inline(always)]
    pub const fn comp0_load(&self) -> &COMP0_LOAD {
        &self.comp0_load
    }
    #[doc = "0x54 - COMP1 synchronization register"]
    #[inline(always)]
    pub const fn comp1_load(&self) -> &COMP1_LOAD {
        &self.comp1_load
    }
    #[doc = "0x58 - COMP2 synchronization register"]
    #[inline(always)]
    pub const fn comp2_load(&self) -> &COMP2_LOAD {
        &self.comp2_load
    }
    #[doc = "0x5c - UNIT0 synchronization register"]
    #[inline(always)]
    pub const fn unit0_load(&self) -> &UNIT0_LOAD {
        &self.unit0_load
    }
    #[doc = "0x60 - UNIT1 synchronization register"]
    #[inline(always)]
    pub const fn unit1_load(&self) -> &UNIT1_LOAD {
        &self.unit1_load
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
    #[doc = "0x74 - Actual target value of COMP0, low 32 bits"]
    #[inline(always)]
    pub const fn real_target0_lo(&self) -> &REAL_TARGET0_LO {
        &self.real_target0_lo
    }
    #[doc = "0x78 - Actual target value of COMP0, high 20 bits"]
    #[inline(always)]
    pub const fn real_target0_hi(&self) -> &REAL_TARGET0_HI {
        &self.real_target0_hi
    }
    #[doc = "0x7c - Actual target value of COMP1, low 32 bits"]
    #[inline(always)]
    pub const fn real_target1_lo(&self) -> &REAL_TARGET1_LO {
        &self.real_target1_lo
    }
    #[doc = "0x80 - Actual target value of COMP1, high 20 bits"]
    #[inline(always)]
    pub const fn real_target1_hi(&self) -> &REAL_TARGET1_HI {
        &self.real_target1_hi
    }
    #[doc = "0x84 - Actual target value of COMP2, low 32 bits"]
    #[inline(always)]
    pub const fn real_target2_lo(&self) -> &REAL_TARGET2_LO {
        &self.real_target2_lo
    }
    #[doc = "0x88 - Actual target value of COMP2, high 20 bits"]
    #[inline(always)]
    pub const fn real_target2_hi(&self) -> &REAL_TARGET2_HI {
        &self.real_target2_hi
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
#[doc = "UNIT0_OP (rw) register accessor: Read UNIT0 value to registers\n\nYou can [`read`](crate::Reg::read) this register and get [`unit0_op::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unit0_op::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unit0_op`] module"]
pub type UNIT0_OP = crate::Reg<unit0_op::UNIT0_OP_SPEC>;
#[doc = "Read UNIT0 value to registers"]
pub mod unit0_op;
#[doc = "UNIT1_OP (rw) register accessor: Read UNIT1 value to registers\n\nYou can [`read`](crate::Reg::read) this register and get [`unit1_op::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unit1_op::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unit1_op`] module"]
pub type UNIT1_OP = crate::Reg<unit1_op::UNIT1_OP_SPEC>;
#[doc = "Read UNIT1 value to registers"]
pub mod unit1_op;
#[doc = "UNIT0_LOAD_HI (rw) register accessor: High 20 bits to be loaded to UNIT0\n\nYou can [`read`](crate::Reg::read) this register and get [`unit0_load_hi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unit0_load_hi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unit0_load_hi`] module"]
pub type UNIT0_LOAD_HI = crate::Reg<unit0_load_hi::UNIT0_LOAD_HI_SPEC>;
#[doc = "High 20 bits to be loaded to UNIT0"]
pub mod unit0_load_hi;
#[doc = "UNIT0_LOAD_LO (rw) register accessor: Low 32 bits to be loaded to UNIT0\n\nYou can [`read`](crate::Reg::read) this register and get [`unit0_load_lo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unit0_load_lo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unit0_load_lo`] module"]
pub type UNIT0_LOAD_LO = crate::Reg<unit0_load_lo::UNIT0_LOAD_LO_SPEC>;
#[doc = "Low 32 bits to be loaded to UNIT0"]
pub mod unit0_load_lo;
#[doc = "UNIT1_LOAD_HI (rw) register accessor: High 20 bits to be loaded to UNIT1\n\nYou can [`read`](crate::Reg::read) this register and get [`unit1_load_hi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unit1_load_hi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unit1_load_hi`] module"]
pub type UNIT1_LOAD_HI = crate::Reg<unit1_load_hi::UNIT1_LOAD_HI_SPEC>;
#[doc = "High 20 bits to be loaded to UNIT1"]
pub mod unit1_load_hi;
#[doc = "UNIT1_LOAD_LO (rw) register accessor: Low 32 bits to be loaded to UNIT1\n\nYou can [`read`](crate::Reg::read) this register and get [`unit1_load_lo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unit1_load_lo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unit1_load_lo`] module"]
pub type UNIT1_LOAD_LO = crate::Reg<unit1_load_lo::UNIT1_LOAD_LO_SPEC>;
#[doc = "Low 32 bits to be loaded to UNIT1"]
pub mod unit1_load_lo;
#[doc = "TARGET0_HI (rw) register accessor: Alarm value to be loaded to COMP0, high 20 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`target0_hi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`target0_hi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target0_hi`] module"]
pub type TARGET0_HI = crate::Reg<target0_hi::TARGET0_HI_SPEC>;
#[doc = "Alarm value to be loaded to COMP0, high 20 bits"]
pub mod target0_hi;
#[doc = "TARGET0_LO (rw) register accessor: Alarm value to be loaded to COMP0, low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`target0_lo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`target0_lo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target0_lo`] module"]
pub type TARGET0_LO = crate::Reg<target0_lo::TARGET0_LO_SPEC>;
#[doc = "Alarm value to be loaded to COMP0, low 32 bits"]
pub mod target0_lo;
#[doc = "TARGET1_HI (rw) register accessor: Alarm value to be loaded to COMP1, high 20 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`target1_hi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`target1_hi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target1_hi`] module"]
pub type TARGET1_HI = crate::Reg<target1_hi::TARGET1_HI_SPEC>;
#[doc = "Alarm value to be loaded to COMP1, high 20 bits"]
pub mod target1_hi;
#[doc = "TARGET1_LO (rw) register accessor: Alarm value to be loaded to COMP1, low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`target1_lo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`target1_lo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target1_lo`] module"]
pub type TARGET1_LO = crate::Reg<target1_lo::TARGET1_LO_SPEC>;
#[doc = "Alarm value to be loaded to COMP1, low 32 bits"]
pub mod target1_lo;
#[doc = "TARGET2_HI (rw) register accessor: Alarm value to be loaded to COMP2, high 20 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`target2_hi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`target2_hi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target2_hi`] module"]
pub type TARGET2_HI = crate::Reg<target2_hi::TARGET2_HI_SPEC>;
#[doc = "Alarm value to be loaded to COMP2, high 20 bits"]
pub mod target2_hi;
#[doc = "TARGET2_LO (rw) register accessor: Alarm value to be loaded to COMP2, low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`target2_lo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`target2_lo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target2_lo`] module"]
pub type TARGET2_LO = crate::Reg<target2_lo::TARGET2_LO_SPEC>;
#[doc = "Alarm value to be loaded to COMP2, low 32 bits"]
pub mod target2_lo;
#[doc = "TARGET0_CONF (rw) register accessor: Configure COMP0 alarm mode\n\nYou can [`read`](crate::Reg::read) this register and get [`target0_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`target0_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target0_conf`] module"]
pub type TARGET0_CONF = crate::Reg<target0_conf::TARGET0_CONF_SPEC>;
#[doc = "Configure COMP0 alarm mode"]
pub mod target0_conf;
#[doc = "TARGET1_CONF (rw) register accessor: Configure COMP1 alarm mode\n\nYou can [`read`](crate::Reg::read) this register and get [`target1_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`target1_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target1_conf`] module"]
pub type TARGET1_CONF = crate::Reg<target1_conf::TARGET1_CONF_SPEC>;
#[doc = "Configure COMP1 alarm mode"]
pub mod target1_conf;
#[doc = "TARGET2_CONF (rw) register accessor: Configure COMP2 alarm mode\n\nYou can [`read`](crate::Reg::read) this register and get [`target2_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`target2_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target2_conf`] module"]
pub type TARGET2_CONF = crate::Reg<target2_conf::TARGET2_CONF_SPEC>;
#[doc = "Configure COMP2 alarm mode"]
pub mod target2_conf;
#[doc = "UNIT0_VALUE_HI (r) register accessor: UNIT0 value, high 20 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`unit0_value_hi::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unit0_value_hi`] module"]
pub type UNIT0_VALUE_HI = crate::Reg<unit0_value_hi::UNIT0_VALUE_HI_SPEC>;
#[doc = "UNIT0 value, high 20 bits"]
pub mod unit0_value_hi;
#[doc = "UNIT0_VALUE_LO (r) register accessor: UNIT0 value, low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`unit0_value_lo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unit0_value_lo`] module"]
pub type UNIT0_VALUE_LO = crate::Reg<unit0_value_lo::UNIT0_VALUE_LO_SPEC>;
#[doc = "UNIT0 value, low 32 bits"]
pub mod unit0_value_lo;
#[doc = "UNIT1_VALUE_HI (r) register accessor: UNIT1 value, high 20 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`unit1_value_hi::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unit1_value_hi`] module"]
pub type UNIT1_VALUE_HI = crate::Reg<unit1_value_hi::UNIT1_VALUE_HI_SPEC>;
#[doc = "UNIT1 value, high 20 bits"]
pub mod unit1_value_hi;
#[doc = "UNIT1_VALUE_LO (r) register accessor: UNIT1 value, low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`unit1_value_lo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unit1_value_lo`] module"]
pub type UNIT1_VALUE_LO = crate::Reg<unit1_value_lo::UNIT1_VALUE_LO_SPEC>;
#[doc = "UNIT1 value, low 32 bits"]
pub mod unit1_value_lo;
#[doc = "COMP0_LOAD (w) register accessor: COMP0 synchronization register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0_load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0_load`] module"]
pub type COMP0_LOAD = crate::Reg<comp0_load::COMP0_LOAD_SPEC>;
#[doc = "COMP0 synchronization register"]
pub mod comp0_load;
#[doc = "COMP1_LOAD (w) register accessor: COMP1 synchronization register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1_load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp1_load`] module"]
pub type COMP1_LOAD = crate::Reg<comp1_load::COMP1_LOAD_SPEC>;
#[doc = "COMP1 synchronization register"]
pub mod comp1_load;
#[doc = "COMP2_LOAD (w) register accessor: COMP2 synchronization register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp2_load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp2_load`] module"]
pub type COMP2_LOAD = crate::Reg<comp2_load::COMP2_LOAD_SPEC>;
#[doc = "COMP2 synchronization register"]
pub mod comp2_load;
#[doc = "UNIT0_LOAD (w) register accessor: UNIT0 synchronization register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unit0_load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unit0_load`] module"]
pub type UNIT0_LOAD = crate::Reg<unit0_load::UNIT0_LOAD_SPEC>;
#[doc = "UNIT0 synchronization register"]
pub mod unit0_load;
#[doc = "UNIT1_LOAD (w) register accessor: UNIT1 synchronization register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unit1_load::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unit1_load`] module"]
pub type UNIT1_LOAD = crate::Reg<unit1_load::UNIT1_LOAD_SPEC>;
#[doc = "UNIT1 synchronization register"]
pub mod unit1_load;
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
#[doc = "REAL_TARGET0_LO (r) register accessor: Actual target value of COMP0, low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`real_target0_lo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@real_target0_lo`] module"]
pub type REAL_TARGET0_LO = crate::Reg<real_target0_lo::REAL_TARGET0_LO_SPEC>;
#[doc = "Actual target value of COMP0, low 32 bits"]
pub mod real_target0_lo;
#[doc = "REAL_TARGET0_HI (r) register accessor: Actual target value of COMP0, high 20 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`real_target0_hi::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@real_target0_hi`] module"]
pub type REAL_TARGET0_HI = crate::Reg<real_target0_hi::REAL_TARGET0_HI_SPEC>;
#[doc = "Actual target value of COMP0, high 20 bits"]
pub mod real_target0_hi;
#[doc = "REAL_TARGET1_LO (r) register accessor: Actual target value of COMP1, low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`real_target1_lo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@real_target1_lo`] module"]
pub type REAL_TARGET1_LO = crate::Reg<real_target1_lo::REAL_TARGET1_LO_SPEC>;
#[doc = "Actual target value of COMP1, low 32 bits"]
pub mod real_target1_lo;
#[doc = "REAL_TARGET1_HI (r) register accessor: Actual target value of COMP1, high 20 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`real_target1_hi::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@real_target1_hi`] module"]
pub type REAL_TARGET1_HI = crate::Reg<real_target1_hi::REAL_TARGET1_HI_SPEC>;
#[doc = "Actual target value of COMP1, high 20 bits"]
pub mod real_target1_hi;
#[doc = "REAL_TARGET2_LO (r) register accessor: Actual target value of COMP2, low 32 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`real_target2_lo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@real_target2_lo`] module"]
pub type REAL_TARGET2_LO = crate::Reg<real_target2_lo::REAL_TARGET2_LO_SPEC>;
#[doc = "Actual target value of COMP2, low 32 bits"]
pub mod real_target2_lo;
#[doc = "REAL_TARGET2_HI (r) register accessor: Actual target value of COMP2, high 20 bits\n\nYou can [`read`](crate::Reg::read) this register and get [`real_target2_hi::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@real_target2_hi`] module"]
pub type REAL_TARGET2_HI = crate::Reg<real_target2_hi::REAL_TARGET2_HI_SPEC>;
#[doc = "Actual target value of COMP2, high 20 bits"]
pub mod real_target2_hi;
#[doc = "DATE (rw) register accessor: Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
