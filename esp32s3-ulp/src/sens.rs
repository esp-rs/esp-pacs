#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    sar_slave_addr1: SAR_SLAVE_ADDR1,
    sar_slave_addr2: SAR_SLAVE_ADDR2,
    sar_slave_addr3: SAR_SLAVE_ADDR3,
    sar_slave_addr4: SAR_SLAVE_ADDR4,
    _reserved4: [u8; 0x08],
    sar_i2c_ctrl: SAR_I2C_CTRL,
    _reserved5: [u8; 0x8c],
    sar_cocpu_int_raw: SAR_COCPU_INT_RAW,
    sar_cocpu_int_ena: SAR_COCPU_INT_ENA,
    sar_cocpu_int_st: SAR_COCPU_INT_ST,
    sar_cocpu_int_clr: SAR_COCPU_INT_CLR,
}
impl RegisterBlock {
    ///0x40 - configure i2c slave address
    #[inline(always)]
    pub const fn sar_slave_addr1(&self) -> &SAR_SLAVE_ADDR1 {
        &self.sar_slave_addr1
    }
    ///0x44 - configure i2c slave address
    #[inline(always)]
    pub const fn sar_slave_addr2(&self) -> &SAR_SLAVE_ADDR2 {
        &self.sar_slave_addr2
    }
    ///0x48 - configure i2c slave address
    #[inline(always)]
    pub const fn sar_slave_addr3(&self) -> &SAR_SLAVE_ADDR3 {
        &self.sar_slave_addr3
    }
    ///0x4c - configure i2c slave address
    #[inline(always)]
    pub const fn sar_slave_addr4(&self) -> &SAR_SLAVE_ADDR4 {
        &self.sar_slave_addr4
    }
    ///0x58 - configure rtc i2c controller by sw
    #[inline(always)]
    pub const fn sar_i2c_ctrl(&self) -> &SAR_I2C_CTRL {
        &self.sar_i2c_ctrl
    }
    ///0xe8 - the interrupt raw of ulp
    #[inline(always)]
    pub const fn sar_cocpu_int_raw(&self) -> &SAR_COCPU_INT_RAW {
        &self.sar_cocpu_int_raw
    }
    ///0xec - the interrupt enable of ulp
    #[inline(always)]
    pub const fn sar_cocpu_int_ena(&self) -> &SAR_COCPU_INT_ENA {
        &self.sar_cocpu_int_ena
    }
    ///0xf0 - the interrupt state of ulp
    #[inline(always)]
    pub const fn sar_cocpu_int_st(&self) -> &SAR_COCPU_INT_ST {
        &self.sar_cocpu_int_st
    }
    ///0xf4 - the interrupt clear of ulp
    #[inline(always)]
    pub const fn sar_cocpu_int_clr(&self) -> &SAR_COCPU_INT_CLR {
        &self.sar_cocpu_int_clr
    }
}
/**SAR_SLAVE_ADDR1 (rw) register accessor: configure i2c slave address

You can [`read`](crate::generic::Reg::read) this register and get [`sar_slave_addr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_slave_addr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sar_slave_addr1`] module*/
pub type SAR_SLAVE_ADDR1 = crate::Reg<sar_slave_addr1::SAR_SLAVE_ADDR1_SPEC>;
///configure i2c slave address
pub mod sar_slave_addr1;
/**SAR_SLAVE_ADDR2 (rw) register accessor: configure i2c slave address

You can [`read`](crate::generic::Reg::read) this register and get [`sar_slave_addr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_slave_addr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sar_slave_addr2`] module*/
pub type SAR_SLAVE_ADDR2 = crate::Reg<sar_slave_addr2::SAR_SLAVE_ADDR2_SPEC>;
///configure i2c slave address
pub mod sar_slave_addr2;
/**SAR_SLAVE_ADDR3 (rw) register accessor: configure i2c slave address

You can [`read`](crate::generic::Reg::read) this register and get [`sar_slave_addr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_slave_addr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sar_slave_addr3`] module*/
pub type SAR_SLAVE_ADDR3 = crate::Reg<sar_slave_addr3::SAR_SLAVE_ADDR3_SPEC>;
///configure i2c slave address
pub mod sar_slave_addr3;
/**SAR_SLAVE_ADDR4 (rw) register accessor: configure i2c slave address

You can [`read`](crate::generic::Reg::read) this register and get [`sar_slave_addr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_slave_addr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sar_slave_addr4`] module*/
pub type SAR_SLAVE_ADDR4 = crate::Reg<sar_slave_addr4::SAR_SLAVE_ADDR4_SPEC>;
///configure i2c slave address
pub mod sar_slave_addr4;
/**SAR_I2C_CTRL (rw) register accessor: configure rtc i2c controller by sw

You can [`read`](crate::generic::Reg::read) this register and get [`sar_i2c_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_i2c_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sar_i2c_ctrl`] module*/
pub type SAR_I2C_CTRL = crate::Reg<sar_i2c_ctrl::SAR_I2C_CTRL_SPEC>;
///configure rtc i2c controller by sw
pub mod sar_i2c_ctrl;
/**SAR_COCPU_INT_RAW (r) register accessor: the interrupt raw of ulp

You can [`read`](crate::generic::Reg::read) this register and get [`sar_cocpu_int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sar_cocpu_int_raw`] module*/
pub type SAR_COCPU_INT_RAW = crate::Reg<sar_cocpu_int_raw::SAR_COCPU_INT_RAW_SPEC>;
///the interrupt raw of ulp
pub mod sar_cocpu_int_raw;
/**SAR_COCPU_INT_ENA (rw) register accessor: the interrupt enable of ulp

You can [`read`](crate::generic::Reg::read) this register and get [`sar_cocpu_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_cocpu_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sar_cocpu_int_ena`] module*/
pub type SAR_COCPU_INT_ENA = crate::Reg<sar_cocpu_int_ena::SAR_COCPU_INT_ENA_SPEC>;
///the interrupt enable of ulp
pub mod sar_cocpu_int_ena;
/**SAR_COCPU_INT_ST (r) register accessor: the interrupt state of ulp

You can [`read`](crate::generic::Reg::read) this register and get [`sar_cocpu_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sar_cocpu_int_st`] module*/
pub type SAR_COCPU_INT_ST = crate::Reg<sar_cocpu_int_st::SAR_COCPU_INT_ST_SPEC>;
///the interrupt state of ulp
pub mod sar_cocpu_int_st;
/**SAR_COCPU_INT_CLR (w) register accessor: the interrupt clear of ulp

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_cocpu_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sar_cocpu_int_clr`] module*/
pub type SAR_COCPU_INT_CLR = crate::Reg<sar_cocpu_int_clr::SAR_COCPU_INT_CLR_SPEC>;
///the interrupt clear of ulp
pub mod sar_cocpu_int_clr;
