#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    conf: CONF,
    debug_conf: DEBUG_CONF,
    condition_key0: CONDITION_KEY0,
    condition_key1: CONDITION_KEY1,
    condition_key2: CONDITION_KEY2,
    condition_key3: CONDITION_KEY3,
    condition_key4: CONDITION_KEY4,
    condition_key5: CONDITION_KEY5,
    condition_key6: CONDITION_KEY6,
    condition_key7: CONDITION_KEY7,
    int_ena: INT_ENA,
    int_raw: INT_RAW,
    int_clr: INT_CLR,
    int_st: INT_ST,
    arbit_cfg: ARBIT_CFG,
    sw_read: SW_READ,
    sw_read_st: SW_READ_ST,
    crc_data: CRC_DATA,
    crc_sync_data: CRC_SYNC_DATA,
    _reserved19: [u8; 0xb0],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - CFG reg"]
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    #[doc = "0x04 - reserved"]
    #[inline(always)]
    pub const fn debug_conf(&self) -> &DEBUG_CONF {
        &self.debug_conf
    }
    #[doc = "0x08 - condition key reg"]
    #[inline(always)]
    pub const fn condition_key0(&self) -> &CONDITION_KEY0 {
        &self.condition_key0
    }
    #[doc = "0x0c - condition key reg"]
    #[inline(always)]
    pub const fn condition_key1(&self) -> &CONDITION_KEY1 {
        &self.condition_key1
    }
    #[doc = "0x10 - condition key reg"]
    #[inline(always)]
    pub const fn condition_key2(&self) -> &CONDITION_KEY2 {
        &self.condition_key2
    }
    #[doc = "0x14 - condition key reg"]
    #[inline(always)]
    pub const fn condition_key3(&self) -> &CONDITION_KEY3 {
        &self.condition_key3
    }
    #[doc = "0x18 - condition key reg"]
    #[inline(always)]
    pub const fn condition_key4(&self) -> &CONDITION_KEY4 {
        &self.condition_key4
    }
    #[doc = "0x1c - condition key reg"]
    #[inline(always)]
    pub const fn condition_key5(&self) -> &CONDITION_KEY5 {
        &self.condition_key5
    }
    #[doc = "0x20 - condition key reg"]
    #[inline(always)]
    pub const fn condition_key6(&self) -> &CONDITION_KEY6 {
        &self.condition_key6
    }
    #[doc = "0x24 - condition key reg"]
    #[inline(always)]
    pub const fn condition_key7(&self) -> &CONDITION_KEY7 {
        &self.condition_key7
    }
    #[doc = "0x28 - Read only register for error and done"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x2c - Read only register for error and done"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x30 - Read only register for error and done"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x34 - Read only register for error and done"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x38 - arbit cfg"]
    #[inline(always)]
    pub const fn arbit_cfg(&self) -> &ARBIT_CFG {
        &self.arbit_cfg
    }
    #[doc = "0x3c - sw read random reg"]
    #[inline(always)]
    pub const fn sw_read(&self) -> &SW_READ {
        &self.sw_read
    }
    #[doc = "0x40 - sw read st reg"]
    #[inline(always)]
    pub const fn sw_read_st(&self) -> &SW_READ_ST {
        &self.sw_read_st
    }
    #[doc = "0x44 - sw read data"]
    #[inline(always)]
    pub const fn crc_data(&self) -> &CRC_DATA {
        &self.crc_data
    }
    #[doc = "0x48 - sw read data sync"]
    #[inline(always)]
    pub const fn crc_sync_data(&self) -> &CRC_SYNC_DATA {
        &self.crc_sync_data
    }
    #[doc = "0xfc - Date register."]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CONF (rw) register accessor: CFG reg\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`] module"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "CFG reg"]
pub mod conf;
#[doc = "DEBUG_CONF (rw) register accessor: reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_conf`] module"]
pub type DEBUG_CONF = crate::Reg<debug_conf::DEBUG_CONF_SPEC>;
#[doc = "reserved"]
pub mod debug_conf;
#[doc = "CONDITION_KEY0 (rw) register accessor: condition key reg\n\nYou can [`read`](crate::Reg::read) this register and get [`condition_key0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`condition_key0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@condition_key0`] module"]
pub type CONDITION_KEY0 = crate::Reg<condition_key0::CONDITION_KEY0_SPEC>;
#[doc = "condition key reg"]
pub mod condition_key0;
#[doc = "CONDITION_KEY1 (rw) register accessor: condition key reg\n\nYou can [`read`](crate::Reg::read) this register and get [`condition_key1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`condition_key1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@condition_key1`] module"]
pub type CONDITION_KEY1 = crate::Reg<condition_key1::CONDITION_KEY1_SPEC>;
#[doc = "condition key reg"]
pub mod condition_key1;
#[doc = "CONDITION_KEY2 (rw) register accessor: condition key reg\n\nYou can [`read`](crate::Reg::read) this register and get [`condition_key2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`condition_key2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@condition_key2`] module"]
pub type CONDITION_KEY2 = crate::Reg<condition_key2::CONDITION_KEY2_SPEC>;
#[doc = "condition key reg"]
pub mod condition_key2;
#[doc = "CONDITION_KEY3 (rw) register accessor: condition key reg\n\nYou can [`read`](crate::Reg::read) this register and get [`condition_key3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`condition_key3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@condition_key3`] module"]
pub type CONDITION_KEY3 = crate::Reg<condition_key3::CONDITION_KEY3_SPEC>;
#[doc = "condition key reg"]
pub mod condition_key3;
#[doc = "CONDITION_KEY4 (rw) register accessor: condition key reg\n\nYou can [`read`](crate::Reg::read) this register and get [`condition_key4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`condition_key4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@condition_key4`] module"]
pub type CONDITION_KEY4 = crate::Reg<condition_key4::CONDITION_KEY4_SPEC>;
#[doc = "condition key reg"]
pub mod condition_key4;
#[doc = "CONDITION_KEY5 (rw) register accessor: condition key reg\n\nYou can [`read`](crate::Reg::read) this register and get [`condition_key5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`condition_key5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@condition_key5`] module"]
pub type CONDITION_KEY5 = crate::Reg<condition_key5::CONDITION_KEY5_SPEC>;
#[doc = "condition key reg"]
pub mod condition_key5;
#[doc = "CONDITION_KEY6 (rw) register accessor: condition key reg\n\nYou can [`read`](crate::Reg::read) this register and get [`condition_key6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`condition_key6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@condition_key6`] module"]
pub type CONDITION_KEY6 = crate::Reg<condition_key6::CONDITION_KEY6_SPEC>;
#[doc = "condition key reg"]
pub mod condition_key6;
#[doc = "CONDITION_KEY7 (rw) register accessor: condition key reg\n\nYou can [`read`](crate::Reg::read) this register and get [`condition_key7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`condition_key7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@condition_key7`] module"]
pub type CONDITION_KEY7 = crate::Reg<condition_key7::CONDITION_KEY7_SPEC>;
#[doc = "condition key reg"]
pub mod condition_key7;
#[doc = "INT_ENA (rw) register accessor: Read only register for error and done\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Read only register for error and done"]
pub mod int_ena;
#[doc = "INT_RAW (rw) register accessor: Read only register for error and done\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Read only register for error and done"]
pub mod int_raw;
#[doc = "INT_CLR (w) register accessor: Read only register for error and done\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Read only register for error and done"]
pub mod int_clr;
#[doc = "INT_ST (r) register accessor: Read only register for error and done\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Read only register for error and done"]
pub mod int_st;
#[doc = "ARBIT_CFG (rw) register accessor: arbit cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`arbit_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arbit_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arbit_cfg`] module"]
pub type ARBIT_CFG = crate::Reg<arbit_cfg::ARBIT_CFG_SPEC>;
#[doc = "arbit cfg"]
pub mod arbit_cfg;
#[doc = "SW_READ (r) register accessor: sw read random reg\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_read::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_read`] module"]
pub type SW_READ = crate::Reg<sw_read::SW_READ_SPEC>;
#[doc = "sw read random reg"]
pub mod sw_read;
#[doc = "SW_READ_ST (rw) register accessor: sw read st reg\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_read_st::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_read_st::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sw_read_st`] module"]
pub type SW_READ_ST = crate::Reg<sw_read_st::SW_READ_ST_SPEC>;
#[doc = "sw read st reg"]
pub mod sw_read_st;
#[doc = "CRC_DATA (r) register accessor: sw read data\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_data`] module"]
pub type CRC_DATA = crate::Reg<crc_data::CRC_DATA_SPEC>;
#[doc = "sw read data"]
pub mod crc_data;
#[doc = "CRC_SYNC_DATA (r) register accessor: sw read data sync\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_sync_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_sync_data`] module"]
pub type CRC_SYNC_DATA = crate::Reg<crc_sync_data::CRC_SYNC_DATA_SPEC>;
#[doc = "sw read data sync"]
pub mod crc_sync_data;
#[doc = "DATE (rw) register accessor: Date register.\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Date register."]
pub mod date;
