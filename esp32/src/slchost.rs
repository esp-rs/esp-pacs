#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    host_slchost_func2_0: HOST_SLCHOST_FUNC2_0,
    host_slchost_func2_1: HOST_SLCHOST_FUNC2_1,
    _reserved2: [u8; 0x08],
    host_slchost_func2_2: HOST_SLCHOST_FUNC2_2,
    _reserved3: [u8; 0x10],
    host_slchost_gpio_status0: HOST_SLCHOST_GPIO_STATUS0,
    host_slchost_gpio_status1: HOST_SLCHOST_GPIO_STATUS1,
    host_slchost_gpio_in0: HOST_SLCHOST_GPIO_IN0,
    host_slchost_gpio_in1: HOST_SLCHOST_GPIO_IN1,
    host_slc0host_token_rdata: HOST_SLC0HOST_TOKEN_RDATA,
    host_slc0_host_pf: HOST_SLC0_HOST_PF,
    host_slc1_host_pf: HOST_SLC1_HOST_PF,
    host_slc0host_int_raw: HOST_SLC0HOST_INT_RAW,
    host_slc1host_int_raw: HOST_SLC1HOST_INT_RAW,
    host_slc0host_int_st: HOST_SLC0HOST_INT_ST,
    host_slc1host_int_st: HOST_SLC1HOST_INT_ST,
    host_slchost_pkt_len: HOST_SLCHOST_PKT_LEN,
    host_slchost_state_w0: HOST_SLCHOST_STATE_W0,
    host_slchost_state_w1: HOST_SLCHOST_STATE_W1,
    host_slchost_conf_w0: HOST_SLCHOST_CONF_W0,
    host_slchost_conf_w1: HOST_SLCHOST_CONF_W1,
    host_slchost_conf_w2: HOST_SLCHOST_CONF_W2,
    host_slchost_conf_w3: HOST_SLCHOST_CONF_W3,
    host_slchost_conf_w4: HOST_SLCHOST_CONF_W4,
    host_slchost_conf_w5: HOST_SLCHOST_CONF_W5,
    host_slchost_win_cmd: HOST_SLCHOST_WIN_CMD,
    host_slchost_conf_w6: HOST_SLCHOST_CONF_W6,
    host_slchost_conf_w7: HOST_SLCHOST_CONF_W7,
    host_slchost_pkt_len0: HOST_SLCHOST_PKT_LEN0,
    host_slchost_pkt_len1: HOST_SLCHOST_PKT_LEN1,
    host_slchost_pkt_len2: HOST_SLCHOST_PKT_LEN2,
    host_slchost_conf_w8: HOST_SLCHOST_CONF_W8,
    host_slchost_conf_w9: HOST_SLCHOST_CONF_W9,
    host_slchost_conf_w10: HOST_SLCHOST_CONF_W10,
    host_slchost_conf_w11: HOST_SLCHOST_CONF_W11,
    host_slchost_conf_w12: HOST_SLCHOST_CONF_W12,
    host_slchost_conf_w13: HOST_SLCHOST_CONF_W13,
    host_slchost_conf_w14: HOST_SLCHOST_CONF_W14,
    host_slchost_conf_w15: HOST_SLCHOST_CONF_W15,
    host_slchost_check_sum0: HOST_SLCHOST_CHECK_SUM0,
    host_slchost_check_sum1: HOST_SLCHOST_CHECK_SUM1,
    host_slc1host_token_rdata: HOST_SLC1HOST_TOKEN_RDATA,
    host_slc0host_token_wdata: HOST_SLC0HOST_TOKEN_WDATA,
    host_slc1host_token_wdata: HOST_SLC1HOST_TOKEN_WDATA,
    host_slchost_token_con: HOST_SLCHOST_TOKEN_CON,
    host_slc0host_int_clr: HOST_SLC0HOST_INT_CLR,
    host_slc1host_int_clr: HOST_SLC1HOST_INT_CLR,
    host_slc0host_func1_int_ena: HOST_SLC0HOST_FUNC1_INT_ENA,
    host_slc1host_func1_int_ena: HOST_SLC1HOST_FUNC1_INT_ENA,
    host_slc0host_func2_int_ena: HOST_SLC0HOST_FUNC2_INT_ENA,
    host_slc1host_func2_int_ena: HOST_SLC1HOST_FUNC2_INT_ENA,
    host_slc0host_int_ena: HOST_SLC0HOST_INT_ENA,
    host_slc1host_int_ena: HOST_SLC1HOST_INT_ENA,
    host_slc0host_rx_infor: HOST_SLC0HOST_RX_INFOR,
    host_slc1host_rx_infor: HOST_SLC1HOST_RX_INFOR,
    host_slc0host_len_wd: HOST_SLC0HOST_LEN_WD,
    host_slc_apbwin_wdata: HOST_SLC_APBWIN_WDATA,
    host_slc_apbwin_conf: HOST_SLC_APBWIN_CONF,
    host_slc_apbwin_rdata: HOST_SLC_APBWIN_RDATA,
    host_slchost_rdclr0: HOST_SLCHOST_RDCLR0,
    host_slchost_rdclr1: HOST_SLCHOST_RDCLR1,
    host_slc0host_int_ena1: HOST_SLC0HOST_INT_ENA1,
    host_slc1host_int_ena1: HOST_SLC1HOST_INT_ENA1,
    _reserved61: [u8; 0x5c],
    host_slchostdate: HOST_SLCHOSTDATE,
    host_slchostid: HOST_SLCHOSTID,
    _reserved63: [u8; 0x70],
    host_slchost_conf: HOST_SLCHOST_CONF,
    host_slchost_inf_st: HOST_SLCHOST_INF_ST,
}
impl RegisterBlock {
    ///0x10 -
    #[inline(always)]
    pub const fn host_slchost_func2_0(&self) -> &HOST_SLCHOST_FUNC2_0 {
        &self.host_slchost_func2_0
    }
    ///0x14 -
    #[inline(always)]
    pub const fn host_slchost_func2_1(&self) -> &HOST_SLCHOST_FUNC2_1 {
        &self.host_slchost_func2_1
    }
    ///0x20 -
    #[inline(always)]
    pub const fn host_slchost_func2_2(&self) -> &HOST_SLCHOST_FUNC2_2 {
        &self.host_slchost_func2_2
    }
    ///0x34 -
    #[inline(always)]
    pub const fn host_slchost_gpio_status0(&self) -> &HOST_SLCHOST_GPIO_STATUS0 {
        &self.host_slchost_gpio_status0
    }
    ///0x38 -
    #[inline(always)]
    pub const fn host_slchost_gpio_status1(&self) -> &HOST_SLCHOST_GPIO_STATUS1 {
        &self.host_slchost_gpio_status1
    }
    ///0x3c -
    #[inline(always)]
    pub const fn host_slchost_gpio_in0(&self) -> &HOST_SLCHOST_GPIO_IN0 {
        &self.host_slchost_gpio_in0
    }
    ///0x40 -
    #[inline(always)]
    pub const fn host_slchost_gpio_in1(&self) -> &HOST_SLCHOST_GPIO_IN1 {
        &self.host_slchost_gpio_in1
    }
    ///0x44 -
    #[inline(always)]
    pub const fn host_slc0host_token_rdata(&self) -> &HOST_SLC0HOST_TOKEN_RDATA {
        &self.host_slc0host_token_rdata
    }
    ///0x48 -
    #[inline(always)]
    pub const fn host_slc0_host_pf(&self) -> &HOST_SLC0_HOST_PF {
        &self.host_slc0_host_pf
    }
    ///0x4c -
    #[inline(always)]
    pub const fn host_slc1_host_pf(&self) -> &HOST_SLC1_HOST_PF {
        &self.host_slc1_host_pf
    }
    ///0x50 -
    #[inline(always)]
    pub const fn host_slc0host_int_raw(&self) -> &HOST_SLC0HOST_INT_RAW {
        &self.host_slc0host_int_raw
    }
    ///0x54 -
    #[inline(always)]
    pub const fn host_slc1host_int_raw(&self) -> &HOST_SLC1HOST_INT_RAW {
        &self.host_slc1host_int_raw
    }
    ///0x58 -
    #[inline(always)]
    pub const fn host_slc0host_int_st(&self) -> &HOST_SLC0HOST_INT_ST {
        &self.host_slc0host_int_st
    }
    ///0x5c -
    #[inline(always)]
    pub const fn host_slc1host_int_st(&self) -> &HOST_SLC1HOST_INT_ST {
        &self.host_slc1host_int_st
    }
    ///0x60 -
    #[inline(always)]
    pub const fn host_slchost_pkt_len(&self) -> &HOST_SLCHOST_PKT_LEN {
        &self.host_slchost_pkt_len
    }
    ///0x64 -
    #[inline(always)]
    pub const fn host_slchost_state_w0(&self) -> &HOST_SLCHOST_STATE_W0 {
        &self.host_slchost_state_w0
    }
    ///0x68 -
    #[inline(always)]
    pub const fn host_slchost_state_w1(&self) -> &HOST_SLCHOST_STATE_W1 {
        &self.host_slchost_state_w1
    }
    ///0x6c -
    #[inline(always)]
    pub const fn host_slchost_conf_w0(&self) -> &HOST_SLCHOST_CONF_W0 {
        &self.host_slchost_conf_w0
    }
    ///0x70 -
    #[inline(always)]
    pub const fn host_slchost_conf_w1(&self) -> &HOST_SLCHOST_CONF_W1 {
        &self.host_slchost_conf_w1
    }
    ///0x74 -
    #[inline(always)]
    pub const fn host_slchost_conf_w2(&self) -> &HOST_SLCHOST_CONF_W2 {
        &self.host_slchost_conf_w2
    }
    ///0x78 -
    #[inline(always)]
    pub const fn host_slchost_conf_w3(&self) -> &HOST_SLCHOST_CONF_W3 {
        &self.host_slchost_conf_w3
    }
    ///0x7c -
    #[inline(always)]
    pub const fn host_slchost_conf_w4(&self) -> &HOST_SLCHOST_CONF_W4 {
        &self.host_slchost_conf_w4
    }
    ///0x80 -
    #[inline(always)]
    pub const fn host_slchost_conf_w5(&self) -> &HOST_SLCHOST_CONF_W5 {
        &self.host_slchost_conf_w5
    }
    ///0x84 -
    #[inline(always)]
    pub const fn host_slchost_win_cmd(&self) -> &HOST_SLCHOST_WIN_CMD {
        &self.host_slchost_win_cmd
    }
    ///0x88 -
    #[inline(always)]
    pub const fn host_slchost_conf_w6(&self) -> &HOST_SLCHOST_CONF_W6 {
        &self.host_slchost_conf_w6
    }
    ///0x8c -
    #[inline(always)]
    pub const fn host_slchost_conf_w7(&self) -> &HOST_SLCHOST_CONF_W7 {
        &self.host_slchost_conf_w7
    }
    ///0x90 -
    #[inline(always)]
    pub const fn host_slchost_pkt_len0(&self) -> &HOST_SLCHOST_PKT_LEN0 {
        &self.host_slchost_pkt_len0
    }
    ///0x94 -
    #[inline(always)]
    pub const fn host_slchost_pkt_len1(&self) -> &HOST_SLCHOST_PKT_LEN1 {
        &self.host_slchost_pkt_len1
    }
    ///0x98 -
    #[inline(always)]
    pub const fn host_slchost_pkt_len2(&self) -> &HOST_SLCHOST_PKT_LEN2 {
        &self.host_slchost_pkt_len2
    }
    ///0x9c -
    #[inline(always)]
    pub const fn host_slchost_conf_w8(&self) -> &HOST_SLCHOST_CONF_W8 {
        &self.host_slchost_conf_w8
    }
    ///0xa0 -
    #[inline(always)]
    pub const fn host_slchost_conf_w9(&self) -> &HOST_SLCHOST_CONF_W9 {
        &self.host_slchost_conf_w9
    }
    ///0xa4 -
    #[inline(always)]
    pub const fn host_slchost_conf_w10(&self) -> &HOST_SLCHOST_CONF_W10 {
        &self.host_slchost_conf_w10
    }
    ///0xa8 -
    #[inline(always)]
    pub const fn host_slchost_conf_w11(&self) -> &HOST_SLCHOST_CONF_W11 {
        &self.host_slchost_conf_w11
    }
    ///0xac -
    #[inline(always)]
    pub const fn host_slchost_conf_w12(&self) -> &HOST_SLCHOST_CONF_W12 {
        &self.host_slchost_conf_w12
    }
    ///0xb0 -
    #[inline(always)]
    pub const fn host_slchost_conf_w13(&self) -> &HOST_SLCHOST_CONF_W13 {
        &self.host_slchost_conf_w13
    }
    ///0xb4 -
    #[inline(always)]
    pub const fn host_slchost_conf_w14(&self) -> &HOST_SLCHOST_CONF_W14 {
        &self.host_slchost_conf_w14
    }
    ///0xb8 -
    #[inline(always)]
    pub const fn host_slchost_conf_w15(&self) -> &HOST_SLCHOST_CONF_W15 {
        &self.host_slchost_conf_w15
    }
    ///0xbc -
    #[inline(always)]
    pub const fn host_slchost_check_sum0(&self) -> &HOST_SLCHOST_CHECK_SUM0 {
        &self.host_slchost_check_sum0
    }
    ///0xc0 -
    #[inline(always)]
    pub const fn host_slchost_check_sum1(&self) -> &HOST_SLCHOST_CHECK_SUM1 {
        &self.host_slchost_check_sum1
    }
    ///0xc4 -
    #[inline(always)]
    pub const fn host_slc1host_token_rdata(&self) -> &HOST_SLC1HOST_TOKEN_RDATA {
        &self.host_slc1host_token_rdata
    }
    ///0xc8 -
    #[inline(always)]
    pub const fn host_slc0host_token_wdata(&self) -> &HOST_SLC0HOST_TOKEN_WDATA {
        &self.host_slc0host_token_wdata
    }
    ///0xcc -
    #[inline(always)]
    pub const fn host_slc1host_token_wdata(&self) -> &HOST_SLC1HOST_TOKEN_WDATA {
        &self.host_slc1host_token_wdata
    }
    ///0xd0 -
    #[inline(always)]
    pub const fn host_slchost_token_con(&self) -> &HOST_SLCHOST_TOKEN_CON {
        &self.host_slchost_token_con
    }
    ///0xd4 -
    #[inline(always)]
    pub const fn host_slc0host_int_clr(&self) -> &HOST_SLC0HOST_INT_CLR {
        &self.host_slc0host_int_clr
    }
    ///0xd8 -
    #[inline(always)]
    pub const fn host_slc1host_int_clr(&self) -> &HOST_SLC1HOST_INT_CLR {
        &self.host_slc1host_int_clr
    }
    ///0xdc -
    #[inline(always)]
    pub const fn host_slc0host_func1_int_ena(&self) -> &HOST_SLC0HOST_FUNC1_INT_ENA {
        &self.host_slc0host_func1_int_ena
    }
    ///0xe0 -
    #[inline(always)]
    pub const fn host_slc1host_func1_int_ena(&self) -> &HOST_SLC1HOST_FUNC1_INT_ENA {
        &self.host_slc1host_func1_int_ena
    }
    ///0xe4 -
    #[inline(always)]
    pub const fn host_slc0host_func2_int_ena(&self) -> &HOST_SLC0HOST_FUNC2_INT_ENA {
        &self.host_slc0host_func2_int_ena
    }
    ///0xe8 -
    #[inline(always)]
    pub const fn host_slc1host_func2_int_ena(&self) -> &HOST_SLC1HOST_FUNC2_INT_ENA {
        &self.host_slc1host_func2_int_ena
    }
    ///0xec -
    #[inline(always)]
    pub const fn host_slc0host_int_ena(&self) -> &HOST_SLC0HOST_INT_ENA {
        &self.host_slc0host_int_ena
    }
    ///0xf0 -
    #[inline(always)]
    pub const fn host_slc1host_int_ena(&self) -> &HOST_SLC1HOST_INT_ENA {
        &self.host_slc1host_int_ena
    }
    ///0xf4 -
    #[inline(always)]
    pub const fn host_slc0host_rx_infor(&self) -> &HOST_SLC0HOST_RX_INFOR {
        &self.host_slc0host_rx_infor
    }
    ///0xf8 -
    #[inline(always)]
    pub const fn host_slc1host_rx_infor(&self) -> &HOST_SLC1HOST_RX_INFOR {
        &self.host_slc1host_rx_infor
    }
    ///0xfc -
    #[inline(always)]
    pub const fn host_slc0host_len_wd(&self) -> &HOST_SLC0HOST_LEN_WD {
        &self.host_slc0host_len_wd
    }
    ///0x100 -
    #[inline(always)]
    pub const fn host_slc_apbwin_wdata(&self) -> &HOST_SLC_APBWIN_WDATA {
        &self.host_slc_apbwin_wdata
    }
    ///0x104 -
    #[inline(always)]
    pub const fn host_slc_apbwin_conf(&self) -> &HOST_SLC_APBWIN_CONF {
        &self.host_slc_apbwin_conf
    }
    ///0x108 -
    #[inline(always)]
    pub const fn host_slc_apbwin_rdata(&self) -> &HOST_SLC_APBWIN_RDATA {
        &self.host_slc_apbwin_rdata
    }
    ///0x10c -
    #[inline(always)]
    pub const fn host_slchost_rdclr0(&self) -> &HOST_SLCHOST_RDCLR0 {
        &self.host_slchost_rdclr0
    }
    ///0x110 -
    #[inline(always)]
    pub const fn host_slchost_rdclr1(&self) -> &HOST_SLCHOST_RDCLR1 {
        &self.host_slchost_rdclr1
    }
    ///0x114 -
    #[inline(always)]
    pub const fn host_slc0host_int_ena1(&self) -> &HOST_SLC0HOST_INT_ENA1 {
        &self.host_slc0host_int_ena1
    }
    ///0x118 -
    #[inline(always)]
    pub const fn host_slc1host_int_ena1(&self) -> &HOST_SLC1HOST_INT_ENA1 {
        &self.host_slc1host_int_ena1
    }
    ///0x178 -
    #[inline(always)]
    pub const fn host_slchostdate(&self) -> &HOST_SLCHOSTDATE {
        &self.host_slchostdate
    }
    ///0x17c -
    #[inline(always)]
    pub const fn host_slchostid(&self) -> &HOST_SLCHOSTID {
        &self.host_slchostid
    }
    ///0x1f0 -
    #[inline(always)]
    pub const fn host_slchost_conf(&self) -> &HOST_SLCHOST_CONF {
        &self.host_slchost_conf
    }
    ///0x1f4 -
    #[inline(always)]
    pub const fn host_slchost_inf_st(&self) -> &HOST_SLCHOST_INF_ST {
        &self.host_slchost_inf_st
    }
}
/**HOST_SLCHOST_FUNC2_0 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_func2_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_func2_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_func2_0`] module*/
pub type HOST_SLCHOST_FUNC2_0 = crate::Reg<host_slchost_func2_0::HOST_SLCHOST_FUNC2_0_SPEC>;
///
pub mod host_slchost_func2_0;
/**HOST_SLCHOST_FUNC2_1 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_func2_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_func2_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_func2_1`] module*/
pub type HOST_SLCHOST_FUNC2_1 = crate::Reg<host_slchost_func2_1::HOST_SLCHOST_FUNC2_1_SPEC>;
///
pub mod host_slchost_func2_1;
/**HOST_SLCHOST_FUNC2_2 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_func2_2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_func2_2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_func2_2`] module*/
pub type HOST_SLCHOST_FUNC2_2 = crate::Reg<host_slchost_func2_2::HOST_SLCHOST_FUNC2_2_SPEC>;
///
pub mod host_slchost_func2_2;
/**HOST_SLCHOST_GPIO_STATUS0 (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_gpio_status0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_gpio_status0`] module*/
pub type HOST_SLCHOST_GPIO_STATUS0 =
    crate::Reg<host_slchost_gpio_status0::HOST_SLCHOST_GPIO_STATUS0_SPEC>;
///
pub mod host_slchost_gpio_status0;
/**HOST_SLCHOST_GPIO_STATUS1 (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_gpio_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_gpio_status1`] module*/
pub type HOST_SLCHOST_GPIO_STATUS1 =
    crate::Reg<host_slchost_gpio_status1::HOST_SLCHOST_GPIO_STATUS1_SPEC>;
///
pub mod host_slchost_gpio_status1;
/**HOST_SLCHOST_GPIO_IN0 (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_gpio_in0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_gpio_in0`] module*/
pub type HOST_SLCHOST_GPIO_IN0 = crate::Reg<host_slchost_gpio_in0::HOST_SLCHOST_GPIO_IN0_SPEC>;
///
pub mod host_slchost_gpio_in0;
/**HOST_SLCHOST_GPIO_IN1 (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_gpio_in1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_gpio_in1`] module*/
pub type HOST_SLCHOST_GPIO_IN1 = crate::Reg<host_slchost_gpio_in1::HOST_SLCHOST_GPIO_IN1_SPEC>;
///
pub mod host_slchost_gpio_in1;
/**HOST_SLC0HOST_TOKEN_RDATA (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slc0host_token_rdata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slc0host_token_rdata`] module*/
pub type HOST_SLC0HOST_TOKEN_RDATA =
    crate::Reg<host_slc0host_token_rdata::HOST_SLC0HOST_TOKEN_RDATA_SPEC>;
///
pub mod host_slc0host_token_rdata;
/**HOST_SLC0_HOST_PF (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slc0_host_pf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slc0_host_pf`] module*/
pub type HOST_SLC0_HOST_PF = crate::Reg<host_slc0_host_pf::HOST_SLC0_HOST_PF_SPEC>;
///
pub mod host_slc0_host_pf;
/**HOST_SLC1_HOST_PF (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slc1_host_pf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slc1_host_pf`] module*/
pub type HOST_SLC1_HOST_PF = crate::Reg<host_slc1_host_pf::HOST_SLC1_HOST_PF_SPEC>;
///
pub mod host_slc1_host_pf;
/**HOST_SLC0HOST_INT_RAW (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slc0host_int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slc0host_int_raw`] module*/
pub type HOST_SLC0HOST_INT_RAW = crate::Reg<host_slc0host_int_raw::HOST_SLC0HOST_INT_RAW_SPEC>;
///
pub mod host_slc0host_int_raw;
/**HOST_SLC1HOST_INT_RAW (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slc1host_int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slc1host_int_raw`] module*/
pub type HOST_SLC1HOST_INT_RAW = crate::Reg<host_slc1host_int_raw::HOST_SLC1HOST_INT_RAW_SPEC>;
///
pub mod host_slc1host_int_raw;
/**HOST_SLC0HOST_INT_ST (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slc0host_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slc0host_int_st`] module*/
pub type HOST_SLC0HOST_INT_ST = crate::Reg<host_slc0host_int_st::HOST_SLC0HOST_INT_ST_SPEC>;
///
pub mod host_slc0host_int_st;
/**HOST_SLC1HOST_INT_ST (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slc1host_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slc1host_int_st`] module*/
pub type HOST_SLC1HOST_INT_ST = crate::Reg<host_slc1host_int_st::HOST_SLC1HOST_INT_ST_SPEC>;
///
pub mod host_slc1host_int_st;
/**HOST_SLCHOST_PKT_LEN (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_pkt_len::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_pkt_len`] module*/
pub type HOST_SLCHOST_PKT_LEN = crate::Reg<host_slchost_pkt_len::HOST_SLCHOST_PKT_LEN_SPEC>;
///
pub mod host_slchost_pkt_len;
/**HOST_SLCHOST_STATE_W0 (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_state_w0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_state_w0`] module*/
pub type HOST_SLCHOST_STATE_W0 = crate::Reg<host_slchost_state_w0::HOST_SLCHOST_STATE_W0_SPEC>;
///
pub mod host_slchost_state_w0;
/**HOST_SLCHOST_STATE_W1 (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_state_w1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_state_w1`] module*/
pub type HOST_SLCHOST_STATE_W1 = crate::Reg<host_slchost_state_w1::HOST_SLCHOST_STATE_W1_SPEC>;
///
pub mod host_slchost_state_w1;
/**HOST_SLCHOST_CONF_W0 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf_w0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf_w0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_conf_w0`] module*/
pub type HOST_SLCHOST_CONF_W0 = crate::Reg<host_slchost_conf_w0::HOST_SLCHOST_CONF_W0_SPEC>;
///
pub mod host_slchost_conf_w0;
/**HOST_SLCHOST_CONF_W1 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf_w1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf_w1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_conf_w1`] module*/
pub type HOST_SLCHOST_CONF_W1 = crate::Reg<host_slchost_conf_w1::HOST_SLCHOST_CONF_W1_SPEC>;
///
pub mod host_slchost_conf_w1;
/**HOST_SLCHOST_CONF_W2 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf_w2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf_w2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_conf_w2`] module*/
pub type HOST_SLCHOST_CONF_W2 = crate::Reg<host_slchost_conf_w2::HOST_SLCHOST_CONF_W2_SPEC>;
///
pub mod host_slchost_conf_w2;
/**HOST_SLCHOST_CONF_W3 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf_w3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf_w3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_conf_w3`] module*/
pub type HOST_SLCHOST_CONF_W3 = crate::Reg<host_slchost_conf_w3::HOST_SLCHOST_CONF_W3_SPEC>;
///
pub mod host_slchost_conf_w3;
/**HOST_SLCHOST_CONF_W4 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf_w4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf_w4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_conf_w4`] module*/
pub type HOST_SLCHOST_CONF_W4 = crate::Reg<host_slchost_conf_w4::HOST_SLCHOST_CONF_W4_SPEC>;
///
pub mod host_slchost_conf_w4;
/**HOST_SLCHOST_CONF_W5 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf_w5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf_w5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_conf_w5`] module*/
pub type HOST_SLCHOST_CONF_W5 = crate::Reg<host_slchost_conf_w5::HOST_SLCHOST_CONF_W5_SPEC>;
///
pub mod host_slchost_conf_w5;
/**HOST_SLCHOST_WIN_CMD (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_win_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_win_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_win_cmd`] module*/
pub type HOST_SLCHOST_WIN_CMD = crate::Reg<host_slchost_win_cmd::HOST_SLCHOST_WIN_CMD_SPEC>;
///
pub mod host_slchost_win_cmd;
/**HOST_SLCHOST_CONF_W6 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf_w6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf_w6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_conf_w6`] module*/
pub type HOST_SLCHOST_CONF_W6 = crate::Reg<host_slchost_conf_w6::HOST_SLCHOST_CONF_W6_SPEC>;
///
pub mod host_slchost_conf_w6;
/**HOST_SLCHOST_CONF_W7 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf_w7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf_w7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_conf_w7`] module*/
pub type HOST_SLCHOST_CONF_W7 = crate::Reg<host_slchost_conf_w7::HOST_SLCHOST_CONF_W7_SPEC>;
///
pub mod host_slchost_conf_w7;
/**HOST_SLCHOST_PKT_LEN0 (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_pkt_len0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_pkt_len0`] module*/
pub type HOST_SLCHOST_PKT_LEN0 = crate::Reg<host_slchost_pkt_len0::HOST_SLCHOST_PKT_LEN0_SPEC>;
///
pub mod host_slchost_pkt_len0;
/**HOST_SLCHOST_PKT_LEN1 (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_pkt_len1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_pkt_len1`] module*/
pub type HOST_SLCHOST_PKT_LEN1 = crate::Reg<host_slchost_pkt_len1::HOST_SLCHOST_PKT_LEN1_SPEC>;
///
pub mod host_slchost_pkt_len1;
/**HOST_SLCHOST_PKT_LEN2 (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_pkt_len2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_pkt_len2`] module*/
pub type HOST_SLCHOST_PKT_LEN2 = crate::Reg<host_slchost_pkt_len2::HOST_SLCHOST_PKT_LEN2_SPEC>;
///
pub mod host_slchost_pkt_len2;
/**HOST_SLCHOST_CONF_W8 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf_w8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf_w8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_conf_w8`] module*/
pub type HOST_SLCHOST_CONF_W8 = crate::Reg<host_slchost_conf_w8::HOST_SLCHOST_CONF_W8_SPEC>;
///
pub mod host_slchost_conf_w8;
/**HOST_SLCHOST_CONF_W9 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf_w9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf_w9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_conf_w9`] module*/
pub type HOST_SLCHOST_CONF_W9 = crate::Reg<host_slchost_conf_w9::HOST_SLCHOST_CONF_W9_SPEC>;
///
pub mod host_slchost_conf_w9;
/**HOST_SLCHOST_CONF_W10 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf_w10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf_w10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_conf_w10`] module*/
pub type HOST_SLCHOST_CONF_W10 = crate::Reg<host_slchost_conf_w10::HOST_SLCHOST_CONF_W10_SPEC>;
///
pub mod host_slchost_conf_w10;
/**HOST_SLCHOST_CONF_W11 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf_w11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf_w11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_conf_w11`] module*/
pub type HOST_SLCHOST_CONF_W11 = crate::Reg<host_slchost_conf_w11::HOST_SLCHOST_CONF_W11_SPEC>;
///
pub mod host_slchost_conf_w11;
/**HOST_SLCHOST_CONF_W12 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf_w12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf_w12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_conf_w12`] module*/
pub type HOST_SLCHOST_CONF_W12 = crate::Reg<host_slchost_conf_w12::HOST_SLCHOST_CONF_W12_SPEC>;
///
pub mod host_slchost_conf_w12;
/**HOST_SLCHOST_CONF_W13 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf_w13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf_w13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_conf_w13`] module*/
pub type HOST_SLCHOST_CONF_W13 = crate::Reg<host_slchost_conf_w13::HOST_SLCHOST_CONF_W13_SPEC>;
///
pub mod host_slchost_conf_w13;
/**HOST_SLCHOST_CONF_W14 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf_w14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf_w14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_conf_w14`] module*/
pub type HOST_SLCHOST_CONF_W14 = crate::Reg<host_slchost_conf_w14::HOST_SLCHOST_CONF_W14_SPEC>;
///
pub mod host_slchost_conf_w14;
/**HOST_SLCHOST_CONF_W15 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf_w15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf_w15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_conf_w15`] module*/
pub type HOST_SLCHOST_CONF_W15 = crate::Reg<host_slchost_conf_w15::HOST_SLCHOST_CONF_W15_SPEC>;
///
pub mod host_slchost_conf_w15;
/**HOST_SLCHOST_CHECK_SUM0 (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_check_sum0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_check_sum0`] module*/
pub type HOST_SLCHOST_CHECK_SUM0 =
    crate::Reg<host_slchost_check_sum0::HOST_SLCHOST_CHECK_SUM0_SPEC>;
///
pub mod host_slchost_check_sum0;
/**HOST_SLCHOST_CHECK_SUM1 (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_check_sum1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_check_sum1`] module*/
pub type HOST_SLCHOST_CHECK_SUM1 =
    crate::Reg<host_slchost_check_sum1::HOST_SLCHOST_CHECK_SUM1_SPEC>;
///
pub mod host_slchost_check_sum1;
/**HOST_SLC1HOST_TOKEN_RDATA (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slc1host_token_rdata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slc1host_token_rdata`] module*/
pub type HOST_SLC1HOST_TOKEN_RDATA =
    crate::Reg<host_slc1host_token_rdata::HOST_SLC1HOST_TOKEN_RDATA_SPEC>;
///
pub mod host_slc1host_token_rdata;
/**HOST_SLC0HOST_TOKEN_WDATA (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slc0host_token_wdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slc0host_token_wdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slc0host_token_wdata`] module*/
pub type HOST_SLC0HOST_TOKEN_WDATA =
    crate::Reg<host_slc0host_token_wdata::HOST_SLC0HOST_TOKEN_WDATA_SPEC>;
///
pub mod host_slc0host_token_wdata;
/**HOST_SLC1HOST_TOKEN_WDATA (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slc1host_token_wdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slc1host_token_wdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slc1host_token_wdata`] module*/
pub type HOST_SLC1HOST_TOKEN_WDATA =
    crate::Reg<host_slc1host_token_wdata::HOST_SLC1HOST_TOKEN_WDATA_SPEC>;
///
pub mod host_slc1host_token_wdata;
/**HOST_SLCHOST_TOKEN_CON (w) register accessor:

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_token_con::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_token_con`] module*/
pub type HOST_SLCHOST_TOKEN_CON = crate::Reg<host_slchost_token_con::HOST_SLCHOST_TOKEN_CON_SPEC>;
///
pub mod host_slchost_token_con;
/**HOST_SLC0HOST_INT_CLR (w) register accessor:

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slc0host_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slc0host_int_clr`] module*/
pub type HOST_SLC0HOST_INT_CLR = crate::Reg<host_slc0host_int_clr::HOST_SLC0HOST_INT_CLR_SPEC>;
///
pub mod host_slc0host_int_clr;
/**HOST_SLC1HOST_INT_CLR (w) register accessor:

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slc1host_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slc1host_int_clr`] module*/
pub type HOST_SLC1HOST_INT_CLR = crate::Reg<host_slc1host_int_clr::HOST_SLC1HOST_INT_CLR_SPEC>;
///
pub mod host_slc1host_int_clr;
/**HOST_SLC0HOST_FUNC1_INT_ENA (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slc0host_func1_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slc0host_func1_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slc0host_func1_int_ena`] module*/
pub type HOST_SLC0HOST_FUNC1_INT_ENA =
    crate::Reg<host_slc0host_func1_int_ena::HOST_SLC0HOST_FUNC1_INT_ENA_SPEC>;
///
pub mod host_slc0host_func1_int_ena;
/**HOST_SLC1HOST_FUNC1_INT_ENA (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slc1host_func1_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slc1host_func1_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slc1host_func1_int_ena`] module*/
pub type HOST_SLC1HOST_FUNC1_INT_ENA =
    crate::Reg<host_slc1host_func1_int_ena::HOST_SLC1HOST_FUNC1_INT_ENA_SPEC>;
///
pub mod host_slc1host_func1_int_ena;
/**HOST_SLC0HOST_FUNC2_INT_ENA (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slc0host_func2_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slc0host_func2_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slc0host_func2_int_ena`] module*/
pub type HOST_SLC0HOST_FUNC2_INT_ENA =
    crate::Reg<host_slc0host_func2_int_ena::HOST_SLC0HOST_FUNC2_INT_ENA_SPEC>;
///
pub mod host_slc0host_func2_int_ena;
/**HOST_SLC1HOST_FUNC2_INT_ENA (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slc1host_func2_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slc1host_func2_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slc1host_func2_int_ena`] module*/
pub type HOST_SLC1HOST_FUNC2_INT_ENA =
    crate::Reg<host_slc1host_func2_int_ena::HOST_SLC1HOST_FUNC2_INT_ENA_SPEC>;
///
pub mod host_slc1host_func2_int_ena;
/**HOST_SLC0HOST_INT_ENA (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slc0host_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slc0host_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slc0host_int_ena`] module*/
pub type HOST_SLC0HOST_INT_ENA = crate::Reg<host_slc0host_int_ena::HOST_SLC0HOST_INT_ENA_SPEC>;
///
pub mod host_slc0host_int_ena;
/**HOST_SLC1HOST_INT_ENA (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slc1host_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slc1host_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slc1host_int_ena`] module*/
pub type HOST_SLC1HOST_INT_ENA = crate::Reg<host_slc1host_int_ena::HOST_SLC1HOST_INT_ENA_SPEC>;
///
pub mod host_slc1host_int_ena;
/**HOST_SLC0HOST_RX_INFOR (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slc0host_rx_infor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slc0host_rx_infor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slc0host_rx_infor`] module*/
pub type HOST_SLC0HOST_RX_INFOR = crate::Reg<host_slc0host_rx_infor::HOST_SLC0HOST_RX_INFOR_SPEC>;
///
pub mod host_slc0host_rx_infor;
/**HOST_SLC1HOST_RX_INFOR (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slc1host_rx_infor::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slc1host_rx_infor::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slc1host_rx_infor`] module*/
pub type HOST_SLC1HOST_RX_INFOR = crate::Reg<host_slc1host_rx_infor::HOST_SLC1HOST_RX_INFOR_SPEC>;
///
pub mod host_slc1host_rx_infor;
/**HOST_SLC0HOST_LEN_WD (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slc0host_len_wd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slc0host_len_wd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slc0host_len_wd`] module*/
pub type HOST_SLC0HOST_LEN_WD = crate::Reg<host_slc0host_len_wd::HOST_SLC0HOST_LEN_WD_SPEC>;
///
pub mod host_slc0host_len_wd;
/**HOST_SLC_APBWIN_WDATA (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slc_apbwin_wdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slc_apbwin_wdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slc_apbwin_wdata`] module*/
pub type HOST_SLC_APBWIN_WDATA = crate::Reg<host_slc_apbwin_wdata::HOST_SLC_APBWIN_WDATA_SPEC>;
///
pub mod host_slc_apbwin_wdata;
/**HOST_SLC_APBWIN_CONF (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slc_apbwin_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slc_apbwin_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slc_apbwin_conf`] module*/
pub type HOST_SLC_APBWIN_CONF = crate::Reg<host_slc_apbwin_conf::HOST_SLC_APBWIN_CONF_SPEC>;
///
pub mod host_slc_apbwin_conf;
/**HOST_SLC_APBWIN_RDATA (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slc_apbwin_rdata::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slc_apbwin_rdata`] module*/
pub type HOST_SLC_APBWIN_RDATA = crate::Reg<host_slc_apbwin_rdata::HOST_SLC_APBWIN_RDATA_SPEC>;
///
pub mod host_slc_apbwin_rdata;
/**HOST_SLCHOST_RDCLR0 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_rdclr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_rdclr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_rdclr0`] module*/
pub type HOST_SLCHOST_RDCLR0 = crate::Reg<host_slchost_rdclr0::HOST_SLCHOST_RDCLR0_SPEC>;
///
pub mod host_slchost_rdclr0;
/**HOST_SLCHOST_RDCLR1 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_rdclr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_rdclr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_rdclr1`] module*/
pub type HOST_SLCHOST_RDCLR1 = crate::Reg<host_slchost_rdclr1::HOST_SLCHOST_RDCLR1_SPEC>;
///
pub mod host_slchost_rdclr1;
/**HOST_SLC0HOST_INT_ENA1 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slc0host_int_ena1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slc0host_int_ena1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slc0host_int_ena1`] module*/
pub type HOST_SLC0HOST_INT_ENA1 = crate::Reg<host_slc0host_int_ena1::HOST_SLC0HOST_INT_ENA1_SPEC>;
///
pub mod host_slc0host_int_ena1;
/**HOST_SLC1HOST_INT_ENA1 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slc1host_int_ena1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slc1host_int_ena1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slc1host_int_ena1`] module*/
pub type HOST_SLC1HOST_INT_ENA1 = crate::Reg<host_slc1host_int_ena1::HOST_SLC1HOST_INT_ENA1_SPEC>;
///
pub mod host_slc1host_int_ena1;
/**HOST_SLCHOSTDATE (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchostdate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchostdate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchostdate`] module*/
pub type HOST_SLCHOSTDATE = crate::Reg<host_slchostdate::HOST_SLCHOSTDATE_SPEC>;
///
pub mod host_slchostdate;
/**HOST_SLCHOSTID (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchostid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchostid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchostid`] module*/
pub type HOST_SLCHOSTID = crate::Reg<host_slchostid::HOST_SLCHOSTID_SPEC>;
///
pub mod host_slchostid;
/**HOST_SLCHOST_CONF (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_conf`] module*/
pub type HOST_SLCHOST_CONF = crate::Reg<host_slchost_conf::HOST_SLCHOST_CONF_SPEC>;
///
pub mod host_slchost_conf;
/**HOST_SLCHOST_INF_ST (r) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_inf_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@host_slchost_inf_st`] module*/
pub type HOST_SLCHOST_INF_ST = crate::Reg<host_slchost_inf_st::HOST_SLCHOST_INF_ST_SPEC>;
///
pub mod host_slchost_inf_st;
