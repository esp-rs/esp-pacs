#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    pgm_data: [PGM_DATA; 8],
    pgm_check_value: [PGM_CHECK_VALUE; 3],
    rd_wr_dis: RD_WR_DIS,
    rd_repeat_data0: RD_REPEAT_DATA0,
    rd_repeat_data1: RD_REPEAT_DATA1,
    rd_repeat_data2: RD_REPEAT_DATA2,
    rd_repeat_data3: RD_REPEAT_DATA3,
    rd_repeat_data4: RD_REPEAT_DATA4,
    rd_mac_spi_sys_0: RD_MAC_SPI_SYS_0,
    rd_mac_spi_sys_1: RD_MAC_SPI_SYS_1,
    rd_mac_spi_sys_2: RD_MAC_SPI_SYS_2,
    rd_mac_spi_sys_3: RD_MAC_SPI_SYS_3,
    rd_mac_spi_sys_4: RD_MAC_SPI_SYS_4,
    rd_mac_spi_sys_5: RD_MAC_SPI_SYS_5,
    rd_sys_part1_data: [RD_SYS_PART1_DATA; 8],
    rd_usr_data: [RD_USR_DATA; 8],
    rd_key: [RD_KEY; 6],
    rd_sys_part2_data: [RD_SYS_PART2_DATA; 8],
    rd_repeat_err0: RD_REPEAT_ERR0,
    rd_repeat_err1: RD_REPEAT_ERR1,
    rd_repeat_err2: RD_REPEAT_ERR2,
    rd_repeat_err3: RD_REPEAT_ERR3,
    _reserved22: [u8; 0x04],
    rd_repeat_err4: RD_REPEAT_ERR4,
    _reserved23: [u8; 0x2c],
    rd_rs_err0: RD_RS_ERR0,
    rd_rs_err1: RD_RS_ERR1,
    clk: CLK,
    conf: CONF,
    status: STATUS,
    cmd: CMD,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    dac_conf: DAC_CONF,
    rd_tim_conf: RD_TIM_CONF,
    wr_tim_conf0: WR_TIM_CONF0,
    wr_tim_conf1: WR_TIM_CONF1,
    wr_tim_conf2: WR_TIM_CONF2,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - Register %s that stores data to be programmed."]
    #[inline(always)]
    pub const fn pgm_data(&self, n: usize) -> &PGM_DATA {
        &self.pgm_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - Register %s that stores data to be programmed."]
    #[inline(always)]
    pub fn pgm_data_iter(&self) -> impl Iterator<Item = &PGM_DATA> {
        self.pgm_data.iter()
    }
    #[doc = "0x20..0x2c - Register %s that stores the RS code to be programmed."]
    #[inline(always)]
    pub const fn pgm_check_value(&self, n: usize) -> &PGM_CHECK_VALUE {
        &self.pgm_check_value[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x2c - Register %s that stores the RS code to be programmed."]
    #[inline(always)]
    pub fn pgm_check_value_iter(&self) -> impl Iterator<Item = &PGM_CHECK_VALUE> {
        self.pgm_check_value.iter()
    }
    #[doc = "0x2c - Register 0 of BLOCK0."]
    #[inline(always)]
    pub const fn rd_wr_dis(&self) -> &RD_WR_DIS {
        &self.rd_wr_dis
    }
    #[doc = "0x30 - Register 1 of BLOCK0."]
    #[inline(always)]
    pub const fn rd_repeat_data0(&self) -> &RD_REPEAT_DATA0 {
        &self.rd_repeat_data0
    }
    #[doc = "0x34 - Register 2 of BLOCK0."]
    #[inline(always)]
    pub const fn rd_repeat_data1(&self) -> &RD_REPEAT_DATA1 {
        &self.rd_repeat_data1
    }
    #[doc = "0x38 - Register 3 of BLOCK0."]
    #[inline(always)]
    pub const fn rd_repeat_data2(&self) -> &RD_REPEAT_DATA2 {
        &self.rd_repeat_data2
    }
    #[doc = "0x3c - Register 4 of BLOCK0."]
    #[inline(always)]
    pub const fn rd_repeat_data3(&self) -> &RD_REPEAT_DATA3 {
        &self.rd_repeat_data3
    }
    #[doc = "0x40 - Register 5 of BLOCK0."]
    #[inline(always)]
    pub const fn rd_repeat_data4(&self) -> &RD_REPEAT_DATA4 {
        &self.rd_repeat_data4
    }
    #[doc = "0x44 - Register 0 of BLOCK1."]
    #[inline(always)]
    pub const fn rd_mac_spi_sys_0(&self) -> &RD_MAC_SPI_SYS_0 {
        &self.rd_mac_spi_sys_0
    }
    #[doc = "0x48 - Register 1 of BLOCK1."]
    #[inline(always)]
    pub const fn rd_mac_spi_sys_1(&self) -> &RD_MAC_SPI_SYS_1 {
        &self.rd_mac_spi_sys_1
    }
    #[doc = "0x4c - Register 2 of BLOCK1."]
    #[inline(always)]
    pub const fn rd_mac_spi_sys_2(&self) -> &RD_MAC_SPI_SYS_2 {
        &self.rd_mac_spi_sys_2
    }
    #[doc = "0x50 - Register 3 of BLOCK1."]
    #[inline(always)]
    pub const fn rd_mac_spi_sys_3(&self) -> &RD_MAC_SPI_SYS_3 {
        &self.rd_mac_spi_sys_3
    }
    #[doc = "0x54 - Register 4 of BLOCK1."]
    #[inline(always)]
    pub const fn rd_mac_spi_sys_4(&self) -> &RD_MAC_SPI_SYS_4 {
        &self.rd_mac_spi_sys_4
    }
    #[doc = "0x58 - Register 5 of BLOCK1."]
    #[inline(always)]
    pub const fn rd_mac_spi_sys_5(&self) -> &RD_MAC_SPI_SYS_5 {
        &self.rd_mac_spi_sys_5
    }
    #[doc = "0x5c..0x7c - Register %s of BLOCK2 (system)."]
    #[inline(always)]
    pub const fn rd_sys_part1_data(&self, n: usize) -> &RD_SYS_PART1_DATA {
        &self.rd_sys_part1_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x5c..0x7c - Register %s of BLOCK2 (system)."]
    #[inline(always)]
    pub fn rd_sys_part1_data_iter(&self) -> impl Iterator<Item = &RD_SYS_PART1_DATA> {
        self.rd_sys_part1_data.iter()
    }
    #[doc = "0x7c..0x9c - Register %s of BLOCK3 (user)."]
    #[inline(always)]
    pub const fn rd_usr_data(&self, n: usize) -> &RD_USR_DATA {
        &self.rd_usr_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x7c..0x9c - Register %s of BLOCK3 (user)."]
    #[inline(always)]
    pub fn rd_usr_data_iter(&self) -> impl Iterator<Item = &RD_USR_DATA> {
        self.rd_usr_data.iter()
    }
    #[doc = "0x9c..0x15c - Cluster RD_KEY%s, containing RD_KEY?_DATA%s"]
    #[inline(always)]
    pub const fn rd_key(&self, n: usize) -> &RD_KEY {
        &self.rd_key[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x9c..0x15c - Cluster RD_KEY%s, containing RD_KEY?_DATA%s"]
    #[inline(always)]
    pub fn rd_key_iter(&self) -> impl Iterator<Item = &RD_KEY> {
        self.rd_key.iter()
    }
    #[doc = "0x15c..0x17c - Register %s of BLOCK10 (system)."]
    #[inline(always)]
    pub const fn rd_sys_part2_data(&self, n: usize) -> &RD_SYS_PART2_DATA {
        &self.rd_sys_part2_data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x15c..0x17c - Register %s of BLOCK10 (system)."]
    #[inline(always)]
    pub fn rd_sys_part2_data_iter(&self) -> impl Iterator<Item = &RD_SYS_PART2_DATA> {
        self.rd_sys_part2_data.iter()
    }
    #[doc = "0x17c - Programming error record register 0 of BLOCK0."]
    #[inline(always)]
    pub const fn rd_repeat_err0(&self) -> &RD_REPEAT_ERR0 {
        &self.rd_repeat_err0
    }
    #[doc = "0x180 - Programming error record register 1 of BLOCK0."]
    #[inline(always)]
    pub const fn rd_repeat_err1(&self) -> &RD_REPEAT_ERR1 {
        &self.rd_repeat_err1
    }
    #[doc = "0x184 - Programming error record register 2 of BLOCK0."]
    #[inline(always)]
    pub const fn rd_repeat_err2(&self) -> &RD_REPEAT_ERR2 {
        &self.rd_repeat_err2
    }
    #[doc = "0x188 - Programming error record register 3 of BLOCK0."]
    #[inline(always)]
    pub const fn rd_repeat_err3(&self) -> &RD_REPEAT_ERR3 {
        &self.rd_repeat_err3
    }
    #[doc = "0x190 - Programming error record register 4 of BLOCK0."]
    #[inline(always)]
    pub const fn rd_repeat_err4(&self) -> &RD_REPEAT_ERR4 {
        &self.rd_repeat_err4
    }
    #[doc = "0x1c0 - Programming error record register 0 of BLOCK1-10."]
    #[inline(always)]
    pub const fn rd_rs_err0(&self) -> &RD_RS_ERR0 {
        &self.rd_rs_err0
    }
    #[doc = "0x1c4 - Programming error record register 1 of BLOCK1-10."]
    #[inline(always)]
    pub const fn rd_rs_err1(&self) -> &RD_RS_ERR1 {
        &self.rd_rs_err1
    }
    #[doc = "0x1c8 - eFuse clock configuration register."]
    #[inline(always)]
    pub const fn clk(&self) -> &CLK {
        &self.clk
    }
    #[doc = "0x1cc - eFuse operation mode configuration register."]
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    #[doc = "0x1d0 - eFuse status register."]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x1d4 - eFuse command register."]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x1d8 - eFuse raw interrupt register."]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x1dc - eFuse interrupt status register."]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x1e0 - eFuse interrupt enable register."]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x1e4 - eFuse interrupt clear register."]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x1e8 - Controls the eFuse programming voltage."]
    #[inline(always)]
    pub const fn dac_conf(&self) -> &DAC_CONF {
        &self.dac_conf
    }
    #[doc = "0x1ec - Configures read timing parameters."]
    #[inline(always)]
    pub const fn rd_tim_conf(&self) -> &RD_TIM_CONF {
        &self.rd_tim_conf
    }
    #[doc = "0x1f0 - Configuration register 0 of eFuse programming timing parameters."]
    #[inline(always)]
    pub const fn wr_tim_conf0(&self) -> &WR_TIM_CONF0 {
        &self.wr_tim_conf0
    }
    #[doc = "0x1f4 - Configuration register 1 of eFuse programming timing parameters."]
    #[inline(always)]
    pub const fn wr_tim_conf1(&self) -> &WR_TIM_CONF1 {
        &self.wr_tim_conf1
    }
    #[doc = "0x1f8 - Configuration register 2 of eFuse programming timing parameters."]
    #[inline(always)]
    pub const fn wr_tim_conf2(&self) -> &WR_TIM_CONF2 {
        &self.wr_tim_conf2
    }
    #[doc = "0x1fc - Version control register."]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "PGM_DATA (rw) register accessor: Register %s that stores data to be programmed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pgm_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pgm_data`] module"]
pub type PGM_DATA = crate::Reg<pgm_data::PGM_DATA_SPEC>;
#[doc = "Register %s that stores data to be programmed."]
pub mod pgm_data;
#[doc = "PGM_CHECK_VALUE (rw) register accessor: Register %s that stores the RS code to be programmed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pgm_check_value::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pgm_check_value::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pgm_check_value`] module"]
pub type PGM_CHECK_VALUE = crate::Reg<pgm_check_value::PGM_CHECK_VALUE_SPEC>;
#[doc = "Register %s that stores the RS code to be programmed."]
pub mod pgm_check_value;
#[doc = "RD_WR_DIS (r) register accessor: Register 0 of BLOCK0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_wr_dis::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_wr_dis`] module"]
pub type RD_WR_DIS = crate::Reg<rd_wr_dis::RD_WR_DIS_SPEC>;
#[doc = "Register 0 of BLOCK0."]
pub mod rd_wr_dis;
#[doc = "RD_REPEAT_DATA0 (r) register accessor: Register 1 of BLOCK0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_data0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_data0`] module"]
pub type RD_REPEAT_DATA0 = crate::Reg<rd_repeat_data0::RD_REPEAT_DATA0_SPEC>;
#[doc = "Register 1 of BLOCK0."]
pub mod rd_repeat_data0;
#[doc = "RD_REPEAT_DATA1 (r) register accessor: Register 2 of BLOCK0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_data1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_data1`] module"]
pub type RD_REPEAT_DATA1 = crate::Reg<rd_repeat_data1::RD_REPEAT_DATA1_SPEC>;
#[doc = "Register 2 of BLOCK0."]
pub mod rd_repeat_data1;
#[doc = "RD_REPEAT_DATA2 (r) register accessor: Register 3 of BLOCK0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_data2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_data2`] module"]
pub type RD_REPEAT_DATA2 = crate::Reg<rd_repeat_data2::RD_REPEAT_DATA2_SPEC>;
#[doc = "Register 3 of BLOCK0."]
pub mod rd_repeat_data2;
#[doc = "RD_REPEAT_DATA3 (r) register accessor: Register 4 of BLOCK0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_data3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_data3`] module"]
pub type RD_REPEAT_DATA3 = crate::Reg<rd_repeat_data3::RD_REPEAT_DATA3_SPEC>;
#[doc = "Register 4 of BLOCK0."]
pub mod rd_repeat_data3;
#[doc = "RD_REPEAT_DATA4 (r) register accessor: Register 5 of BLOCK0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_data4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_data4`] module"]
pub type RD_REPEAT_DATA4 = crate::Reg<rd_repeat_data4::RD_REPEAT_DATA4_SPEC>;
#[doc = "Register 5 of BLOCK0."]
pub mod rd_repeat_data4;
#[doc = "RD_MAC_SPI_SYS_0 (r) register accessor: Register 0 of BLOCK1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_spi_sys_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_mac_spi_sys_0`] module"]
pub type RD_MAC_SPI_SYS_0 = crate::Reg<rd_mac_spi_sys_0::RD_MAC_SPI_SYS_0_SPEC>;
#[doc = "Register 0 of BLOCK1."]
pub mod rd_mac_spi_sys_0;
#[doc = "RD_MAC_SPI_SYS_1 (r) register accessor: Register 1 of BLOCK1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_spi_sys_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_mac_spi_sys_1`] module"]
pub type RD_MAC_SPI_SYS_1 = crate::Reg<rd_mac_spi_sys_1::RD_MAC_SPI_SYS_1_SPEC>;
#[doc = "Register 1 of BLOCK1."]
pub mod rd_mac_spi_sys_1;
#[doc = "RD_MAC_SPI_SYS_2 (r) register accessor: Register 2 of BLOCK1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_spi_sys_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_mac_spi_sys_2`] module"]
pub type RD_MAC_SPI_SYS_2 = crate::Reg<rd_mac_spi_sys_2::RD_MAC_SPI_SYS_2_SPEC>;
#[doc = "Register 2 of BLOCK1."]
pub mod rd_mac_spi_sys_2;
#[doc = "RD_MAC_SPI_SYS_3 (r) register accessor: Register 3 of BLOCK1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_spi_sys_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_mac_spi_sys_3`] module"]
pub type RD_MAC_SPI_SYS_3 = crate::Reg<rd_mac_spi_sys_3::RD_MAC_SPI_SYS_3_SPEC>;
#[doc = "Register 3 of BLOCK1."]
pub mod rd_mac_spi_sys_3;
#[doc = "RD_MAC_SPI_SYS_4 (r) register accessor: Register 4 of BLOCK1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_spi_sys_4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_mac_spi_sys_4`] module"]
pub type RD_MAC_SPI_SYS_4 = crate::Reg<rd_mac_spi_sys_4::RD_MAC_SPI_SYS_4_SPEC>;
#[doc = "Register 4 of BLOCK1."]
pub mod rd_mac_spi_sys_4;
#[doc = "RD_MAC_SPI_SYS_5 (r) register accessor: Register 5 of BLOCK1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_spi_sys_5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_mac_spi_sys_5`] module"]
pub type RD_MAC_SPI_SYS_5 = crate::Reg<rd_mac_spi_sys_5::RD_MAC_SPI_SYS_5_SPEC>;
#[doc = "Register 5 of BLOCK1."]
pub mod rd_mac_spi_sys_5;
#[doc = "RD_SYS_PART1_DATA (r) register accessor: Register %s of BLOCK2 (system).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_part1_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_sys_part1_data`] module"]
pub type RD_SYS_PART1_DATA = crate::Reg<rd_sys_part1_data::RD_SYS_PART1_DATA_SPEC>;
#[doc = "Register %s of BLOCK2 (system)."]
pub mod rd_sys_part1_data;
#[doc = "RD_USR_DATA (r) register accessor: Register %s of BLOCK3 (user).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_usr_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_usr_data`] module"]
pub type RD_USR_DATA = crate::Reg<rd_usr_data::RD_USR_DATA_SPEC>;
#[doc = "Register %s of BLOCK3 (user)."]
pub mod rd_usr_data;
#[doc = "Cluster RD_KEY%s, containing RD_KEY?_DATA%s"]
pub use self::rd_key::RD_KEY;
#[doc = r"Cluster"]
#[doc = "Cluster RD_KEY%s, containing RD_KEY?_DATA%s"]
pub mod rd_key;
#[doc = "RD_SYS_PART2_DATA (r) register accessor: Register %s of BLOCK10 (system).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_sys_part2_data::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_sys_part2_data`] module"]
pub type RD_SYS_PART2_DATA = crate::Reg<rd_sys_part2_data::RD_SYS_PART2_DATA_SPEC>;
#[doc = "Register %s of BLOCK10 (system)."]
pub mod rd_sys_part2_data;
#[doc = "RD_REPEAT_ERR0 (r) register accessor: Programming error record register 0 of BLOCK0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_err0`] module"]
pub type RD_REPEAT_ERR0 = crate::Reg<rd_repeat_err0::RD_REPEAT_ERR0_SPEC>;
#[doc = "Programming error record register 0 of BLOCK0."]
pub mod rd_repeat_err0;
#[doc = "RD_REPEAT_ERR1 (r) register accessor: Programming error record register 1 of BLOCK0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_err1`] module"]
pub type RD_REPEAT_ERR1 = crate::Reg<rd_repeat_err1::RD_REPEAT_ERR1_SPEC>;
#[doc = "Programming error record register 1 of BLOCK0."]
pub mod rd_repeat_err1;
#[doc = "RD_REPEAT_ERR2 (r) register accessor: Programming error record register 2 of BLOCK0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_err2`] module"]
pub type RD_REPEAT_ERR2 = crate::Reg<rd_repeat_err2::RD_REPEAT_ERR2_SPEC>;
#[doc = "Programming error record register 2 of BLOCK0."]
pub mod rd_repeat_err2;
#[doc = "RD_REPEAT_ERR3 (r) register accessor: Programming error record register 3 of BLOCK0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_err3`] module"]
pub type RD_REPEAT_ERR3 = crate::Reg<rd_repeat_err3::RD_REPEAT_ERR3_SPEC>;
#[doc = "Programming error record register 3 of BLOCK0."]
pub mod rd_repeat_err3;
#[doc = "RD_REPEAT_ERR4 (r) register accessor: Programming error record register 4 of BLOCK0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_repeat_err4`] module"]
pub type RD_REPEAT_ERR4 = crate::Reg<rd_repeat_err4::RD_REPEAT_ERR4_SPEC>;
#[doc = "Programming error record register 4 of BLOCK0."]
pub mod rd_repeat_err4;
#[doc = "RD_RS_ERR0 (r) register accessor: Programming error record register 0 of BLOCK1-10.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_rs_err0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_rs_err0`] module"]
pub type RD_RS_ERR0 = crate::Reg<rd_rs_err0::RD_RS_ERR0_SPEC>;
#[doc = "Programming error record register 0 of BLOCK1-10."]
pub mod rd_rs_err0;
#[doc = "RD_RS_ERR1 (r) register accessor: Programming error record register 1 of BLOCK1-10.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_rs_err1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_rs_err1`] module"]
pub type RD_RS_ERR1 = crate::Reg<rd_rs_err1::RD_RS_ERR1_SPEC>;
#[doc = "Programming error record register 1 of BLOCK1-10."]
pub mod rd_rs_err1;
#[doc = "CLK (rw) register accessor: eFuse clock configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk`] module"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = "eFuse clock configuration register."]
pub mod clk;
#[doc = "CONF (rw) register accessor: eFuse operation mode configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`] module"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "eFuse operation mode configuration register."]
pub mod conf;
#[doc = "STATUS (r) register accessor: eFuse status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "eFuse status register."]
pub mod status;
#[doc = "CMD (rw) register accessor: eFuse command register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "eFuse command register."]
pub mod cmd;
#[doc = "INT_RAW (r) register accessor: eFuse raw interrupt register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "eFuse raw interrupt register."]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: eFuse interrupt status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "eFuse interrupt status register."]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: eFuse interrupt enable register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "eFuse interrupt enable register."]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: eFuse interrupt clear register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "eFuse interrupt clear register."]
pub mod int_clr;
#[doc = "DAC_CONF (rw) register accessor: Controls the eFuse programming voltage.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dac_conf`] module"]
pub type DAC_CONF = crate::Reg<dac_conf::DAC_CONF_SPEC>;
#[doc = "Controls the eFuse programming voltage."]
pub mod dac_conf;
#[doc = "RD_TIM_CONF (rw) register accessor: Configures read timing parameters.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_tim_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_tim_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_tim_conf`] module"]
pub type RD_TIM_CONF = crate::Reg<rd_tim_conf::RD_TIM_CONF_SPEC>;
#[doc = "Configures read timing parameters."]
pub mod rd_tim_conf;
#[doc = "WR_TIM_CONF0 (rw) register accessor: Configuration register 0 of eFuse programming timing parameters.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_tim_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_tim_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_tim_conf0`] module"]
pub type WR_TIM_CONF0 = crate::Reg<wr_tim_conf0::WR_TIM_CONF0_SPEC>;
#[doc = "Configuration register 0 of eFuse programming timing parameters."]
pub mod wr_tim_conf0;
#[doc = "WR_TIM_CONF1 (rw) register accessor: Configuration register 1 of eFuse programming timing parameters.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_tim_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_tim_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_tim_conf1`] module"]
pub type WR_TIM_CONF1 = crate::Reg<wr_tim_conf1::WR_TIM_CONF1_SPEC>;
#[doc = "Configuration register 1 of eFuse programming timing parameters."]
pub mod wr_tim_conf1;
#[doc = "WR_TIM_CONF2 (rw) register accessor: Configuration register 2 of eFuse programming timing parameters.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_tim_conf2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_tim_conf2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_tim_conf2`] module"]
pub type WR_TIM_CONF2 = crate::Reg<wr_tim_conf2::WR_TIM_CONF2_SPEC>;
#[doc = "Configuration register 2 of eFuse programming timing parameters."]
pub mod wr_tim_conf2;
#[doc = "DATE (rw) register accessor: Version control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register."]
pub mod date;
