#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00..0x20 - Register %s that stores data to be programmed."]
    pub pgm_data: [PGM_DATA; 8],
    #[doc = "0x20..0x2c - Register %s that stores the RS code to be programmed."]
    pub pgm_check_value: [PGM_CHECK_VALUE; 3],
    #[doc = "0x2c - Register 0 of BLOCK0."]
    pub rd_wr_dis: RD_WR_DIS,
    #[doc = "0x30 - Register 1 of BLOCK0."]
    pub rd_repeat_data0: RD_REPEAT_DATA0,
    #[doc = "0x34 - Register 2 of BLOCK0."]
    pub rd_repeat_data1: RD_REPEAT_DATA1,
    #[doc = "0x38 - Register 3 of BLOCK0."]
    pub rd_repeat_data2: RD_REPEAT_DATA2,
    #[doc = "0x3c - Register 4 of BLOCK0."]
    pub rd_repeat_data3: RD_REPEAT_DATA3,
    #[doc = "0x40 - Register 5 of BLOCK0."]
    pub rd_repeat_data4: RD_REPEAT_DATA4,
    #[doc = "0x44 - Register 0 of BLOCK1."]
    pub rd_mac_spi_sys_0: RD_MAC_SPI_SYS_0,
    #[doc = "0x48 - Register 1 of BLOCK1."]
    pub rd_mac_spi_sys_1: RD_MAC_SPI_SYS_1,
    #[doc = "0x4c - Register 2 of BLOCK1."]
    pub rd_mac_spi_sys_2: RD_MAC_SPI_SYS_2,
    #[doc = "0x50 - Register 3 of BLOCK1."]
    pub rd_mac_spi_sys_3: RD_MAC_SPI_SYS_3,
    #[doc = "0x54 - Register 4 of BLOCK1."]
    pub rd_mac_spi_sys_4: RD_MAC_SPI_SYS_4,
    #[doc = "0x58 - Register 5 of BLOCK1."]
    pub rd_mac_spi_sys_5: RD_MAC_SPI_SYS_5,
    #[doc = "0x5c..0x7c - Register %s of BLOCK2 (system)."]
    pub rd_sys_data_part1_: [RD_SYS_DATA_PART1_; 8],
    #[doc = "0x7c..0x9c - Register %s of BLOCK3 (user)."]
    pub rd_usr_data: [RD_USR_DATA; 8],
    #[doc = "0x9c..0xbc - Register %s of BLOCK4 (KEY0)."]
    pub rd_key0_data: [RD_KEY0_DATA; 8],
    #[doc = "0xbc..0xdc - Register %s of BLOCK5 (KEY1)."]
    pub rd_key1_data: [RD_KEY1_DATA; 8],
    #[doc = "0xdc..0xfc - Register %s of BLOCK6 (KEY2)."]
    pub rd_key2_data: [RD_KEY2_DATA; 8],
    #[doc = "0xfc..0x11c - Register %s of BLOCK7 (KEY3)."]
    pub rd_key3_data: [RD_KEY3_DATA; 8],
    #[doc = "0x11c..0x13c - Register %s of BLOCK8 (KEY4)."]
    pub rd_key4_data: [RD_KEY4_DATA; 8],
    #[doc = "0x13c..0x15c - Register %s of BLOCK9 (KEY5)."]
    pub rd_key5_data: [RD_KEY5_DATA; 8],
    #[doc = "0x15c..0x17c - Register %s of BLOCK10 (system)."]
    pub rd_sys_data_part2_: [RD_SYS_DATA_PART2_; 8],
    #[doc = "0x17c - Programming error record register 0 of BLOCK0."]
    pub rd_repeat_err0: RD_REPEAT_ERR0,
    #[doc = "0x180 - Programming error record register 1 of BLOCK0."]
    pub rd_repeat_err1: RD_REPEAT_ERR1,
    #[doc = "0x184 - Programming error record register 2 of BLOCK0."]
    pub rd_repeat_err2: RD_REPEAT_ERR2,
    #[doc = "0x188 - Programming error record register 3 of BLOCK0."]
    pub rd_repeat_err3: RD_REPEAT_ERR3,
    _reserved27: [u8; 0x04],
    #[doc = "0x190 - Programming error record register 4 of BLOCK0."]
    pub rd_repeat_err4: RD_REPEAT_ERR4,
    _reserved28: [u8; 0x2c],
    #[doc = "0x1c0 - Programming error record register 0 of BLOCK1-10."]
    pub rd_rs_err0: RD_RS_ERR0,
    #[doc = "0x1c4 - Programming error record register 1 of BLOCK1-10."]
    pub rd_rs_err1: RD_RS_ERR1,
    #[doc = "0x1c8 - eFuse clock configuration register."]
    pub clk: CLK,
    #[doc = "0x1cc - eFuse operation mode configuration register."]
    pub conf: CONF,
    #[doc = "0x1d0 - eFuse status register."]
    pub status: STATUS,
    #[doc = "0x1d4 - eFuse command register."]
    pub cmd: CMD,
    #[doc = "0x1d8 - eFuse raw interrupt register."]
    pub int_raw: INT_RAW,
    #[doc = "0x1dc - eFuse interrupt status register."]
    pub int_st: INT_ST,
    #[doc = "0x1e0 - eFuse interrupt enable register."]
    pub int_ena: INT_ENA,
    #[doc = "0x1e4 - eFuse interrupt clear register."]
    pub int_clr: INT_CLR,
    #[doc = "0x1e8 - Controls the eFuse programming voltage."]
    pub dac_conf: DAC_CONF,
    #[doc = "0x1ec - Configures read timing parameters."]
    pub rd_tim_conf: RD_TIM_CONF,
    #[doc = "0x1f0 - Configuration register 0 of eFuse programming timing parameters."]
    pub wr_tim_conf0: WR_TIM_CONF0,
    #[doc = "0x1f4 - Configuration register 1 of eFuse programming timing parameters."]
    pub wr_tim_conf1: WR_TIM_CONF1,
    #[doc = "0x1f8 - Configuration register 2 of eFuse programming timing parameters."]
    pub wr_tim_conf2: WR_TIM_CONF2,
    #[doc = "0x1fc - Version control register."]
    pub date: DATE,
}
#[doc = "PGM_DATA (rw) register accessor: Register %s that stores data to be programmed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pgm_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pgm_data`] module"]
pub type PGM_DATA = crate::Reg<pgm_data::PGM_DATA_SPEC>;
#[doc = "Register %s that stores data to be programmed."]
pub mod pgm_data;
#[doc = "PGM_CHECK_VALUE (rw) register accessor: Register %s that stores the RS code to be programmed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pgm_check_value::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_check_value::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pgm_check_value`] module"]
pub type PGM_CHECK_VALUE = crate::Reg<pgm_check_value::PGM_CHECK_VALUE_SPEC>;
#[doc = "Register %s that stores the RS code to be programmed."]
pub mod pgm_check_value;
#[doc = "RD_WR_DIS (r) register accessor: Register 0 of BLOCK0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_wr_dis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_wr_dis`] module"]
pub type RD_WR_DIS = crate::Reg<rd_wr_dis::RD_WR_DIS_SPEC>;
#[doc = "Register 0 of BLOCK0."]
pub mod rd_wr_dis;
#[doc = "RD_REPEAT_DATA0 (r) register accessor: Register 1 of BLOCK0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_data0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_repeat_data0`] module"]
pub type RD_REPEAT_DATA0 = crate::Reg<rd_repeat_data0::RD_REPEAT_DATA0_SPEC>;
#[doc = "Register 1 of BLOCK0."]
pub mod rd_repeat_data0;
#[doc = "RD_REPEAT_DATA1 (r) register accessor: Register 2 of BLOCK0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_data1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_repeat_data1`] module"]
pub type RD_REPEAT_DATA1 = crate::Reg<rd_repeat_data1::RD_REPEAT_DATA1_SPEC>;
#[doc = "Register 2 of BLOCK0."]
pub mod rd_repeat_data1;
#[doc = "RD_REPEAT_DATA2 (r) register accessor: Register 3 of BLOCK0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_data2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_repeat_data2`] module"]
pub type RD_REPEAT_DATA2 = crate::Reg<rd_repeat_data2::RD_REPEAT_DATA2_SPEC>;
#[doc = "Register 3 of BLOCK0."]
pub mod rd_repeat_data2;
#[doc = "RD_REPEAT_DATA3 (r) register accessor: Register 4 of BLOCK0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_data3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_repeat_data3`] module"]
pub type RD_REPEAT_DATA3 = crate::Reg<rd_repeat_data3::RD_REPEAT_DATA3_SPEC>;
#[doc = "Register 4 of BLOCK0."]
pub mod rd_repeat_data3;
#[doc = "RD_REPEAT_DATA4 (r) register accessor: Register 5 of BLOCK0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_data4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_repeat_data4`] module"]
pub type RD_REPEAT_DATA4 = crate::Reg<rd_repeat_data4::RD_REPEAT_DATA4_SPEC>;
#[doc = "Register 5 of BLOCK0."]
pub mod rd_repeat_data4;
#[doc = "RD_MAC_SPI_SYS_0 (r) register accessor: Register 0 of BLOCK1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_spi_sys_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_mac_spi_sys_0`] module"]
pub type RD_MAC_SPI_SYS_0 = crate::Reg<rd_mac_spi_sys_0::RD_MAC_SPI_SYS_0_SPEC>;
#[doc = "Register 0 of BLOCK1."]
pub mod rd_mac_spi_sys_0;
#[doc = "RD_MAC_SPI_SYS_1 (r) register accessor: Register 1 of BLOCK1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_spi_sys_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_mac_spi_sys_1`] module"]
pub type RD_MAC_SPI_SYS_1 = crate::Reg<rd_mac_spi_sys_1::RD_MAC_SPI_SYS_1_SPEC>;
#[doc = "Register 1 of BLOCK1."]
pub mod rd_mac_spi_sys_1;
#[doc = "RD_MAC_SPI_SYS_2 (r) register accessor: Register 2 of BLOCK1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_spi_sys_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_mac_spi_sys_2`] module"]
pub type RD_MAC_SPI_SYS_2 = crate::Reg<rd_mac_spi_sys_2::RD_MAC_SPI_SYS_2_SPEC>;
#[doc = "Register 2 of BLOCK1."]
pub mod rd_mac_spi_sys_2;
#[doc = "RD_MAC_SPI_SYS_3 (r) register accessor: Register 3 of BLOCK1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_spi_sys_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_mac_spi_sys_3`] module"]
pub type RD_MAC_SPI_SYS_3 = crate::Reg<rd_mac_spi_sys_3::RD_MAC_SPI_SYS_3_SPEC>;
#[doc = "Register 3 of BLOCK1."]
pub mod rd_mac_spi_sys_3;
#[doc = "RD_MAC_SPI_SYS_4 (r) register accessor: Register 4 of BLOCK1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_spi_sys_4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_mac_spi_sys_4`] module"]
pub type RD_MAC_SPI_SYS_4 = crate::Reg<rd_mac_spi_sys_4::RD_MAC_SPI_SYS_4_SPEC>;
#[doc = "Register 4 of BLOCK1."]
pub mod rd_mac_spi_sys_4;
#[doc = "RD_MAC_SPI_SYS_5 (r) register accessor: Register 5 of BLOCK1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_spi_sys_5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_mac_spi_sys_5`] module"]
pub type RD_MAC_SPI_SYS_5 = crate::Reg<rd_mac_spi_sys_5::RD_MAC_SPI_SYS_5_SPEC>;
#[doc = "Register 5 of BLOCK1."]
pub mod rd_mac_spi_sys_5;
#[doc = "RD_SYS_DATA_PART1_ (r) register accessor: Register %s of BLOCK2 (system).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_data_part1_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_sys_data_part1_`] module"]
pub type RD_SYS_DATA_PART1_ = crate::Reg<rd_sys_data_part1_::RD_SYS_DATA_PART1__SPEC>;
#[doc = "Register %s of BLOCK2 (system)."]
pub mod rd_sys_data_part1_;
#[doc = "RD_USR_DATA (r) register accessor: Register %s of BLOCK3 (user).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_usr_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_usr_data`] module"]
pub type RD_USR_DATA = crate::Reg<rd_usr_data::RD_USR_DATA_SPEC>;
#[doc = "Register %s of BLOCK3 (user)."]
pub mod rd_usr_data;
#[doc = "RD_KEY0_DATA (r) register accessor: Register %s of BLOCK4 (KEY0).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_key0_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_key0_data`] module"]
pub type RD_KEY0_DATA = crate::Reg<rd_key0_data::RD_KEY0_DATA_SPEC>;
#[doc = "Register %s of BLOCK4 (KEY0)."]
pub mod rd_key0_data;
#[doc = "RD_KEY1_DATA (r) register accessor: Register %s of BLOCK5 (KEY1).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_key1_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_key1_data`] module"]
pub type RD_KEY1_DATA = crate::Reg<rd_key1_data::RD_KEY1_DATA_SPEC>;
#[doc = "Register %s of BLOCK5 (KEY1)."]
pub mod rd_key1_data;
#[doc = "RD_KEY2_DATA (r) register accessor: Register %s of BLOCK6 (KEY2).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_key2_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_key2_data`] module"]
pub type RD_KEY2_DATA = crate::Reg<rd_key2_data::RD_KEY2_DATA_SPEC>;
#[doc = "Register %s of BLOCK6 (KEY2)."]
pub mod rd_key2_data;
#[doc = "RD_KEY3_DATA (r) register accessor: Register %s of BLOCK7 (KEY3).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_key3_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_key3_data`] module"]
pub type RD_KEY3_DATA = crate::Reg<rd_key3_data::RD_KEY3_DATA_SPEC>;
#[doc = "Register %s of BLOCK7 (KEY3)."]
pub mod rd_key3_data;
#[doc = "RD_KEY4_DATA (r) register accessor: Register %s of BLOCK8 (KEY4).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_key4_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_key4_data`] module"]
pub type RD_KEY4_DATA = crate::Reg<rd_key4_data::RD_KEY4_DATA_SPEC>;
#[doc = "Register %s of BLOCK8 (KEY4)."]
pub mod rd_key4_data;
#[doc = "RD_KEY5_DATA (r) register accessor: Register %s of BLOCK9 (KEY5).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_key5_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_key5_data`] module"]
pub type RD_KEY5_DATA = crate::Reg<rd_key5_data::RD_KEY5_DATA_SPEC>;
#[doc = "Register %s of BLOCK9 (KEY5)."]
pub mod rd_key5_data;
#[doc = "RD_SYS_DATA_PART2_ (r) register accessor: Register %s of BLOCK10 (system).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_data_part2_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_sys_data_part2_`] module"]
pub type RD_SYS_DATA_PART2_ = crate::Reg<rd_sys_data_part2_::RD_SYS_DATA_PART2__SPEC>;
#[doc = "Register %s of BLOCK10 (system)."]
pub mod rd_sys_data_part2_;
#[doc = "RD_REPEAT_ERR0 (r) register accessor: Programming error record register 0 of BLOCK0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_repeat_err0`] module"]
pub type RD_REPEAT_ERR0 = crate::Reg<rd_repeat_err0::RD_REPEAT_ERR0_SPEC>;
#[doc = "Programming error record register 0 of BLOCK0."]
pub mod rd_repeat_err0;
#[doc = "RD_REPEAT_ERR1 (r) register accessor: Programming error record register 1 of BLOCK0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_repeat_err1`] module"]
pub type RD_REPEAT_ERR1 = crate::Reg<rd_repeat_err1::RD_REPEAT_ERR1_SPEC>;
#[doc = "Programming error record register 1 of BLOCK0."]
pub mod rd_repeat_err1;
#[doc = "RD_REPEAT_ERR2 (r) register accessor: Programming error record register 2 of BLOCK0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_repeat_err2`] module"]
pub type RD_REPEAT_ERR2 = crate::Reg<rd_repeat_err2::RD_REPEAT_ERR2_SPEC>;
#[doc = "Programming error record register 2 of BLOCK0."]
pub mod rd_repeat_err2;
#[doc = "RD_REPEAT_ERR3 (r) register accessor: Programming error record register 3 of BLOCK0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_repeat_err3`] module"]
pub type RD_REPEAT_ERR3 = crate::Reg<rd_repeat_err3::RD_REPEAT_ERR3_SPEC>;
#[doc = "Programming error record register 3 of BLOCK0."]
pub mod rd_repeat_err3;
#[doc = "RD_REPEAT_ERR4 (r) register accessor: Programming error record register 4 of BLOCK0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_repeat_err4`] module"]
pub type RD_REPEAT_ERR4 = crate::Reg<rd_repeat_err4::RD_REPEAT_ERR4_SPEC>;
#[doc = "Programming error record register 4 of BLOCK0."]
pub mod rd_repeat_err4;
#[doc = "RD_RS_ERR0 (r) register accessor: Programming error record register 0 of BLOCK1-10.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_rs_err0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_rs_err0`] module"]
pub type RD_RS_ERR0 = crate::Reg<rd_rs_err0::RD_RS_ERR0_SPEC>;
#[doc = "Programming error record register 0 of BLOCK1-10."]
pub mod rd_rs_err0;
#[doc = "RD_RS_ERR1 (r) register accessor: Programming error record register 1 of BLOCK1-10.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_rs_err1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rd_rs_err1`] module"]
pub type RD_RS_ERR1 = crate::Reg<rd_rs_err1::RD_RS_ERR1_SPEC>;
#[doc = "Programming error record register 1 of BLOCK1-10."]
pub mod rd_rs_err1;
#[doc = "CLK (rw) register accessor: eFuse clock configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk`] module"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = "eFuse clock configuration register."]
pub mod clk;
#[doc = "CONF (rw) register accessor: eFuse operation mode configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`conf`] module"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "eFuse operation mode configuration register."]
pub mod conf;
#[doc = "STATUS (r) register accessor: eFuse status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "eFuse status register."]
pub mod status;
#[doc = "CMD (rw) register accessor: eFuse command register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmd`] module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "eFuse command register."]
pub mod cmd;
#[doc = "INT_RAW (r) register accessor: eFuse raw interrupt register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_raw`] module"]
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
#[doc = "WR_TIM_CONF0 (rw) register accessor: Configuration register 0 of eFuse programming timing parameters.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_tim_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_tim_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wr_tim_conf0`] module"]
pub type WR_TIM_CONF0 = crate::Reg<wr_tim_conf0::WR_TIM_CONF0_SPEC>;
#[doc = "Configuration register 0 of eFuse programming timing parameters."]
pub mod wr_tim_conf0;
#[doc = "WR_TIM_CONF1 (rw) register accessor: Configuration register 1 of eFuse programming timing parameters.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_tim_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_tim_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wr_tim_conf1`] module"]
pub type WR_TIM_CONF1 = crate::Reg<wr_tim_conf1::WR_TIM_CONF1_SPEC>;
#[doc = "Configuration register 1 of eFuse programming timing parameters."]
pub mod wr_tim_conf1;
#[doc = "WR_TIM_CONF2 (rw) register accessor: Configuration register 2 of eFuse programming timing parameters.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_tim_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_tim_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wr_tim_conf2`] module"]
pub type WR_TIM_CONF2 = crate::Reg<wr_tim_conf2::WR_TIM_CONF2_SPEC>;
#[doc = "Configuration register 2 of eFuse programming timing parameters."]
pub mod wr_tim_conf2;
#[doc = "DATE (rw) register accessor: Version control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register."]
pub mod date;
