#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - *******Description***********"]
    pub func2_0: FUNC2_0,
    #[doc = "0x14 - *******Description***********"]
    pub func2_1: FUNC2_1,
    _reserved2: [u8; 0x08],
    #[doc = "0x20 - *******Description***********"]
    pub func2_2: FUNC2_2,
    _reserved3: [u8; 0x10],
    #[doc = "0x34 - *******Description***********"]
    pub gpio_status0: GPIO_STATUS0,
    #[doc = "0x38 - *******Description***********"]
    pub gpio_status1: GPIO_STATUS1,
    #[doc = "0x3c - *******Description***********"]
    pub gpio_in0: GPIO_IN0,
    #[doc = "0x40 - *******Description***********"]
    pub gpio_in1: GPIO_IN1,
    #[doc = "0x44 - *******Description***********"]
    pub slc0host_token_rdata: SLC0HOST_TOKEN_RDATA,
    #[doc = "0x48 - *******Description***********"]
    pub slc0_host_pf: SLC0_HOST_PF,
    #[doc = "0x4c - *******Description***********"]
    pub slc1_host_pf: SLC1_HOST_PF,
    #[doc = "0x50 - *******Description***********"]
    pub slc0host_int_raw: SLC0HOST_INT_RAW,
    #[doc = "0x54 - *******Description***********"]
    pub slc1host_int_raw: SLC1HOST_INT_RAW,
    #[doc = "0x58 - *******Description***********"]
    pub slc0host_int_st: SLC0HOST_INT_ST,
    #[doc = "0x5c - *******Description***********"]
    pub slc1host_int_st: SLC1HOST_INT_ST,
    #[doc = "0x60 - *******Description***********"]
    pub pkt_len: PKT_LEN,
    #[doc = "0x64 - *******Description***********"]
    pub state_w0: STATE_W0,
    #[doc = "0x68 - *******Description***********"]
    pub state_w1: STATE_W1,
    #[doc = "0x6c - *******Description***********"]
    pub conf_w0: CONF_W0,
    #[doc = "0x70 - *******Description***********"]
    pub conf_w1: CONF_W1,
    #[doc = "0x74 - *******Description***********"]
    pub conf_w2: CONF_W2,
    #[doc = "0x78 - *******Description***********"]
    pub conf_w3: CONF_W3,
    #[doc = "0x7c - *******Description***********"]
    pub conf_w4: CONF_W4,
    #[doc = "0x80 - *******Description***********"]
    pub conf_w5: CONF_W5,
    #[doc = "0x84 - *******Description***********"]
    pub win_cmd: WIN_CMD,
    #[doc = "0x88 - *******Description***********"]
    pub conf_w6: CONF_W6,
    #[doc = "0x8c - *******Description***********"]
    pub conf_w7: CONF_W7,
    #[doc = "0x90 - *******Description***********"]
    pub pkt_len0: PKT_LEN0,
    #[doc = "0x94 - *******Description***********"]
    pub pkt_len1: PKT_LEN1,
    #[doc = "0x98 - *******Description***********"]
    pub pkt_len2: PKT_LEN2,
    #[doc = "0x9c - *******Description***********"]
    pub conf_w8: CONF_W8,
    #[doc = "0xa0 - *******Description***********"]
    pub conf_w9: CONF_W9,
    #[doc = "0xa4 - *******Description***********"]
    pub conf_w10: CONF_W10,
    #[doc = "0xa8 - *******Description***********"]
    pub conf_w11: CONF_W11,
    #[doc = "0xac - *******Description***********"]
    pub conf_w12: CONF_W12,
    #[doc = "0xb0 - *******Description***********"]
    pub conf_w13: CONF_W13,
    #[doc = "0xb4 - *******Description***********"]
    pub conf_w14: CONF_W14,
    #[doc = "0xb8 - *******Description***********"]
    pub conf_w15: CONF_W15,
    #[doc = "0xbc - *******Description***********"]
    pub check_sum0: CHECK_SUM0,
    #[doc = "0xc0 - *******Description***********"]
    pub check_sum1: CHECK_SUM1,
    #[doc = "0xc4 - *******Description***********"]
    pub slc1host_token_rdata: SLC1HOST_TOKEN_RDATA,
    #[doc = "0xc8 - *******Description***********"]
    pub slc0host_token_wdata: SLC0HOST_TOKEN_WDATA,
    #[doc = "0xcc - *******Description***********"]
    pub slc1host_token_wdata: SLC1HOST_TOKEN_WDATA,
    #[doc = "0xd0 - *******Description***********"]
    pub token_con: TOKEN_CON,
    #[doc = "0xd4 - *******Description***********"]
    pub slc0host_int_clr: SLC0HOST_INT_CLR,
    #[doc = "0xd8 - *******Description***********"]
    pub slc1host_int_clr: SLC1HOST_INT_CLR,
    #[doc = "0xdc - *******Description***********"]
    pub slc0host_func1_int_ena: SLC0HOST_FUNC1_INT_ENA,
    #[doc = "0xe0 - *******Description***********"]
    pub slc1host_func1_int_ena: SLC1HOST_FUNC1_INT_ENA,
    #[doc = "0xe4 - *******Description***********"]
    pub slc0host_func2_int_ena: SLC0HOST_FUNC2_INT_ENA,
    #[doc = "0xe8 - *******Description***********"]
    pub slc1host_func2_int_ena: SLC1HOST_FUNC2_INT_ENA,
    #[doc = "0xec - *******Description***********"]
    pub slc0host_int_ena: SLC0HOST_INT_ENA,
    #[doc = "0xf0 - *******Description***********"]
    pub slc1host_int_ena: SLC1HOST_INT_ENA,
    #[doc = "0xf4 - *******Description***********"]
    pub slc0host_rx_infor: SLC0HOST_RX_INFOR,
    #[doc = "0xf8 - *******Description***********"]
    pub slc1host_rx_infor: SLC1HOST_RX_INFOR,
    #[doc = "0xfc - *******Description***********"]
    pub slc0host_len_wd: SLC0HOST_LEN_WD,
    #[doc = "0x100 - *******Description***********"]
    pub slc_apbwin_wdata: SLC_APBWIN_WDATA,
    #[doc = "0x104 - *******Description***********"]
    pub slc_apbwin_conf: SLC_APBWIN_CONF,
    #[doc = "0x108 - *******Description***********"]
    pub slc_apbwin_rdata: SLC_APBWIN_RDATA,
    #[doc = "0x10c - *******Description***********"]
    pub rdclr0: RDCLR0,
    #[doc = "0x110 - *******Description***********"]
    pub rdclr1: RDCLR1,
    #[doc = "0x114 - *******Description***********"]
    pub slc0host_int_ena1: SLC0HOST_INT_ENA1,
    #[doc = "0x118 - *******Description***********"]
    pub slc1host_int_ena1: SLC1HOST_INT_ENA1,
    _reserved61: [u8; 0x5c],
    #[doc = "0x178 - *******Description***********"]
    pub slchostdate: SLCHOSTDATE,
    #[doc = "0x17c - *******Description***********"]
    pub slchostid: SLCHOSTID,
    _reserved63: [u8; 0x70],
    #[doc = "0x1f0 - *******Description***********"]
    pub conf: CONF,
    #[doc = "0x1f4 - *******Description***********"]
    pub inf_st: INF_ST,
}
#[doc = "FUNC2_0 (rw) register accessor: an alias for `Reg<FUNC2_0_SPEC>`"]
pub type FUNC2_0 = crate::Reg<func2_0::FUNC2_0_SPEC>;
#[doc = "*******Description***********"]
pub mod func2_0;
#[doc = "FUNC2_1 (rw) register accessor: an alias for `Reg<FUNC2_1_SPEC>`"]
pub type FUNC2_1 = crate::Reg<func2_1::FUNC2_1_SPEC>;
#[doc = "*******Description***********"]
pub mod func2_1;
#[doc = "FUNC2_2 (rw) register accessor: an alias for `Reg<FUNC2_2_SPEC>`"]
pub type FUNC2_2 = crate::Reg<func2_2::FUNC2_2_SPEC>;
#[doc = "*******Description***********"]
pub mod func2_2;
#[doc = "GPIO_STATUS0 (r) register accessor: an alias for `Reg<GPIO_STATUS0_SPEC>`"]
pub type GPIO_STATUS0 = crate::Reg<gpio_status0::GPIO_STATUS0_SPEC>;
#[doc = "*******Description***********"]
pub mod gpio_status0;
#[doc = "GPIO_STATUS1 (r) register accessor: an alias for `Reg<GPIO_STATUS1_SPEC>`"]
pub type GPIO_STATUS1 = crate::Reg<gpio_status1::GPIO_STATUS1_SPEC>;
#[doc = "*******Description***********"]
pub mod gpio_status1;
#[doc = "GPIO_IN0 (r) register accessor: an alias for `Reg<GPIO_IN0_SPEC>`"]
pub type GPIO_IN0 = crate::Reg<gpio_in0::GPIO_IN0_SPEC>;
#[doc = "*******Description***********"]
pub mod gpio_in0;
#[doc = "GPIO_IN1 (r) register accessor: an alias for `Reg<GPIO_IN1_SPEC>`"]
pub type GPIO_IN1 = crate::Reg<gpio_in1::GPIO_IN1_SPEC>;
#[doc = "*******Description***********"]
pub mod gpio_in1;
#[doc = "SLC0HOST_TOKEN_RDATA (r) register accessor: an alias for `Reg<SLC0HOST_TOKEN_RDATA_SPEC>`"]
pub type SLC0HOST_TOKEN_RDATA = crate::Reg<slc0host_token_rdata::SLC0HOST_TOKEN_RDATA_SPEC>;
#[doc = "*******Description***********"]
pub mod slc0host_token_rdata;
#[doc = "SLC0_HOST_PF (r) register accessor: an alias for `Reg<SLC0_HOST_PF_SPEC>`"]
pub type SLC0_HOST_PF = crate::Reg<slc0_host_pf::SLC0_HOST_PF_SPEC>;
#[doc = "*******Description***********"]
pub mod slc0_host_pf;
#[doc = "SLC1_HOST_PF (r) register accessor: an alias for `Reg<SLC1_HOST_PF_SPEC>`"]
pub type SLC1_HOST_PF = crate::Reg<slc1_host_pf::SLC1_HOST_PF_SPEC>;
#[doc = "*******Description***********"]
pub mod slc1_host_pf;
#[doc = "SLC0HOST_INT_RAW (rw) register accessor: an alias for `Reg<SLC0HOST_INT_RAW_SPEC>`"]
pub type SLC0HOST_INT_RAW = crate::Reg<slc0host_int_raw::SLC0HOST_INT_RAW_SPEC>;
#[doc = "*******Description***********"]
pub mod slc0host_int_raw;
#[doc = "SLC1HOST_INT_RAW (rw) register accessor: an alias for `Reg<SLC1HOST_INT_RAW_SPEC>`"]
pub type SLC1HOST_INT_RAW = crate::Reg<slc1host_int_raw::SLC1HOST_INT_RAW_SPEC>;
#[doc = "*******Description***********"]
pub mod slc1host_int_raw;
#[doc = "SLC0HOST_INT_ST (r) register accessor: an alias for `Reg<SLC0HOST_INT_ST_SPEC>`"]
pub type SLC0HOST_INT_ST = crate::Reg<slc0host_int_st::SLC0HOST_INT_ST_SPEC>;
#[doc = "*******Description***********"]
pub mod slc0host_int_st;
#[doc = "SLC1HOST_INT_ST (r) register accessor: an alias for `Reg<SLC1HOST_INT_ST_SPEC>`"]
pub type SLC1HOST_INT_ST = crate::Reg<slc1host_int_st::SLC1HOST_INT_ST_SPEC>;
#[doc = "*******Description***********"]
pub mod slc1host_int_st;
#[doc = "PKT_LEN (r) register accessor: an alias for `Reg<PKT_LEN_SPEC>`"]
pub type PKT_LEN = crate::Reg<pkt_len::PKT_LEN_SPEC>;
#[doc = "*******Description***********"]
pub mod pkt_len;
#[doc = "STATE_W0 (r) register accessor: an alias for `Reg<STATE_W0_SPEC>`"]
pub type STATE_W0 = crate::Reg<state_w0::STATE_W0_SPEC>;
#[doc = "*******Description***********"]
pub mod state_w0;
#[doc = "STATE_W1 (r) register accessor: an alias for `Reg<STATE_W1_SPEC>`"]
pub type STATE_W1 = crate::Reg<state_w1::STATE_W1_SPEC>;
#[doc = "*******Description***********"]
pub mod state_w1;
#[doc = "CONF_W0 (rw) register accessor: an alias for `Reg<CONF_W0_SPEC>`"]
pub type CONF_W0 = crate::Reg<conf_w0::CONF_W0_SPEC>;
#[doc = "*******Description***********"]
pub mod conf_w0;
#[doc = "CONF_W1 (rw) register accessor: an alias for `Reg<CONF_W1_SPEC>`"]
pub type CONF_W1 = crate::Reg<conf_w1::CONF_W1_SPEC>;
#[doc = "*******Description***********"]
pub mod conf_w1;
#[doc = "CONF_W2 (rw) register accessor: an alias for `Reg<CONF_W2_SPEC>`"]
pub type CONF_W2 = crate::Reg<conf_w2::CONF_W2_SPEC>;
#[doc = "*******Description***********"]
pub mod conf_w2;
#[doc = "CONF_W3 (rw) register accessor: an alias for `Reg<CONF_W3_SPEC>`"]
pub type CONF_W3 = crate::Reg<conf_w3::CONF_W3_SPEC>;
#[doc = "*******Description***********"]
pub mod conf_w3;
#[doc = "CONF_W4 (rw) register accessor: an alias for `Reg<CONF_W4_SPEC>`"]
pub type CONF_W4 = crate::Reg<conf_w4::CONF_W4_SPEC>;
#[doc = "*******Description***********"]
pub mod conf_w4;
#[doc = "CONF_W5 (rw) register accessor: an alias for `Reg<CONF_W5_SPEC>`"]
pub type CONF_W5 = crate::Reg<conf_w5::CONF_W5_SPEC>;
#[doc = "*******Description***********"]
pub mod conf_w5;
#[doc = "WIN_CMD (rw) register accessor: an alias for `Reg<WIN_CMD_SPEC>`"]
pub type WIN_CMD = crate::Reg<win_cmd::WIN_CMD_SPEC>;
#[doc = "*******Description***********"]
pub mod win_cmd;
#[doc = "CONF_W6 (rw) register accessor: an alias for `Reg<CONF_W6_SPEC>`"]
pub type CONF_W6 = crate::Reg<conf_w6::CONF_W6_SPEC>;
#[doc = "*******Description***********"]
pub mod conf_w6;
#[doc = "CONF_W7 (rw) register accessor: an alias for `Reg<CONF_W7_SPEC>`"]
pub type CONF_W7 = crate::Reg<conf_w7::CONF_W7_SPEC>;
#[doc = "*******Description***********"]
pub mod conf_w7;
#[doc = "PKT_LEN0 (r) register accessor: an alias for `Reg<PKT_LEN0_SPEC>`"]
pub type PKT_LEN0 = crate::Reg<pkt_len0::PKT_LEN0_SPEC>;
#[doc = "*******Description***********"]
pub mod pkt_len0;
#[doc = "PKT_LEN1 (r) register accessor: an alias for `Reg<PKT_LEN1_SPEC>`"]
pub type PKT_LEN1 = crate::Reg<pkt_len1::PKT_LEN1_SPEC>;
#[doc = "*******Description***********"]
pub mod pkt_len1;
#[doc = "PKT_LEN2 (r) register accessor: an alias for `Reg<PKT_LEN2_SPEC>`"]
pub type PKT_LEN2 = crate::Reg<pkt_len2::PKT_LEN2_SPEC>;
#[doc = "*******Description***********"]
pub mod pkt_len2;
#[doc = "CONF_W8 (rw) register accessor: an alias for `Reg<CONF_W8_SPEC>`"]
pub type CONF_W8 = crate::Reg<conf_w8::CONF_W8_SPEC>;
#[doc = "*******Description***********"]
pub mod conf_w8;
#[doc = "CONF_W9 (rw) register accessor: an alias for `Reg<CONF_W9_SPEC>`"]
pub type CONF_W9 = crate::Reg<conf_w9::CONF_W9_SPEC>;
#[doc = "*******Description***********"]
pub mod conf_w9;
#[doc = "CONF_W10 (rw) register accessor: an alias for `Reg<CONF_W10_SPEC>`"]
pub type CONF_W10 = crate::Reg<conf_w10::CONF_W10_SPEC>;
#[doc = "*******Description***********"]
pub mod conf_w10;
#[doc = "CONF_W11 (rw) register accessor: an alias for `Reg<CONF_W11_SPEC>`"]
pub type CONF_W11 = crate::Reg<conf_w11::CONF_W11_SPEC>;
#[doc = "*******Description***********"]
pub mod conf_w11;
#[doc = "CONF_W12 (rw) register accessor: an alias for `Reg<CONF_W12_SPEC>`"]
pub type CONF_W12 = crate::Reg<conf_w12::CONF_W12_SPEC>;
#[doc = "*******Description***********"]
pub mod conf_w12;
#[doc = "CONF_W13 (rw) register accessor: an alias for `Reg<CONF_W13_SPEC>`"]
pub type CONF_W13 = crate::Reg<conf_w13::CONF_W13_SPEC>;
#[doc = "*******Description***********"]
pub mod conf_w13;
#[doc = "CONF_W14 (rw) register accessor: an alias for `Reg<CONF_W14_SPEC>`"]
pub type CONF_W14 = crate::Reg<conf_w14::CONF_W14_SPEC>;
#[doc = "*******Description***********"]
pub mod conf_w14;
#[doc = "CONF_W15 (rw) register accessor: an alias for `Reg<CONF_W15_SPEC>`"]
pub type CONF_W15 = crate::Reg<conf_w15::CONF_W15_SPEC>;
#[doc = "*******Description***********"]
pub mod conf_w15;
#[doc = "CHECK_SUM0 (r) register accessor: an alias for `Reg<CHECK_SUM0_SPEC>`"]
pub type CHECK_SUM0 = crate::Reg<check_sum0::CHECK_SUM0_SPEC>;
#[doc = "*******Description***********"]
pub mod check_sum0;
#[doc = "CHECK_SUM1 (r) register accessor: an alias for `Reg<CHECK_SUM1_SPEC>`"]
pub type CHECK_SUM1 = crate::Reg<check_sum1::CHECK_SUM1_SPEC>;
#[doc = "*******Description***********"]
pub mod check_sum1;
#[doc = "SLC1HOST_TOKEN_RDATA (r) register accessor: an alias for `Reg<SLC1HOST_TOKEN_RDATA_SPEC>`"]
pub type SLC1HOST_TOKEN_RDATA = crate::Reg<slc1host_token_rdata::SLC1HOST_TOKEN_RDATA_SPEC>;
#[doc = "*******Description***********"]
pub mod slc1host_token_rdata;
#[doc = "SLC0HOST_TOKEN_WDATA (rw) register accessor: an alias for `Reg<SLC0HOST_TOKEN_WDATA_SPEC>`"]
pub type SLC0HOST_TOKEN_WDATA = crate::Reg<slc0host_token_wdata::SLC0HOST_TOKEN_WDATA_SPEC>;
#[doc = "*******Description***********"]
pub mod slc0host_token_wdata;
#[doc = "SLC1HOST_TOKEN_WDATA (rw) register accessor: an alias for `Reg<SLC1HOST_TOKEN_WDATA_SPEC>`"]
pub type SLC1HOST_TOKEN_WDATA = crate::Reg<slc1host_token_wdata::SLC1HOST_TOKEN_WDATA_SPEC>;
#[doc = "*******Description***********"]
pub mod slc1host_token_wdata;
#[doc = "TOKEN_CON (w) register accessor: an alias for `Reg<TOKEN_CON_SPEC>`"]
pub type TOKEN_CON = crate::Reg<token_con::TOKEN_CON_SPEC>;
#[doc = "*******Description***********"]
pub mod token_con;
#[doc = "SLC0HOST_INT_CLR (w) register accessor: an alias for `Reg<SLC0HOST_INT_CLR_SPEC>`"]
pub type SLC0HOST_INT_CLR = crate::Reg<slc0host_int_clr::SLC0HOST_INT_CLR_SPEC>;
#[doc = "*******Description***********"]
pub mod slc0host_int_clr;
#[doc = "SLC1HOST_INT_CLR (w) register accessor: an alias for `Reg<SLC1HOST_INT_CLR_SPEC>`"]
pub type SLC1HOST_INT_CLR = crate::Reg<slc1host_int_clr::SLC1HOST_INT_CLR_SPEC>;
#[doc = "*******Description***********"]
pub mod slc1host_int_clr;
#[doc = "SLC0HOST_FUNC1_INT_ENA (rw) register accessor: an alias for `Reg<SLC0HOST_FUNC1_INT_ENA_SPEC>`"]
pub type SLC0HOST_FUNC1_INT_ENA = crate::Reg<slc0host_func1_int_ena::SLC0HOST_FUNC1_INT_ENA_SPEC>;
#[doc = "*******Description***********"]
pub mod slc0host_func1_int_ena;
#[doc = "SLC1HOST_FUNC1_INT_ENA (rw) register accessor: an alias for `Reg<SLC1HOST_FUNC1_INT_ENA_SPEC>`"]
pub type SLC1HOST_FUNC1_INT_ENA = crate::Reg<slc1host_func1_int_ena::SLC1HOST_FUNC1_INT_ENA_SPEC>;
#[doc = "*******Description***********"]
pub mod slc1host_func1_int_ena;
#[doc = "SLC0HOST_FUNC2_INT_ENA (rw) register accessor: an alias for `Reg<SLC0HOST_FUNC2_INT_ENA_SPEC>`"]
pub type SLC0HOST_FUNC2_INT_ENA = crate::Reg<slc0host_func2_int_ena::SLC0HOST_FUNC2_INT_ENA_SPEC>;
#[doc = "*******Description***********"]
pub mod slc0host_func2_int_ena;
#[doc = "SLC1HOST_FUNC2_INT_ENA (rw) register accessor: an alias for `Reg<SLC1HOST_FUNC2_INT_ENA_SPEC>`"]
pub type SLC1HOST_FUNC2_INT_ENA = crate::Reg<slc1host_func2_int_ena::SLC1HOST_FUNC2_INT_ENA_SPEC>;
#[doc = "*******Description***********"]
pub mod slc1host_func2_int_ena;
#[doc = "SLC0HOST_INT_ENA (rw) register accessor: an alias for `Reg<SLC0HOST_INT_ENA_SPEC>`"]
pub type SLC0HOST_INT_ENA = crate::Reg<slc0host_int_ena::SLC0HOST_INT_ENA_SPEC>;
#[doc = "*******Description***********"]
pub mod slc0host_int_ena;
#[doc = "SLC1HOST_INT_ENA (rw) register accessor: an alias for `Reg<SLC1HOST_INT_ENA_SPEC>`"]
pub type SLC1HOST_INT_ENA = crate::Reg<slc1host_int_ena::SLC1HOST_INT_ENA_SPEC>;
#[doc = "*******Description***********"]
pub mod slc1host_int_ena;
#[doc = "SLC0HOST_RX_INFOR (rw) register accessor: an alias for `Reg<SLC0HOST_RX_INFOR_SPEC>`"]
pub type SLC0HOST_RX_INFOR = crate::Reg<slc0host_rx_infor::SLC0HOST_RX_INFOR_SPEC>;
#[doc = "*******Description***********"]
pub mod slc0host_rx_infor;
#[doc = "SLC1HOST_RX_INFOR (rw) register accessor: an alias for `Reg<SLC1HOST_RX_INFOR_SPEC>`"]
pub type SLC1HOST_RX_INFOR = crate::Reg<slc1host_rx_infor::SLC1HOST_RX_INFOR_SPEC>;
#[doc = "*******Description***********"]
pub mod slc1host_rx_infor;
#[doc = "SLC0HOST_LEN_WD (rw) register accessor: an alias for `Reg<SLC0HOST_LEN_WD_SPEC>`"]
pub type SLC0HOST_LEN_WD = crate::Reg<slc0host_len_wd::SLC0HOST_LEN_WD_SPEC>;
#[doc = "*******Description***********"]
pub mod slc0host_len_wd;
#[doc = "SLC_APBWIN_WDATA (rw) register accessor: an alias for `Reg<SLC_APBWIN_WDATA_SPEC>`"]
pub type SLC_APBWIN_WDATA = crate::Reg<slc_apbwin_wdata::SLC_APBWIN_WDATA_SPEC>;
#[doc = "*******Description***********"]
pub mod slc_apbwin_wdata;
#[doc = "SLC_APBWIN_CONF (rw) register accessor: an alias for `Reg<SLC_APBWIN_CONF_SPEC>`"]
pub type SLC_APBWIN_CONF = crate::Reg<slc_apbwin_conf::SLC_APBWIN_CONF_SPEC>;
#[doc = "*******Description***********"]
pub mod slc_apbwin_conf;
#[doc = "SLC_APBWIN_RDATA (r) register accessor: an alias for `Reg<SLC_APBWIN_RDATA_SPEC>`"]
pub type SLC_APBWIN_RDATA = crate::Reg<slc_apbwin_rdata::SLC_APBWIN_RDATA_SPEC>;
#[doc = "*******Description***********"]
pub mod slc_apbwin_rdata;
#[doc = "RDCLR0 (rw) register accessor: an alias for `Reg<RDCLR0_SPEC>`"]
pub type RDCLR0 = crate::Reg<rdclr0::RDCLR0_SPEC>;
#[doc = "*******Description***********"]
pub mod rdclr0;
#[doc = "RDCLR1 (rw) register accessor: an alias for `Reg<RDCLR1_SPEC>`"]
pub type RDCLR1 = crate::Reg<rdclr1::RDCLR1_SPEC>;
#[doc = "*******Description***********"]
pub mod rdclr1;
#[doc = "SLC0HOST_INT_ENA1 (rw) register accessor: an alias for `Reg<SLC0HOST_INT_ENA1_SPEC>`"]
pub type SLC0HOST_INT_ENA1 = crate::Reg<slc0host_int_ena1::SLC0HOST_INT_ENA1_SPEC>;
#[doc = "*******Description***********"]
pub mod slc0host_int_ena1;
#[doc = "SLC1HOST_INT_ENA1 (rw) register accessor: an alias for `Reg<SLC1HOST_INT_ENA1_SPEC>`"]
pub type SLC1HOST_INT_ENA1 = crate::Reg<slc1host_int_ena1::SLC1HOST_INT_ENA1_SPEC>;
#[doc = "*******Description***********"]
pub mod slc1host_int_ena1;
#[doc = "SLCHOSTDATE (rw) register accessor: an alias for `Reg<SLCHOSTDATE_SPEC>`"]
pub type SLCHOSTDATE = crate::Reg<slchostdate::SLCHOSTDATE_SPEC>;
#[doc = "*******Description***********"]
pub mod slchostdate;
#[doc = "SLCHOSTID (rw) register accessor: an alias for `Reg<SLCHOSTID_SPEC>`"]
pub type SLCHOSTID = crate::Reg<slchostid::SLCHOSTID_SPEC>;
#[doc = "*******Description***********"]
pub mod slchostid;
#[doc = "CONF (rw) register accessor: an alias for `Reg<CONF_SPEC>`"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = "*******Description***********"]
pub mod conf;
#[doc = "INF_ST (rw) register accessor: an alias for `Reg<INF_ST_SPEC>`"]
pub type INF_ST = crate::Reg<inf_st::INF_ST_SPEC>;
#[doc = "*******Description***********"]
pub mod inf_st;
