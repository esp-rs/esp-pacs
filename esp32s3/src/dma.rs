#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - Configure 0 register of Rx channel 0"]
    pub in_conf0_ch0: IN_CONF0_CH,
    #[doc = "0x04 - Configure 1 register of Rx channel 0"]
    pub in_conf1_ch0: IN_CONF1_CH,
    #[doc = "0x08 - Raw status interrupt of Rx channel 0"]
    pub in_int_raw_ch0: IN_INT_RAW_CH,
    #[doc = "0x0c - Masked interrupt of Rx channel 0"]
    pub in_int_st_ch0: IN_INT_ST_CH,
    #[doc = "0x10 - Interrupt enable bits of Rx channel 0"]
    pub in_int_ena_ch0: IN_INT_ENA_CH,
    #[doc = "0x14 - Interrupt clear bits of Rx channel 0"]
    pub in_int_clr_ch0: IN_INT_CLR_CH,
    #[doc = "0x18 - Receive FIFO status of Rx channel 0"]
    pub infifo_status_ch0: INFIFO_STATUS_CH,
    #[doc = "0x1c - Pop control register of Rx channel 0"]
    pub in_pop_ch0: IN_POP_CH,
    #[doc = "0x20 - Link descriptor configure and control register of Rx channel 0"]
    pub in_link_ch0: IN_LINK_CH,
    #[doc = "0x24 - Receive status of Rx channel 0"]
    pub in_state_ch0: IN_STATE_CH,
    #[doc = "0x28 - Inlink descriptor address when EOF occurs of Rx channel 0"]
    pub in_suc_eof_des_addr_ch0: IN_SUC_EOF_DES_ADDR_CH,
    #[doc = "0x2c - Inlink descriptor address when errors occur of Rx channel 0"]
    pub in_err_eof_des_addr_ch0: IN_ERR_EOF_DES_ADDR_CH,
    #[doc = "0x30 - Current inlink descriptor address of Rx channel 0"]
    pub in_dscr_ch0: IN_DSCR_CH,
    #[doc = "0x34 - The last inlink descriptor address of Rx channel 0"]
    pub in_dscr_bf0_ch0: IN_DSCR_BF0_CH,
    #[doc = "0x38 - The second-to-last inlink descriptor address of Rx channel 0"]
    pub in_dscr_bf1_ch0: IN_DSCR_BF1_CH,
    #[doc = "0x3c - Weight register of Rx channel 0"]
    pub in_wight_ch0: IN_WIGHT_CH,
    _reserved16: [u8; 0x04],
    #[doc = "0x44 - Priority register of Rx channel 0"]
    pub in_pri_ch0: IN_PRI_CH,
    #[doc = "0x48 - Peripheral selection of Rx channel 0"]
    pub in_peri_sel_ch0: IN_PERI_SEL_CH,
    _reserved18: [u8; 0x14],
    #[doc = "0x60 - Configure 0 register of Tx channel 0"]
    pub out_conf0_ch0: OUT_CONF0_CH,
    #[doc = "0x64 - Configure 1 register of Tx channel 0"]
    pub out_conf1_ch0: OUT_CONF1_CH,
    #[doc = "0x68 - Raw status interrupt of Tx channel 0"]
    pub out_int_raw_ch0: OUT_INT_RAW_CH,
    #[doc = "0x6c - Masked interrupt of Tx channel 0"]
    pub out_int_st_ch0: OUT_INT_ST_CH,
    #[doc = "0x70 - Interrupt enable bits of Tx channel 0"]
    pub out_int_ena_ch0: OUT_INT_ENA_CH,
    #[doc = "0x74 - Interrupt clear bits of Tx channel 0"]
    pub out_int_clr_ch0: OUT_INT_CLR_CH,
    #[doc = "0x78 - Transmit FIFO status of Tx channel 0"]
    pub outfifo_status_ch0: OUTFIFO_STATUS_CH,
    #[doc = "0x7c - Push control register of Rx channel 0"]
    pub out_push_ch0: OUT_PUSH_CH,
    #[doc = "0x80 - Link descriptor configure and control register of Tx channel 0"]
    pub out_link_ch0: OUT_LINK_CH,
    #[doc = "0x84 - Transmit status of Tx channel 0"]
    pub out_state_ch0: OUT_STATE_CH,
    #[doc = "0x88 - Outlink descriptor address when EOF occurs of Tx channel 0"]
    pub out_eof_des_addr_ch0: OUT_EOF_DES_ADDR_CH,
    #[doc = "0x8c - The last outlink descriptor address when EOF occurs of Tx channel 0"]
    pub out_eof_bfr_des_addr_ch0: OUT_EOF_BFR_DES_ADDR_CH,
    #[doc = "0x90 - Current inlink descriptor address of Tx channel 0"]
    pub out_dscr_ch0: OUT_DSCR_CH,
    #[doc = "0x94 - The last inlink descriptor address of Tx channel 0"]
    pub out_dscr_bf0_ch0: OUT_DSCR_BF0_CH,
    #[doc = "0x98 - The second-to-last inlink descriptor address of Tx channel 0"]
    pub out_dscr_bf1_ch0: OUT_DSCR_BF1_CH,
    #[doc = "0x9c - Weight register of Rx channel 0"]
    pub out_wight_ch0: OUT_WIGHT_CH,
    _reserved34: [u8; 0x04],
    #[doc = "0xa4 - Priority register of Tx channel 0."]
    pub out_pri_ch0: OUT_PRI_CH,
    #[doc = "0xa8 - Peripheral selection of Tx channel 0"]
    pub out_peri_sel_ch0: OUT_PERI_SEL_CH,
    _reserved36: [u8; 0x14],
    #[doc = "0xc0 - Configure 0 register of Rx channel 0"]
    pub in_conf0_ch1: IN_CONF0_CH,
    #[doc = "0xc4 - Configure 1 register of Rx channel 0"]
    pub in_conf1_ch1: IN_CONF1_CH,
    #[doc = "0xc8 - Raw status interrupt of Rx channel 0"]
    pub in_int_raw_ch1: IN_INT_RAW_CH,
    #[doc = "0xcc - Masked interrupt of Rx channel 0"]
    pub in_int_st_ch1: IN_INT_ST_CH,
    #[doc = "0xd0 - Interrupt enable bits of Rx channel 0"]
    pub in_int_ena_ch1: IN_INT_ENA_CH,
    #[doc = "0xd4 - Interrupt clear bits of Rx channel 0"]
    pub in_int_clr_ch1: IN_INT_CLR_CH,
    #[doc = "0xd8 - Receive FIFO status of Rx channel 0"]
    pub infifo_status_ch1: INFIFO_STATUS_CH,
    #[doc = "0xdc - Pop control register of Rx channel 0"]
    pub in_pop_ch1: IN_POP_CH,
    #[doc = "0xe0 - Link descriptor configure and control register of Rx channel 0"]
    pub in_link_ch1: IN_LINK_CH,
    #[doc = "0xe4 - Receive status of Rx channel 0"]
    pub in_state_ch1: IN_STATE_CH,
    #[doc = "0xe8 - Inlink descriptor address when EOF occurs of Rx channel 0"]
    pub in_suc_eof_des_addr_ch1: IN_SUC_EOF_DES_ADDR_CH,
    #[doc = "0xec - Inlink descriptor address when errors occur of Rx channel 0"]
    pub in_err_eof_des_addr_ch1: IN_ERR_EOF_DES_ADDR_CH,
    #[doc = "0xf0 - Current inlink descriptor address of Rx channel 0"]
    pub in_dscr_ch1: IN_DSCR_CH,
    #[doc = "0xf4 - The last inlink descriptor address of Rx channel 0"]
    pub in_dscr_bf0_ch1: IN_DSCR_BF0_CH,
    #[doc = "0xf8 - The second-to-last inlink descriptor address of Rx channel 0"]
    pub in_dscr_bf1_ch1: IN_DSCR_BF1_CH,
    #[doc = "0xfc - Weight register of Rx channel 0"]
    pub in_wight_ch1: IN_WIGHT_CH,
    _reserved52: [u8; 0x04],
    #[doc = "0x104 - Priority register of Rx channel 0"]
    pub in_pri_ch1: IN_PRI_CH,
    #[doc = "0x108 - Peripheral selection of Rx channel 0"]
    pub in_peri_sel_ch1: IN_PERI_SEL_CH,
    _reserved54: [u8; 0x14],
    #[doc = "0x120 - Configure 0 register of Tx channel 0"]
    pub out_conf0_ch1: OUT_CONF0_CH,
    #[doc = "0x124 - Configure 1 register of Tx channel 0"]
    pub out_conf1_ch1: OUT_CONF1_CH,
    #[doc = "0x128 - Raw status interrupt of Tx channel 0"]
    pub out_int_raw_ch1: OUT_INT_RAW_CH,
    #[doc = "0x12c - Masked interrupt of Tx channel 0"]
    pub out_int_st_ch1: OUT_INT_ST_CH,
    #[doc = "0x130 - Interrupt enable bits of Tx channel 0"]
    pub out_int_ena_ch1: OUT_INT_ENA_CH,
    #[doc = "0x134 - Interrupt clear bits of Tx channel 0"]
    pub out_int_clr_ch1: OUT_INT_CLR_CH,
    #[doc = "0x138 - Transmit FIFO status of Tx channel 0"]
    pub outfifo_status_ch1: OUTFIFO_STATUS_CH,
    #[doc = "0x13c - Push control register of Rx channel 0"]
    pub out_push_ch1: OUT_PUSH_CH,
    #[doc = "0x140 - Link descriptor configure and control register of Tx channel 0"]
    pub out_link_ch1: OUT_LINK_CH,
    #[doc = "0x144 - Transmit status of Tx channel 0"]
    pub out_state_ch1: OUT_STATE_CH,
    #[doc = "0x148 - Outlink descriptor address when EOF occurs of Tx channel 0"]
    pub out_eof_des_addr_ch1: OUT_EOF_DES_ADDR_CH,
    #[doc = "0x14c - The last outlink descriptor address when EOF occurs of Tx channel 0"]
    pub out_eof_bfr_des_addr_ch1: OUT_EOF_BFR_DES_ADDR_CH,
    #[doc = "0x150 - Current inlink descriptor address of Tx channel 0"]
    pub out_dscr_ch1: OUT_DSCR_CH,
    #[doc = "0x154 - The last inlink descriptor address of Tx channel 0"]
    pub out_dscr_bf0_ch1: OUT_DSCR_BF0_CH,
    #[doc = "0x158 - The second-to-last inlink descriptor address of Tx channel 0"]
    pub out_dscr_bf1_ch1: OUT_DSCR_BF1_CH,
    #[doc = "0x15c - Weight register of Rx channel 0"]
    pub out_wight_ch1: OUT_WIGHT_CH,
    _reserved70: [u8; 0x04],
    #[doc = "0x164 - Priority register of Tx channel 0."]
    pub out_pri_ch1: OUT_PRI_CH,
    #[doc = "0x168 - Peripheral selection of Tx channel 0"]
    pub out_peri_sel_ch1: OUT_PERI_SEL_CH,
    _reserved72: [u8; 0x14],
    #[doc = "0x180 - Configure 0 register of Rx channel 0"]
    pub in_conf0_ch2: IN_CONF0_CH,
    #[doc = "0x184 - Configure 1 register of Rx channel 0"]
    pub in_conf1_ch2: IN_CONF1_CH,
    #[doc = "0x188 - Raw status interrupt of Rx channel 0"]
    pub in_int_raw_ch2: IN_INT_RAW_CH,
    #[doc = "0x18c - Masked interrupt of Rx channel 0"]
    pub in_int_st_ch2: IN_INT_ST_CH,
    #[doc = "0x190 - Interrupt enable bits of Rx channel 0"]
    pub in_int_ena_ch2: IN_INT_ENA_CH,
    #[doc = "0x194 - Interrupt clear bits of Rx channel 0"]
    pub in_int_clr_ch2: IN_INT_CLR_CH,
    #[doc = "0x198 - Receive FIFO status of Rx channel 0"]
    pub infifo_status_ch2: INFIFO_STATUS_CH,
    #[doc = "0x19c - Pop control register of Rx channel 0"]
    pub in_pop_ch2: IN_POP_CH,
    #[doc = "0x1a0 - Link descriptor configure and control register of Rx channel 0"]
    pub in_link_ch2: IN_LINK_CH,
    #[doc = "0x1a4 - Receive status of Rx channel 0"]
    pub in_state_ch2: IN_STATE_CH,
    #[doc = "0x1a8 - Inlink descriptor address when EOF occurs of Rx channel 0"]
    pub in_suc_eof_des_addr_ch2: IN_SUC_EOF_DES_ADDR_CH,
    #[doc = "0x1ac - Inlink descriptor address when errors occur of Rx channel 0"]
    pub in_err_eof_des_addr_ch2: IN_ERR_EOF_DES_ADDR_CH,
    #[doc = "0x1b0 - Current inlink descriptor address of Rx channel 0"]
    pub in_dscr_ch2: IN_DSCR_CH,
    #[doc = "0x1b4 - The last inlink descriptor address of Rx channel 0"]
    pub in_dscr_bf0_ch2: IN_DSCR_BF0_CH,
    #[doc = "0x1b8 - The second-to-last inlink descriptor address of Rx channel 0"]
    pub in_dscr_bf1_ch2: IN_DSCR_BF1_CH,
    #[doc = "0x1bc - Weight register of Rx channel 0"]
    pub in_wight_ch2: IN_WIGHT_CH,
    _reserved88: [u8; 0x04],
    #[doc = "0x1c4 - Priority register of Rx channel 0"]
    pub in_pri_ch2: IN_PRI_CH,
    #[doc = "0x1c8 - Peripheral selection of Rx channel 0"]
    pub in_peri_sel_ch2: IN_PERI_SEL_CH,
    _reserved90: [u8; 0x14],
    #[doc = "0x1e0 - Configure 0 register of Tx channel 0"]
    pub out_conf0_ch2: OUT_CONF0_CH,
    #[doc = "0x1e4 - Configure 1 register of Tx channel 0"]
    pub out_conf1_ch2: OUT_CONF1_CH,
    #[doc = "0x1e8 - Raw status interrupt of Tx channel 0"]
    pub out_int_raw_ch2: OUT_INT_RAW_CH,
    #[doc = "0x1ec - Masked interrupt of Tx channel 0"]
    pub out_int_st_ch2: OUT_INT_ST_CH,
    #[doc = "0x1f0 - Interrupt enable bits of Tx channel 0"]
    pub out_int_ena_ch2: OUT_INT_ENA_CH,
    #[doc = "0x1f4 - Interrupt clear bits of Tx channel 0"]
    pub out_int_clr_ch2: OUT_INT_CLR_CH,
    #[doc = "0x1f8 - Transmit FIFO status of Tx channel 0"]
    pub outfifo_status_ch2: OUTFIFO_STATUS_CH,
    #[doc = "0x1fc - Push control register of Rx channel 0"]
    pub out_push_ch2: OUT_PUSH_CH,
    #[doc = "0x200 - Link descriptor configure and control register of Tx channel 0"]
    pub out_link_ch2: OUT_LINK_CH,
    #[doc = "0x204 - Transmit status of Tx channel 0"]
    pub out_state_ch2: OUT_STATE_CH,
    #[doc = "0x208 - Outlink descriptor address when EOF occurs of Tx channel 0"]
    pub out_eof_des_addr_ch2: OUT_EOF_DES_ADDR_CH,
    #[doc = "0x20c - The last outlink descriptor address when EOF occurs of Tx channel 0"]
    pub out_eof_bfr_des_addr_ch2: OUT_EOF_BFR_DES_ADDR_CH,
    #[doc = "0x210 - Current inlink descriptor address of Tx channel 0"]
    pub out_dscr_ch2: OUT_DSCR_CH,
    #[doc = "0x214 - The last inlink descriptor address of Tx channel 0"]
    pub out_dscr_bf0_ch2: OUT_DSCR_BF0_CH,
    #[doc = "0x218 - The second-to-last inlink descriptor address of Tx channel 0"]
    pub out_dscr_bf1_ch2: OUT_DSCR_BF1_CH,
    #[doc = "0x21c - Weight register of Rx channel 0"]
    pub out_wight_ch2: OUT_WIGHT_CH,
    _reserved106: [u8; 0x04],
    #[doc = "0x224 - Priority register of Tx channel 0."]
    pub out_pri_ch2: OUT_PRI_CH,
    #[doc = "0x228 - Peripheral selection of Tx channel 0"]
    pub out_peri_sel_ch2: OUT_PERI_SEL_CH,
    _reserved108: [u8; 0x14],
    #[doc = "0x240 - Configure 0 register of Rx channel 0"]
    pub in_conf0_ch3: IN_CONF0_CH,
    #[doc = "0x244 - Configure 1 register of Rx channel 0"]
    pub in_conf1_ch3: IN_CONF1_CH,
    #[doc = "0x248 - Raw status interrupt of Rx channel 0"]
    pub in_int_raw_ch3: IN_INT_RAW_CH,
    #[doc = "0x24c - Masked interrupt of Rx channel 0"]
    pub in_int_st_ch3: IN_INT_ST_CH,
    #[doc = "0x250 - Interrupt enable bits of Rx channel 0"]
    pub in_int_ena_ch3: IN_INT_ENA_CH,
    #[doc = "0x254 - Interrupt clear bits of Rx channel 0"]
    pub in_int_clr_ch3: IN_INT_CLR_CH,
    #[doc = "0x258 - Receive FIFO status of Rx channel 0"]
    pub infifo_status_ch3: INFIFO_STATUS_CH,
    #[doc = "0x25c - Pop control register of Rx channel 0"]
    pub in_pop_ch3: IN_POP_CH,
    #[doc = "0x260 - Link descriptor configure and control register of Rx channel 0"]
    pub in_link_ch3: IN_LINK_CH,
    #[doc = "0x264 - Receive status of Rx channel 0"]
    pub in_state_ch3: IN_STATE_CH,
    #[doc = "0x268 - Inlink descriptor address when EOF occurs of Rx channel 0"]
    pub in_suc_eof_des_addr_ch3: IN_SUC_EOF_DES_ADDR_CH,
    #[doc = "0x26c - Inlink descriptor address when errors occur of Rx channel 0"]
    pub in_err_eof_des_addr_ch3: IN_ERR_EOF_DES_ADDR_CH,
    #[doc = "0x270 - Current inlink descriptor address of Rx channel 0"]
    pub in_dscr_ch3: IN_DSCR_CH,
    #[doc = "0x274 - The last inlink descriptor address of Rx channel 0"]
    pub in_dscr_bf0_ch3: IN_DSCR_BF0_CH,
    #[doc = "0x278 - The second-to-last inlink descriptor address of Rx channel 0"]
    pub in_dscr_bf1_ch3: IN_DSCR_BF1_CH,
    #[doc = "0x27c - Weight register of Rx channel 0"]
    pub in_wight_ch3: IN_WIGHT_CH,
    _reserved124: [u8; 0x04],
    #[doc = "0x284 - Priority register of Rx channel 0"]
    pub in_pri_ch3: IN_PRI_CH,
    #[doc = "0x288 - Peripheral selection of Rx channel 0"]
    pub in_peri_sel_ch3: IN_PERI_SEL_CH,
    _reserved126: [u8; 0x14],
    #[doc = "0x2a0 - Configure 0 register of Tx channel 0"]
    pub out_conf0_ch3: OUT_CONF0_CH,
    #[doc = "0x2a4 - Configure 1 register of Tx channel 0"]
    pub out_conf1_ch3: OUT_CONF1_CH,
    #[doc = "0x2a8 - Raw status interrupt of Tx channel 0"]
    pub out_int_raw_ch3: OUT_INT_RAW_CH,
    #[doc = "0x2ac - Masked interrupt of Tx channel 0"]
    pub out_int_st_ch3: OUT_INT_ST_CH,
    #[doc = "0x2b0 - Interrupt enable bits of Tx channel 0"]
    pub out_int_ena_ch3: OUT_INT_ENA_CH,
    #[doc = "0x2b4 - Interrupt clear bits of Tx channel 0"]
    pub out_int_clr_ch3: OUT_INT_CLR_CH,
    #[doc = "0x2b8 - Transmit FIFO status of Tx channel 0"]
    pub outfifo_status_ch3: OUTFIFO_STATUS_CH,
    #[doc = "0x2bc - Push control register of Rx channel 0"]
    pub out_push_ch3: OUT_PUSH_CH,
    #[doc = "0x2c0 - Link descriptor configure and control register of Tx channel 0"]
    pub out_link_ch3: OUT_LINK_CH,
    #[doc = "0x2c4 - Transmit status of Tx channel 0"]
    pub out_state_ch3: OUT_STATE_CH,
    #[doc = "0x2c8 - Outlink descriptor address when EOF occurs of Tx channel 0"]
    pub out_eof_des_addr_ch3: OUT_EOF_DES_ADDR_CH,
    #[doc = "0x2cc - The last outlink descriptor address when EOF occurs of Tx channel 0"]
    pub out_eof_bfr_des_addr_ch3: OUT_EOF_BFR_DES_ADDR_CH,
    #[doc = "0x2d0 - Current inlink descriptor address of Tx channel 0"]
    pub out_dscr_ch3: OUT_DSCR_CH,
    #[doc = "0x2d4 - The last inlink descriptor address of Tx channel 0"]
    pub out_dscr_bf0_ch3: OUT_DSCR_BF0_CH,
    #[doc = "0x2d8 - The second-to-last inlink descriptor address of Tx channel 0"]
    pub out_dscr_bf1_ch3: OUT_DSCR_BF1_CH,
    #[doc = "0x2dc - Weight register of Rx channel 0"]
    pub out_wight_ch3: OUT_WIGHT_CH,
    _reserved142: [u8; 0x04],
    #[doc = "0x2e4 - Priority register of Tx channel 0."]
    pub out_pri_ch3: OUT_PRI_CH,
    #[doc = "0x2e8 - Peripheral selection of Tx channel 0"]
    pub out_peri_sel_ch3: OUT_PERI_SEL_CH,
    _reserved144: [u8; 0x14],
    #[doc = "0x300 - Configure 0 register of Rx channel 0"]
    pub in_conf0_ch4: IN_CONF0_CH,
    #[doc = "0x304 - Configure 1 register of Rx channel 0"]
    pub in_conf1_ch4: IN_CONF1_CH,
    #[doc = "0x308 - Raw status interrupt of Rx channel 0"]
    pub in_int_raw_ch4: IN_INT_RAW_CH,
    #[doc = "0x30c - Masked interrupt of Rx channel 0"]
    pub in_int_st_ch4: IN_INT_ST_CH,
    #[doc = "0x310 - Interrupt enable bits of Rx channel 0"]
    pub in_int_ena_ch4: IN_INT_ENA_CH,
    #[doc = "0x314 - Interrupt clear bits of Rx channel 0"]
    pub in_int_clr_ch4: IN_INT_CLR_CH,
    #[doc = "0x318 - Receive FIFO status of Rx channel 0"]
    pub infifo_status_ch4: INFIFO_STATUS_CH,
    #[doc = "0x31c - Pop control register of Rx channel 0"]
    pub in_pop_ch4: IN_POP_CH,
    #[doc = "0x320 - Link descriptor configure and control register of Rx channel 0"]
    pub in_link_ch4: IN_LINK_CH,
    #[doc = "0x324 - Receive status of Rx channel 0"]
    pub in_state_ch4: IN_STATE_CH,
    #[doc = "0x328 - Inlink descriptor address when EOF occurs of Rx channel 0"]
    pub in_suc_eof_des_addr_ch4: IN_SUC_EOF_DES_ADDR_CH,
    #[doc = "0x32c - Inlink descriptor address when errors occur of Rx channel 0"]
    pub in_err_eof_des_addr_ch4: IN_ERR_EOF_DES_ADDR_CH,
    #[doc = "0x330 - Current inlink descriptor address of Rx channel 0"]
    pub in_dscr_ch4: IN_DSCR_CH,
    #[doc = "0x334 - The last inlink descriptor address of Rx channel 0"]
    pub in_dscr_bf0_ch4: IN_DSCR_BF0_CH,
    #[doc = "0x338 - The second-to-last inlink descriptor address of Rx channel 0"]
    pub in_dscr_bf1_ch4: IN_DSCR_BF1_CH,
    #[doc = "0x33c - Weight register of Rx channel 0"]
    pub in_wight_ch4: IN_WIGHT_CH,
    _reserved160: [u8; 0x04],
    #[doc = "0x344 - Priority register of Rx channel 0"]
    pub in_pri_ch4: IN_PRI_CH,
    #[doc = "0x348 - Peripheral selection of Rx channel 0"]
    pub in_peri_sel_ch4: IN_PERI_SEL_CH,
    _reserved162: [u8; 0x14],
    #[doc = "0x360 - Configure 0 register of Tx channel 0"]
    pub out_conf0_ch4: OUT_CONF0_CH,
    #[doc = "0x364 - Configure 1 register of Tx channel 0"]
    pub out_conf1_ch4: OUT_CONF1_CH,
    #[doc = "0x368 - Raw status interrupt of Tx channel 0"]
    pub out_int_raw_ch4: OUT_INT_RAW_CH,
    #[doc = "0x36c - Masked interrupt of Tx channel 0"]
    pub out_int_st_ch4: OUT_INT_ST_CH,
    #[doc = "0x370 - Interrupt enable bits of Tx channel 0"]
    pub out_int_ena_ch4: OUT_INT_ENA_CH,
    #[doc = "0x374 - Interrupt clear bits of Tx channel 0"]
    pub out_int_clr_ch4: OUT_INT_CLR_CH,
    #[doc = "0x378 - Transmit FIFO status of Tx channel 0"]
    pub outfifo_status_ch4: OUTFIFO_STATUS_CH,
    #[doc = "0x37c - Push control register of Rx channel 0"]
    pub out_push_ch4: OUT_PUSH_CH,
    #[doc = "0x380 - Link descriptor configure and control register of Tx channel 0"]
    pub out_link_ch4: OUT_LINK_CH,
    #[doc = "0x384 - Transmit status of Tx channel 0"]
    pub out_state_ch4: OUT_STATE_CH,
    #[doc = "0x388 - Outlink descriptor address when EOF occurs of Tx channel 0"]
    pub out_eof_des_addr_ch4: OUT_EOF_DES_ADDR_CH,
    #[doc = "0x38c - The last outlink descriptor address when EOF occurs of Tx channel 0"]
    pub out_eof_bfr_des_addr_ch4: OUT_EOF_BFR_DES_ADDR_CH,
    #[doc = "0x390 - Current inlink descriptor address of Tx channel 0"]
    pub out_dscr_ch4: OUT_DSCR_CH,
    #[doc = "0x394 - The last inlink descriptor address of Tx channel 0"]
    pub out_dscr_bf0_ch4: OUT_DSCR_BF0_CH,
    #[doc = "0x398 - The second-to-last inlink descriptor address of Tx channel 0"]
    pub out_dscr_bf1_ch4: OUT_DSCR_BF1_CH,
    #[doc = "0x39c - Weight register of Rx channel 0"]
    pub out_wight_ch4: OUT_WIGHT_CH,
    _reserved178: [u8; 0x04],
    #[doc = "0x3a4 - Priority register of Tx channel 0."]
    pub out_pri_ch4: OUT_PRI_CH,
    #[doc = "0x3a8 - Peripheral selection of Tx channel 0"]
    pub out_peri_sel_ch4: OUT_PERI_SEL_CH,
    _reserved180: [u8; 0x14],
    #[doc = "0x3c0 - reserved"]
    pub ahb_test: AHB_TEST,
    #[doc = "0x3c4 - reserved"]
    pub pd_conf: PD_CONF,
    #[doc = "0x3c8 - MISC register"]
    pub misc_conf: MISC_CONF,
    #[doc = "0x3cc - Receive L2 FIFO depth of Rx channel 0"]
    pub in_sram_size_ch0: IN_SRAM_SIZE_CH,
    #[doc = "0x3d0 - Transmit L2 FIFO depth of Tx channel 0"]
    pub out_sram_size_ch0: OUT_SRAM_SIZE_CH,
    #[doc = "0x3d4 - Receive L2 FIFO depth of Rx channel 0"]
    pub in_sram_size_ch1: IN_SRAM_SIZE_CH,
    #[doc = "0x3d8 - Transmit L2 FIFO depth of Tx channel 0"]
    pub out_sram_size_ch1: OUT_SRAM_SIZE_CH,
    #[doc = "0x3dc - Receive L2 FIFO depth of Rx channel 0"]
    pub in_sram_size_ch2: IN_SRAM_SIZE_CH,
    #[doc = "0x3e0 - Transmit L2 FIFO depth of Tx channel 0"]
    pub out_sram_size_ch2: OUT_SRAM_SIZE_CH,
    #[doc = "0x3e4 - Receive L2 FIFO depth of Rx channel 0"]
    pub in_sram_size_ch3: IN_SRAM_SIZE_CH,
    #[doc = "0x3e8 - Transmit L2 FIFO depth of Tx channel 0"]
    pub out_sram_size_ch3: OUT_SRAM_SIZE_CH,
    #[doc = "0x3ec - Receive L2 FIFO depth of Rx channel 0"]
    pub in_sram_size_ch4: IN_SRAM_SIZE_CH,
    #[doc = "0x3f0 - Transmit L2 FIFO depth of Tx channel 0"]
    pub out_sram_size_ch4: OUT_SRAM_SIZE_CH,
    #[doc = "0x3f4 - Reject address accessing external RAM"]
    pub extmem_reject_addr: EXTMEM_REJECT_ADDR,
    #[doc = "0x3f8 - Reject status accessing external RAM"]
    pub extmem_reject_st: EXTMEM_REJECT_ST,
    #[doc = "0x3fc - Raw interrupt status of external RAM permission"]
    pub extmem_reject_int_raw: EXTMEM_REJECT_INT_RAW,
    #[doc = "0x400 - Masked interrupt status of external RAM permission"]
    pub extmem_reject_int_st: EXTMEM_REJECT_INT_ST,
    #[doc = "0x404 - Interrupt enable bits of external RAM permission"]
    pub extmem_reject_int_ena: EXTMEM_REJECT_INT_ENA,
    #[doc = "0x408 - Interrupt clear bits of external RAM permission"]
    pub extmem_reject_int_clr: EXTMEM_REJECT_INT_CLR,
    #[doc = "0x40c - Version control register"]
    pub date: DATE,
}
#[doc = "IN_CONF0_CH (rw) register accessor: an alias for `Reg<IN_CONF0_CH_SPEC>`"]
pub type IN_CONF0_CH = crate::Reg<in_conf0_ch::IN_CONF0_CH_SPEC>;
#[doc = "Configure 0 register of Rx channel 0"]
pub mod in_conf0_ch;
#[doc = "IN_CONF1_CH (rw) register accessor: an alias for `Reg<IN_CONF1_CH_SPEC>`"]
pub type IN_CONF1_CH = crate::Reg<in_conf1_ch::IN_CONF1_CH_SPEC>;
#[doc = "Configure 1 register of Rx channel 0"]
pub mod in_conf1_ch;
#[doc = "IN_INT_RAW_CH (rw) register accessor: an alias for `Reg<IN_INT_RAW_CH_SPEC>`"]
pub type IN_INT_RAW_CH = crate::Reg<in_int_raw_ch::IN_INT_RAW_CH_SPEC>;
#[doc = "Raw status interrupt of Rx channel 0"]
pub mod in_int_raw_ch;
#[doc = "IN_INT_ST_CH (r) register accessor: an alias for `Reg<IN_INT_ST_CH_SPEC>`"]
pub type IN_INT_ST_CH = crate::Reg<in_int_st_ch::IN_INT_ST_CH_SPEC>;
#[doc = "Masked interrupt of Rx channel 0"]
pub mod in_int_st_ch;
#[doc = "IN_INT_ENA_CH (rw) register accessor: an alias for `Reg<IN_INT_ENA_CH_SPEC>`"]
pub type IN_INT_ENA_CH = crate::Reg<in_int_ena_ch::IN_INT_ENA_CH_SPEC>;
#[doc = "Interrupt enable bits of Rx channel 0"]
pub mod in_int_ena_ch;
#[doc = "IN_INT_CLR_CH (w) register accessor: an alias for `Reg<IN_INT_CLR_CH_SPEC>`"]
pub type IN_INT_CLR_CH = crate::Reg<in_int_clr_ch::IN_INT_CLR_CH_SPEC>;
#[doc = "Interrupt clear bits of Rx channel 0"]
pub mod in_int_clr_ch;
#[doc = "INFIFO_STATUS_CH (r) register accessor: an alias for `Reg<INFIFO_STATUS_CH_SPEC>`"]
pub type INFIFO_STATUS_CH = crate::Reg<infifo_status_ch::INFIFO_STATUS_CH_SPEC>;
#[doc = "Receive FIFO status of Rx channel 0"]
pub mod infifo_status_ch;
#[doc = "IN_POP_CH (rw) register accessor: an alias for `Reg<IN_POP_CH_SPEC>`"]
pub type IN_POP_CH = crate::Reg<in_pop_ch::IN_POP_CH_SPEC>;
#[doc = "Pop control register of Rx channel 0"]
pub mod in_pop_ch;
#[doc = "IN_LINK_CH (rw) register accessor: an alias for `Reg<IN_LINK_CH_SPEC>`"]
pub type IN_LINK_CH = crate::Reg<in_link_ch::IN_LINK_CH_SPEC>;
#[doc = "Link descriptor configure and control register of Rx channel 0"]
pub mod in_link_ch;
#[doc = "IN_STATE_CH (r) register accessor: an alias for `Reg<IN_STATE_CH_SPEC>`"]
pub type IN_STATE_CH = crate::Reg<in_state_ch::IN_STATE_CH_SPEC>;
#[doc = "Receive status of Rx channel 0"]
pub mod in_state_ch;
#[doc = "IN_SUC_EOF_DES_ADDR_CH (r) register accessor: an alias for `Reg<IN_SUC_EOF_DES_ADDR_CH_SPEC>`"]
pub type IN_SUC_EOF_DES_ADDR_CH = crate::Reg<in_suc_eof_des_addr_ch::IN_SUC_EOF_DES_ADDR_CH_SPEC>;
#[doc = "Inlink descriptor address when EOF occurs of Rx channel 0"]
pub mod in_suc_eof_des_addr_ch;
#[doc = "IN_ERR_EOF_DES_ADDR_CH (r) register accessor: an alias for `Reg<IN_ERR_EOF_DES_ADDR_CH_SPEC>`"]
pub type IN_ERR_EOF_DES_ADDR_CH = crate::Reg<in_err_eof_des_addr_ch::IN_ERR_EOF_DES_ADDR_CH_SPEC>;
#[doc = "Inlink descriptor address when errors occur of Rx channel 0"]
pub mod in_err_eof_des_addr_ch;
#[doc = "IN_DSCR_CH (r) register accessor: an alias for `Reg<IN_DSCR_CH_SPEC>`"]
pub type IN_DSCR_CH = crate::Reg<in_dscr_ch::IN_DSCR_CH_SPEC>;
#[doc = "Current inlink descriptor address of Rx channel 0"]
pub mod in_dscr_ch;
#[doc = "IN_DSCR_BF0_CH (r) register accessor: an alias for `Reg<IN_DSCR_BF0_CH_SPEC>`"]
pub type IN_DSCR_BF0_CH = crate::Reg<in_dscr_bf0_ch::IN_DSCR_BF0_CH_SPEC>;
#[doc = "The last inlink descriptor address of Rx channel 0"]
pub mod in_dscr_bf0_ch;
#[doc = "IN_DSCR_BF1_CH (r) register accessor: an alias for `Reg<IN_DSCR_BF1_CH_SPEC>`"]
pub type IN_DSCR_BF1_CH = crate::Reg<in_dscr_bf1_ch::IN_DSCR_BF1_CH_SPEC>;
#[doc = "The second-to-last inlink descriptor address of Rx channel 0"]
pub mod in_dscr_bf1_ch;
#[doc = "IN_WIGHT_CH (rw) register accessor: an alias for `Reg<IN_WIGHT_CH_SPEC>`"]
pub type IN_WIGHT_CH = crate::Reg<in_wight_ch::IN_WIGHT_CH_SPEC>;
#[doc = "Weight register of Rx channel 0"]
pub mod in_wight_ch;
#[doc = "IN_PRI_CH (rw) register accessor: an alias for `Reg<IN_PRI_CH_SPEC>`"]
pub type IN_PRI_CH = crate::Reg<in_pri_ch::IN_PRI_CH_SPEC>;
#[doc = "Priority register of Rx channel 0"]
pub mod in_pri_ch;
#[doc = "IN_PERI_SEL_CH (rw) register accessor: an alias for `Reg<IN_PERI_SEL_CH_SPEC>`"]
pub type IN_PERI_SEL_CH = crate::Reg<in_peri_sel_ch::IN_PERI_SEL_CH_SPEC>;
#[doc = "Peripheral selection of Rx channel 0"]
pub mod in_peri_sel_ch;
#[doc = "OUT_CONF0_CH (rw) register accessor: an alias for `Reg<OUT_CONF0_CH_SPEC>`"]
pub type OUT_CONF0_CH = crate::Reg<out_conf0_ch::OUT_CONF0_CH_SPEC>;
#[doc = "Configure 0 register of Tx channel 0"]
pub mod out_conf0_ch;
#[doc = "OUT_CONF1_CH (rw) register accessor: an alias for `Reg<OUT_CONF1_CH_SPEC>`"]
pub type OUT_CONF1_CH = crate::Reg<out_conf1_ch::OUT_CONF1_CH_SPEC>;
#[doc = "Configure 1 register of Tx channel 0"]
pub mod out_conf1_ch;
#[doc = "OUT_INT_RAW_CH (rw) register accessor: an alias for `Reg<OUT_INT_RAW_CH_SPEC>`"]
pub type OUT_INT_RAW_CH = crate::Reg<out_int_raw_ch::OUT_INT_RAW_CH_SPEC>;
#[doc = "Raw status interrupt of Tx channel 0"]
pub mod out_int_raw_ch;
#[doc = "OUT_INT_ST_CH (r) register accessor: an alias for `Reg<OUT_INT_ST_CH_SPEC>`"]
pub type OUT_INT_ST_CH = crate::Reg<out_int_st_ch::OUT_INT_ST_CH_SPEC>;
#[doc = "Masked interrupt of Tx channel 0"]
pub mod out_int_st_ch;
#[doc = "OUT_INT_ENA_CH (rw) register accessor: an alias for `Reg<OUT_INT_ENA_CH_SPEC>`"]
pub type OUT_INT_ENA_CH = crate::Reg<out_int_ena_ch::OUT_INT_ENA_CH_SPEC>;
#[doc = "Interrupt enable bits of Tx channel 0"]
pub mod out_int_ena_ch;
#[doc = "OUT_INT_CLR_CH (w) register accessor: an alias for `Reg<OUT_INT_CLR_CH_SPEC>`"]
pub type OUT_INT_CLR_CH = crate::Reg<out_int_clr_ch::OUT_INT_CLR_CH_SPEC>;
#[doc = "Interrupt clear bits of Tx channel 0"]
pub mod out_int_clr_ch;
#[doc = "OUTFIFO_STATUS_CH (r) register accessor: an alias for `Reg<OUTFIFO_STATUS_CH_SPEC>`"]
pub type OUTFIFO_STATUS_CH = crate::Reg<outfifo_status_ch::OUTFIFO_STATUS_CH_SPEC>;
#[doc = "Transmit FIFO status of Tx channel 0"]
pub mod outfifo_status_ch;
#[doc = "OUT_PUSH_CH (rw) register accessor: an alias for `Reg<OUT_PUSH_CH_SPEC>`"]
pub type OUT_PUSH_CH = crate::Reg<out_push_ch::OUT_PUSH_CH_SPEC>;
#[doc = "Push control register of Rx channel 0"]
pub mod out_push_ch;
#[doc = "OUT_LINK_CH (rw) register accessor: an alias for `Reg<OUT_LINK_CH_SPEC>`"]
pub type OUT_LINK_CH = crate::Reg<out_link_ch::OUT_LINK_CH_SPEC>;
#[doc = "Link descriptor configure and control register of Tx channel 0"]
pub mod out_link_ch;
#[doc = "OUT_STATE_CH (r) register accessor: an alias for `Reg<OUT_STATE_CH_SPEC>`"]
pub type OUT_STATE_CH = crate::Reg<out_state_ch::OUT_STATE_CH_SPEC>;
#[doc = "Transmit status of Tx channel 0"]
pub mod out_state_ch;
#[doc = "OUT_EOF_DES_ADDR_CH (r) register accessor: an alias for `Reg<OUT_EOF_DES_ADDR_CH_SPEC>`"]
pub type OUT_EOF_DES_ADDR_CH = crate::Reg<out_eof_des_addr_ch::OUT_EOF_DES_ADDR_CH_SPEC>;
#[doc = "Outlink descriptor address when EOF occurs of Tx channel 0"]
pub mod out_eof_des_addr_ch;
#[doc = "OUT_EOF_BFR_DES_ADDR_CH (r) register accessor: an alias for `Reg<OUT_EOF_BFR_DES_ADDR_CH_SPEC>`"]
pub type OUT_EOF_BFR_DES_ADDR_CH =
    crate::Reg<out_eof_bfr_des_addr_ch::OUT_EOF_BFR_DES_ADDR_CH_SPEC>;
#[doc = "The last outlink descriptor address when EOF occurs of Tx channel 0"]
pub mod out_eof_bfr_des_addr_ch;
#[doc = "OUT_DSCR_CH (r) register accessor: an alias for `Reg<OUT_DSCR_CH_SPEC>`"]
pub type OUT_DSCR_CH = crate::Reg<out_dscr_ch::OUT_DSCR_CH_SPEC>;
#[doc = "Current inlink descriptor address of Tx channel 0"]
pub mod out_dscr_ch;
#[doc = "OUT_DSCR_BF0_CH (r) register accessor: an alias for `Reg<OUT_DSCR_BF0_CH_SPEC>`"]
pub type OUT_DSCR_BF0_CH = crate::Reg<out_dscr_bf0_ch::OUT_DSCR_BF0_CH_SPEC>;
#[doc = "The last inlink descriptor address of Tx channel 0"]
pub mod out_dscr_bf0_ch;
#[doc = "OUT_DSCR_BF1_CH (r) register accessor: an alias for `Reg<OUT_DSCR_BF1_CH_SPEC>`"]
pub type OUT_DSCR_BF1_CH = crate::Reg<out_dscr_bf1_ch::OUT_DSCR_BF1_CH_SPEC>;
#[doc = "The second-to-last inlink descriptor address of Tx channel 0"]
pub mod out_dscr_bf1_ch;
#[doc = "OUT_WIGHT_CH (rw) register accessor: an alias for `Reg<OUT_WIGHT_CH_SPEC>`"]
pub type OUT_WIGHT_CH = crate::Reg<out_wight_ch::OUT_WIGHT_CH_SPEC>;
#[doc = "Weight register of Rx channel 0"]
pub mod out_wight_ch;
#[doc = "OUT_PRI_CH (rw) register accessor: an alias for `Reg<OUT_PRI_CH_SPEC>`"]
pub type OUT_PRI_CH = crate::Reg<out_pri_ch::OUT_PRI_CH_SPEC>;
#[doc = "Priority register of Tx channel 0."]
pub mod out_pri_ch;
#[doc = "OUT_PERI_SEL_CH (rw) register accessor: an alias for `Reg<OUT_PERI_SEL_CH_SPEC>`"]
pub type OUT_PERI_SEL_CH = crate::Reg<out_peri_sel_ch::OUT_PERI_SEL_CH_SPEC>;
#[doc = "Peripheral selection of Tx channel 0"]
pub mod out_peri_sel_ch;
#[doc = "AHB_TEST (rw) register accessor: an alias for `Reg<AHB_TEST_SPEC>`"]
pub type AHB_TEST = crate::Reg<ahb_test::AHB_TEST_SPEC>;
#[doc = "reserved"]
pub mod ahb_test;
#[doc = "PD_CONF (rw) register accessor: an alias for `Reg<PD_CONF_SPEC>`"]
pub type PD_CONF = crate::Reg<pd_conf::PD_CONF_SPEC>;
#[doc = "reserved"]
pub mod pd_conf;
#[doc = "MISC_CONF (rw) register accessor: an alias for `Reg<MISC_CONF_SPEC>`"]
pub type MISC_CONF = crate::Reg<misc_conf::MISC_CONF_SPEC>;
#[doc = "MISC register"]
pub mod misc_conf;
#[doc = "IN_SRAM_SIZE_CH (rw) register accessor: an alias for `Reg<IN_SRAM_SIZE_CH_SPEC>`"]
pub type IN_SRAM_SIZE_CH = crate::Reg<in_sram_size_ch::IN_SRAM_SIZE_CH_SPEC>;
#[doc = "Receive L2 FIFO depth of Rx channel 0"]
pub mod in_sram_size_ch;
#[doc = "OUT_SRAM_SIZE_CH (rw) register accessor: an alias for `Reg<OUT_SRAM_SIZE_CH_SPEC>`"]
pub type OUT_SRAM_SIZE_CH = crate::Reg<out_sram_size_ch::OUT_SRAM_SIZE_CH_SPEC>;
#[doc = "Transmit L2 FIFO depth of Tx channel 0"]
pub mod out_sram_size_ch;
#[doc = "EXTMEM_REJECT_ADDR (r) register accessor: an alias for `Reg<EXTMEM_REJECT_ADDR_SPEC>`"]
pub type EXTMEM_REJECT_ADDR = crate::Reg<extmem_reject_addr::EXTMEM_REJECT_ADDR_SPEC>;
#[doc = "Reject address accessing external RAM"]
pub mod extmem_reject_addr;
#[doc = "EXTMEM_REJECT_ST (r) register accessor: an alias for `Reg<EXTMEM_REJECT_ST_SPEC>`"]
pub type EXTMEM_REJECT_ST = crate::Reg<extmem_reject_st::EXTMEM_REJECT_ST_SPEC>;
#[doc = "Reject status accessing external RAM"]
pub mod extmem_reject_st;
#[doc = "EXTMEM_REJECT_INT_RAW (rw) register accessor: an alias for `Reg<EXTMEM_REJECT_INT_RAW_SPEC>`"]
pub type EXTMEM_REJECT_INT_RAW = crate::Reg<extmem_reject_int_raw::EXTMEM_REJECT_INT_RAW_SPEC>;
#[doc = "Raw interrupt status of external RAM permission"]
pub mod extmem_reject_int_raw;
#[doc = "EXTMEM_REJECT_INT_ST (r) register accessor: an alias for `Reg<EXTMEM_REJECT_INT_ST_SPEC>`"]
pub type EXTMEM_REJECT_INT_ST = crate::Reg<extmem_reject_int_st::EXTMEM_REJECT_INT_ST_SPEC>;
#[doc = "Masked interrupt status of external RAM permission"]
pub mod extmem_reject_int_st;
#[doc = "EXTMEM_REJECT_INT_ENA (rw) register accessor: an alias for `Reg<EXTMEM_REJECT_INT_ENA_SPEC>`"]
pub type EXTMEM_REJECT_INT_ENA = crate::Reg<extmem_reject_int_ena::EXTMEM_REJECT_INT_ENA_SPEC>;
#[doc = "Interrupt enable bits of external RAM permission"]
pub mod extmem_reject_int_ena;
#[doc = "EXTMEM_REJECT_INT_CLR (w) register accessor: an alias for `Reg<EXTMEM_REJECT_INT_CLR_SPEC>`"]
pub type EXTMEM_REJECT_INT_CLR = crate::Reg<extmem_reject_int_clr::EXTMEM_REJECT_INT_CLR_SPEC>;
#[doc = "Interrupt clear bits of external RAM permission"]
pub mod extmem_reject_int_clr;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Version control register"]
pub mod date;
