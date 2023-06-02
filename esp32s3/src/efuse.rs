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
    #[doc = "0x34 - BLOCK0 data register 2."]
    pub rd_repeat_data1: RD_REPEAT_DATA1,
    #[doc = "0x38 - BLOCK0 data register 3."]
    pub rd_repeat_data2: RD_REPEAT_DATA2,
    #[doc = "0x3c - BLOCK0 data register 4."]
    pub rd_repeat_data3: RD_REPEAT_DATA3,
    #[doc = "0x40 - BLOCK0 data register 5."]
    pub rd_repeat_data4: RD_REPEAT_DATA4,
    #[doc = "0x44 - BLOCK1 data register 0."]
    pub rd_mac_spi_sys_0: RD_MAC_SPI_SYS_0,
    #[doc = "0x48 - BLOCK1 data register 1."]
    pub rd_mac_spi_sys_1: RD_MAC_SPI_SYS_1,
    #[doc = "0x4c - BLOCK1 data register 2."]
    pub rd_mac_spi_sys_2: RD_MAC_SPI_SYS_2,
    #[doc = "0x50 - BLOCK1 data register 3."]
    pub rd_mac_spi_sys_3: RD_MAC_SPI_SYS_3,
    #[doc = "0x54 - BLOCK1 data register 4."]
    pub rd_mac_spi_sys_4: RD_MAC_SPI_SYS_4,
    #[doc = "0x58 - BLOCK1 data register 5."]
    pub rd_mac_spi_sys_5: RD_MAC_SPI_SYS_5,
    #[doc = "0x5c - Register 0 of BLOCK2 (system)."]
    pub rd_sys_part1_data0: RD_SYS_PART1_DATA0,
    #[doc = "0x60 - Register 1 of BLOCK2 (system)."]
    pub rd_sys_part1_data1: RD_SYS_PART1_DATA1,
    #[doc = "0x64 - Register 2 of BLOCK2 (system)."]
    pub rd_sys_part1_data2: RD_SYS_PART1_DATA2,
    #[doc = "0x68 - Register 3 of BLOCK2 (system)."]
    pub rd_sys_part1_data3: RD_SYS_PART1_DATA3,
    #[doc = "0x6c - Register 4 of BLOCK2 (system)."]
    pub rd_sys_part1_data4: RD_SYS_PART1_DATA4,
    #[doc = "0x70 - Register 5 of BLOCK2 (system)."]
    pub rd_sys_part1_data5: RD_SYS_PART1_DATA5,
    #[doc = "0x74 - Register 6 of BLOCK2 (system)."]
    pub rd_sys_part1_data6: RD_SYS_PART1_DATA6,
    #[doc = "0x78 - Register 7 of BLOCK2 (system)."]
    pub rd_sys_part1_data7: RD_SYS_PART1_DATA7,
    #[doc = "0x7c - Register 0 of BLOCK3 (user)."]
    pub rd_usr_data0: RD_USR_DATA0,
    #[doc = "0x80 - Register 1 of BLOCK3 (user)."]
    pub rd_usr_data1: RD_USR_DATA1,
    #[doc = "0x84 - Register 2 of BLOCK3 (user)."]
    pub rd_usr_data2: RD_USR_DATA2,
    #[doc = "0x88 - Register 3 of BLOCK3 (user)."]
    pub rd_usr_data3: RD_USR_DATA3,
    #[doc = "0x8c - Register 4 of BLOCK3 (user)."]
    pub rd_usr_data4: RD_USR_DATA4,
    #[doc = "0x90 - Register 5 of BLOCK3 (user)."]
    pub rd_usr_data5: RD_USR_DATA5,
    #[doc = "0x94 - Register 6 of BLOCK3 (user)."]
    pub rd_usr_data6: RD_USR_DATA6,
    #[doc = "0x98 - Register 7 of BLOCK3 (user)."]
    pub rd_usr_data7: RD_USR_DATA7,
    #[doc = "0x9c - Register 0 of BLOCK4 (KEY0)."]
    pub rd_key0_data0: RD_KEY0_DATA0,
    #[doc = "0xa0 - Register 1 of BLOCK4 (KEY0)."]
    pub rd_key0_data1: RD_KEY0_DATA1,
    #[doc = "0xa4 - Register 2 of BLOCK4 (KEY0)."]
    pub rd_key0_data2: RD_KEY0_DATA2,
    #[doc = "0xa8 - Register 3 of BLOCK4 (KEY0)."]
    pub rd_key0_data3: RD_KEY0_DATA3,
    #[doc = "0xac - Register 4 of BLOCK4 (KEY0)."]
    pub rd_key0_data4: RD_KEY0_DATA4,
    #[doc = "0xb0 - Register 5 of BLOCK4 (KEY0)."]
    pub rd_key0_data5: RD_KEY0_DATA5,
    #[doc = "0xb4 - Register 6 of BLOCK4 (KEY0)."]
    pub rd_key0_data6: RD_KEY0_DATA6,
    #[doc = "0xb8 - Register 7 of BLOCK4 (KEY0)."]
    pub rd_key0_data7: RD_KEY0_DATA7,
    #[doc = "0xbc - Register 0 of BLOCK5 (KEY1)."]
    pub rd_key1_data0: RD_KEY1_DATA0,
    #[doc = "0xc0 - Register 1 of BLOCK5 (KEY1)."]
    pub rd_key1_data1: RD_KEY1_DATA1,
    #[doc = "0xc4 - Register 2 of BLOCK5 (KEY1)."]
    pub rd_key1_data2: RD_KEY1_DATA2,
    #[doc = "0xc8 - Register 3 of BLOCK5 (KEY1)."]
    pub rd_key1_data3: RD_KEY1_DATA3,
    #[doc = "0xcc - Register 4 of BLOCK5 (KEY1)."]
    pub rd_key1_data4: RD_KEY1_DATA4,
    #[doc = "0xd0 - Register 5 of BLOCK5 (KEY1)."]
    pub rd_key1_data5: RD_KEY1_DATA5,
    #[doc = "0xd4 - Register 6 of BLOCK5 (KEY1)."]
    pub rd_key1_data6: RD_KEY1_DATA6,
    #[doc = "0xd8 - Register 7 of BLOCK5 (KEY1)."]
    pub rd_key1_data7: RD_KEY1_DATA7,
    #[doc = "0xdc - Register 0 of BLOCK6 (KEY2)."]
    pub rd_key2_data0: RD_KEY2_DATA0,
    #[doc = "0xe0 - Register 1 of BLOCK6 (KEY2)."]
    pub rd_key2_data1: RD_KEY2_DATA1,
    #[doc = "0xe4 - Register 2 of BLOCK6 (KEY2)."]
    pub rd_key2_data2: RD_KEY2_DATA2,
    #[doc = "0xe8 - Register 3 of BLOCK6 (KEY2)."]
    pub rd_key2_data3: RD_KEY2_DATA3,
    #[doc = "0xec - Register 4 of BLOCK6 (KEY2)."]
    pub rd_key2_data4: RD_KEY2_DATA4,
    #[doc = "0xf0 - Register 5 of BLOCK6 (KEY2)."]
    pub rd_key2_data5: RD_KEY2_DATA5,
    #[doc = "0xf4 - Register 6 of BLOCK6 (KEY2)."]
    pub rd_key2_data6: RD_KEY2_DATA6,
    #[doc = "0xf8 - Register 7 of BLOCK6 (KEY2)."]
    pub rd_key2_data7: RD_KEY2_DATA7,
    #[doc = "0xfc - Register 0 of BLOCK7 (KEY3)."]
    pub rd_key3_data0: RD_KEY3_DATA0,
    #[doc = "0x100 - Register 1 of BLOCK7 (KEY3)."]
    pub rd_key3_data1: RD_KEY3_DATA1,
    #[doc = "0x104 - Register 2 of BLOCK7 (KEY3)."]
    pub rd_key3_data2: RD_KEY3_DATA2,
    #[doc = "0x108 - Register 3 of BLOCK7 (KEY3)."]
    pub rd_key3_data3: RD_KEY3_DATA3,
    #[doc = "0x10c - Register 4 of BLOCK7 (KEY3)."]
    pub rd_key3_data4: RD_KEY3_DATA4,
    #[doc = "0x110 - Register 5 of BLOCK7 (KEY3)."]
    pub rd_key3_data5: RD_KEY3_DATA5,
    #[doc = "0x114 - Register 6 of BLOCK7 (KEY3)."]
    pub rd_key3_data6: RD_KEY3_DATA6,
    #[doc = "0x118 - Register 7 of BLOCK7 (KEY3)."]
    pub rd_key3_data7: RD_KEY3_DATA7,
    #[doc = "0x11c - Register 0 of BLOCK8 (KEY4)."]
    pub rd_key4_data0: RD_KEY4_DATA0,
    #[doc = "0x120 - Register 1 of BLOCK8 (KEY4)."]
    pub rd_key4_data1: RD_KEY4_DATA1,
    #[doc = "0x124 - Register 2 of BLOCK8 (KEY4)."]
    pub rd_key4_data2: RD_KEY4_DATA2,
    #[doc = "0x128 - Register 3 of BLOCK8 (KEY4)."]
    pub rd_key4_data3: RD_KEY4_DATA3,
    #[doc = "0x12c - Register 4 of BLOCK8 (KEY4)."]
    pub rd_key4_data4: RD_KEY4_DATA4,
    #[doc = "0x130 - Register 5 of BLOCK8 (KEY4)."]
    pub rd_key4_data5: RD_KEY4_DATA5,
    #[doc = "0x134 - Register 6 of BLOCK8 (KEY4)."]
    pub rd_key4_data6: RD_KEY4_DATA6,
    #[doc = "0x138 - Register 7 of BLOCK8 (KEY4)."]
    pub rd_key4_data7: RD_KEY4_DATA7,
    #[doc = "0x13c - Register 0 of BLOCK9 (KEY5)."]
    pub rd_key5_data0: RD_KEY5_DATA0,
    #[doc = "0x140 - Register 1 of BLOCK9 (KEY5)."]
    pub rd_key5_data1: RD_KEY5_DATA1,
    #[doc = "0x144 - Register 2 of BLOCK9 (KEY5)."]
    pub rd_key5_data2: RD_KEY5_DATA2,
    #[doc = "0x148 - Register 3 of BLOCK9 (KEY5)."]
    pub rd_key5_data3: RD_KEY5_DATA3,
    #[doc = "0x14c - Register 4 of BLOCK9 (KEY5)."]
    pub rd_key5_data4: RD_KEY5_DATA4,
    #[doc = "0x150 - Register 5 of BLOCK9 (KEY5)."]
    pub rd_key5_data5: RD_KEY5_DATA5,
    #[doc = "0x154 - Register 6 of BLOCK9 (KEY5)."]
    pub rd_key5_data6: RD_KEY5_DATA6,
    #[doc = "0x158 - Register 7 of BLOCK9 (KEY5)."]
    pub rd_key5_data7: RD_KEY5_DATA7,
    #[doc = "0x15c - Register 0 of BLOCK10 (system)."]
    pub rd_sys_part2_data0: RD_SYS_PART2_DATA0,
    #[doc = "0x160 - Register 1 of BLOCK9 (KEY5)."]
    pub rd_sys_part2_data1: RD_SYS_PART2_DATA1,
    #[doc = "0x164 - Register 2 of BLOCK10 (system)."]
    pub rd_sys_part2_data2: RD_SYS_PART2_DATA2,
    #[doc = "0x168 - Register 3 of BLOCK10 (system)."]
    pub rd_sys_part2_data3: RD_SYS_PART2_DATA3,
    #[doc = "0x16c - Register 4 of BLOCK10 (system)."]
    pub rd_sys_part2_data4: RD_SYS_PART2_DATA4,
    #[doc = "0x170 - Register 5 of BLOCK10 (system)."]
    pub rd_sys_part2_data5: RD_SYS_PART2_DATA5,
    #[doc = "0x174 - Register 6 of BLOCK10 (system)."]
    pub rd_sys_part2_data6: RD_SYS_PART2_DATA6,
    #[doc = "0x178 - Register 7 of BLOCK10 (system)."]
    pub rd_sys_part2_data7: RD_SYS_PART2_DATA7,
    #[doc = "0x17c - Programming error record register 0 of BLOCK0."]
    pub rd_repeat_err0: RD_REPEAT_ERR0,
    #[doc = "0x180 - Programming error record register 1 of BLOCK0."]
    pub rd_repeat_err1: RD_REPEAT_ERR1,
    #[doc = "0x184 - Programming error record register 2 of BLOCK0."]
    pub rd_repeat_err2: RD_REPEAT_ERR2,
    #[doc = "0x188 - Programming error record register 3 of BLOCK0."]
    pub rd_repeat_err3: RD_REPEAT_ERR3,
    _reserved99: [u8; 0x04],
    #[doc = "0x190 - Programming error record register 4 of BLOCK0."]
    pub rd_repeat_err4: RD_REPEAT_ERR4,
    _reserved100: [u8; 0x2c],
    #[doc = "0x1c0 - Programming error record register 0 of BLOCK1-10."]
    pub rd_rs_err0: RD_RS_ERR0,
    #[doc = "0x1c4 - Programming error record register 1 of BLOCK1-10."]
    pub rd_rs_err1: RD_RS_ERR1,
    #[doc = "0x1c8 - eFuse clcok configuration register."]
    pub clk: CLK,
    #[doc = "0x1cc - eFuse operation mode configuraiton register"]
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
    _reserved112: [u8; 0x04],
    #[doc = "0x1f4 - Configurarion register 1 of eFuse programming timing parameters."]
    pub wr_tim_conf1: WR_TIM_CONF1,
    #[doc = "0x1f8 - Configurarion register 2 of eFuse programming timing parameters."]
    pub wr_tim_conf2: WR_TIM_CONF2,
    #[doc = "0x1fc - eFuse version register."]
    pub date: DATE,
}
#[doc = "PGM_DATA0 (rw) register accessor: an alias for `Reg<PGM_DATA0_SPEC>`"]
pub type PGM_DATA0 = crate::Reg<pgm_data0::PGM_DATA0_SPEC>;
#[doc = "Register 0 that stores data to be programmed."]
pub mod pgm_data0;
#[doc = "PGM_DATA1 (rw) register accessor: an alias for `Reg<PGM_DATA1_SPEC>`"]
pub type PGM_DATA1 = crate::Reg<pgm_data1::PGM_DATA1_SPEC>;
#[doc = "Register 1 that stores data to be programmed."]
pub mod pgm_data1;
#[doc = "PGM_DATA2 (rw) register accessor: an alias for `Reg<PGM_DATA2_SPEC>`"]
pub type PGM_DATA2 = crate::Reg<pgm_data2::PGM_DATA2_SPEC>;
#[doc = "Register 2 that stores data to be programmed."]
pub mod pgm_data2;
#[doc = "PGM_DATA3 (rw) register accessor: an alias for `Reg<PGM_DATA3_SPEC>`"]
pub type PGM_DATA3 = crate::Reg<pgm_data3::PGM_DATA3_SPEC>;
#[doc = "Register 3 that stores data to be programmed."]
pub mod pgm_data3;
#[doc = "PGM_DATA4 (rw) register accessor: an alias for `Reg<PGM_DATA4_SPEC>`"]
pub type PGM_DATA4 = crate::Reg<pgm_data4::PGM_DATA4_SPEC>;
#[doc = "Register 4 that stores data to be programmed."]
pub mod pgm_data4;
#[doc = "PGM_DATA5 (rw) register accessor: an alias for `Reg<PGM_DATA5_SPEC>`"]
pub type PGM_DATA5 = crate::Reg<pgm_data5::PGM_DATA5_SPEC>;
#[doc = "Register 5 that stores data to be programmed."]
pub mod pgm_data5;
#[doc = "PGM_DATA6 (rw) register accessor: an alias for `Reg<PGM_DATA6_SPEC>`"]
pub type PGM_DATA6 = crate::Reg<pgm_data6::PGM_DATA6_SPEC>;
#[doc = "Register 6 that stores data to be programmed."]
pub mod pgm_data6;
#[doc = "PGM_DATA7 (rw) register accessor: an alias for `Reg<PGM_DATA7_SPEC>`"]
pub type PGM_DATA7 = crate::Reg<pgm_data7::PGM_DATA7_SPEC>;
#[doc = "Register 7 that stores data to be programmed."]
pub mod pgm_data7;
#[doc = "PGM_CHECK_VALUE0 (rw) register accessor: an alias for `Reg<PGM_CHECK_VALUE0_SPEC>`"]
pub type PGM_CHECK_VALUE0 = crate::Reg<pgm_check_value0::PGM_CHECK_VALUE0_SPEC>;
#[doc = "Register 0 that stores the RS code to be programmed."]
pub mod pgm_check_value0;
#[doc = "PGM_CHECK_VALUE1 (rw) register accessor: an alias for `Reg<PGM_CHECK_VALUE1_SPEC>`"]
pub type PGM_CHECK_VALUE1 = crate::Reg<pgm_check_value1::PGM_CHECK_VALUE1_SPEC>;
#[doc = "Register 1 that stores the RS code to be programmed."]
pub mod pgm_check_value1;
#[doc = "PGM_CHECK_VALUE2 (rw) register accessor: an alias for `Reg<PGM_CHECK_VALUE2_SPEC>`"]
pub type PGM_CHECK_VALUE2 = crate::Reg<pgm_check_value2::PGM_CHECK_VALUE2_SPEC>;
#[doc = "Register 2 that stores the RS code to be programmed."]
pub mod pgm_check_value2;
#[doc = "RD_WR_DIS (r) register accessor: an alias for `Reg<RD_WR_DIS_SPEC>`"]
pub type RD_WR_DIS = crate::Reg<rd_wr_dis::RD_WR_DIS_SPEC>;
#[doc = "BLOCK0 data register 0."]
pub mod rd_wr_dis;
#[doc = "RD_REPEAT_DATA0 (r) register accessor: an alias for `Reg<RD_REPEAT_DATA0_SPEC>`"]
pub type RD_REPEAT_DATA0 = crate::Reg<rd_repeat_data0::RD_REPEAT_DATA0_SPEC>;
#[doc = "BLOCK0 data register 1."]
pub mod rd_repeat_data0;
#[doc = "RD_REPEAT_DATA1 (r) register accessor: an alias for `Reg<RD_REPEAT_DATA1_SPEC>`"]
pub type RD_REPEAT_DATA1 = crate::Reg<rd_repeat_data1::RD_REPEAT_DATA1_SPEC>;
#[doc = "BLOCK0 data register 2."]
pub mod rd_repeat_data1;
#[doc = "RD_REPEAT_DATA2 (r) register accessor: an alias for `Reg<RD_REPEAT_DATA2_SPEC>`"]
pub type RD_REPEAT_DATA2 = crate::Reg<rd_repeat_data2::RD_REPEAT_DATA2_SPEC>;
#[doc = "BLOCK0 data register 3."]
pub mod rd_repeat_data2;
#[doc = "RD_REPEAT_DATA3 (r) register accessor: an alias for `Reg<RD_REPEAT_DATA3_SPEC>`"]
pub type RD_REPEAT_DATA3 = crate::Reg<rd_repeat_data3::RD_REPEAT_DATA3_SPEC>;
#[doc = "BLOCK0 data register 4."]
pub mod rd_repeat_data3;
#[doc = "RD_REPEAT_DATA4 (r) register accessor: an alias for `Reg<RD_REPEAT_DATA4_SPEC>`"]
pub type RD_REPEAT_DATA4 = crate::Reg<rd_repeat_data4::RD_REPEAT_DATA4_SPEC>;
#[doc = "BLOCK0 data register 5."]
pub mod rd_repeat_data4;
#[doc = "RD_MAC_SPI_SYS_0 (r) register accessor: an alias for `Reg<RD_MAC_SPI_SYS_0_SPEC>`"]
pub type RD_MAC_SPI_SYS_0 = crate::Reg<rd_mac_spi_sys_0::RD_MAC_SPI_SYS_0_SPEC>;
#[doc = "BLOCK1 data register 0."]
pub mod rd_mac_spi_sys_0;
#[doc = "RD_MAC_SPI_SYS_1 (r) register accessor: an alias for `Reg<RD_MAC_SPI_SYS_1_SPEC>`"]
pub type RD_MAC_SPI_SYS_1 = crate::Reg<rd_mac_spi_sys_1::RD_MAC_SPI_SYS_1_SPEC>;
#[doc = "BLOCK1 data register 1."]
pub mod rd_mac_spi_sys_1;
#[doc = "RD_MAC_SPI_SYS_2 (r) register accessor: an alias for `Reg<RD_MAC_SPI_SYS_2_SPEC>`"]
pub type RD_MAC_SPI_SYS_2 = crate::Reg<rd_mac_spi_sys_2::RD_MAC_SPI_SYS_2_SPEC>;
#[doc = "BLOCK1 data register 2."]
pub mod rd_mac_spi_sys_2;
#[doc = "RD_MAC_SPI_SYS_3 (r) register accessor: an alias for `Reg<RD_MAC_SPI_SYS_3_SPEC>`"]
pub type RD_MAC_SPI_SYS_3 = crate::Reg<rd_mac_spi_sys_3::RD_MAC_SPI_SYS_3_SPEC>;
#[doc = "BLOCK1 data register 3."]
pub mod rd_mac_spi_sys_3;
#[doc = "RD_MAC_SPI_SYS_4 (r) register accessor: an alias for `Reg<RD_MAC_SPI_SYS_4_SPEC>`"]
pub type RD_MAC_SPI_SYS_4 = crate::Reg<rd_mac_spi_sys_4::RD_MAC_SPI_SYS_4_SPEC>;
#[doc = "BLOCK1 data register 4."]
pub mod rd_mac_spi_sys_4;
#[doc = "RD_MAC_SPI_SYS_5 (r) register accessor: an alias for `Reg<RD_MAC_SPI_SYS_5_SPEC>`"]
pub type RD_MAC_SPI_SYS_5 = crate::Reg<rd_mac_spi_sys_5::RD_MAC_SPI_SYS_5_SPEC>;
#[doc = "BLOCK1 data register 5."]
pub mod rd_mac_spi_sys_5;
#[doc = "RD_SYS_PART1_DATA0 (r) register accessor: an alias for `Reg<RD_SYS_PART1_DATA0_SPEC>`"]
pub type RD_SYS_PART1_DATA0 = crate::Reg<rd_sys_part1_data0::RD_SYS_PART1_DATA0_SPEC>;
#[doc = "Register 0 of BLOCK2 (system)."]
pub mod rd_sys_part1_data0;
#[doc = "RD_SYS_PART1_DATA1 (r) register accessor: an alias for `Reg<RD_SYS_PART1_DATA1_SPEC>`"]
pub type RD_SYS_PART1_DATA1 = crate::Reg<rd_sys_part1_data1::RD_SYS_PART1_DATA1_SPEC>;
#[doc = "Register 1 of BLOCK2 (system)."]
pub mod rd_sys_part1_data1;
#[doc = "RD_SYS_PART1_DATA2 (r) register accessor: an alias for `Reg<RD_SYS_PART1_DATA2_SPEC>`"]
pub type RD_SYS_PART1_DATA2 = crate::Reg<rd_sys_part1_data2::RD_SYS_PART1_DATA2_SPEC>;
#[doc = "Register 2 of BLOCK2 (system)."]
pub mod rd_sys_part1_data2;
#[doc = "RD_SYS_PART1_DATA3 (r) register accessor: an alias for `Reg<RD_SYS_PART1_DATA3_SPEC>`"]
pub type RD_SYS_PART1_DATA3 = crate::Reg<rd_sys_part1_data3::RD_SYS_PART1_DATA3_SPEC>;
#[doc = "Register 3 of BLOCK2 (system)."]
pub mod rd_sys_part1_data3;
#[doc = "RD_SYS_PART1_DATA4 (r) register accessor: an alias for `Reg<RD_SYS_PART1_DATA4_SPEC>`"]
pub type RD_SYS_PART1_DATA4 = crate::Reg<rd_sys_part1_data4::RD_SYS_PART1_DATA4_SPEC>;
#[doc = "Register 4 of BLOCK2 (system)."]
pub mod rd_sys_part1_data4;
#[doc = "RD_SYS_PART1_DATA5 (r) register accessor: an alias for `Reg<RD_SYS_PART1_DATA5_SPEC>`"]
pub type RD_SYS_PART1_DATA5 = crate::Reg<rd_sys_part1_data5::RD_SYS_PART1_DATA5_SPEC>;
#[doc = "Register 5 of BLOCK2 (system)."]
pub mod rd_sys_part1_data5;
#[doc = "RD_SYS_PART1_DATA6 (r) register accessor: an alias for `Reg<RD_SYS_PART1_DATA6_SPEC>`"]
pub type RD_SYS_PART1_DATA6 = crate::Reg<rd_sys_part1_data6::RD_SYS_PART1_DATA6_SPEC>;
#[doc = "Register 6 of BLOCK2 (system)."]
pub mod rd_sys_part1_data6;
#[doc = "RD_SYS_PART1_DATA7 (r) register accessor: an alias for `Reg<RD_SYS_PART1_DATA7_SPEC>`"]
pub type RD_SYS_PART1_DATA7 = crate::Reg<rd_sys_part1_data7::RD_SYS_PART1_DATA7_SPEC>;
#[doc = "Register 7 of BLOCK2 (system)."]
pub mod rd_sys_part1_data7;
#[doc = "RD_USR_DATA0 (r) register accessor: an alias for `Reg<RD_USR_DATA0_SPEC>`"]
pub type RD_USR_DATA0 = crate::Reg<rd_usr_data0::RD_USR_DATA0_SPEC>;
#[doc = "Register 0 of BLOCK3 (user)."]
pub mod rd_usr_data0;
#[doc = "RD_USR_DATA1 (r) register accessor: an alias for `Reg<RD_USR_DATA1_SPEC>`"]
pub type RD_USR_DATA1 = crate::Reg<rd_usr_data1::RD_USR_DATA1_SPEC>;
#[doc = "Register 1 of BLOCK3 (user)."]
pub mod rd_usr_data1;
#[doc = "RD_USR_DATA2 (r) register accessor: an alias for `Reg<RD_USR_DATA2_SPEC>`"]
pub type RD_USR_DATA2 = crate::Reg<rd_usr_data2::RD_USR_DATA2_SPEC>;
#[doc = "Register 2 of BLOCK3 (user)."]
pub mod rd_usr_data2;
#[doc = "RD_USR_DATA3 (r) register accessor: an alias for `Reg<RD_USR_DATA3_SPEC>`"]
pub type RD_USR_DATA3 = crate::Reg<rd_usr_data3::RD_USR_DATA3_SPEC>;
#[doc = "Register 3 of BLOCK3 (user)."]
pub mod rd_usr_data3;
#[doc = "RD_USR_DATA4 (r) register accessor: an alias for `Reg<RD_USR_DATA4_SPEC>`"]
pub type RD_USR_DATA4 = crate::Reg<rd_usr_data4::RD_USR_DATA4_SPEC>;
#[doc = "Register 4 of BLOCK3 (user)."]
pub mod rd_usr_data4;
#[doc = "RD_USR_DATA5 (r) register accessor: an alias for `Reg<RD_USR_DATA5_SPEC>`"]
pub type RD_USR_DATA5 = crate::Reg<rd_usr_data5::RD_USR_DATA5_SPEC>;
#[doc = "Register 5 of BLOCK3 (user)."]
pub mod rd_usr_data5;
#[doc = "RD_USR_DATA6 (r) register accessor: an alias for `Reg<RD_USR_DATA6_SPEC>`"]
pub type RD_USR_DATA6 = crate::Reg<rd_usr_data6::RD_USR_DATA6_SPEC>;
#[doc = "Register 6 of BLOCK3 (user)."]
pub mod rd_usr_data6;
#[doc = "RD_USR_DATA7 (r) register accessor: an alias for `Reg<RD_USR_DATA7_SPEC>`"]
pub type RD_USR_DATA7 = crate::Reg<rd_usr_data7::RD_USR_DATA7_SPEC>;
#[doc = "Register 7 of BLOCK3 (user)."]
pub mod rd_usr_data7;
#[doc = "RD_KEY0_DATA0 (r) register accessor: an alias for `Reg<RD_KEY0_DATA0_SPEC>`"]
pub type RD_KEY0_DATA0 = crate::Reg<rd_key0_data0::RD_KEY0_DATA0_SPEC>;
#[doc = "Register 0 of BLOCK4 (KEY0)."]
pub mod rd_key0_data0;
#[doc = "RD_KEY0_DATA1 (r) register accessor: an alias for `Reg<RD_KEY0_DATA1_SPEC>`"]
pub type RD_KEY0_DATA1 = crate::Reg<rd_key0_data1::RD_KEY0_DATA1_SPEC>;
#[doc = "Register 1 of BLOCK4 (KEY0)."]
pub mod rd_key0_data1;
#[doc = "RD_KEY0_DATA2 (r) register accessor: an alias for `Reg<RD_KEY0_DATA2_SPEC>`"]
pub type RD_KEY0_DATA2 = crate::Reg<rd_key0_data2::RD_KEY0_DATA2_SPEC>;
#[doc = "Register 2 of BLOCK4 (KEY0)."]
pub mod rd_key0_data2;
#[doc = "RD_KEY0_DATA3 (r) register accessor: an alias for `Reg<RD_KEY0_DATA3_SPEC>`"]
pub type RD_KEY0_DATA3 = crate::Reg<rd_key0_data3::RD_KEY0_DATA3_SPEC>;
#[doc = "Register 3 of BLOCK4 (KEY0)."]
pub mod rd_key0_data3;
#[doc = "RD_KEY0_DATA4 (r) register accessor: an alias for `Reg<RD_KEY0_DATA4_SPEC>`"]
pub type RD_KEY0_DATA4 = crate::Reg<rd_key0_data4::RD_KEY0_DATA4_SPEC>;
#[doc = "Register 4 of BLOCK4 (KEY0)."]
pub mod rd_key0_data4;
#[doc = "RD_KEY0_DATA5 (r) register accessor: an alias for `Reg<RD_KEY0_DATA5_SPEC>`"]
pub type RD_KEY0_DATA5 = crate::Reg<rd_key0_data5::RD_KEY0_DATA5_SPEC>;
#[doc = "Register 5 of BLOCK4 (KEY0)."]
pub mod rd_key0_data5;
#[doc = "RD_KEY0_DATA6 (r) register accessor: an alias for `Reg<RD_KEY0_DATA6_SPEC>`"]
pub type RD_KEY0_DATA6 = crate::Reg<rd_key0_data6::RD_KEY0_DATA6_SPEC>;
#[doc = "Register 6 of BLOCK4 (KEY0)."]
pub mod rd_key0_data6;
#[doc = "RD_KEY0_DATA7 (r) register accessor: an alias for `Reg<RD_KEY0_DATA7_SPEC>`"]
pub type RD_KEY0_DATA7 = crate::Reg<rd_key0_data7::RD_KEY0_DATA7_SPEC>;
#[doc = "Register 7 of BLOCK4 (KEY0)."]
pub mod rd_key0_data7;
#[doc = "RD_KEY1_DATA0 (r) register accessor: an alias for `Reg<RD_KEY1_DATA0_SPEC>`"]
pub type RD_KEY1_DATA0 = crate::Reg<rd_key1_data0::RD_KEY1_DATA0_SPEC>;
#[doc = "Register 0 of BLOCK5 (KEY1)."]
pub mod rd_key1_data0;
#[doc = "RD_KEY1_DATA1 (r) register accessor: an alias for `Reg<RD_KEY1_DATA1_SPEC>`"]
pub type RD_KEY1_DATA1 = crate::Reg<rd_key1_data1::RD_KEY1_DATA1_SPEC>;
#[doc = "Register 1 of BLOCK5 (KEY1)."]
pub mod rd_key1_data1;
#[doc = "RD_KEY1_DATA2 (r) register accessor: an alias for `Reg<RD_KEY1_DATA2_SPEC>`"]
pub type RD_KEY1_DATA2 = crate::Reg<rd_key1_data2::RD_KEY1_DATA2_SPEC>;
#[doc = "Register 2 of BLOCK5 (KEY1)."]
pub mod rd_key1_data2;
#[doc = "RD_KEY1_DATA3 (r) register accessor: an alias for `Reg<RD_KEY1_DATA3_SPEC>`"]
pub type RD_KEY1_DATA3 = crate::Reg<rd_key1_data3::RD_KEY1_DATA3_SPEC>;
#[doc = "Register 3 of BLOCK5 (KEY1)."]
pub mod rd_key1_data3;
#[doc = "RD_KEY1_DATA4 (r) register accessor: an alias for `Reg<RD_KEY1_DATA4_SPEC>`"]
pub type RD_KEY1_DATA4 = crate::Reg<rd_key1_data4::RD_KEY1_DATA4_SPEC>;
#[doc = "Register 4 of BLOCK5 (KEY1)."]
pub mod rd_key1_data4;
#[doc = "RD_KEY1_DATA5 (r) register accessor: an alias for `Reg<RD_KEY1_DATA5_SPEC>`"]
pub type RD_KEY1_DATA5 = crate::Reg<rd_key1_data5::RD_KEY1_DATA5_SPEC>;
#[doc = "Register 5 of BLOCK5 (KEY1)."]
pub mod rd_key1_data5;
#[doc = "RD_KEY1_DATA6 (r) register accessor: an alias for `Reg<RD_KEY1_DATA6_SPEC>`"]
pub type RD_KEY1_DATA6 = crate::Reg<rd_key1_data6::RD_KEY1_DATA6_SPEC>;
#[doc = "Register 6 of BLOCK5 (KEY1)."]
pub mod rd_key1_data6;
#[doc = "RD_KEY1_DATA7 (r) register accessor: an alias for `Reg<RD_KEY1_DATA7_SPEC>`"]
pub type RD_KEY1_DATA7 = crate::Reg<rd_key1_data7::RD_KEY1_DATA7_SPEC>;
#[doc = "Register 7 of BLOCK5 (KEY1)."]
pub mod rd_key1_data7;
#[doc = "RD_KEY2_DATA0 (r) register accessor: an alias for `Reg<RD_KEY2_DATA0_SPEC>`"]
pub type RD_KEY2_DATA0 = crate::Reg<rd_key2_data0::RD_KEY2_DATA0_SPEC>;
#[doc = "Register 0 of BLOCK6 (KEY2)."]
pub mod rd_key2_data0;
#[doc = "RD_KEY2_DATA1 (r) register accessor: an alias for `Reg<RD_KEY2_DATA1_SPEC>`"]
pub type RD_KEY2_DATA1 = crate::Reg<rd_key2_data1::RD_KEY2_DATA1_SPEC>;
#[doc = "Register 1 of BLOCK6 (KEY2)."]
pub mod rd_key2_data1;
#[doc = "RD_KEY2_DATA2 (r) register accessor: an alias for `Reg<RD_KEY2_DATA2_SPEC>`"]
pub type RD_KEY2_DATA2 = crate::Reg<rd_key2_data2::RD_KEY2_DATA2_SPEC>;
#[doc = "Register 2 of BLOCK6 (KEY2)."]
pub mod rd_key2_data2;
#[doc = "RD_KEY2_DATA3 (r) register accessor: an alias for `Reg<RD_KEY2_DATA3_SPEC>`"]
pub type RD_KEY2_DATA3 = crate::Reg<rd_key2_data3::RD_KEY2_DATA3_SPEC>;
#[doc = "Register 3 of BLOCK6 (KEY2)."]
pub mod rd_key2_data3;
#[doc = "RD_KEY2_DATA4 (r) register accessor: an alias for `Reg<RD_KEY2_DATA4_SPEC>`"]
pub type RD_KEY2_DATA4 = crate::Reg<rd_key2_data4::RD_KEY2_DATA4_SPEC>;
#[doc = "Register 4 of BLOCK6 (KEY2)."]
pub mod rd_key2_data4;
#[doc = "RD_KEY2_DATA5 (r) register accessor: an alias for `Reg<RD_KEY2_DATA5_SPEC>`"]
pub type RD_KEY2_DATA5 = crate::Reg<rd_key2_data5::RD_KEY2_DATA5_SPEC>;
#[doc = "Register 5 of BLOCK6 (KEY2)."]
pub mod rd_key2_data5;
#[doc = "RD_KEY2_DATA6 (r) register accessor: an alias for `Reg<RD_KEY2_DATA6_SPEC>`"]
pub type RD_KEY2_DATA6 = crate::Reg<rd_key2_data6::RD_KEY2_DATA6_SPEC>;
#[doc = "Register 6 of BLOCK6 (KEY2)."]
pub mod rd_key2_data6;
#[doc = "RD_KEY2_DATA7 (r) register accessor: an alias for `Reg<RD_KEY2_DATA7_SPEC>`"]
pub type RD_KEY2_DATA7 = crate::Reg<rd_key2_data7::RD_KEY2_DATA7_SPEC>;
#[doc = "Register 7 of BLOCK6 (KEY2)."]
pub mod rd_key2_data7;
#[doc = "RD_KEY3_DATA0 (r) register accessor: an alias for `Reg<RD_KEY3_DATA0_SPEC>`"]
pub type RD_KEY3_DATA0 = crate::Reg<rd_key3_data0::RD_KEY3_DATA0_SPEC>;
#[doc = "Register 0 of BLOCK7 (KEY3)."]
pub mod rd_key3_data0;
#[doc = "RD_KEY3_DATA1 (r) register accessor: an alias for `Reg<RD_KEY3_DATA1_SPEC>`"]
pub type RD_KEY3_DATA1 = crate::Reg<rd_key3_data1::RD_KEY3_DATA1_SPEC>;
#[doc = "Register 1 of BLOCK7 (KEY3)."]
pub mod rd_key3_data1;
#[doc = "RD_KEY3_DATA2 (r) register accessor: an alias for `Reg<RD_KEY3_DATA2_SPEC>`"]
pub type RD_KEY3_DATA2 = crate::Reg<rd_key3_data2::RD_KEY3_DATA2_SPEC>;
#[doc = "Register 2 of BLOCK7 (KEY3)."]
pub mod rd_key3_data2;
#[doc = "RD_KEY3_DATA3 (r) register accessor: an alias for `Reg<RD_KEY3_DATA3_SPEC>`"]
pub type RD_KEY3_DATA3 = crate::Reg<rd_key3_data3::RD_KEY3_DATA3_SPEC>;
#[doc = "Register 3 of BLOCK7 (KEY3)."]
pub mod rd_key3_data3;
#[doc = "RD_KEY3_DATA4 (r) register accessor: an alias for `Reg<RD_KEY3_DATA4_SPEC>`"]
pub type RD_KEY3_DATA4 = crate::Reg<rd_key3_data4::RD_KEY3_DATA4_SPEC>;
#[doc = "Register 4 of BLOCK7 (KEY3)."]
pub mod rd_key3_data4;
#[doc = "RD_KEY3_DATA5 (r) register accessor: an alias for `Reg<RD_KEY3_DATA5_SPEC>`"]
pub type RD_KEY3_DATA5 = crate::Reg<rd_key3_data5::RD_KEY3_DATA5_SPEC>;
#[doc = "Register 5 of BLOCK7 (KEY3)."]
pub mod rd_key3_data5;
#[doc = "RD_KEY3_DATA6 (r) register accessor: an alias for `Reg<RD_KEY3_DATA6_SPEC>`"]
pub type RD_KEY3_DATA6 = crate::Reg<rd_key3_data6::RD_KEY3_DATA6_SPEC>;
#[doc = "Register 6 of BLOCK7 (KEY3)."]
pub mod rd_key3_data6;
#[doc = "RD_KEY3_DATA7 (r) register accessor: an alias for `Reg<RD_KEY3_DATA7_SPEC>`"]
pub type RD_KEY3_DATA7 = crate::Reg<rd_key3_data7::RD_KEY3_DATA7_SPEC>;
#[doc = "Register 7 of BLOCK7 (KEY3)."]
pub mod rd_key3_data7;
#[doc = "RD_KEY4_DATA0 (r) register accessor: an alias for `Reg<RD_KEY4_DATA0_SPEC>`"]
pub type RD_KEY4_DATA0 = crate::Reg<rd_key4_data0::RD_KEY4_DATA0_SPEC>;
#[doc = "Register 0 of BLOCK8 (KEY4)."]
pub mod rd_key4_data0;
#[doc = "RD_KEY4_DATA1 (r) register accessor: an alias for `Reg<RD_KEY4_DATA1_SPEC>`"]
pub type RD_KEY4_DATA1 = crate::Reg<rd_key4_data1::RD_KEY4_DATA1_SPEC>;
#[doc = "Register 1 of BLOCK8 (KEY4)."]
pub mod rd_key4_data1;
#[doc = "RD_KEY4_DATA2 (r) register accessor: an alias for `Reg<RD_KEY4_DATA2_SPEC>`"]
pub type RD_KEY4_DATA2 = crate::Reg<rd_key4_data2::RD_KEY4_DATA2_SPEC>;
#[doc = "Register 2 of BLOCK8 (KEY4)."]
pub mod rd_key4_data2;
#[doc = "RD_KEY4_DATA3 (r) register accessor: an alias for `Reg<RD_KEY4_DATA3_SPEC>`"]
pub type RD_KEY4_DATA3 = crate::Reg<rd_key4_data3::RD_KEY4_DATA3_SPEC>;
#[doc = "Register 3 of BLOCK8 (KEY4)."]
pub mod rd_key4_data3;
#[doc = "RD_KEY4_DATA4 (r) register accessor: an alias for `Reg<RD_KEY4_DATA4_SPEC>`"]
pub type RD_KEY4_DATA4 = crate::Reg<rd_key4_data4::RD_KEY4_DATA4_SPEC>;
#[doc = "Register 4 of BLOCK8 (KEY4)."]
pub mod rd_key4_data4;
#[doc = "RD_KEY4_DATA5 (r) register accessor: an alias for `Reg<RD_KEY4_DATA5_SPEC>`"]
pub type RD_KEY4_DATA5 = crate::Reg<rd_key4_data5::RD_KEY4_DATA5_SPEC>;
#[doc = "Register 5 of BLOCK8 (KEY4)."]
pub mod rd_key4_data5;
#[doc = "RD_KEY4_DATA6 (r) register accessor: an alias for `Reg<RD_KEY4_DATA6_SPEC>`"]
pub type RD_KEY4_DATA6 = crate::Reg<rd_key4_data6::RD_KEY4_DATA6_SPEC>;
#[doc = "Register 6 of BLOCK8 (KEY4)."]
pub mod rd_key4_data6;
#[doc = "RD_KEY4_DATA7 (r) register accessor: an alias for `Reg<RD_KEY4_DATA7_SPEC>`"]
pub type RD_KEY4_DATA7 = crate::Reg<rd_key4_data7::RD_KEY4_DATA7_SPEC>;
#[doc = "Register 7 of BLOCK8 (KEY4)."]
pub mod rd_key4_data7;
#[doc = "RD_KEY5_DATA0 (r) register accessor: an alias for `Reg<RD_KEY5_DATA0_SPEC>`"]
pub type RD_KEY5_DATA0 = crate::Reg<rd_key5_data0::RD_KEY5_DATA0_SPEC>;
#[doc = "Register 0 of BLOCK9 (KEY5)."]
pub mod rd_key5_data0;
#[doc = "RD_KEY5_DATA1 (r) register accessor: an alias for `Reg<RD_KEY5_DATA1_SPEC>`"]
pub type RD_KEY5_DATA1 = crate::Reg<rd_key5_data1::RD_KEY5_DATA1_SPEC>;
#[doc = "Register 1 of BLOCK9 (KEY5)."]
pub mod rd_key5_data1;
#[doc = "RD_KEY5_DATA2 (r) register accessor: an alias for `Reg<RD_KEY5_DATA2_SPEC>`"]
pub type RD_KEY5_DATA2 = crate::Reg<rd_key5_data2::RD_KEY5_DATA2_SPEC>;
#[doc = "Register 2 of BLOCK9 (KEY5)."]
pub mod rd_key5_data2;
#[doc = "RD_KEY5_DATA3 (r) register accessor: an alias for `Reg<RD_KEY5_DATA3_SPEC>`"]
pub type RD_KEY5_DATA3 = crate::Reg<rd_key5_data3::RD_KEY5_DATA3_SPEC>;
#[doc = "Register 3 of BLOCK9 (KEY5)."]
pub mod rd_key5_data3;
#[doc = "RD_KEY5_DATA4 (r) register accessor: an alias for `Reg<RD_KEY5_DATA4_SPEC>`"]
pub type RD_KEY5_DATA4 = crate::Reg<rd_key5_data4::RD_KEY5_DATA4_SPEC>;
#[doc = "Register 4 of BLOCK9 (KEY5)."]
pub mod rd_key5_data4;
#[doc = "RD_KEY5_DATA5 (r) register accessor: an alias for `Reg<RD_KEY5_DATA5_SPEC>`"]
pub type RD_KEY5_DATA5 = crate::Reg<rd_key5_data5::RD_KEY5_DATA5_SPEC>;
#[doc = "Register 5 of BLOCK9 (KEY5)."]
pub mod rd_key5_data5;
#[doc = "RD_KEY5_DATA6 (r) register accessor: an alias for `Reg<RD_KEY5_DATA6_SPEC>`"]
pub type RD_KEY5_DATA6 = crate::Reg<rd_key5_data6::RD_KEY5_DATA6_SPEC>;
#[doc = "Register 6 of BLOCK9 (KEY5)."]
pub mod rd_key5_data6;
#[doc = "RD_KEY5_DATA7 (r) register accessor: an alias for `Reg<RD_KEY5_DATA7_SPEC>`"]
pub type RD_KEY5_DATA7 = crate::Reg<rd_key5_data7::RD_KEY5_DATA7_SPEC>;
#[doc = "Register 7 of BLOCK9 (KEY5)."]
pub mod rd_key5_data7;
#[doc = "RD_SYS_PART2_DATA0 (r) register accessor: an alias for `Reg<RD_SYS_PART2_DATA0_SPEC>`"]
pub type RD_SYS_PART2_DATA0 = crate::Reg<rd_sys_part2_data0::RD_SYS_PART2_DATA0_SPEC>;
#[doc = "Register 0 of BLOCK10 (system)."]
pub mod rd_sys_part2_data0;
#[doc = "RD_SYS_PART2_DATA1 (r) register accessor: an alias for `Reg<RD_SYS_PART2_DATA1_SPEC>`"]
pub type RD_SYS_PART2_DATA1 = crate::Reg<rd_sys_part2_data1::RD_SYS_PART2_DATA1_SPEC>;
#[doc = "Register 1 of BLOCK9 (KEY5)."]
pub mod rd_sys_part2_data1;
#[doc = "RD_SYS_PART2_DATA2 (r) register accessor: an alias for `Reg<RD_SYS_PART2_DATA2_SPEC>`"]
pub type RD_SYS_PART2_DATA2 = crate::Reg<rd_sys_part2_data2::RD_SYS_PART2_DATA2_SPEC>;
#[doc = "Register 2 of BLOCK10 (system)."]
pub mod rd_sys_part2_data2;
#[doc = "RD_SYS_PART2_DATA3 (r) register accessor: an alias for `Reg<RD_SYS_PART2_DATA3_SPEC>`"]
pub type RD_SYS_PART2_DATA3 = crate::Reg<rd_sys_part2_data3::RD_SYS_PART2_DATA3_SPEC>;
#[doc = "Register 3 of BLOCK10 (system)."]
pub mod rd_sys_part2_data3;
#[doc = "RD_SYS_PART2_DATA4 (r) register accessor: an alias for `Reg<RD_SYS_PART2_DATA4_SPEC>`"]
pub type RD_SYS_PART2_DATA4 = crate::Reg<rd_sys_part2_data4::RD_SYS_PART2_DATA4_SPEC>;
#[doc = "Register 4 of BLOCK10 (system)."]
pub mod rd_sys_part2_data4;
#[doc = "RD_SYS_PART2_DATA5 (r) register accessor: an alias for `Reg<RD_SYS_PART2_DATA5_SPEC>`"]
pub type RD_SYS_PART2_DATA5 = crate::Reg<rd_sys_part2_data5::RD_SYS_PART2_DATA5_SPEC>;
#[doc = "Register 5 of BLOCK10 (system)."]
pub mod rd_sys_part2_data5;
#[doc = "RD_SYS_PART2_DATA6 (r) register accessor: an alias for `Reg<RD_SYS_PART2_DATA6_SPEC>`"]
pub type RD_SYS_PART2_DATA6 = crate::Reg<rd_sys_part2_data6::RD_SYS_PART2_DATA6_SPEC>;
#[doc = "Register 6 of BLOCK10 (system)."]
pub mod rd_sys_part2_data6;
#[doc = "RD_SYS_PART2_DATA7 (r) register accessor: an alias for `Reg<RD_SYS_PART2_DATA7_SPEC>`"]
pub type RD_SYS_PART2_DATA7 = crate::Reg<rd_sys_part2_data7::RD_SYS_PART2_DATA7_SPEC>;
#[doc = "Register 7 of BLOCK10 (system)."]
pub mod rd_sys_part2_data7;
#[doc = "RD_REPEAT_ERR0 (r) register accessor: an alias for `Reg<RD_REPEAT_ERR0_SPEC>`"]
pub type RD_REPEAT_ERR0 = crate::Reg<rd_repeat_err0::RD_REPEAT_ERR0_SPEC>;
#[doc = "Programming error record register 0 of BLOCK0."]
pub mod rd_repeat_err0;
#[doc = "RD_REPEAT_ERR1 (r) register accessor: an alias for `Reg<RD_REPEAT_ERR1_SPEC>`"]
pub type RD_REPEAT_ERR1 = crate::Reg<rd_repeat_err1::RD_REPEAT_ERR1_SPEC>;
#[doc = "Programming error record register 1 of BLOCK0."]
pub mod rd_repeat_err1;
#[doc = "RD_REPEAT_ERR2 (r) register accessor: an alias for `Reg<RD_REPEAT_ERR2_SPEC>`"]
pub type RD_REPEAT_ERR2 = crate::Reg<rd_repeat_err2::RD_REPEAT_ERR2_SPEC>;
#[doc = "Programming error record register 2 of BLOCK0."]
pub mod rd_repeat_err2;
#[doc = "RD_REPEAT_ERR3 (r) register accessor: an alias for `Reg<RD_REPEAT_ERR3_SPEC>`"]
pub type RD_REPEAT_ERR3 = crate::Reg<rd_repeat_err3::RD_REPEAT_ERR3_SPEC>;
#[doc = "Programming error record register 3 of BLOCK0."]
pub mod rd_repeat_err3;
#[doc = "RD_REPEAT_ERR4 (r) register accessor: an alias for `Reg<RD_REPEAT_ERR4_SPEC>`"]
pub type RD_REPEAT_ERR4 = crate::Reg<rd_repeat_err4::RD_REPEAT_ERR4_SPEC>;
#[doc = "Programming error record register 4 of BLOCK0."]
pub mod rd_repeat_err4;
#[doc = "RD_RS_ERR0 (r) register accessor: an alias for `Reg<RD_RS_ERR0_SPEC>`"]
pub type RD_RS_ERR0 = crate::Reg<rd_rs_err0::RD_RS_ERR0_SPEC>;
#[doc = "Programming error record register 0 of BLOCK1-10."]
pub mod rd_rs_err0;
#[doc = "RD_RS_ERR1 (r) register accessor: an alias for `Reg<RD_RS_ERR1_SPEC>`"]
pub type RD_RS_ERR1 = crate::Reg<rd_rs_err1::RD_RS_ERR1_SPEC>;
#[doc = "Programming error record register 1 of BLOCK1-10."]
pub mod rd_rs_err1;
#[doc = "CLK (rw) register accessor: an alias for `Reg<CLK_SPEC>`"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = "eFuse clcok configuration register."]
pub mod clk;
#[doc = "CONF (rw) register accessor: an alias for `Reg<CONF_SPEC>`"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "eFuse operation mode configuraiton register"]
pub mod conf;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "eFuse status register."]
pub mod status;
#[doc = "CMD (rw) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "eFuse command register."]
pub mod cmd;
#[doc = "INT_RAW (rw) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "eFuse raw interrupt register."]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: an alias for `Reg<INT_ST_SPEC>`"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "eFuse interrupt status register."]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "eFuse interrupt enable register."]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "eFuse interrupt clear register."]
pub mod int_clr;
#[doc = "DAC_CONF (rw) register accessor: an alias for `Reg<DAC_CONF_SPEC>`"]
pub type DAC_CONF = crate::Reg<dac_conf::DAC_CONF_SPEC>;
#[doc = "Controls the eFuse programming voltage."]
pub mod dac_conf;
#[doc = "RD_TIM_CONF (rw) register accessor: an alias for `Reg<RD_TIM_CONF_SPEC>`"]
pub type RD_TIM_CONF = crate::Reg<rd_tim_conf::RD_TIM_CONF_SPEC>;
#[doc = "Configures read timing parameters."]
pub mod rd_tim_conf;
#[doc = "WR_TIM_CONF1 (rw) register accessor: an alias for `Reg<WR_TIM_CONF1_SPEC>`"]
pub type WR_TIM_CONF1 = crate::Reg<wr_tim_conf1::WR_TIM_CONF1_SPEC>;
#[doc = "Configurarion register 1 of eFuse programming timing parameters."]
pub mod wr_tim_conf1;
#[doc = "WR_TIM_CONF2 (rw) register accessor: an alias for `Reg<WR_TIM_CONF2_SPEC>`"]
pub type WR_TIM_CONF2 = crate::Reg<wr_tim_conf2::WR_TIM_CONF2_SPEC>;
#[doc = "Configurarion register 2 of eFuse programming timing parameters."]
pub mod wr_tim_conf2;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "eFuse version register."]
pub mod date;
