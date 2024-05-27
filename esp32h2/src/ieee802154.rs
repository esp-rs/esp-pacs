#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    command: COMMAND,
    ctrl_cfg: CTRL_CFG,
    inf0_short_addr: INF0_SHORT_ADDR,
    inf0_pan_id: INF0_PAN_ID,
    inf0_extend_addr0: INF0_EXTEND_ADDR0,
    inf0_extend_addr1: INF0_EXTEND_ADDR1,
    inf1_short_addr: INF1_SHORT_ADDR,
    inf1_pan_id: INF1_PAN_ID,
    inf1_extend_addr0: INF1_EXTEND_ADDR0,
    inf1_extend_addr1: INF1_EXTEND_ADDR1,
    inf2_short_addr: INF2_SHORT_ADDR,
    inf2_pan_id: INF2_PAN_ID,
    inf2_extend_addr0: INF2_EXTEND_ADDR0,
    inf2_extend_addr1: INF2_EXTEND_ADDR1,
    inf3_short_addr: INF3_SHORT_ADDR,
    inf3_pan_id: INF3_PAN_ID,
    inf3_extend_addr0: INF3_EXTEND_ADDR0,
    inf3_extend_addr1: INF3_EXTEND_ADDR1,
    channel: CHANNEL,
    tx_power: TX_POWER,
    ed_scan_duration: ED_SCAN_DURATION,
    ed_scan_cfg: ED_SCAN_CFG,
    ifs: IFS,
    ack_timeout: ACK_TIMEOUT,
    event_en: EVENT_EN,
    event_status: EVENT_STATUS,
    rx_abort_intr_ctrl: RX_ABORT_INTR_CTRL,
    ack_frame_pending_en: ACK_FRAME_PENDING_EN,
    coex_pti: COEX_PTI,
    core_dummy_data: CORE_DUMMY_DATA,
    tx_abort_interrupt_control: TX_ABORT_INTERRUPT_CONTROL,
    enhance_ack_cfg: ENHANCE_ACK_CFG,
    rx_status: RX_STATUS,
    tx_status: TX_STATUS,
    txrx_status: TXRX_STATUS,
    tx_ccm_schedule_status: TX_CCM_SCHEDULE_STATUS,
    core_gck_cfg: CORE_GCK_CFG,
    test_control: TEST_CONTROL,
    dtm_config: DTM_CONFIG,
    dtm_tx_pkt_config: DTM_TX_PKT_CONFIG,
    dtm_pkt_counter: DTM_PKT_COUNTER,
    rx_length: RX_LENGTH,
    time0_threshold: TIME0_THRESHOLD,
    time0_value: TIME0_VALUE,
    time1_threshold: TIME1_THRESHOLD,
    time1_value: TIME1_VALUE,
    clk_counter_match_val: CLK_COUNTER_MATCH_VAL,
    clk_counter: CLK_COUNTER,
    ifs_counter: IFS_COUNTER,
    sfd_wait_symbol: SFD_WAIT_SYMBOL,
    txrx_path_delay: TXRX_PATH_DELAY,
    bb_clk: BB_CLK,
    txdma_addr: TXDMA_ADDR,
    txdma_ctrl_state: TXDMA_CTRL_STATE,
    txdma_err: TXDMA_ERR,
    _reserved55: [u8; 0x04],
    rxdma_addr: RXDMA_ADDR,
    rxdma_ctrl_state: RXDMA_CTRL_STATE,
    rxdma_err: RXDMA_ERR,
    _reserved58: [u8; 0x04],
    dma_gck_cfg: DMA_GCK_CFG,
    dma_dummy: DMA_DUMMY,
    _reserved60: [u8; 0x08],
    paon_delay: PAON_DELAY,
    txon_delay: TXON_DELAY,
    txen_stop_delay: TXEN_STOP_DELAY,
    txoff_delay: TXOFF_DELAY,
    rxon_delay: RXON_DELAY,
    txrx_switch_delay: TXRX_SWITCH_DELAY,
    cont_rx_delay: CONT_RX_DELAY,
    dcdc_ctrl: DCDC_CTRL,
    debug_ctrl: DEBUG_CTRL,
    _reserved69: [u8; 0x04],
    sec_ctrl: SEC_CTRL,
    sec_extend_address0: SEC_EXTEND_ADDRESS0,
    sec_extend_address1: SEC_EXTEND_ADDRESS1,
    sec_key0: SEC_KEY0,
    sec_key1: SEC_KEY1,
    sec_key2: SEC_KEY2,
    sec_key3: SEC_KEY3,
    sfd_timeout_cnt: SFD_TIMEOUT_CNT,
    crc_error_cnt: CRC_ERROR_CNT,
    ed_abort_cnt: ED_ABORT_CNT,
    cca_fail_cnt: CCA_FAIL_CNT,
    rx_filter_fail_cnt: RX_FILTER_FAIL_CNT,
    no_rss_detect_cnt: NO_RSS_DETECT_CNT,
    rx_abort_coex_cnt: RX_ABORT_COEX_CNT,
    rx_restart_cnt: RX_RESTART_CNT,
    tx_ack_abort_coex_cnt: TX_ACK_ABORT_COEX_CNT,
    ed_scan_coex_cnt: ED_SCAN_COEX_CNT,
    rx_ack_abort_coex_cnt: RX_ACK_ABORT_COEX_CNT,
    rx_ack_timeout_cnt: RX_ACK_TIMEOUT_CNT,
    tx_break_coex_cnt: TX_BREAK_COEX_CNT,
    tx_security_error_cnt: TX_SECURITY_ERROR_CNT,
    cca_busy_cnt: CCA_BUSY_CNT,
    error_cnt_clear: ERROR_CNT_CLEAR,
    debug_sel_cfg0: DEBUG_SEL_CFG0,
    debug_sel_cfg1: DEBUG_SEL_CFG1,
    mac_date: MAC_DATE,
}
impl RegisterBlock {
    ///0x00 -
    #[inline(always)]
    pub const fn command(&self) -> &COMMAND {
        &self.command
    }
    ///0x04 -
    #[inline(always)]
    pub const fn ctrl_cfg(&self) -> &CTRL_CFG {
        &self.ctrl_cfg
    }
    ///0x08 -
    #[inline(always)]
    pub const fn inf0_short_addr(&self) -> &INF0_SHORT_ADDR {
        &self.inf0_short_addr
    }
    ///0x0c -
    #[inline(always)]
    pub const fn inf0_pan_id(&self) -> &INF0_PAN_ID {
        &self.inf0_pan_id
    }
    ///0x10 -
    #[inline(always)]
    pub const fn inf0_extend_addr0(&self) -> &INF0_EXTEND_ADDR0 {
        &self.inf0_extend_addr0
    }
    ///0x14 -
    #[inline(always)]
    pub const fn inf0_extend_addr1(&self) -> &INF0_EXTEND_ADDR1 {
        &self.inf0_extend_addr1
    }
    ///0x18 -
    #[inline(always)]
    pub const fn inf1_short_addr(&self) -> &INF1_SHORT_ADDR {
        &self.inf1_short_addr
    }
    ///0x1c -
    #[inline(always)]
    pub const fn inf1_pan_id(&self) -> &INF1_PAN_ID {
        &self.inf1_pan_id
    }
    ///0x20 -
    #[inline(always)]
    pub const fn inf1_extend_addr0(&self) -> &INF1_EXTEND_ADDR0 {
        &self.inf1_extend_addr0
    }
    ///0x24 -
    #[inline(always)]
    pub const fn inf1_extend_addr1(&self) -> &INF1_EXTEND_ADDR1 {
        &self.inf1_extend_addr1
    }
    ///0x28 -
    #[inline(always)]
    pub const fn inf2_short_addr(&self) -> &INF2_SHORT_ADDR {
        &self.inf2_short_addr
    }
    ///0x2c -
    #[inline(always)]
    pub const fn inf2_pan_id(&self) -> &INF2_PAN_ID {
        &self.inf2_pan_id
    }
    ///0x30 -
    #[inline(always)]
    pub const fn inf2_extend_addr0(&self) -> &INF2_EXTEND_ADDR0 {
        &self.inf2_extend_addr0
    }
    ///0x34 -
    #[inline(always)]
    pub const fn inf2_extend_addr1(&self) -> &INF2_EXTEND_ADDR1 {
        &self.inf2_extend_addr1
    }
    ///0x38 -
    #[inline(always)]
    pub const fn inf3_short_addr(&self) -> &INF3_SHORT_ADDR {
        &self.inf3_short_addr
    }
    ///0x3c -
    #[inline(always)]
    pub const fn inf3_pan_id(&self) -> &INF3_PAN_ID {
        &self.inf3_pan_id
    }
    ///0x40 -
    #[inline(always)]
    pub const fn inf3_extend_addr0(&self) -> &INF3_EXTEND_ADDR0 {
        &self.inf3_extend_addr0
    }
    ///0x44 -
    #[inline(always)]
    pub const fn inf3_extend_addr1(&self) -> &INF3_EXTEND_ADDR1 {
        &self.inf3_extend_addr1
    }
    ///0x48 -
    #[inline(always)]
    pub const fn channel(&self) -> &CHANNEL {
        &self.channel
    }
    ///0x4c -
    #[inline(always)]
    pub const fn tx_power(&self) -> &TX_POWER {
        &self.tx_power
    }
    ///0x50 -
    #[inline(always)]
    pub const fn ed_scan_duration(&self) -> &ED_SCAN_DURATION {
        &self.ed_scan_duration
    }
    ///0x54 -
    #[inline(always)]
    pub const fn ed_scan_cfg(&self) -> &ED_SCAN_CFG {
        &self.ed_scan_cfg
    }
    ///0x58 -
    #[inline(always)]
    pub const fn ifs(&self) -> &IFS {
        &self.ifs
    }
    ///0x5c -
    #[inline(always)]
    pub const fn ack_timeout(&self) -> &ACK_TIMEOUT {
        &self.ack_timeout
    }
    ///0x60 -
    #[inline(always)]
    pub const fn event_en(&self) -> &EVENT_EN {
        &self.event_en
    }
    ///0x64 -
    #[inline(always)]
    pub const fn event_status(&self) -> &EVENT_STATUS {
        &self.event_status
    }
    ///0x68 -
    #[inline(always)]
    pub const fn rx_abort_intr_ctrl(&self) -> &RX_ABORT_INTR_CTRL {
        &self.rx_abort_intr_ctrl
    }
    ///0x6c -
    #[inline(always)]
    pub const fn ack_frame_pending_en(&self) -> &ACK_FRAME_PENDING_EN {
        &self.ack_frame_pending_en
    }
    ///0x70 -
    #[inline(always)]
    pub const fn coex_pti(&self) -> &COEX_PTI {
        &self.coex_pti
    }
    ///0x74 -
    #[inline(always)]
    pub const fn core_dummy_data(&self) -> &CORE_DUMMY_DATA {
        &self.core_dummy_data
    }
    ///0x78 -
    #[inline(always)]
    pub const fn tx_abort_interrupt_control(&self) -> &TX_ABORT_INTERRUPT_CONTROL {
        &self.tx_abort_interrupt_control
    }
    ///0x7c -
    #[inline(always)]
    pub const fn enhance_ack_cfg(&self) -> &ENHANCE_ACK_CFG {
        &self.enhance_ack_cfg
    }
    ///0x80 -
    #[inline(always)]
    pub const fn rx_status(&self) -> &RX_STATUS {
        &self.rx_status
    }
    ///0x84 -
    #[inline(always)]
    pub const fn tx_status(&self) -> &TX_STATUS {
        &self.tx_status
    }
    ///0x88 -
    #[inline(always)]
    pub const fn txrx_status(&self) -> &TXRX_STATUS {
        &self.txrx_status
    }
    ///0x8c -
    #[inline(always)]
    pub const fn tx_ccm_schedule_status(&self) -> &TX_CCM_SCHEDULE_STATUS {
        &self.tx_ccm_schedule_status
    }
    ///0x90 -
    #[inline(always)]
    pub const fn core_gck_cfg(&self) -> &CORE_GCK_CFG {
        &self.core_gck_cfg
    }
    ///0x94 -
    #[inline(always)]
    pub const fn test_control(&self) -> &TEST_CONTROL {
        &self.test_control
    }
    ///0x98 -
    #[inline(always)]
    pub const fn dtm_config(&self) -> &DTM_CONFIG {
        &self.dtm_config
    }
    ///0x9c -
    #[inline(always)]
    pub const fn dtm_tx_pkt_config(&self) -> &DTM_TX_PKT_CONFIG {
        &self.dtm_tx_pkt_config
    }
    ///0xa0 -
    #[inline(always)]
    pub const fn dtm_pkt_counter(&self) -> &DTM_PKT_COUNTER {
        &self.dtm_pkt_counter
    }
    ///0xa4 -
    #[inline(always)]
    pub const fn rx_length(&self) -> &RX_LENGTH {
        &self.rx_length
    }
    ///0xa8 -
    #[inline(always)]
    pub const fn time0_threshold(&self) -> &TIME0_THRESHOLD {
        &self.time0_threshold
    }
    ///0xac -
    #[inline(always)]
    pub const fn time0_value(&self) -> &TIME0_VALUE {
        &self.time0_value
    }
    ///0xb0 -
    #[inline(always)]
    pub const fn time1_threshold(&self) -> &TIME1_THRESHOLD {
        &self.time1_threshold
    }
    ///0xb4 -
    #[inline(always)]
    pub const fn time1_value(&self) -> &TIME1_VALUE {
        &self.time1_value
    }
    ///0xb8 -
    #[inline(always)]
    pub const fn clk_counter_match_val(&self) -> &CLK_COUNTER_MATCH_VAL {
        &self.clk_counter_match_val
    }
    ///0xbc -
    #[inline(always)]
    pub const fn clk_counter(&self) -> &CLK_COUNTER {
        &self.clk_counter
    }
    ///0xc0 -
    #[inline(always)]
    pub const fn ifs_counter(&self) -> &IFS_COUNTER {
        &self.ifs_counter
    }
    ///0xc4 -
    #[inline(always)]
    pub const fn sfd_wait_symbol(&self) -> &SFD_WAIT_SYMBOL {
        &self.sfd_wait_symbol
    }
    ///0xc8 -
    #[inline(always)]
    pub const fn txrx_path_delay(&self) -> &TXRX_PATH_DELAY {
        &self.txrx_path_delay
    }
    ///0xcc -
    #[inline(always)]
    pub const fn bb_clk(&self) -> &BB_CLK {
        &self.bb_clk
    }
    ///0xd0 -
    #[inline(always)]
    pub const fn txdma_addr(&self) -> &TXDMA_ADDR {
        &self.txdma_addr
    }
    ///0xd4 -
    #[inline(always)]
    pub const fn txdma_ctrl_state(&self) -> &TXDMA_CTRL_STATE {
        &self.txdma_ctrl_state
    }
    ///0xd8 -
    #[inline(always)]
    pub const fn txdma_err(&self) -> &TXDMA_ERR {
        &self.txdma_err
    }
    ///0xe0 -
    #[inline(always)]
    pub const fn rxdma_addr(&self) -> &RXDMA_ADDR {
        &self.rxdma_addr
    }
    ///0xe4 -
    #[inline(always)]
    pub const fn rxdma_ctrl_state(&self) -> &RXDMA_CTRL_STATE {
        &self.rxdma_ctrl_state
    }
    ///0xe8 -
    #[inline(always)]
    pub const fn rxdma_err(&self) -> &RXDMA_ERR {
        &self.rxdma_err
    }
    ///0xf0 -
    #[inline(always)]
    pub const fn dma_gck_cfg(&self) -> &DMA_GCK_CFG {
        &self.dma_gck_cfg
    }
    ///0xf4 -
    #[inline(always)]
    pub const fn dma_dummy(&self) -> &DMA_DUMMY {
        &self.dma_dummy
    }
    ///0x100 -
    #[inline(always)]
    pub const fn paon_delay(&self) -> &PAON_DELAY {
        &self.paon_delay
    }
    ///0x104 -
    #[inline(always)]
    pub const fn txon_delay(&self) -> &TXON_DELAY {
        &self.txon_delay
    }
    ///0x108 -
    #[inline(always)]
    pub const fn txen_stop_delay(&self) -> &TXEN_STOP_DELAY {
        &self.txen_stop_delay
    }
    ///0x10c -
    #[inline(always)]
    pub const fn txoff_delay(&self) -> &TXOFF_DELAY {
        &self.txoff_delay
    }
    ///0x110 -
    #[inline(always)]
    pub const fn rxon_delay(&self) -> &RXON_DELAY {
        &self.rxon_delay
    }
    ///0x114 -
    #[inline(always)]
    pub const fn txrx_switch_delay(&self) -> &TXRX_SWITCH_DELAY {
        &self.txrx_switch_delay
    }
    ///0x118 -
    #[inline(always)]
    pub const fn cont_rx_delay(&self) -> &CONT_RX_DELAY {
        &self.cont_rx_delay
    }
    ///0x11c -
    #[inline(always)]
    pub const fn dcdc_ctrl(&self) -> &DCDC_CTRL {
        &self.dcdc_ctrl
    }
    ///0x120 -
    #[inline(always)]
    pub const fn debug_ctrl(&self) -> &DEBUG_CTRL {
        &self.debug_ctrl
    }
    ///0x128 -
    #[inline(always)]
    pub const fn sec_ctrl(&self) -> &SEC_CTRL {
        &self.sec_ctrl
    }
    ///0x12c -
    #[inline(always)]
    pub const fn sec_extend_address0(&self) -> &SEC_EXTEND_ADDRESS0 {
        &self.sec_extend_address0
    }
    ///0x130 -
    #[inline(always)]
    pub const fn sec_extend_address1(&self) -> &SEC_EXTEND_ADDRESS1 {
        &self.sec_extend_address1
    }
    ///0x134 -
    #[inline(always)]
    pub const fn sec_key0(&self) -> &SEC_KEY0 {
        &self.sec_key0
    }
    ///0x138 -
    #[inline(always)]
    pub const fn sec_key1(&self) -> &SEC_KEY1 {
        &self.sec_key1
    }
    ///0x13c -
    #[inline(always)]
    pub const fn sec_key2(&self) -> &SEC_KEY2 {
        &self.sec_key2
    }
    ///0x140 -
    #[inline(always)]
    pub const fn sec_key3(&self) -> &SEC_KEY3 {
        &self.sec_key3
    }
    ///0x144 -
    #[inline(always)]
    pub const fn sfd_timeout_cnt(&self) -> &SFD_TIMEOUT_CNT {
        &self.sfd_timeout_cnt
    }
    ///0x148 -
    #[inline(always)]
    pub const fn crc_error_cnt(&self) -> &CRC_ERROR_CNT {
        &self.crc_error_cnt
    }
    ///0x14c -
    #[inline(always)]
    pub const fn ed_abort_cnt(&self) -> &ED_ABORT_CNT {
        &self.ed_abort_cnt
    }
    ///0x150 -
    #[inline(always)]
    pub const fn cca_fail_cnt(&self) -> &CCA_FAIL_CNT {
        &self.cca_fail_cnt
    }
    ///0x154 -
    #[inline(always)]
    pub const fn rx_filter_fail_cnt(&self) -> &RX_FILTER_FAIL_CNT {
        &self.rx_filter_fail_cnt
    }
    ///0x158 -
    #[inline(always)]
    pub const fn no_rss_detect_cnt(&self) -> &NO_RSS_DETECT_CNT {
        &self.no_rss_detect_cnt
    }
    ///0x15c -
    #[inline(always)]
    pub const fn rx_abort_coex_cnt(&self) -> &RX_ABORT_COEX_CNT {
        &self.rx_abort_coex_cnt
    }
    ///0x160 -
    #[inline(always)]
    pub const fn rx_restart_cnt(&self) -> &RX_RESTART_CNT {
        &self.rx_restart_cnt
    }
    ///0x164 -
    #[inline(always)]
    pub const fn tx_ack_abort_coex_cnt(&self) -> &TX_ACK_ABORT_COEX_CNT {
        &self.tx_ack_abort_coex_cnt
    }
    ///0x168 -
    #[inline(always)]
    pub const fn ed_scan_coex_cnt(&self) -> &ED_SCAN_COEX_CNT {
        &self.ed_scan_coex_cnt
    }
    ///0x16c -
    #[inline(always)]
    pub const fn rx_ack_abort_coex_cnt(&self) -> &RX_ACK_ABORT_COEX_CNT {
        &self.rx_ack_abort_coex_cnt
    }
    ///0x170 -
    #[inline(always)]
    pub const fn rx_ack_timeout_cnt(&self) -> &RX_ACK_TIMEOUT_CNT {
        &self.rx_ack_timeout_cnt
    }
    ///0x174 -
    #[inline(always)]
    pub const fn tx_break_coex_cnt(&self) -> &TX_BREAK_COEX_CNT {
        &self.tx_break_coex_cnt
    }
    ///0x178 -
    #[inline(always)]
    pub const fn tx_security_error_cnt(&self) -> &TX_SECURITY_ERROR_CNT {
        &self.tx_security_error_cnt
    }
    ///0x17c -
    #[inline(always)]
    pub const fn cca_busy_cnt(&self) -> &CCA_BUSY_CNT {
        &self.cca_busy_cnt
    }
    ///0x180 -
    #[inline(always)]
    pub const fn error_cnt_clear(&self) -> &ERROR_CNT_CLEAR {
        &self.error_cnt_clear
    }
    ///0x184 -
    #[inline(always)]
    pub const fn debug_sel_cfg0(&self) -> &DEBUG_SEL_CFG0 {
        &self.debug_sel_cfg0
    }
    ///0x188 -
    #[inline(always)]
    pub const fn debug_sel_cfg1(&self) -> &DEBUG_SEL_CFG1 {
        &self.debug_sel_cfg1
    }
    ///0x18c -
    #[inline(always)]
    pub const fn mac_date(&self) -> &MAC_DATE {
        &self.mac_date
    }
}
/**COMMAND (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`command::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`command::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@command`] module*/
pub type COMMAND = crate::Reg<command::COMMAND_SPEC>;
///
pub mod command;
/**CTRL_CFG (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl_cfg`] module*/
pub type CTRL_CFG = crate::Reg<ctrl_cfg::CTRL_CFG_SPEC>;
///
pub mod ctrl_cfg;
/**INF0_SHORT_ADDR (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`inf0_short_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inf0_short_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@inf0_short_addr`] module*/
pub type INF0_SHORT_ADDR = crate::Reg<inf0_short_addr::INF0_SHORT_ADDR_SPEC>;
///
pub mod inf0_short_addr;
/**INF0_PAN_ID (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`inf0_pan_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inf0_pan_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@inf0_pan_id`] module*/
pub type INF0_PAN_ID = crate::Reg<inf0_pan_id::INF0_PAN_ID_SPEC>;
///
pub mod inf0_pan_id;
/**INF0_EXTEND_ADDR0 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`inf0_extend_addr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inf0_extend_addr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@inf0_extend_addr0`] module*/
pub type INF0_EXTEND_ADDR0 = crate::Reg<inf0_extend_addr0::INF0_EXTEND_ADDR0_SPEC>;
///
pub mod inf0_extend_addr0;
/**INF0_EXTEND_ADDR1 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`inf0_extend_addr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inf0_extend_addr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@inf0_extend_addr1`] module*/
pub type INF0_EXTEND_ADDR1 = crate::Reg<inf0_extend_addr1::INF0_EXTEND_ADDR1_SPEC>;
///
pub mod inf0_extend_addr1;
/**INF1_SHORT_ADDR (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`inf1_short_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inf1_short_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@inf1_short_addr`] module*/
pub type INF1_SHORT_ADDR = crate::Reg<inf1_short_addr::INF1_SHORT_ADDR_SPEC>;
///
pub mod inf1_short_addr;
/**INF1_PAN_ID (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`inf1_pan_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inf1_pan_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@inf1_pan_id`] module*/
pub type INF1_PAN_ID = crate::Reg<inf1_pan_id::INF1_PAN_ID_SPEC>;
///
pub mod inf1_pan_id;
/**INF1_EXTEND_ADDR0 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`inf1_extend_addr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inf1_extend_addr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@inf1_extend_addr0`] module*/
pub type INF1_EXTEND_ADDR0 = crate::Reg<inf1_extend_addr0::INF1_EXTEND_ADDR0_SPEC>;
///
pub mod inf1_extend_addr0;
/**INF1_EXTEND_ADDR1 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`inf1_extend_addr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inf1_extend_addr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@inf1_extend_addr1`] module*/
pub type INF1_EXTEND_ADDR1 = crate::Reg<inf1_extend_addr1::INF1_EXTEND_ADDR1_SPEC>;
///
pub mod inf1_extend_addr1;
/**INF2_SHORT_ADDR (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`inf2_short_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inf2_short_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@inf2_short_addr`] module*/
pub type INF2_SHORT_ADDR = crate::Reg<inf2_short_addr::INF2_SHORT_ADDR_SPEC>;
///
pub mod inf2_short_addr;
/**INF2_PAN_ID (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`inf2_pan_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inf2_pan_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@inf2_pan_id`] module*/
pub type INF2_PAN_ID = crate::Reg<inf2_pan_id::INF2_PAN_ID_SPEC>;
///
pub mod inf2_pan_id;
/**INF2_EXTEND_ADDR0 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`inf2_extend_addr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inf2_extend_addr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@inf2_extend_addr0`] module*/
pub type INF2_EXTEND_ADDR0 = crate::Reg<inf2_extend_addr0::INF2_EXTEND_ADDR0_SPEC>;
///
pub mod inf2_extend_addr0;
/**INF2_EXTEND_ADDR1 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`inf2_extend_addr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inf2_extend_addr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@inf2_extend_addr1`] module*/
pub type INF2_EXTEND_ADDR1 = crate::Reg<inf2_extend_addr1::INF2_EXTEND_ADDR1_SPEC>;
///
pub mod inf2_extend_addr1;
/**INF3_SHORT_ADDR (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`inf3_short_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inf3_short_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@inf3_short_addr`] module*/
pub type INF3_SHORT_ADDR = crate::Reg<inf3_short_addr::INF3_SHORT_ADDR_SPEC>;
///
pub mod inf3_short_addr;
/**INF3_PAN_ID (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`inf3_pan_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inf3_pan_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@inf3_pan_id`] module*/
pub type INF3_PAN_ID = crate::Reg<inf3_pan_id::INF3_PAN_ID_SPEC>;
///
pub mod inf3_pan_id;
/**INF3_EXTEND_ADDR0 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`inf3_extend_addr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inf3_extend_addr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@inf3_extend_addr0`] module*/
pub type INF3_EXTEND_ADDR0 = crate::Reg<inf3_extend_addr0::INF3_EXTEND_ADDR0_SPEC>;
///
pub mod inf3_extend_addr0;
/**INF3_EXTEND_ADDR1 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`inf3_extend_addr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inf3_extend_addr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@inf3_extend_addr1`] module*/
pub type INF3_EXTEND_ADDR1 = crate::Reg<inf3_extend_addr1::INF3_EXTEND_ADDR1_SPEC>;
///
pub mod inf3_extend_addr1;
/**CHANNEL (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`channel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`channel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@channel`] module*/
pub type CHANNEL = crate::Reg<channel::CHANNEL_SPEC>;
///
pub mod channel;
/**TX_POWER (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`tx_power::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_power::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tx_power`] module*/
pub type TX_POWER = crate::Reg<tx_power::TX_POWER_SPEC>;
///
pub mod tx_power;
/**ED_SCAN_DURATION (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`ed_scan_duration::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ed_scan_duration::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ed_scan_duration`] module*/
pub type ED_SCAN_DURATION = crate::Reg<ed_scan_duration::ED_SCAN_DURATION_SPEC>;
///
pub mod ed_scan_duration;
/**ED_SCAN_CFG (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`ed_scan_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ed_scan_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ed_scan_cfg`] module*/
pub type ED_SCAN_CFG = crate::Reg<ed_scan_cfg::ED_SCAN_CFG_SPEC>;
///
pub mod ed_scan_cfg;
/**IFS (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`ifs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ifs`] module*/
pub type IFS = crate::Reg<ifs::IFS_SPEC>;
///
pub mod ifs;
/**ACK_TIMEOUT (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`ack_timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ack_timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ack_timeout`] module*/
pub type ACK_TIMEOUT = crate::Reg<ack_timeout::ACK_TIMEOUT_SPEC>;
///
pub mod ack_timeout;
/**EVENT_EN (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`event_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`event_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@event_en`] module*/
pub type EVENT_EN = crate::Reg<event_en::EVENT_EN_SPEC>;
///
pub mod event_en;
/**EVENT_STATUS (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`event_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`event_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@event_status`] module*/
pub type EVENT_STATUS = crate::Reg<event_status::EVENT_STATUS_SPEC>;
///
pub mod event_status;
/**RX_ABORT_INTR_CTRL (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`rx_abort_intr_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_abort_intr_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_abort_intr_ctrl`] module*/
pub type RX_ABORT_INTR_CTRL = crate::Reg<rx_abort_intr_ctrl::RX_ABORT_INTR_CTRL_SPEC>;
///
pub mod rx_abort_intr_ctrl;
/**ACK_FRAME_PENDING_EN (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`ack_frame_pending_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ack_frame_pending_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ack_frame_pending_en`] module*/
pub type ACK_FRAME_PENDING_EN = crate::Reg<ack_frame_pending_en::ACK_FRAME_PENDING_EN_SPEC>;
///
pub mod ack_frame_pending_en;
/**COEX_PTI (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`coex_pti::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`coex_pti::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@coex_pti`] module*/
pub type COEX_PTI = crate::Reg<coex_pti::COEX_PTI_SPEC>;
///
pub mod coex_pti;
/**CORE_DUMMY_DATA (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`core_dummy_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_dummy_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@core_dummy_data`] module*/
pub type CORE_DUMMY_DATA = crate::Reg<core_dummy_data::CORE_DUMMY_DATA_SPEC>;
///
pub mod core_dummy_data;
/**TX_ABORT_INTERRUPT_CONTROL (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`tx_abort_interrupt_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_abort_interrupt_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tx_abort_interrupt_control`] module*/
pub type TX_ABORT_INTERRUPT_CONTROL =
    crate::Reg<tx_abort_interrupt_control::TX_ABORT_INTERRUPT_CONTROL_SPEC>;
///
pub mod tx_abort_interrupt_control;
/**ENHANCE_ACK_CFG (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`enhance_ack_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enhance_ack_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@enhance_ack_cfg`] module*/
pub type ENHANCE_ACK_CFG = crate::Reg<enhance_ack_cfg::ENHANCE_ACK_CFG_SPEC>;
///
pub mod enhance_ack_cfg;
/**RX_STATUS (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`rx_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_status`] module*/
pub type RX_STATUS = crate::Reg<rx_status::RX_STATUS_SPEC>;
///
pub mod rx_status;
/**TX_STATUS (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`tx_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tx_status`] module*/
pub type TX_STATUS = crate::Reg<tx_status::TX_STATUS_SPEC>;
///
pub mod tx_status;
/**TXRX_STATUS (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`txrx_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txrx_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@txrx_status`] module*/
pub type TXRX_STATUS = crate::Reg<txrx_status::TXRX_STATUS_SPEC>;
///
pub mod txrx_status;
/**TX_CCM_SCHEDULE_STATUS (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`tx_ccm_schedule_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_ccm_schedule_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tx_ccm_schedule_status`] module*/
pub type TX_CCM_SCHEDULE_STATUS = crate::Reg<tx_ccm_schedule_status::TX_CCM_SCHEDULE_STATUS_SPEC>;
///
pub mod tx_ccm_schedule_status;
/**CORE_GCK_CFG (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`core_gck_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_gck_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@core_gck_cfg`] module*/
pub type CORE_GCK_CFG = crate::Reg<core_gck_cfg::CORE_GCK_CFG_SPEC>;
///
pub mod core_gck_cfg;
/**TEST_CONTROL (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`test_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@test_control`] module*/
pub type TEST_CONTROL = crate::Reg<test_control::TEST_CONTROL_SPEC>;
///
pub mod test_control;
/**DTM_CONFIG (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`dtm_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtm_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dtm_config`] module*/
pub type DTM_CONFIG = crate::Reg<dtm_config::DTM_CONFIG_SPEC>;
///
pub mod dtm_config;
/**DTM_TX_PKT_CONFIG (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`dtm_tx_pkt_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtm_tx_pkt_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dtm_tx_pkt_config`] module*/
pub type DTM_TX_PKT_CONFIG = crate::Reg<dtm_tx_pkt_config::DTM_TX_PKT_CONFIG_SPEC>;
///
pub mod dtm_tx_pkt_config;
/**DTM_PKT_COUNTER (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`dtm_pkt_counter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtm_pkt_counter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dtm_pkt_counter`] module*/
pub type DTM_PKT_COUNTER = crate::Reg<dtm_pkt_counter::DTM_PKT_COUNTER_SPEC>;
///
pub mod dtm_pkt_counter;
/**RX_LENGTH (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`rx_length::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_length::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_length`] module*/
pub type RX_LENGTH = crate::Reg<rx_length::RX_LENGTH_SPEC>;
///
pub mod rx_length;
/**TIME0_THRESHOLD (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`time0_threshold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time0_threshold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@time0_threshold`] module*/
pub type TIME0_THRESHOLD = crate::Reg<time0_threshold::TIME0_THRESHOLD_SPEC>;
///
pub mod time0_threshold;
/**TIME0_VALUE (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`time0_value::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time0_value::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@time0_value`] module*/
pub type TIME0_VALUE = crate::Reg<time0_value::TIME0_VALUE_SPEC>;
///
pub mod time0_value;
/**TIME1_THRESHOLD (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`time1_threshold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time1_threshold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@time1_threshold`] module*/
pub type TIME1_THRESHOLD = crate::Reg<time1_threshold::TIME1_THRESHOLD_SPEC>;
///
pub mod time1_threshold;
/**TIME1_VALUE (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`time1_value::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time1_value::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@time1_value`] module*/
pub type TIME1_VALUE = crate::Reg<time1_value::TIME1_VALUE_SPEC>;
///
pub mod time1_value;
/**CLK_COUNTER_MATCH_VAL (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`clk_counter_match_val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_counter_match_val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk_counter_match_val`] module*/
pub type CLK_COUNTER_MATCH_VAL = crate::Reg<clk_counter_match_val::CLK_COUNTER_MATCH_VAL_SPEC>;
///
pub mod clk_counter_match_val;
/**CLK_COUNTER (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`clk_counter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_counter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk_counter`] module*/
pub type CLK_COUNTER = crate::Reg<clk_counter::CLK_COUNTER_SPEC>;
///
pub mod clk_counter;
/**IFS_COUNTER (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`ifs_counter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifs_counter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ifs_counter`] module*/
pub type IFS_COUNTER = crate::Reg<ifs_counter::IFS_COUNTER_SPEC>;
///
pub mod ifs_counter;
/**SFD_WAIT_SYMBOL (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`sfd_wait_symbol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfd_wait_symbol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfd_wait_symbol`] module*/
pub type SFD_WAIT_SYMBOL = crate::Reg<sfd_wait_symbol::SFD_WAIT_SYMBOL_SPEC>;
///
pub mod sfd_wait_symbol;
/**TXRX_PATH_DELAY (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`txrx_path_delay::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txrx_path_delay::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@txrx_path_delay`] module*/
pub type TXRX_PATH_DELAY = crate::Reg<txrx_path_delay::TXRX_PATH_DELAY_SPEC>;
///
pub mod txrx_path_delay;
/**BB_CLK (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`bb_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bb_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bb_clk`] module*/
pub type BB_CLK = crate::Reg<bb_clk::BB_CLK_SPEC>;
///
pub mod bb_clk;
/**TXDMA_ADDR (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`txdma_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdma_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@txdma_addr`] module*/
pub type TXDMA_ADDR = crate::Reg<txdma_addr::TXDMA_ADDR_SPEC>;
///
pub mod txdma_addr;
/**TXDMA_CTRL_STATE (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`txdma_ctrl_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdma_ctrl_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@txdma_ctrl_state`] module*/
pub type TXDMA_CTRL_STATE = crate::Reg<txdma_ctrl_state::TXDMA_CTRL_STATE_SPEC>;
///
pub mod txdma_ctrl_state;
/**TXDMA_ERR (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`txdma_err::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdma_err::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@txdma_err`] module*/
pub type TXDMA_ERR = crate::Reg<txdma_err::TXDMA_ERR_SPEC>;
///
pub mod txdma_err;
/**RXDMA_ADDR (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`rxdma_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rxdma_addr`] module*/
pub type RXDMA_ADDR = crate::Reg<rxdma_addr::RXDMA_ADDR_SPEC>;
///
pub mod rxdma_addr;
/**RXDMA_CTRL_STATE (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`rxdma_ctrl_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_ctrl_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rxdma_ctrl_state`] module*/
pub type RXDMA_CTRL_STATE = crate::Reg<rxdma_ctrl_state::RXDMA_CTRL_STATE_SPEC>;
///
pub mod rxdma_ctrl_state;
/**RXDMA_ERR (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`rxdma_err::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdma_err::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rxdma_err`] module*/
pub type RXDMA_ERR = crate::Reg<rxdma_err::RXDMA_ERR_SPEC>;
///
pub mod rxdma_err;
/**DMA_GCK_CFG (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`dma_gck_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_gck_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_gck_cfg`] module*/
pub type DMA_GCK_CFG = crate::Reg<dma_gck_cfg::DMA_GCK_CFG_SPEC>;
///
pub mod dma_gck_cfg;
/**DMA_DUMMY (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`dma_dummy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_dummy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dma_dummy`] module*/
pub type DMA_DUMMY = crate::Reg<dma_dummy::DMA_DUMMY_SPEC>;
///
pub mod dma_dummy;
/**PAON_DELAY (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`paon_delay::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`paon_delay::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@paon_delay`] module*/
pub type PAON_DELAY = crate::Reg<paon_delay::PAON_DELAY_SPEC>;
///
pub mod paon_delay;
/**TXON_DELAY (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`txon_delay::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txon_delay::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@txon_delay`] module*/
pub type TXON_DELAY = crate::Reg<txon_delay::TXON_DELAY_SPEC>;
///
pub mod txon_delay;
/**TXEN_STOP_DELAY (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`txen_stop_delay::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txen_stop_delay::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@txen_stop_delay`] module*/
pub type TXEN_STOP_DELAY = crate::Reg<txen_stop_delay::TXEN_STOP_DELAY_SPEC>;
///
pub mod txen_stop_delay;
/**TXOFF_DELAY (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`txoff_delay::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txoff_delay::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@txoff_delay`] module*/
pub type TXOFF_DELAY = crate::Reg<txoff_delay::TXOFF_DELAY_SPEC>;
///
pub mod txoff_delay;
/**RXON_DELAY (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`rxon_delay::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxon_delay::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rxon_delay`] module*/
pub type RXON_DELAY = crate::Reg<rxon_delay::RXON_DELAY_SPEC>;
///
pub mod rxon_delay;
/**TXRX_SWITCH_DELAY (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`txrx_switch_delay::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txrx_switch_delay::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@txrx_switch_delay`] module*/
pub type TXRX_SWITCH_DELAY = crate::Reg<txrx_switch_delay::TXRX_SWITCH_DELAY_SPEC>;
///
pub mod txrx_switch_delay;
/**CONT_RX_DELAY (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`cont_rx_delay::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cont_rx_delay::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cont_rx_delay`] module*/
pub type CONT_RX_DELAY = crate::Reg<cont_rx_delay::CONT_RX_DELAY_SPEC>;
///
pub mod cont_rx_delay;
/**DCDC_CTRL (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`dcdc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcdc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dcdc_ctrl`] module*/
pub type DCDC_CTRL = crate::Reg<dcdc_ctrl::DCDC_CTRL_SPEC>;
///
pub mod dcdc_ctrl;
/**DEBUG_CTRL (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`debug_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@debug_ctrl`] module*/
pub type DEBUG_CTRL = crate::Reg<debug_ctrl::DEBUG_CTRL_SPEC>;
///
pub mod debug_ctrl;
/**SEC_CTRL (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`sec_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sec_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sec_ctrl`] module*/
pub type SEC_CTRL = crate::Reg<sec_ctrl::SEC_CTRL_SPEC>;
///
pub mod sec_ctrl;
/**SEC_EXTEND_ADDRESS0 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`sec_extend_address0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sec_extend_address0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sec_extend_address0`] module*/
pub type SEC_EXTEND_ADDRESS0 = crate::Reg<sec_extend_address0::SEC_EXTEND_ADDRESS0_SPEC>;
///
pub mod sec_extend_address0;
/**SEC_EXTEND_ADDRESS1 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`sec_extend_address1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sec_extend_address1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sec_extend_address1`] module*/
pub type SEC_EXTEND_ADDRESS1 = crate::Reg<sec_extend_address1::SEC_EXTEND_ADDRESS1_SPEC>;
///
pub mod sec_extend_address1;
/**SEC_KEY0 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`sec_key0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sec_key0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sec_key0`] module*/
pub type SEC_KEY0 = crate::Reg<sec_key0::SEC_KEY0_SPEC>;
///
pub mod sec_key0;
/**SEC_KEY1 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`sec_key1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sec_key1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sec_key1`] module*/
pub type SEC_KEY1 = crate::Reg<sec_key1::SEC_KEY1_SPEC>;
///
pub mod sec_key1;
/**SEC_KEY2 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`sec_key2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sec_key2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sec_key2`] module*/
pub type SEC_KEY2 = crate::Reg<sec_key2::SEC_KEY2_SPEC>;
///
pub mod sec_key2;
/**SEC_KEY3 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`sec_key3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sec_key3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sec_key3`] module*/
pub type SEC_KEY3 = crate::Reg<sec_key3::SEC_KEY3_SPEC>;
///
pub mod sec_key3;
/**SFD_TIMEOUT_CNT (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`sfd_timeout_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfd_timeout_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfd_timeout_cnt`] module*/
pub type SFD_TIMEOUT_CNT = crate::Reg<sfd_timeout_cnt::SFD_TIMEOUT_CNT_SPEC>;
///
pub mod sfd_timeout_cnt;
/**CRC_ERROR_CNT (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`crc_error_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_error_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crc_error_cnt`] module*/
pub type CRC_ERROR_CNT = crate::Reg<crc_error_cnt::CRC_ERROR_CNT_SPEC>;
///
pub mod crc_error_cnt;
/**ED_ABORT_CNT (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`ed_abort_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ed_abort_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ed_abort_cnt`] module*/
pub type ED_ABORT_CNT = crate::Reg<ed_abort_cnt::ED_ABORT_CNT_SPEC>;
///
pub mod ed_abort_cnt;
/**CCA_FAIL_CNT (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`cca_fail_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cca_fail_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cca_fail_cnt`] module*/
pub type CCA_FAIL_CNT = crate::Reg<cca_fail_cnt::CCA_FAIL_CNT_SPEC>;
///
pub mod cca_fail_cnt;
/**RX_FILTER_FAIL_CNT (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`rx_filter_fail_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_filter_fail_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_filter_fail_cnt`] module*/
pub type RX_FILTER_FAIL_CNT = crate::Reg<rx_filter_fail_cnt::RX_FILTER_FAIL_CNT_SPEC>;
///
pub mod rx_filter_fail_cnt;
/**NO_RSS_DETECT_CNT (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`no_rss_detect_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`no_rss_detect_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@no_rss_detect_cnt`] module*/
pub type NO_RSS_DETECT_CNT = crate::Reg<no_rss_detect_cnt::NO_RSS_DETECT_CNT_SPEC>;
///
pub mod no_rss_detect_cnt;
/**RX_ABORT_COEX_CNT (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`rx_abort_coex_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_abort_coex_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_abort_coex_cnt`] module*/
pub type RX_ABORT_COEX_CNT = crate::Reg<rx_abort_coex_cnt::RX_ABORT_COEX_CNT_SPEC>;
///
pub mod rx_abort_coex_cnt;
/**RX_RESTART_CNT (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`rx_restart_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_restart_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_restart_cnt`] module*/
pub type RX_RESTART_CNT = crate::Reg<rx_restart_cnt::RX_RESTART_CNT_SPEC>;
///
pub mod rx_restart_cnt;
/**TX_ACK_ABORT_COEX_CNT (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`tx_ack_abort_coex_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_ack_abort_coex_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tx_ack_abort_coex_cnt`] module*/
pub type TX_ACK_ABORT_COEX_CNT = crate::Reg<tx_ack_abort_coex_cnt::TX_ACK_ABORT_COEX_CNT_SPEC>;
///
pub mod tx_ack_abort_coex_cnt;
/**ED_SCAN_COEX_CNT (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`ed_scan_coex_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ed_scan_coex_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ed_scan_coex_cnt`] module*/
pub type ED_SCAN_COEX_CNT = crate::Reg<ed_scan_coex_cnt::ED_SCAN_COEX_CNT_SPEC>;
///
pub mod ed_scan_coex_cnt;
/**RX_ACK_ABORT_COEX_CNT (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`rx_ack_abort_coex_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_ack_abort_coex_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_ack_abort_coex_cnt`] module*/
pub type RX_ACK_ABORT_COEX_CNT = crate::Reg<rx_ack_abort_coex_cnt::RX_ACK_ABORT_COEX_CNT_SPEC>;
///
pub mod rx_ack_abort_coex_cnt;
/**RX_ACK_TIMEOUT_CNT (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`rx_ack_timeout_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_ack_timeout_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rx_ack_timeout_cnt`] module*/
pub type RX_ACK_TIMEOUT_CNT = crate::Reg<rx_ack_timeout_cnt::RX_ACK_TIMEOUT_CNT_SPEC>;
///
pub mod rx_ack_timeout_cnt;
/**TX_BREAK_COEX_CNT (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`tx_break_coex_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_break_coex_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tx_break_coex_cnt`] module*/
pub type TX_BREAK_COEX_CNT = crate::Reg<tx_break_coex_cnt::TX_BREAK_COEX_CNT_SPEC>;
///
pub mod tx_break_coex_cnt;
/**TX_SECURITY_ERROR_CNT (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`tx_security_error_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_security_error_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tx_security_error_cnt`] module*/
pub type TX_SECURITY_ERROR_CNT = crate::Reg<tx_security_error_cnt::TX_SECURITY_ERROR_CNT_SPEC>;
///
pub mod tx_security_error_cnt;
/**CCA_BUSY_CNT (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`cca_busy_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cca_busy_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cca_busy_cnt`] module*/
pub type CCA_BUSY_CNT = crate::Reg<cca_busy_cnt::CCA_BUSY_CNT_SPEC>;
///
pub mod cca_busy_cnt;
/**ERROR_CNT_CLEAR (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`error_cnt_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`error_cnt_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@error_cnt_clear`] module*/
pub type ERROR_CNT_CLEAR = crate::Reg<error_cnt_clear::ERROR_CNT_CLEAR_SPEC>;
///
pub mod error_cnt_clear;
/**DEBUG_SEL_CFG0 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`debug_sel_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_sel_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@debug_sel_cfg0`] module*/
pub type DEBUG_SEL_CFG0 = crate::Reg<debug_sel_cfg0::DEBUG_SEL_CFG0_SPEC>;
///
pub mod debug_sel_cfg0;
/**DEBUG_SEL_CFG1 (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`debug_sel_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug_sel_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@debug_sel_cfg1`] module*/
pub type DEBUG_SEL_CFG1 = crate::Reg<debug_sel_cfg1::DEBUG_SEL_CFG1_SPEC>;
///
pub mod debug_sel_cfg1;
/**MAC_DATE (rw) register accessor:

You can [`read`](crate::generic::Reg::read) this register and get [`mac_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mac_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mac_date`] module*/
pub type MAC_DATE = crate::Reg<mac_date::MAC_DATE_SPEC>;
///
pub mod mac_date;
