#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Register 0 that stores data to be programmed."]
    pub pgm_data0: PGM_DATA0,
    #[doc = "0x04 - Register 1 that stores data to be programmed."]
    pub pgm_data1: PGM_DATA1,
    #[doc = "0x08 - Register 2 that stores data to be programmed."]
    pub pgm_data2: PGM_DATA2,
    #[doc = "0x0c - Register 3 that stores data to be programmed."]
    pub pgm_data3: PGM_DATA3,
    #[doc = "0x10 - Register 4 that stores data to be programmed."]
    pub pgm_data4: PGM_DATA4,
    #[doc = "0x14 - Register 5 that stores data to be programmed."]
    pub pgm_data5: PGM_DATA5,
    #[doc = "0x18 - Register 6 that stores data to be programmed."]
    pub pgm_data6: PGM_DATA6,
    #[doc = "0x1c - Register 7 that stores data to be programmed."]
    pub pgm_data7: PGM_DATA7,
    #[doc = "0x20 - Register 0 that stores the RS code to be programmed."]
    pub pgm_check_value0: PGM_CHECK_VALUE0,
    #[doc = "0x24 - Register 1 that stores the RS code to be programmed."]
    pub pgm_check_value1: PGM_CHECK_VALUE1,
    #[doc = "0x28 - Register 2 that stores the RS code to be programmed."]
    pub pgm_check_value2: PGM_CHECK_VALUE2,
    #[doc = "0x2c - BLOCK0 data register 0."]
    pub rd_wr_dis: RD_WR_DIS,
    #[doc = "0x30 - BLOCK0 data register 1."]
    pub rd_repeat_data0: RD_REPEAT_DATA0,
    #[doc = "0x34 - BLOCK1 data register 0."]
    pub rd_blk1_data0: RD_BLK1_DATA0,
    #[doc = "0x38 - BLOCK1 data register 1."]
    pub rd_blk1_data1: RD_BLK1_DATA1,
    #[doc = "0x3c - BLOCK1 data register 2."]
    pub rd_blk1_data2: RD_BLK1_DATA2,
    #[doc = "0x40 - Register 0 of BLOCK2."]
    pub rd_blk2_data0: RD_BLK2_DATA0,
    #[doc = "0x44 - Register 1 of BLOCK2."]
    pub rd_blk2_data1: RD_BLK2_DATA1,
    #[doc = "0x48 - Register 2 of BLOCK2."]
    pub rd_blk2_data2: RD_BLK2_DATA2,
    #[doc = "0x4c - Register 3 of BLOCK2."]
    pub rd_blk2_data3: RD_BLK2_DATA3,
    #[doc = "0x50 - Register 4 of BLOCK2."]
    pub rd_blk2_data4: RD_BLK2_DATA4,
    #[doc = "0x54 - Register 5 of BLOCK2."]
    pub rd_blk2_data5: RD_BLK2_DATA5,
    #[doc = "0x58 - Register 6 of BLOCK2."]
    pub rd_blk2_data6: RD_BLK2_DATA6,
    #[doc = "0x5c - Register 7 of BLOCK2."]
    pub rd_blk2_data7: RD_BLK2_DATA7,
    #[doc = "0x60 - Register 0 of BLOCK3."]
    pub rd_blk3_data0: RD_BLK3_DATA0,
    #[doc = "0x64 - Register 1 of BLOCK3."]
    pub rd_blk3_data1: RD_BLK3_DATA1,
    #[doc = "0x68 - Register 2 of BLOCK3."]
    pub rd_blk3_data2: RD_BLK3_DATA2,
    #[doc = "0x6c - Register 3 of BLOCK3."]
    pub rd_blk3_data3: RD_BLK3_DATA3,
    #[doc = "0x70 - Register 4 of BLOCK3."]
    pub rd_blk3_data4: RD_BLK3_DATA4,
    #[doc = "0x74 - Register 5 of BLOCK3."]
    pub rd_blk3_data5: RD_BLK3_DATA5,
    #[doc = "0x78 - Register 6 of BLOCK3."]
    pub rd_blk3_data6: RD_BLK3_DATA6,
    #[doc = "0x7c - Register 7 of BLOCK3."]
    pub rd_blk3_data7: RD_BLK3_DATA7,
    #[doc = "0x80 - Programming error record register 0 of BLOCK0."]
    pub rd_repeat_err: RD_REPEAT_ERR,
    #[doc = "0x84 - Programming error record register 0 of BLOCK1-10."]
    pub rd_rs_err: RD_RS_ERR,
    #[doc = "0x88 - eFuse clcok configuration register."]
    pub clk: CLK,
    #[doc = "0x8c - eFuse operation mode configuraiton register"]
    pub conf: CONF,
    #[doc = "0x90 - eFuse status register."]
    pub status: STATUS,
    #[doc = "0x94 - eFuse command register."]
    pub cmd: CMD,
    #[doc = "0x98 - eFuse raw interrupt register."]
    pub int_raw: INT_RAW,
    #[doc = "0x9c - eFuse interrupt status register."]
    pub int_st: INT_ST,
    _reserved40: [u8; 0x60],
    #[doc = "0x100 - eFuse interrupt enable register."]
    pub int_ena: INT_ENA,
    #[doc = "0x104 - eFuse interrupt clear register."]
    pub int_clr: INT_CLR,
    #[doc = "0x108 - Controls the eFuse programming voltage."]
    pub dac_conf: DAC_CONF,
    #[doc = "0x10c - Configures read timing parameters."]
    pub rd_tim_conf: RD_TIM_CONF,
    #[doc = "0x110 - Configurarion register 0 of eFuse programming timing parameters."]
    pub wr_tim_conf0: WR_TIM_CONF0,
    #[doc = "0x114 - Configurarion register 1 of eFuse programming timing parameters."]
    pub wr_tim_conf1: WR_TIM_CONF1,
    #[doc = "0x118 - Configurarion register 2 of eFuse programming timing parameters."]
    pub wr_tim_conf2: WR_TIM_CONF2,
    _reserved47: [u8; 0xe0],
    #[doc = "0x1fc - eFuse version register."]
    pub date: DATE,
}
#[doc = "PGM_DATA0 (rw) register accessor: Register 0 that stores data to be programmed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pgm_data0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pgm_data0`] module"]
pub type PGM_DATA0 = crate::Reg<pgm_data0::PGM_DATA0_SPEC>;
#[doc = "Register 0 that stores data to be programmed."]
pub mod pgm_data0;
#[doc = "PGM_DATA1 (rw) register accessor: Register 1 that stores data to be programmed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pgm_data1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pgm_data1`] module"]
pub type PGM_DATA1 = crate::Reg<pgm_data1::PGM_DATA1_SPEC>;
#[doc = "Register 1 that stores data to be programmed."]
pub mod pgm_data1;
#[doc = "PGM_DATA2 (rw) register accessor: Register 2 that stores data to be programmed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pgm_data2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pgm_data2`] module"]
pub type PGM_DATA2 = crate::Reg<pgm_data2::PGM_DATA2_SPEC>;
#[doc = "Register 2 that stores data to be programmed."]
pub mod pgm_data2;
#[doc = "PGM_DATA3 (rw) register accessor: Register 3 that stores data to be programmed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pgm_data3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pgm_data3`] module"]
pub type PGM_DATA3 = crate::Reg<pgm_data3::PGM_DATA3_SPEC>;
#[doc = "Register 3 that stores data to be programmed."]
pub mod pgm_data3;
#[doc = "PGM_DATA4 (rw) register accessor: Register 4 that stores data to be programmed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pgm_data4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pgm_data4`] module"]
pub type PGM_DATA4 = crate::Reg<pgm_data4::PGM_DATA4_SPEC>;
#[doc = "Register 4 that stores data to be programmed."]
pub mod pgm_data4;
#[doc = "PGM_DATA5 (rw) register accessor: Register 5 that stores data to be programmed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pgm_data5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pgm_data5`] module"]
pub type PGM_DATA5 = crate::Reg<pgm_data5::PGM_DATA5_SPEC>;
#[doc = "Register 5 that stores data to be programmed."]
pub mod pgm_data5;
#[doc = "PGM_DATA6 (rw) register accessor: Register 6 that stores data to be programmed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pgm_data6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pgm_data6`] module"]
pub type PGM_DATA6 = crate::Reg<pgm_data6::PGM_DATA6_SPEC>;
#[doc = "Register 6 that stores data to be programmed."]
pub mod pgm_data6;
#[doc = "PGM_DATA7 (rw) register accessor: Register 7 that stores data to be programmed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pgm_data7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pgm_data7`] module"]
pub type PGM_DATA7 = crate::Reg<pgm_data7::PGM_DATA7_SPEC>;
#[doc = "Register 7 that stores data to be programmed."]
pub mod pgm_data7;
#[doc = "PGM_CHECK_VALUE0 (rw) register accessor: Register 0 that stores the RS code to be programmed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pgm_check_value0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_check_value0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pgm_check_value0`] module"]
pub type PGM_CHECK_VALUE0 = crate::Reg<pgm_check_value0::PGM_CHECK_VALUE0_SPEC>;
#[doc = "Register 0 that stores the RS code to be programmed."]
pub mod pgm_check_value0;
#[doc = "PGM_CHECK_VALUE1 (rw) register accessor: Register 1 that stores the RS code to be programmed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pgm_check_value1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_check_value1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pgm_check_value1`] module"]
pub type PGM_CHECK_VALUE1 = crate::Reg<pgm_check_value1::PGM_CHECK_VALUE1_SPEC>;
#[doc = "Register 1 that stores the RS code to be programmed."]
pub mod pgm_check_value1;
#[doc = "PGM_CHECK_VALUE2 (rw) register accessor: Register 2 that stores the RS code to be programmed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pgm_check_value2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_check_value2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pgm_check_value2`] module"]
pub type PGM_CHECK_VALUE2 = crate::Reg<pgm_check_value2::PGM_CHECK_VALUE2_SPEC>;
#[doc = "Register 2 that stores the RS code to be programmed."]
pub mod pgm_check_value2;
#[doc = "RD_WR_DIS (r) register accessor: BLOCK0 data register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_wr_dis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_wr_dis`] module"]
pub type RD_WR_DIS = crate::Reg<rd_wr_dis::RD_WR_DIS_SPEC>;
#[doc = "BLOCK0 data register 0."]
pub mod rd_wr_dis;
#[doc = "RD_REPEAT_DATA0 (r) register accessor: BLOCK0 data register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_data0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_repeat_data0`] module"]
pub type RD_REPEAT_DATA0 = crate::Reg<rd_repeat_data0::RD_REPEAT_DATA0_SPEC>;
#[doc = "BLOCK0 data register 1."]
pub mod rd_repeat_data0;
#[doc = "RD_BLK1_DATA0 (r) register accessor: BLOCK1 data register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_blk1_data0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_blk1_data0`] module"]
pub type RD_BLK1_DATA0 = crate::Reg<rd_blk1_data0::RD_BLK1_DATA0_SPEC>;
#[doc = "BLOCK1 data register 0."]
pub mod rd_blk1_data0;
#[doc = "RD_BLK1_DATA1 (r) register accessor: BLOCK1 data register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_blk1_data1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_blk1_data1`] module"]
pub type RD_BLK1_DATA1 = crate::Reg<rd_blk1_data1::RD_BLK1_DATA1_SPEC>;
#[doc = "BLOCK1 data register 1."]
pub mod rd_blk1_data1;
#[doc = "RD_BLK1_DATA2 (r) register accessor: BLOCK1 data register 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_blk1_data2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_blk1_data2`] module"]
pub type RD_BLK1_DATA2 = crate::Reg<rd_blk1_data2::RD_BLK1_DATA2_SPEC>;
#[doc = "BLOCK1 data register 2."]
pub mod rd_blk1_data2;
#[doc = "RD_BLK2_DATA0 (r) register accessor: Register 0 of BLOCK2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_blk2_data0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_blk2_data0`] module"]
pub type RD_BLK2_DATA0 = crate::Reg<rd_blk2_data0::RD_BLK2_DATA0_SPEC>;
#[doc = "Register 0 of BLOCK2."]
pub mod rd_blk2_data0;
#[doc = "RD_BLK2_DATA1 (r) register accessor: Register 1 of BLOCK2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_blk2_data1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_blk2_data1`] module"]
pub type RD_BLK2_DATA1 = crate::Reg<rd_blk2_data1::RD_BLK2_DATA1_SPEC>;
#[doc = "Register 1 of BLOCK2."]
pub mod rd_blk2_data1;
#[doc = "RD_BLK2_DATA2 (r) register accessor: Register 2 of BLOCK2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_blk2_data2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_blk2_data2`] module"]
pub type RD_BLK2_DATA2 = crate::Reg<rd_blk2_data2::RD_BLK2_DATA2_SPEC>;
#[doc = "Register 2 of BLOCK2."]
pub mod rd_blk2_data2;
#[doc = "RD_BLK2_DATA3 (r) register accessor: Register 3 of BLOCK2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_blk2_data3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_blk2_data3`] module"]
pub type RD_BLK2_DATA3 = crate::Reg<rd_blk2_data3::RD_BLK2_DATA3_SPEC>;
#[doc = "Register 3 of BLOCK2."]
pub mod rd_blk2_data3;
#[doc = "RD_BLK2_DATA4 (r) register accessor: Register 4 of BLOCK2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_blk2_data4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_blk2_data4`] module"]
pub type RD_BLK2_DATA4 = crate::Reg<rd_blk2_data4::RD_BLK2_DATA4_SPEC>;
#[doc = "Register 4 of BLOCK2."]
pub mod rd_blk2_data4;
#[doc = "RD_BLK2_DATA5 (r) register accessor: Register 5 of BLOCK2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_blk2_data5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_blk2_data5`] module"]
pub type RD_BLK2_DATA5 = crate::Reg<rd_blk2_data5::RD_BLK2_DATA5_SPEC>;
#[doc = "Register 5 of BLOCK2."]
pub mod rd_blk2_data5;
#[doc = "RD_BLK2_DATA6 (r) register accessor: Register 6 of BLOCK2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_blk2_data6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_blk2_data6`] module"]
pub type RD_BLK2_DATA6 = crate::Reg<rd_blk2_data6::RD_BLK2_DATA6_SPEC>;
#[doc = "Register 6 of BLOCK2."]
pub mod rd_blk2_data6;
#[doc = "RD_BLK2_DATA7 (r) register accessor: Register 7 of BLOCK2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_blk2_data7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_blk2_data7`] module"]
pub type RD_BLK2_DATA7 = crate::Reg<rd_blk2_data7::RD_BLK2_DATA7_SPEC>;
#[doc = "Register 7 of BLOCK2."]
pub mod rd_blk2_data7;
#[doc = "RD_BLK3_DATA0 (r) register accessor: Register 0 of BLOCK3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_blk3_data0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_blk3_data0`] module"]
pub type RD_BLK3_DATA0 = crate::Reg<rd_blk3_data0::RD_BLK3_DATA0_SPEC>;
#[doc = "Register 0 of BLOCK3."]
pub mod rd_blk3_data0;
#[doc = "RD_BLK3_DATA1 (r) register accessor: Register 1 of BLOCK3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_blk3_data1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_blk3_data1`] module"]
pub type RD_BLK3_DATA1 = crate::Reg<rd_blk3_data1::RD_BLK3_DATA1_SPEC>;
#[doc = "Register 1 of BLOCK3."]
pub mod rd_blk3_data1;
#[doc = "RD_BLK3_DATA2 (r) register accessor: Register 2 of BLOCK3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_blk3_data2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_blk3_data2`] module"]
pub type RD_BLK3_DATA2 = crate::Reg<rd_blk3_data2::RD_BLK3_DATA2_SPEC>;
#[doc = "Register 2 of BLOCK3."]
pub mod rd_blk3_data2;
#[doc = "RD_BLK3_DATA3 (r) register accessor: Register 3 of BLOCK3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_blk3_data3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_blk3_data3`] module"]
pub type RD_BLK3_DATA3 = crate::Reg<rd_blk3_data3::RD_BLK3_DATA3_SPEC>;
#[doc = "Register 3 of BLOCK3."]
pub mod rd_blk3_data3;
#[doc = "RD_BLK3_DATA4 (r) register accessor: Register 4 of BLOCK3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_blk3_data4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_blk3_data4`] module"]
pub type RD_BLK3_DATA4 = crate::Reg<rd_blk3_data4::RD_BLK3_DATA4_SPEC>;
#[doc = "Register 4 of BLOCK3."]
pub mod rd_blk3_data4;
#[doc = "RD_BLK3_DATA5 (r) register accessor: Register 5 of BLOCK3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_blk3_data5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_blk3_data5`] module"]
pub type RD_BLK3_DATA5 = crate::Reg<rd_blk3_data5::RD_BLK3_DATA5_SPEC>;
#[doc = "Register 5 of BLOCK3."]
pub mod rd_blk3_data5;
#[doc = "RD_BLK3_DATA6 (r) register accessor: Register 6 of BLOCK3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_blk3_data6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_blk3_data6`] module"]
pub type RD_BLK3_DATA6 = crate::Reg<rd_blk3_data6::RD_BLK3_DATA6_SPEC>;
#[doc = "Register 6 of BLOCK3."]
pub mod rd_blk3_data6;
#[doc = "RD_BLK3_DATA7 (r) register accessor: Register 7 of BLOCK3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_blk3_data7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_blk3_data7`] module"]
pub type RD_BLK3_DATA7 = crate::Reg<rd_blk3_data7::RD_BLK3_DATA7_SPEC>;
#[doc = "Register 7 of BLOCK3."]
pub mod rd_blk3_data7;
#[doc = "RD_REPEAT_ERR (r) register accessor: Programming error record register 0 of BLOCK0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_repeat_err`] module"]
pub type RD_REPEAT_ERR = crate::Reg<rd_repeat_err::RD_REPEAT_ERR_SPEC>;
#[doc = "Programming error record register 0 of BLOCK0."]
pub mod rd_repeat_err;
#[doc = "RD_RS_ERR (r) register accessor: Programming error record register 0 of BLOCK1-10.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_rs_err::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_rs_err`] module"]
pub type RD_RS_ERR = crate::Reg<rd_rs_err::RD_RS_ERR_SPEC>;
#[doc = "Programming error record register 0 of BLOCK1-10."]
pub mod rd_rs_err;
#[doc = "CLK (rw) register accessor: eFuse clcok configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk`] module"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = "eFuse clcok configuration register."]
pub mod clk;
#[doc = "CONF (rw) register accessor: eFuse operation mode configuraiton register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`conf`] module"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "eFuse operation mode configuraiton register"]
pub mod conf;
#[doc = "STATUS (r) register accessor: eFuse status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "eFuse status register."]
pub mod status;
#[doc = "CMD (rw) register accessor: eFuse command register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd`] module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "eFuse command register."]
pub mod cmd;
#[doc = "INT_RAW (rw) register accessor: eFuse raw interrupt register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "eFuse raw interrupt register."]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: eFuse interrupt status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "eFuse interrupt status register."]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: eFuse interrupt enable register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "eFuse interrupt enable register."]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: eFuse interrupt clear register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "eFuse interrupt clear register."]
pub mod int_clr;
#[doc = "DAC_CONF (rw) register accessor: Controls the eFuse programming voltage.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dac_conf`] module"]
pub type DAC_CONF = crate::Reg<dac_conf::DAC_CONF_SPEC>;
#[doc = "Controls the eFuse programming voltage."]
pub mod dac_conf;
#[doc = "RD_TIM_CONF (rw) register accessor: Configures read timing parameters.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_tim_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_tim_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_tim_conf`] module"]
pub type RD_TIM_CONF = crate::Reg<rd_tim_conf::RD_TIM_CONF_SPEC>;
#[doc = "Configures read timing parameters."]
pub mod rd_tim_conf;
#[doc = "WR_TIM_CONF0 (rw) register accessor: Configurarion register 0 of eFuse programming timing parameters.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_tim_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_tim_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wr_tim_conf0`] module"]
pub type WR_TIM_CONF0 = crate::Reg<wr_tim_conf0::WR_TIM_CONF0_SPEC>;
#[doc = "Configurarion register 0 of eFuse programming timing parameters."]
pub mod wr_tim_conf0;
#[doc = "WR_TIM_CONF1 (rw) register accessor: Configurarion register 1 of eFuse programming timing parameters.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_tim_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_tim_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wr_tim_conf1`] module"]
pub type WR_TIM_CONF1 = crate::Reg<wr_tim_conf1::WR_TIM_CONF1_SPEC>;
#[doc = "Configurarion register 1 of eFuse programming timing parameters."]
pub mod wr_tim_conf1;
#[doc = "WR_TIM_CONF2 (rw) register accessor: Configurarion register 2 of eFuse programming timing parameters.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_tim_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_tim_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wr_tim_conf2`] module"]
pub type WR_TIM_CONF2 = crate::Reg<wr_tim_conf2::WR_TIM_CONF2_SPEC>;
#[doc = "Configurarion register 2 of eFuse programming timing parameters."]
pub mod wr_tim_conf2;
#[doc = "DATE (rw) register accessor: eFuse version register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "eFuse version register."]
pub mod date;
