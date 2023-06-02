#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - "]
    pub host_slchost_func2_0: HOST_SLCHOST_FUNC2_0,
    #[doc = "0x14 - "]
    pub host_slchost_func2_1: HOST_SLCHOST_FUNC2_1,
    _reserved2: [u8; 0x08],
    #[doc = "0x20 - "]
    pub host_slchost_func2_2: HOST_SLCHOST_FUNC2_2,
    _reserved3: [u8; 0x10],
    #[doc = "0x34 - "]
    pub host_slchost_gpio_status0: HOST_SLCHOST_GPIO_STATUS0,
    #[doc = "0x38 - "]
    pub host_slchost_gpio_status1: HOST_SLCHOST_GPIO_STATUS1,
    #[doc = "0x3c - "]
    pub host_slchost_gpio_in0: HOST_SLCHOST_GPIO_IN0,
    #[doc = "0x40 - "]
    pub host_slchost_gpio_in1: HOST_SLCHOST_GPIO_IN1,
    #[doc = "0x44 - "]
    pub host_slc0host_token_rdata: HOST_SLC0HOST_TOKEN_RDATA,
    #[doc = "0x48 - "]
    pub host_slc0_host_pf: HOST_SLC0_HOST_PF,
    #[doc = "0x4c - "]
    pub host_slc1_host_pf: HOST_SLC1_HOST_PF,
    #[doc = "0x50 - "]
    pub host_slc0host_int_raw: HOST_SLC0HOST_INT_RAW,
    #[doc = "0x54 - "]
    pub host_slc1host_int_raw: HOST_SLC1HOST_INT_RAW,
    #[doc = "0x58 - "]
    pub host_slc0host_int_st: HOST_SLC0HOST_INT_ST,
    #[doc = "0x5c - "]
    pub host_slc1host_int_st: HOST_SLC1HOST_INT_ST,
    #[doc = "0x60 - "]
    pub host_slchost_pkt_len: HOST_SLCHOST_PKT_LEN,
    #[doc = "0x64 - "]
    pub host_slchost_state_w0: HOST_SLCHOST_STATE_W0,
    #[doc = "0x68 - "]
    pub host_slchost_state_w1: HOST_SLCHOST_STATE_W1,
    #[doc = "0x6c - "]
    pub host_slchost_conf_w0: HOST_SLCHOST_CONF_W0,
    #[doc = "0x70 - "]
    pub host_slchost_conf_w1: HOST_SLCHOST_CONF_W1,
    #[doc = "0x74 - "]
    pub host_slchost_conf_w2: HOST_SLCHOST_CONF_W2,
    #[doc = "0x78 - "]
    pub host_slchost_conf_w3: HOST_SLCHOST_CONF_W3,
    #[doc = "0x7c - "]
    pub host_slchost_conf_w4: HOST_SLCHOST_CONF_W4,
    #[doc = "0x80 - "]
    pub host_slchost_conf_w5: HOST_SLCHOST_CONF_W5,
    #[doc = "0x84 - "]
    pub host_slchost_win_cmd: HOST_SLCHOST_WIN_CMD,
    #[doc = "0x88 - "]
    pub host_slchost_conf_w6: HOST_SLCHOST_CONF_W6,
    #[doc = "0x8c - "]
    pub host_slchost_conf_w7: HOST_SLCHOST_CONF_W7,
    #[doc = "0x90 - "]
    pub host_slchost_pkt_len0: HOST_SLCHOST_PKT_LEN0,
    #[doc = "0x94 - "]
    pub host_slchost_pkt_len1: HOST_SLCHOST_PKT_LEN1,
    #[doc = "0x98 - "]
    pub host_slchost_pkt_len2: HOST_SLCHOST_PKT_LEN2,
    #[doc = "0x9c - "]
    pub host_slchost_conf_w8: HOST_SLCHOST_CONF_W8,
    #[doc = "0xa0 - "]
    pub host_slchost_conf_w9: HOST_SLCHOST_CONF_W9,
    #[doc = "0xa4 - "]
    pub host_slchost_conf_w10: HOST_SLCHOST_CONF_W10,
    #[doc = "0xa8 - "]
    pub host_slchost_conf_w11: HOST_SLCHOST_CONF_W11,
    #[doc = "0xac - "]
    pub host_slchost_conf_w12: HOST_SLCHOST_CONF_W12,
    #[doc = "0xb0 - "]
    pub host_slchost_conf_w13: HOST_SLCHOST_CONF_W13,
    #[doc = "0xb4 - "]
    pub host_slchost_conf_w14: HOST_SLCHOST_CONF_W14,
    #[doc = "0xb8 - "]
    pub host_slchost_conf_w15: HOST_SLCHOST_CONF_W15,
    #[doc = "0xbc - "]
    pub host_slchost_check_sum0: HOST_SLCHOST_CHECK_SUM0,
    #[doc = "0xc0 - "]
    pub host_slchost_check_sum1: HOST_SLCHOST_CHECK_SUM1,
    #[doc = "0xc4 - "]
    pub host_slc1host_token_rdata: HOST_SLC1HOST_TOKEN_RDATA,
    #[doc = "0xc8 - "]
    pub host_slc0host_token_wdata: HOST_SLC0HOST_TOKEN_WDATA,
    #[doc = "0xcc - "]
    pub host_slc1host_token_wdata: HOST_SLC1HOST_TOKEN_WDATA,
    #[doc = "0xd0 - "]
    pub host_slchost_token_con: HOST_SLCHOST_TOKEN_CON,
    #[doc = "0xd4 - "]
    pub host_slc0host_int_clr: HOST_SLC0HOST_INT_CLR,
    #[doc = "0xd8 - "]
    pub host_slc1host_int_clr: HOST_SLC1HOST_INT_CLR,
    #[doc = "0xdc - "]
    pub host_slc0host_func1_int_ena: HOST_SLC0HOST_FUNC1_INT_ENA,
    #[doc = "0xe0 - "]
    pub host_slc1host_func1_int_ena: HOST_SLC1HOST_FUNC1_INT_ENA,
    #[doc = "0xe4 - "]
    pub host_slc0host_func2_int_ena: HOST_SLC0HOST_FUNC2_INT_ENA,
    #[doc = "0xe8 - "]
    pub host_slc1host_func2_int_ena: HOST_SLC1HOST_FUNC2_INT_ENA,
    #[doc = "0xec - "]
    pub host_slc0host_int_ena: HOST_SLC0HOST_INT_ENA,
    #[doc = "0xf0 - "]
    pub host_slc1host_int_ena: HOST_SLC1HOST_INT_ENA,
    #[doc = "0xf4 - "]
    pub host_slc0host_rx_infor: HOST_SLC0HOST_RX_INFOR,
    #[doc = "0xf8 - "]
    pub host_slc1host_rx_infor: HOST_SLC1HOST_RX_INFOR,
    #[doc = "0xfc - "]
    pub host_slc0host_len_wd: HOST_SLC0HOST_LEN_WD,
    #[doc = "0x100 - "]
    pub host_slc_apbwin_wdata: HOST_SLC_APBWIN_WDATA,
    #[doc = "0x104 - "]
    pub host_slc_apbwin_conf: HOST_SLC_APBWIN_CONF,
    #[doc = "0x108 - "]
    pub host_slc_apbwin_rdata: HOST_SLC_APBWIN_RDATA,
    #[doc = "0x10c - "]
    pub host_slchost_rdclr0: HOST_SLCHOST_RDCLR0,
    #[doc = "0x110 - "]
    pub host_slchost_rdclr1: HOST_SLCHOST_RDCLR1,
    #[doc = "0x114 - "]
    pub host_slc0host_int_ena1: HOST_SLC0HOST_INT_ENA1,
    #[doc = "0x118 - "]
    pub host_slc1host_int_ena1: HOST_SLC1HOST_INT_ENA1,
    _reserved61: [u8; 0x5c],
    #[doc = "0x178 - "]
    pub host_slchostdate: HOST_SLCHOSTDATE,
    #[doc = "0x17c - "]
    pub host_slchostid: HOST_SLCHOSTID,
    _reserved63: [u8; 0x70],
    #[doc = "0x1f0 - "]
    pub host_slchost_conf: HOST_SLCHOST_CONF,
    #[doc = "0x1f4 - "]
    pub host_slchost_inf_st: HOST_SLCHOST_INF_ST,
}
#[doc = "HOST_SLCHOST_FUNC2_0 (rw) register accessor: an alias for `Reg<HOST_SLCHOST_FUNC2_0_SPEC>`"]
pub type HOST_SLCHOST_FUNC2_0 = crate::Reg<host_slchost_func2_0::HOST_SLCHOST_FUNC2_0_SPEC>;
#[doc = ""]
pub mod host_slchost_func2_0;
#[doc = "HOST_SLCHOST_FUNC2_1 (rw) register accessor: an alias for `Reg<HOST_SLCHOST_FUNC2_1_SPEC>`"]
pub type HOST_SLCHOST_FUNC2_1 = crate::Reg<host_slchost_func2_1::HOST_SLCHOST_FUNC2_1_SPEC>;
#[doc = ""]
pub mod host_slchost_func2_1;
#[doc = "HOST_SLCHOST_FUNC2_2 (rw) register accessor: an alias for `Reg<HOST_SLCHOST_FUNC2_2_SPEC>`"]
pub type HOST_SLCHOST_FUNC2_2 = crate::Reg<host_slchost_func2_2::HOST_SLCHOST_FUNC2_2_SPEC>;
#[doc = ""]
pub mod host_slchost_func2_2;
#[doc = "HOST_SLCHOST_GPIO_STATUS0 (r) register accessor: an alias for `Reg<HOST_SLCHOST_GPIO_STATUS0_SPEC>`"]
pub type HOST_SLCHOST_GPIO_STATUS0 =
    crate::Reg<host_slchost_gpio_status0::HOST_SLCHOST_GPIO_STATUS0_SPEC>;
#[doc = ""]
pub mod host_slchost_gpio_status0;
#[doc = "HOST_SLCHOST_GPIO_STATUS1 (r) register accessor: an alias for `Reg<HOST_SLCHOST_GPIO_STATUS1_SPEC>`"]
pub type HOST_SLCHOST_GPIO_STATUS1 =
    crate::Reg<host_slchost_gpio_status1::HOST_SLCHOST_GPIO_STATUS1_SPEC>;
#[doc = ""]
pub mod host_slchost_gpio_status1;
#[doc = "HOST_SLCHOST_GPIO_IN0 (r) register accessor: an alias for `Reg<HOST_SLCHOST_GPIO_IN0_SPEC>`"]
pub type HOST_SLCHOST_GPIO_IN0 = crate::Reg<host_slchost_gpio_in0::HOST_SLCHOST_GPIO_IN0_SPEC>;
#[doc = ""]
pub mod host_slchost_gpio_in0;
#[doc = "HOST_SLCHOST_GPIO_IN1 (r) register accessor: an alias for `Reg<HOST_SLCHOST_GPIO_IN1_SPEC>`"]
pub type HOST_SLCHOST_GPIO_IN1 = crate::Reg<host_slchost_gpio_in1::HOST_SLCHOST_GPIO_IN1_SPEC>;
#[doc = ""]
pub mod host_slchost_gpio_in1;
#[doc = "HOST_SLC0HOST_TOKEN_RDATA (r) register accessor: an alias for `Reg<HOST_SLC0HOST_TOKEN_RDATA_SPEC>`"]
pub type HOST_SLC0HOST_TOKEN_RDATA =
    crate::Reg<host_slc0host_token_rdata::HOST_SLC0HOST_TOKEN_RDATA_SPEC>;
#[doc = ""]
pub mod host_slc0host_token_rdata;
#[doc = "HOST_SLC0_HOST_PF (r) register accessor: an alias for `Reg<HOST_SLC0_HOST_PF_SPEC>`"]
pub type HOST_SLC0_HOST_PF = crate::Reg<host_slc0_host_pf::HOST_SLC0_HOST_PF_SPEC>;
#[doc = ""]
pub mod host_slc0_host_pf;
#[doc = "HOST_SLC1_HOST_PF (r) register accessor: an alias for `Reg<HOST_SLC1_HOST_PF_SPEC>`"]
pub type HOST_SLC1_HOST_PF = crate::Reg<host_slc1_host_pf::HOST_SLC1_HOST_PF_SPEC>;
#[doc = ""]
pub mod host_slc1_host_pf;
#[doc = "HOST_SLC0HOST_INT_RAW (r) register accessor: an alias for `Reg<HOST_SLC0HOST_INT_RAW_SPEC>`"]
pub type HOST_SLC0HOST_INT_RAW = crate::Reg<host_slc0host_int_raw::HOST_SLC0HOST_INT_RAW_SPEC>;
#[doc = ""]
pub mod host_slc0host_int_raw;
#[doc = "HOST_SLC1HOST_INT_RAW (r) register accessor: an alias for `Reg<HOST_SLC1HOST_INT_RAW_SPEC>`"]
pub type HOST_SLC1HOST_INT_RAW = crate::Reg<host_slc1host_int_raw::HOST_SLC1HOST_INT_RAW_SPEC>;
#[doc = ""]
pub mod host_slc1host_int_raw;
#[doc = "HOST_SLC0HOST_INT_ST (r) register accessor: an alias for `Reg<HOST_SLC0HOST_INT_ST_SPEC>`"]
pub type HOST_SLC0HOST_INT_ST = crate::Reg<host_slc0host_int_st::HOST_SLC0HOST_INT_ST_SPEC>;
#[doc = ""]
pub mod host_slc0host_int_st;
#[doc = "HOST_SLC1HOST_INT_ST (r) register accessor: an alias for `Reg<HOST_SLC1HOST_INT_ST_SPEC>`"]
pub type HOST_SLC1HOST_INT_ST = crate::Reg<host_slc1host_int_st::HOST_SLC1HOST_INT_ST_SPEC>;
#[doc = ""]
pub mod host_slc1host_int_st;
#[doc = "HOST_SLCHOST_PKT_LEN (r) register accessor: an alias for `Reg<HOST_SLCHOST_PKT_LEN_SPEC>`"]
pub type HOST_SLCHOST_PKT_LEN = crate::Reg<host_slchost_pkt_len::HOST_SLCHOST_PKT_LEN_SPEC>;
#[doc = ""]
pub mod host_slchost_pkt_len;
#[doc = "HOST_SLCHOST_STATE_W0 (r) register accessor: an alias for `Reg<HOST_SLCHOST_STATE_W0_SPEC>`"]
pub type HOST_SLCHOST_STATE_W0 = crate::Reg<host_slchost_state_w0::HOST_SLCHOST_STATE_W0_SPEC>;
#[doc = ""]
pub mod host_slchost_state_w0;
#[doc = "HOST_SLCHOST_STATE_W1 (r) register accessor: an alias for `Reg<HOST_SLCHOST_STATE_W1_SPEC>`"]
pub type HOST_SLCHOST_STATE_W1 = crate::Reg<host_slchost_state_w1::HOST_SLCHOST_STATE_W1_SPEC>;
#[doc = ""]
pub mod host_slchost_state_w1;
#[doc = "HOST_SLCHOST_CONF_W0 (rw) register accessor: an alias for `Reg<HOST_SLCHOST_CONF_W0_SPEC>`"]
pub type HOST_SLCHOST_CONF_W0 = crate::Reg<host_slchost_conf_w0::HOST_SLCHOST_CONF_W0_SPEC>;
#[doc = ""]
pub mod host_slchost_conf_w0;
#[doc = "HOST_SLCHOST_CONF_W1 (rw) register accessor: an alias for `Reg<HOST_SLCHOST_CONF_W1_SPEC>`"]
pub type HOST_SLCHOST_CONF_W1 = crate::Reg<host_slchost_conf_w1::HOST_SLCHOST_CONF_W1_SPEC>;
#[doc = ""]
pub mod host_slchost_conf_w1;
#[doc = "HOST_SLCHOST_CONF_W2 (rw) register accessor: an alias for `Reg<HOST_SLCHOST_CONF_W2_SPEC>`"]
pub type HOST_SLCHOST_CONF_W2 = crate::Reg<host_slchost_conf_w2::HOST_SLCHOST_CONF_W2_SPEC>;
#[doc = ""]
pub mod host_slchost_conf_w2;
#[doc = "HOST_SLCHOST_CONF_W3 (rw) register accessor: an alias for `Reg<HOST_SLCHOST_CONF_W3_SPEC>`"]
pub type HOST_SLCHOST_CONF_W3 = crate::Reg<host_slchost_conf_w3::HOST_SLCHOST_CONF_W3_SPEC>;
#[doc = ""]
pub mod host_slchost_conf_w3;
#[doc = "HOST_SLCHOST_CONF_W4 (rw) register accessor: an alias for `Reg<HOST_SLCHOST_CONF_W4_SPEC>`"]
pub type HOST_SLCHOST_CONF_W4 = crate::Reg<host_slchost_conf_w4::HOST_SLCHOST_CONF_W4_SPEC>;
#[doc = ""]
pub mod host_slchost_conf_w4;
#[doc = "HOST_SLCHOST_CONF_W5 (rw) register accessor: an alias for `Reg<HOST_SLCHOST_CONF_W5_SPEC>`"]
pub type HOST_SLCHOST_CONF_W5 = crate::Reg<host_slchost_conf_w5::HOST_SLCHOST_CONF_W5_SPEC>;
#[doc = ""]
pub mod host_slchost_conf_w5;
#[doc = "HOST_SLCHOST_WIN_CMD (rw) register accessor: an alias for `Reg<HOST_SLCHOST_WIN_CMD_SPEC>`"]
pub type HOST_SLCHOST_WIN_CMD = crate::Reg<host_slchost_win_cmd::HOST_SLCHOST_WIN_CMD_SPEC>;
#[doc = ""]
pub mod host_slchost_win_cmd;
#[doc = "HOST_SLCHOST_CONF_W6 (rw) register accessor: an alias for `Reg<HOST_SLCHOST_CONF_W6_SPEC>`"]
pub type HOST_SLCHOST_CONF_W6 = crate::Reg<host_slchost_conf_w6::HOST_SLCHOST_CONF_W6_SPEC>;
#[doc = ""]
pub mod host_slchost_conf_w6;
#[doc = "HOST_SLCHOST_CONF_W7 (rw) register accessor: an alias for `Reg<HOST_SLCHOST_CONF_W7_SPEC>`"]
pub type HOST_SLCHOST_CONF_W7 = crate::Reg<host_slchost_conf_w7::HOST_SLCHOST_CONF_W7_SPEC>;
#[doc = ""]
pub mod host_slchost_conf_w7;
#[doc = "HOST_SLCHOST_PKT_LEN0 (r) register accessor: an alias for `Reg<HOST_SLCHOST_PKT_LEN0_SPEC>`"]
pub type HOST_SLCHOST_PKT_LEN0 = crate::Reg<host_slchost_pkt_len0::HOST_SLCHOST_PKT_LEN0_SPEC>;
#[doc = ""]
pub mod host_slchost_pkt_len0;
#[doc = "HOST_SLCHOST_PKT_LEN1 (r) register accessor: an alias for `Reg<HOST_SLCHOST_PKT_LEN1_SPEC>`"]
pub type HOST_SLCHOST_PKT_LEN1 = crate::Reg<host_slchost_pkt_len1::HOST_SLCHOST_PKT_LEN1_SPEC>;
#[doc = ""]
pub mod host_slchost_pkt_len1;
#[doc = "HOST_SLCHOST_PKT_LEN2 (r) register accessor: an alias for `Reg<HOST_SLCHOST_PKT_LEN2_SPEC>`"]
pub type HOST_SLCHOST_PKT_LEN2 = crate::Reg<host_slchost_pkt_len2::HOST_SLCHOST_PKT_LEN2_SPEC>;
#[doc = ""]
pub mod host_slchost_pkt_len2;
#[doc = "HOST_SLCHOST_CONF_W8 (rw) register accessor: an alias for `Reg<HOST_SLCHOST_CONF_W8_SPEC>`"]
pub type HOST_SLCHOST_CONF_W8 = crate::Reg<host_slchost_conf_w8::HOST_SLCHOST_CONF_W8_SPEC>;
#[doc = ""]
pub mod host_slchost_conf_w8;
#[doc = "HOST_SLCHOST_CONF_W9 (rw) register accessor: an alias for `Reg<HOST_SLCHOST_CONF_W9_SPEC>`"]
pub type HOST_SLCHOST_CONF_W9 = crate::Reg<host_slchost_conf_w9::HOST_SLCHOST_CONF_W9_SPEC>;
#[doc = ""]
pub mod host_slchost_conf_w9;
#[doc = "HOST_SLCHOST_CONF_W10 (rw) register accessor: an alias for `Reg<HOST_SLCHOST_CONF_W10_SPEC>`"]
pub type HOST_SLCHOST_CONF_W10 = crate::Reg<host_slchost_conf_w10::HOST_SLCHOST_CONF_W10_SPEC>;
#[doc = ""]
pub mod host_slchost_conf_w10;
#[doc = "HOST_SLCHOST_CONF_W11 (rw) register accessor: an alias for `Reg<HOST_SLCHOST_CONF_W11_SPEC>`"]
pub type HOST_SLCHOST_CONF_W11 = crate::Reg<host_slchost_conf_w11::HOST_SLCHOST_CONF_W11_SPEC>;
#[doc = ""]
pub mod host_slchost_conf_w11;
#[doc = "HOST_SLCHOST_CONF_W12 (rw) register accessor: an alias for `Reg<HOST_SLCHOST_CONF_W12_SPEC>`"]
pub type HOST_SLCHOST_CONF_W12 = crate::Reg<host_slchost_conf_w12::HOST_SLCHOST_CONF_W12_SPEC>;
#[doc = ""]
pub mod host_slchost_conf_w12;
#[doc = "HOST_SLCHOST_CONF_W13 (rw) register accessor: an alias for `Reg<HOST_SLCHOST_CONF_W13_SPEC>`"]
pub type HOST_SLCHOST_CONF_W13 = crate::Reg<host_slchost_conf_w13::HOST_SLCHOST_CONF_W13_SPEC>;
#[doc = ""]
pub mod host_slchost_conf_w13;
#[doc = "HOST_SLCHOST_CONF_W14 (rw) register accessor: an alias for `Reg<HOST_SLCHOST_CONF_W14_SPEC>`"]
pub type HOST_SLCHOST_CONF_W14 = crate::Reg<host_slchost_conf_w14::HOST_SLCHOST_CONF_W14_SPEC>;
#[doc = ""]
pub mod host_slchost_conf_w14;
#[doc = "HOST_SLCHOST_CONF_W15 (rw) register accessor: an alias for `Reg<HOST_SLCHOST_CONF_W15_SPEC>`"]
pub type HOST_SLCHOST_CONF_W15 = crate::Reg<host_slchost_conf_w15::HOST_SLCHOST_CONF_W15_SPEC>;
#[doc = ""]
pub mod host_slchost_conf_w15;
#[doc = "HOST_SLCHOST_CHECK_SUM0 (r) register accessor: an alias for `Reg<HOST_SLCHOST_CHECK_SUM0_SPEC>`"]
pub type HOST_SLCHOST_CHECK_SUM0 =
    crate::Reg<host_slchost_check_sum0::HOST_SLCHOST_CHECK_SUM0_SPEC>;
#[doc = ""]
pub mod host_slchost_check_sum0;
#[doc = "HOST_SLCHOST_CHECK_SUM1 (r) register accessor: an alias for `Reg<HOST_SLCHOST_CHECK_SUM1_SPEC>`"]
pub type HOST_SLCHOST_CHECK_SUM1 =
    crate::Reg<host_slchost_check_sum1::HOST_SLCHOST_CHECK_SUM1_SPEC>;
#[doc = ""]
pub mod host_slchost_check_sum1;
#[doc = "HOST_SLC1HOST_TOKEN_RDATA (r) register accessor: an alias for `Reg<HOST_SLC1HOST_TOKEN_RDATA_SPEC>`"]
pub type HOST_SLC1HOST_TOKEN_RDATA =
    crate::Reg<host_slc1host_token_rdata::HOST_SLC1HOST_TOKEN_RDATA_SPEC>;
#[doc = ""]
pub mod host_slc1host_token_rdata;
#[doc = "HOST_SLC0HOST_TOKEN_WDATA (rw) register accessor: an alias for `Reg<HOST_SLC0HOST_TOKEN_WDATA_SPEC>`"]
pub type HOST_SLC0HOST_TOKEN_WDATA =
    crate::Reg<host_slc0host_token_wdata::HOST_SLC0HOST_TOKEN_WDATA_SPEC>;
#[doc = ""]
pub mod host_slc0host_token_wdata;
#[doc = "HOST_SLC1HOST_TOKEN_WDATA (rw) register accessor: an alias for `Reg<HOST_SLC1HOST_TOKEN_WDATA_SPEC>`"]
pub type HOST_SLC1HOST_TOKEN_WDATA =
    crate::Reg<host_slc1host_token_wdata::HOST_SLC1HOST_TOKEN_WDATA_SPEC>;
#[doc = ""]
pub mod host_slc1host_token_wdata;
#[doc = "HOST_SLCHOST_TOKEN_CON (w) register accessor: an alias for `Reg<HOST_SLCHOST_TOKEN_CON_SPEC>`"]
pub type HOST_SLCHOST_TOKEN_CON = crate::Reg<host_slchost_token_con::HOST_SLCHOST_TOKEN_CON_SPEC>;
#[doc = ""]
pub mod host_slchost_token_con;
#[doc = "HOST_SLC0HOST_INT_CLR (w) register accessor: an alias for `Reg<HOST_SLC0HOST_INT_CLR_SPEC>`"]
pub type HOST_SLC0HOST_INT_CLR = crate::Reg<host_slc0host_int_clr::HOST_SLC0HOST_INT_CLR_SPEC>;
#[doc = ""]
pub mod host_slc0host_int_clr;
#[doc = "HOST_SLC1HOST_INT_CLR (w) register accessor: an alias for `Reg<HOST_SLC1HOST_INT_CLR_SPEC>`"]
pub type HOST_SLC1HOST_INT_CLR = crate::Reg<host_slc1host_int_clr::HOST_SLC1HOST_INT_CLR_SPEC>;
#[doc = ""]
pub mod host_slc1host_int_clr;
#[doc = "HOST_SLC0HOST_FUNC1_INT_ENA (rw) register accessor: an alias for `Reg<HOST_SLC0HOST_FUNC1_INT_ENA_SPEC>`"]
pub type HOST_SLC0HOST_FUNC1_INT_ENA =
    crate::Reg<host_slc0host_func1_int_ena::HOST_SLC0HOST_FUNC1_INT_ENA_SPEC>;
#[doc = ""]
pub mod host_slc0host_func1_int_ena;
#[doc = "HOST_SLC1HOST_FUNC1_INT_ENA (rw) register accessor: an alias for `Reg<HOST_SLC1HOST_FUNC1_INT_ENA_SPEC>`"]
pub type HOST_SLC1HOST_FUNC1_INT_ENA =
    crate::Reg<host_slc1host_func1_int_ena::HOST_SLC1HOST_FUNC1_INT_ENA_SPEC>;
#[doc = ""]
pub mod host_slc1host_func1_int_ena;
#[doc = "HOST_SLC0HOST_FUNC2_INT_ENA (rw) register accessor: an alias for `Reg<HOST_SLC0HOST_FUNC2_INT_ENA_SPEC>`"]
pub type HOST_SLC0HOST_FUNC2_INT_ENA =
    crate::Reg<host_slc0host_func2_int_ena::HOST_SLC0HOST_FUNC2_INT_ENA_SPEC>;
#[doc = ""]
pub mod host_slc0host_func2_int_ena;
#[doc = "HOST_SLC1HOST_FUNC2_INT_ENA (rw) register accessor: an alias for `Reg<HOST_SLC1HOST_FUNC2_INT_ENA_SPEC>`"]
pub type HOST_SLC1HOST_FUNC2_INT_ENA =
    crate::Reg<host_slc1host_func2_int_ena::HOST_SLC1HOST_FUNC2_INT_ENA_SPEC>;
#[doc = ""]
pub mod host_slc1host_func2_int_ena;
#[doc = "HOST_SLC0HOST_INT_ENA (rw) register accessor: an alias for `Reg<HOST_SLC0HOST_INT_ENA_SPEC>`"]
pub type HOST_SLC0HOST_INT_ENA = crate::Reg<host_slc0host_int_ena::HOST_SLC0HOST_INT_ENA_SPEC>;
#[doc = ""]
pub mod host_slc0host_int_ena;
#[doc = "HOST_SLC1HOST_INT_ENA (rw) register accessor: an alias for `Reg<HOST_SLC1HOST_INT_ENA_SPEC>`"]
pub type HOST_SLC1HOST_INT_ENA = crate::Reg<host_slc1host_int_ena::HOST_SLC1HOST_INT_ENA_SPEC>;
#[doc = ""]
pub mod host_slc1host_int_ena;
#[doc = "HOST_SLC0HOST_RX_INFOR (rw) register accessor: an alias for `Reg<HOST_SLC0HOST_RX_INFOR_SPEC>`"]
pub type HOST_SLC0HOST_RX_INFOR = crate::Reg<host_slc0host_rx_infor::HOST_SLC0HOST_RX_INFOR_SPEC>;
#[doc = ""]
pub mod host_slc0host_rx_infor;
#[doc = "HOST_SLC1HOST_RX_INFOR (rw) register accessor: an alias for `Reg<HOST_SLC1HOST_RX_INFOR_SPEC>`"]
pub type HOST_SLC1HOST_RX_INFOR = crate::Reg<host_slc1host_rx_infor::HOST_SLC1HOST_RX_INFOR_SPEC>;
#[doc = ""]
pub mod host_slc1host_rx_infor;
#[doc = "HOST_SLC0HOST_LEN_WD (rw) register accessor: an alias for `Reg<HOST_SLC0HOST_LEN_WD_SPEC>`"]
pub type HOST_SLC0HOST_LEN_WD = crate::Reg<host_slc0host_len_wd::HOST_SLC0HOST_LEN_WD_SPEC>;
#[doc = ""]
pub mod host_slc0host_len_wd;
#[doc = "HOST_SLC_APBWIN_WDATA (rw) register accessor: an alias for `Reg<HOST_SLC_APBWIN_WDATA_SPEC>`"]
pub type HOST_SLC_APBWIN_WDATA = crate::Reg<host_slc_apbwin_wdata::HOST_SLC_APBWIN_WDATA_SPEC>;
#[doc = ""]
pub mod host_slc_apbwin_wdata;
#[doc = "HOST_SLC_APBWIN_CONF (rw) register accessor: an alias for `Reg<HOST_SLC_APBWIN_CONF_SPEC>`"]
pub type HOST_SLC_APBWIN_CONF = crate::Reg<host_slc_apbwin_conf::HOST_SLC_APBWIN_CONF_SPEC>;
#[doc = ""]
pub mod host_slc_apbwin_conf;
#[doc = "HOST_SLC_APBWIN_RDATA (r) register accessor: an alias for `Reg<HOST_SLC_APBWIN_RDATA_SPEC>`"]
pub type HOST_SLC_APBWIN_RDATA = crate::Reg<host_slc_apbwin_rdata::HOST_SLC_APBWIN_RDATA_SPEC>;
#[doc = ""]
pub mod host_slc_apbwin_rdata;
#[doc = "HOST_SLCHOST_RDCLR0 (rw) register accessor: an alias for `Reg<HOST_SLCHOST_RDCLR0_SPEC>`"]
pub type HOST_SLCHOST_RDCLR0 = crate::Reg<host_slchost_rdclr0::HOST_SLCHOST_RDCLR0_SPEC>;
#[doc = ""]
pub mod host_slchost_rdclr0;
#[doc = "HOST_SLCHOST_RDCLR1 (rw) register accessor: an alias for `Reg<HOST_SLCHOST_RDCLR1_SPEC>`"]
pub type HOST_SLCHOST_RDCLR1 = crate::Reg<host_slchost_rdclr1::HOST_SLCHOST_RDCLR1_SPEC>;
#[doc = ""]
pub mod host_slchost_rdclr1;
#[doc = "HOST_SLC0HOST_INT_ENA1 (rw) register accessor: an alias for `Reg<HOST_SLC0HOST_INT_ENA1_SPEC>`"]
pub type HOST_SLC0HOST_INT_ENA1 = crate::Reg<host_slc0host_int_ena1::HOST_SLC0HOST_INT_ENA1_SPEC>;
#[doc = ""]
pub mod host_slc0host_int_ena1;
#[doc = "HOST_SLC1HOST_INT_ENA1 (rw) register accessor: an alias for `Reg<HOST_SLC1HOST_INT_ENA1_SPEC>`"]
pub type HOST_SLC1HOST_INT_ENA1 = crate::Reg<host_slc1host_int_ena1::HOST_SLC1HOST_INT_ENA1_SPEC>;
#[doc = ""]
pub mod host_slc1host_int_ena1;
#[doc = "HOST_SLCHOSTDATE (rw) register accessor: an alias for `Reg<HOST_SLCHOSTDATE_SPEC>`"]
pub type HOST_SLCHOSTDATE = crate::Reg<host_slchostdate::HOST_SLCHOSTDATE_SPEC>;
#[doc = ""]
pub mod host_slchostdate;
#[doc = "HOST_SLCHOSTID (rw) register accessor: an alias for `Reg<HOST_SLCHOSTID_SPEC>`"]
pub type HOST_SLCHOSTID = crate::Reg<host_slchostid::HOST_SLCHOSTID_SPEC>;
#[doc = ""]
pub mod host_slchostid;
#[doc = "HOST_SLCHOST_CONF (rw) register accessor: an alias for `Reg<HOST_SLCHOST_CONF_SPEC>`"]
pub type HOST_SLCHOST_CONF = crate::Reg<host_slchost_conf::HOST_SLCHOST_CONF_SPEC>;
#[doc = ""]
pub mod host_slchost_conf;
#[doc = "HOST_SLCHOST_INF_ST (r) register accessor: an alias for `Reg<HOST_SLCHOST_INF_ST_SPEC>`"]
pub type HOST_SLCHOST_INF_ST = crate::Reg<host_slchost_inf_st::HOST_SLCHOST_INF_ST_SPEC>;
#[doc = ""]
pub mod host_slchost_inf_st;
