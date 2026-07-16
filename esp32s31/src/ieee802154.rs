#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    command: COMMAND,
    ctrl_cfg: CTRL_CFG,
    inf: [INF; 4],
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
    _reserved14: [u8; 0x04],
    tx_abort_interrupt_control: TX_ABORT_INTERRUPT_CONTROL,
    enhance_ack_cfg: ENHANCE_ACK_CFG,
    rx_status: RX_STATUS,
    tx_status: TX_STATUS,
    txrx_status: TXRX_STATUS,
    tx_ccm_schedule_status: TX_CCM_SCHEDULE_STATUS,
    _reserved20: [u8; 0x14],
    rx_length: RX_LENGTH,
    time: [TIME; 2],
    clk_counter_match_val: CLK_COUNTER_MATCH_VAL,
    clk_counter: CLK_COUNTER,
    ifs_counter: IFS_COUNTER,
    sfd_wait_symbol: SFD_WAIT_SYMBOL,
    txrx_path_delay: TXRX_PATH_DELAY,
    bb_clk: BB_CLK,
    txdma_addr: TXDMA_ADDR,
    txdma_ctrl_state: TXDMA_CTRL_STATE,
    txdma_err: TXDMA_ERR,
    _reserved31: [u8; 0x04],
    rxdma_addr: RXDMA_ADDR,
    rxdma_ctrl_state: RXDMA_CTRL_STATE,
    rxdma_err: RXDMA_ERR,
    _reserved34: [u8; 0x04],
    dma_gck_cfg: DMA_GCK_CFG,
    dma_dummy: DMA_DUMMY,
    _reserved36: [u8; 0x08],
    paon_delay: PAON_DELAY,
    txon_delay: TXON_DELAY,
    txen_stop_delay: TXEN_STOP_DELAY,
    txoff_delay: TXOFF_DELAY,
    rxon_delay: RXON_DELAY,
    txrx_switch_delay: TXRX_SWITCH_DELAY,
    cont_rx_delay: CONT_RX_DELAY,
    dcdc_ctrl: DCDC_CTRL,
    debug_ctrl: DEBUG_CTRL,
    _reserved45: [u8; 0x04],
    sec_ctrl: SEC_CTRL,
    sec_extend_address0: SEC_EXTEND_ADDRESS0,
    sec_extend_address1: SEC_EXTEND_ADDRESS1,
    sec_key: [SEC_KEY; 4],
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
    rx_timeout_target_cnt: RX_TIMEOUT_TARGET_CNT,
    modslp_config: MODSLP_CONFIG,
    modslp_status: MODSLP_STATUS,
    modslp_ww: MODSLP_WW,
    modslp_intr_wake_config: MODSLP_INTR_WAKE_CONFIG,
    mac_date: MAC_DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - COMMAND"]
    #[inline(always)]
    pub const fn command(&self) -> &COMMAND {
        &self.command
    }
    #[doc = "0x04 - CTRL_CFG"]
    #[inline(always)]
    pub const fn ctrl_cfg(&self) -> &CTRL_CFG {
        &self.ctrl_cfg
    }
    #[doc = "0x08..0x48 - Cluster INF%s, containing INF?_EXTEND_ADDR0, INF?_EXTEND_ADDR1, INF?_PAN_ID, INF?_SHORT_ADDR"]
    #[inline(always)]
    pub const fn inf(&self, n: usize) -> &INF {
        &self.inf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x48 - Cluster INF%s, containing INF?_EXTEND_ADDR0, INF?_EXTEND_ADDR1, INF?_PAN_ID, INF?_SHORT_ADDR"]
    #[inline(always)]
    pub fn inf_iter(&self) -> impl Iterator<Item = &INF> {
        self.inf.iter()
    }
    #[doc = "0x48 - CHANNEL"]
    #[inline(always)]
    pub const fn channel(&self) -> &CHANNEL {
        &self.channel
    }
    #[doc = "0x4c - TX_POWER"]
    #[inline(always)]
    pub const fn tx_power(&self) -> &TX_POWER {
        &self.tx_power
    }
    #[doc = "0x50 - ED_SCAN_DURATION"]
    #[inline(always)]
    pub const fn ed_scan_duration(&self) -> &ED_SCAN_DURATION {
        &self.ed_scan_duration
    }
    #[doc = "0x54 - ED_SCAN_CFG"]
    #[inline(always)]
    pub const fn ed_scan_cfg(&self) -> &ED_SCAN_CFG {
        &self.ed_scan_cfg
    }
    #[doc = "0x58 - IFS"]
    #[inline(always)]
    pub const fn ifs(&self) -> &IFS {
        &self.ifs
    }
    #[doc = "0x5c - ACK_TIMEOUT"]
    #[inline(always)]
    pub const fn ack_timeout(&self) -> &ACK_TIMEOUT {
        &self.ack_timeout
    }
    #[doc = "0x60 - EVENT_EN"]
    #[inline(always)]
    pub const fn event_en(&self) -> &EVENT_EN {
        &self.event_en
    }
    #[doc = "0x64 - EVENT_STATUS"]
    #[inline(always)]
    pub const fn event_status(&self) -> &EVENT_STATUS {
        &self.event_status
    }
    #[doc = "0x68 - RX_ABORT_INTR_CTRL"]
    #[inline(always)]
    pub const fn rx_abort_intr_ctrl(&self) -> &RX_ABORT_INTR_CTRL {
        &self.rx_abort_intr_ctrl
    }
    #[doc = "0x6c - ACK_FRAME_PENDING_EN"]
    #[inline(always)]
    pub const fn ack_frame_pending_en(&self) -> &ACK_FRAME_PENDING_EN {
        &self.ack_frame_pending_en
    }
    #[doc = "0x70 - COEX_PTI"]
    #[inline(always)]
    pub const fn coex_pti(&self) -> &COEX_PTI {
        &self.coex_pti
    }
    #[doc = "0x78 - TX_ABORT_INTERRUPT_CONTROL"]
    #[inline(always)]
    pub const fn tx_abort_interrupt_control(&self) -> &TX_ABORT_INTERRUPT_CONTROL {
        &self.tx_abort_interrupt_control
    }
    #[doc = "0x7c - ENHANCE_ACK_CFG"]
    #[inline(always)]
    pub const fn enhance_ack_cfg(&self) -> &ENHANCE_ACK_CFG {
        &self.enhance_ack_cfg
    }
    #[doc = "0x80 - RX_STATUS"]
    #[inline(always)]
    pub const fn rx_status(&self) -> &RX_STATUS {
        &self.rx_status
    }
    #[doc = "0x84 - TX_STATUS"]
    #[inline(always)]
    pub const fn tx_status(&self) -> &TX_STATUS {
        &self.tx_status
    }
    #[doc = "0x88 - TXRX_STATUS"]
    #[inline(always)]
    pub const fn txrx_status(&self) -> &TXRX_STATUS {
        &self.txrx_status
    }
    #[doc = "0x8c - TX_CCM_SCHEDULE_STATUS"]
    #[inline(always)]
    pub const fn tx_ccm_schedule_status(&self) -> &TX_CCM_SCHEDULE_STATUS {
        &self.tx_ccm_schedule_status
    }
    #[doc = "0xa4 - RX_LENGTH"]
    #[inline(always)]
    pub const fn rx_length(&self) -> &RX_LENGTH {
        &self.rx_length
    }
    #[doc = "0xa8..0xb8 - Cluster TIME%s, containing TIME?_THRESHOLD, TIME?_VALUE"]
    #[inline(always)]
    pub const fn time(&self, n: usize) -> &TIME {
        &self.time[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa8..0xb8 - Cluster TIME%s, containing TIME?_THRESHOLD, TIME?_VALUE"]
    #[inline(always)]
    pub fn time_iter(&self) -> impl Iterator<Item = &TIME> {
        self.time.iter()
    }
    #[doc = "0xb8 - CLK_COUNTER_MATCH_VAL"]
    #[inline(always)]
    pub const fn clk_counter_match_val(&self) -> &CLK_COUNTER_MATCH_VAL {
        &self.clk_counter_match_val
    }
    #[doc = "0xbc - CLK_COUNTER"]
    #[inline(always)]
    pub const fn clk_counter(&self) -> &CLK_COUNTER {
        &self.clk_counter
    }
    #[doc = "0xc0 - IFS_COUNTER"]
    #[inline(always)]
    pub const fn ifs_counter(&self) -> &IFS_COUNTER {
        &self.ifs_counter
    }
    #[doc = "0xc4 - SFD_WAIT_SYMBOL"]
    #[inline(always)]
    pub const fn sfd_wait_symbol(&self) -> &SFD_WAIT_SYMBOL {
        &self.sfd_wait_symbol
    }
    #[doc = "0xc8 - TXRX_PATH_DELAY"]
    #[inline(always)]
    pub const fn txrx_path_delay(&self) -> &TXRX_PATH_DELAY {
        &self.txrx_path_delay
    }
    #[doc = "0xcc - BB_CLK"]
    #[inline(always)]
    pub const fn bb_clk(&self) -> &BB_CLK {
        &self.bb_clk
    }
    #[doc = "0xd0 - TXDMA_ADDR"]
    #[inline(always)]
    pub const fn txdma_addr(&self) -> &TXDMA_ADDR {
        &self.txdma_addr
    }
    #[doc = "0xd4 - TXDMA_CTRL_STATE"]
    #[inline(always)]
    pub const fn txdma_ctrl_state(&self) -> &TXDMA_CTRL_STATE {
        &self.txdma_ctrl_state
    }
    #[doc = "0xd8 - TXDMA_ERR"]
    #[inline(always)]
    pub const fn txdma_err(&self) -> &TXDMA_ERR {
        &self.txdma_err
    }
    #[doc = "0xe0 - RXDMA_ADDR"]
    #[inline(always)]
    pub const fn rxdma_addr(&self) -> &RXDMA_ADDR {
        &self.rxdma_addr
    }
    #[doc = "0xe4 - RXDMA_CTRL_STATE"]
    #[inline(always)]
    pub const fn rxdma_ctrl_state(&self) -> &RXDMA_CTRL_STATE {
        &self.rxdma_ctrl_state
    }
    #[doc = "0xe8 - RXDMA_ERR"]
    #[inline(always)]
    pub const fn rxdma_err(&self) -> &RXDMA_ERR {
        &self.rxdma_err
    }
    #[doc = "0xf0 - DMA_GCK_CFG"]
    #[inline(always)]
    pub const fn dma_gck_cfg(&self) -> &DMA_GCK_CFG {
        &self.dma_gck_cfg
    }
    #[doc = "0xf4 - DMA_DUMMY"]
    #[inline(always)]
    pub const fn dma_dummy(&self) -> &DMA_DUMMY {
        &self.dma_dummy
    }
    #[doc = "0x100 - PAON_DELAY"]
    #[inline(always)]
    pub const fn paon_delay(&self) -> &PAON_DELAY {
        &self.paon_delay
    }
    #[doc = "0x104 - TXON_DELAY"]
    #[inline(always)]
    pub const fn txon_delay(&self) -> &TXON_DELAY {
        &self.txon_delay
    }
    #[doc = "0x108 - TXEN_STOP_DELAY"]
    #[inline(always)]
    pub const fn txen_stop_delay(&self) -> &TXEN_STOP_DELAY {
        &self.txen_stop_delay
    }
    #[doc = "0x10c - TXOFF_DELAY"]
    #[inline(always)]
    pub const fn txoff_delay(&self) -> &TXOFF_DELAY {
        &self.txoff_delay
    }
    #[doc = "0x110 - RXON_DELAY"]
    #[inline(always)]
    pub const fn rxon_delay(&self) -> &RXON_DELAY {
        &self.rxon_delay
    }
    #[doc = "0x114 - TXRX_SWITCH_DELAY"]
    #[inline(always)]
    pub const fn txrx_switch_delay(&self) -> &TXRX_SWITCH_DELAY {
        &self.txrx_switch_delay
    }
    #[doc = "0x118 - CONT_RX_DELAY"]
    #[inline(always)]
    pub const fn cont_rx_delay(&self) -> &CONT_RX_DELAY {
        &self.cont_rx_delay
    }
    #[doc = "0x11c - DCDC_CTRL"]
    #[inline(always)]
    pub const fn dcdc_ctrl(&self) -> &DCDC_CTRL {
        &self.dcdc_ctrl
    }
    #[doc = "0x120 - DEBUG_CTRL"]
    #[inline(always)]
    pub const fn debug_ctrl(&self) -> &DEBUG_CTRL {
        &self.debug_ctrl
    }
    #[doc = "0x128 - SEC_CTRL"]
    #[inline(always)]
    pub const fn sec_ctrl(&self) -> &SEC_CTRL {
        &self.sec_ctrl
    }
    #[doc = "0x12c - SEC_EXTEND_ADDRESS0"]
    #[inline(always)]
    pub const fn sec_extend_address0(&self) -> &SEC_EXTEND_ADDRESS0 {
        &self.sec_extend_address0
    }
    #[doc = "0x130 - SEC_EXTEND_ADDRESS1"]
    #[inline(always)]
    pub const fn sec_extend_address1(&self) -> &SEC_EXTEND_ADDRESS1 {
        &self.sec_extend_address1
    }
    #[doc = "0x134..0x144 - SEC_KEY%s"]
    #[inline(always)]
    pub const fn sec_key(&self, n: usize) -> &SEC_KEY {
        &self.sec_key[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x134..0x144 - SEC_KEY%s"]
    #[inline(always)]
    pub fn sec_key_iter(&self) -> impl Iterator<Item = &SEC_KEY> {
        self.sec_key.iter()
    }
    #[doc = "0x144 - SFD_TIMEOUT_CNT"]
    #[inline(always)]
    pub const fn sfd_timeout_cnt(&self) -> &SFD_TIMEOUT_CNT {
        &self.sfd_timeout_cnt
    }
    #[doc = "0x148 - CRC_ERROR_CNT"]
    #[inline(always)]
    pub const fn crc_error_cnt(&self) -> &CRC_ERROR_CNT {
        &self.crc_error_cnt
    }
    #[doc = "0x14c - ED_ABORT_CNT"]
    #[inline(always)]
    pub const fn ed_abort_cnt(&self) -> &ED_ABORT_CNT {
        &self.ed_abort_cnt
    }
    #[doc = "0x150 - CCA_FAIL_CNT"]
    #[inline(always)]
    pub const fn cca_fail_cnt(&self) -> &CCA_FAIL_CNT {
        &self.cca_fail_cnt
    }
    #[doc = "0x154 - RX_FILTER_FAIL_CNT"]
    #[inline(always)]
    pub const fn rx_filter_fail_cnt(&self) -> &RX_FILTER_FAIL_CNT {
        &self.rx_filter_fail_cnt
    }
    #[doc = "0x158 - NO_RSS_DETECT_CNT"]
    #[inline(always)]
    pub const fn no_rss_detect_cnt(&self) -> &NO_RSS_DETECT_CNT {
        &self.no_rss_detect_cnt
    }
    #[doc = "0x15c - RX_ABORT_COEX_CNT"]
    #[inline(always)]
    pub const fn rx_abort_coex_cnt(&self) -> &RX_ABORT_COEX_CNT {
        &self.rx_abort_coex_cnt
    }
    #[doc = "0x160 - RX_RESTART_CNT"]
    #[inline(always)]
    pub const fn rx_restart_cnt(&self) -> &RX_RESTART_CNT {
        &self.rx_restart_cnt
    }
    #[doc = "0x164 - TX_ACK_ABORT_COEX_CNT"]
    #[inline(always)]
    pub const fn tx_ack_abort_coex_cnt(&self) -> &TX_ACK_ABORT_COEX_CNT {
        &self.tx_ack_abort_coex_cnt
    }
    #[doc = "0x168 - ED_SCAN_COEX_CNT"]
    #[inline(always)]
    pub const fn ed_scan_coex_cnt(&self) -> &ED_SCAN_COEX_CNT {
        &self.ed_scan_coex_cnt
    }
    #[doc = "0x16c - RX_ACK_ABORT_COEX_CNT"]
    #[inline(always)]
    pub const fn rx_ack_abort_coex_cnt(&self) -> &RX_ACK_ABORT_COEX_CNT {
        &self.rx_ack_abort_coex_cnt
    }
    #[doc = "0x170 - RX_ACK_TIMEOUT_CNT"]
    #[inline(always)]
    pub const fn rx_ack_timeout_cnt(&self) -> &RX_ACK_TIMEOUT_CNT {
        &self.rx_ack_timeout_cnt
    }
    #[doc = "0x174 - TX_BREAK_COEX_CNT"]
    #[inline(always)]
    pub const fn tx_break_coex_cnt(&self) -> &TX_BREAK_COEX_CNT {
        &self.tx_break_coex_cnt
    }
    #[doc = "0x178 - TX_SECURITY_ERROR_CNT"]
    #[inline(always)]
    pub const fn tx_security_error_cnt(&self) -> &TX_SECURITY_ERROR_CNT {
        &self.tx_security_error_cnt
    }
    #[doc = "0x17c - CCA_BUSY_CNT"]
    #[inline(always)]
    pub const fn cca_busy_cnt(&self) -> &CCA_BUSY_CNT {
        &self.cca_busy_cnt
    }
    #[doc = "0x180 - ERROR_CNT_CLEAR"]
    #[inline(always)]
    pub const fn error_cnt_clear(&self) -> &ERROR_CNT_CLEAR {
        &self.error_cnt_clear
    }
    #[doc = "0x184 - DEBUG_SEL_CFG0"]
    #[inline(always)]
    pub const fn debug_sel_cfg0(&self) -> &DEBUG_SEL_CFG0 {
        &self.debug_sel_cfg0
    }
    #[doc = "0x188 - DEBUG_SEL_CFG1"]
    #[inline(always)]
    pub const fn debug_sel_cfg1(&self) -> &DEBUG_SEL_CFG1 {
        &self.debug_sel_cfg1
    }
    #[doc = "0x18c - RX_TIMEOUT_TARGET_CNT"]
    #[inline(always)]
    pub const fn rx_timeout_target_cnt(&self) -> &RX_TIMEOUT_TARGET_CNT {
        &self.rx_timeout_target_cnt
    }
    #[doc = "0x190 - MODSLP_CONFIG"]
    #[inline(always)]
    pub const fn modslp_config(&self) -> &MODSLP_CONFIG {
        &self.modslp_config
    }
    #[doc = "0x194 - MODSLP_STATUS"]
    #[inline(always)]
    pub const fn modslp_status(&self) -> &MODSLP_STATUS {
        &self.modslp_status
    }
    #[doc = "0x198 - MODSLP_WW"]
    #[inline(always)]
    pub const fn modslp_ww(&self) -> &MODSLP_WW {
        &self.modslp_ww
    }
    #[doc = "0x19c - MODSLP_INTR_WAKE_CONFIG"]
    #[inline(always)]
    pub const fn modslp_intr_wake_config(&self) -> &MODSLP_INTR_WAKE_CONFIG {
        &self.modslp_intr_wake_config
    }
    #[doc = "0x1a0 - MAC_DATE"]
    #[inline(always)]
    pub const fn mac_date(&self) -> &MAC_DATE {
        &self.mac_date
    }
}
#[doc = "COMMAND (rw) register accessor: COMMAND\n\nYou can [`read`](crate::Reg::read) this register and get [`command::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@command`] module"]
pub type COMMAND = crate::Reg<command::COMMAND_SPEC>;
#[doc = "COMMAND"]
pub mod command;
#[doc = "CTRL_CFG (rw) register accessor: CTRL_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_cfg`] module"]
pub type CTRL_CFG = crate::Reg<ctrl_cfg::CTRL_CFG_SPEC>;
#[doc = "CTRL_CFG"]
pub mod ctrl_cfg;
#[doc = "Cluster INF%s, containing INF?_EXTEND_ADDR0, INF?_EXTEND_ADDR1, INF?_PAN_ID, INF?_SHORT_ADDR"]
pub use self::inf::INF;
#[doc = r"Cluster"]
#[doc = "Cluster INF%s, containing INF?_EXTEND_ADDR0, INF?_EXTEND_ADDR1, INF?_PAN_ID, INF?_SHORT_ADDR"]
pub mod inf;
#[doc = "CHANNEL (rw) register accessor: CHANNEL\n\nYou can [`read`](crate::Reg::read) this register and get [`channel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`channel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel`] module"]
pub type CHANNEL = crate::Reg<channel::CHANNEL_SPEC>;
#[doc = "CHANNEL"]
pub mod channel;
#[doc = "TX_POWER (rw) register accessor: TX_POWER\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_power`] module"]
pub type TX_POWER = crate::Reg<tx_power::TX_POWER_SPEC>;
#[doc = "TX_POWER"]
pub mod tx_power;
#[doc = "ED_SCAN_DURATION (rw) register accessor: ED_SCAN_DURATION\n\nYou can [`read`](crate::Reg::read) this register and get [`ed_scan_duration::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ed_scan_duration::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ed_scan_duration`] module"]
pub type ED_SCAN_DURATION = crate::Reg<ed_scan_duration::ED_SCAN_DURATION_SPEC>;
#[doc = "ED_SCAN_DURATION"]
pub mod ed_scan_duration;
#[doc = "ED_SCAN_CFG (rw) register accessor: ED_SCAN_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`ed_scan_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ed_scan_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ed_scan_cfg`] module"]
pub type ED_SCAN_CFG = crate::Reg<ed_scan_cfg::ED_SCAN_CFG_SPEC>;
#[doc = "ED_SCAN_CFG"]
pub mod ed_scan_cfg;
#[doc = "IFS (rw) register accessor: IFS\n\nYou can [`read`](crate::Reg::read) this register and get [`ifs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifs`] module"]
pub type IFS = crate::Reg<ifs::IFS_SPEC>;
#[doc = "IFS"]
pub mod ifs;
#[doc = "ACK_TIMEOUT (rw) register accessor: ACK_TIMEOUT\n\nYou can [`read`](crate::Reg::read) this register and get [`ack_timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ack_timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ack_timeout`] module"]
pub type ACK_TIMEOUT = crate::Reg<ack_timeout::ACK_TIMEOUT_SPEC>;
#[doc = "ACK_TIMEOUT"]
pub mod ack_timeout;
#[doc = "EVENT_EN (rw) register accessor: EVENT_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`event_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`event_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@event_en`] module"]
pub type EVENT_EN = crate::Reg<event_en::EVENT_EN_SPEC>;
#[doc = "EVENT_EN"]
pub mod event_en;
#[doc = "EVENT_STATUS (rw) register accessor: EVENT_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`event_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`event_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@event_status`] module"]
pub type EVENT_STATUS = crate::Reg<event_status::EVENT_STATUS_SPEC>;
#[doc = "EVENT_STATUS"]
pub mod event_status;
#[doc = "RX_ABORT_INTR_CTRL (rw) register accessor: RX_ABORT_INTR_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_abort_intr_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_abort_intr_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_abort_intr_ctrl`] module"]
pub type RX_ABORT_INTR_CTRL = crate::Reg<rx_abort_intr_ctrl::RX_ABORT_INTR_CTRL_SPEC>;
#[doc = "RX_ABORT_INTR_CTRL"]
pub mod rx_abort_intr_ctrl;
#[doc = "ACK_FRAME_PENDING_EN (rw) register accessor: ACK_FRAME_PENDING_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`ack_frame_pending_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ack_frame_pending_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ack_frame_pending_en`] module"]
pub type ACK_FRAME_PENDING_EN = crate::Reg<ack_frame_pending_en::ACK_FRAME_PENDING_EN_SPEC>;
#[doc = "ACK_FRAME_PENDING_EN"]
pub mod ack_frame_pending_en;
#[doc = "COEX_PTI (rw) register accessor: COEX_PTI\n\nYou can [`read`](crate::Reg::read) this register and get [`coex_pti::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`coex_pti::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@coex_pti`] module"]
pub type COEX_PTI = crate::Reg<coex_pti::COEX_PTI_SPEC>;
#[doc = "COEX_PTI"]
pub mod coex_pti;
#[doc = "TX_ABORT_INTERRUPT_CONTROL (rw) register accessor: TX_ABORT_INTERRUPT_CONTROL\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_abort_interrupt_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_abort_interrupt_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_abort_interrupt_control`] module"]
pub type TX_ABORT_INTERRUPT_CONTROL =
    crate::Reg<tx_abort_interrupt_control::TX_ABORT_INTERRUPT_CONTROL_SPEC>;
#[doc = "TX_ABORT_INTERRUPT_CONTROL"]
pub mod tx_abort_interrupt_control;
#[doc = "ENHANCE_ACK_CFG (rw) register accessor: ENHANCE_ACK_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`enhance_ack_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enhance_ack_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enhance_ack_cfg`] module"]
pub type ENHANCE_ACK_CFG = crate::Reg<enhance_ack_cfg::ENHANCE_ACK_CFG_SPEC>;
#[doc = "ENHANCE_ACK_CFG"]
pub mod enhance_ack_cfg;
#[doc = "RX_STATUS (rw) register accessor: RX_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_status`] module"]
pub type RX_STATUS = crate::Reg<rx_status::RX_STATUS_SPEC>;
#[doc = "RX_STATUS"]
pub mod rx_status;
#[doc = "TX_STATUS (rw) register accessor: TX_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_status`] module"]
pub type TX_STATUS = crate::Reg<tx_status::TX_STATUS_SPEC>;
#[doc = "TX_STATUS"]
pub mod tx_status;
#[doc = "TXRX_STATUS (rw) register accessor: TXRX_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`txrx_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txrx_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txrx_status`] module"]
pub type TXRX_STATUS = crate::Reg<txrx_status::TXRX_STATUS_SPEC>;
#[doc = "TXRX_STATUS"]
pub mod txrx_status;
#[doc = "TX_CCM_SCHEDULE_STATUS (rw) register accessor: TX_CCM_SCHEDULE_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_ccm_schedule_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ccm_schedule_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_ccm_schedule_status`] module"]
pub type TX_CCM_SCHEDULE_STATUS = crate::Reg<tx_ccm_schedule_status::TX_CCM_SCHEDULE_STATUS_SPEC>;
#[doc = "TX_CCM_SCHEDULE_STATUS"]
pub mod tx_ccm_schedule_status;
#[doc = "RX_LENGTH (rw) register accessor: RX_LENGTH\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_length::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_length::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_length`] module"]
pub type RX_LENGTH = crate::Reg<rx_length::RX_LENGTH_SPEC>;
#[doc = "RX_LENGTH"]
pub mod rx_length;
#[doc = "Cluster TIME%s, containing TIME?_THRESHOLD, TIME?_VALUE"]
pub use self::time::TIME;
#[doc = r"Cluster"]
#[doc = "Cluster TIME%s, containing TIME?_THRESHOLD, TIME?_VALUE"]
pub mod time;
#[doc = "CLK_COUNTER_MATCH_VAL (rw) register accessor: CLK_COUNTER_MATCH_VAL\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_counter_match_val::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_counter_match_val::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_counter_match_val`] module"]
pub type CLK_COUNTER_MATCH_VAL = crate::Reg<clk_counter_match_val::CLK_COUNTER_MATCH_VAL_SPEC>;
#[doc = "CLK_COUNTER_MATCH_VAL"]
pub mod clk_counter_match_val;
#[doc = "CLK_COUNTER (rw) register accessor: CLK_COUNTER\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_counter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_counter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_counter`] module"]
pub type CLK_COUNTER = crate::Reg<clk_counter::CLK_COUNTER_SPEC>;
#[doc = "CLK_COUNTER"]
pub mod clk_counter;
#[doc = "IFS_COUNTER (rw) register accessor: IFS_COUNTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ifs_counter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifs_counter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifs_counter`] module"]
pub type IFS_COUNTER = crate::Reg<ifs_counter::IFS_COUNTER_SPEC>;
#[doc = "IFS_COUNTER"]
pub mod ifs_counter;
#[doc = "SFD_WAIT_SYMBOL (rw) register accessor: SFD_WAIT_SYMBOL\n\nYou can [`read`](crate::Reg::read) this register and get [`sfd_wait_symbol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfd_wait_symbol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfd_wait_symbol`] module"]
pub type SFD_WAIT_SYMBOL = crate::Reg<sfd_wait_symbol::SFD_WAIT_SYMBOL_SPEC>;
#[doc = "SFD_WAIT_SYMBOL"]
pub mod sfd_wait_symbol;
#[doc = "TXRX_PATH_DELAY (rw) register accessor: TXRX_PATH_DELAY\n\nYou can [`read`](crate::Reg::read) this register and get [`txrx_path_delay::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txrx_path_delay::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txrx_path_delay`] module"]
pub type TXRX_PATH_DELAY = crate::Reg<txrx_path_delay::TXRX_PATH_DELAY_SPEC>;
#[doc = "TXRX_PATH_DELAY"]
pub mod txrx_path_delay;
#[doc = "BB_CLK (rw) register accessor: BB_CLK\n\nYou can [`read`](crate::Reg::read) this register and get [`bb_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bb_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bb_clk`] module"]
pub type BB_CLK = crate::Reg<bb_clk::BB_CLK_SPEC>;
#[doc = "BB_CLK"]
pub mod bb_clk;
#[doc = "TXDMA_ADDR (rw) register accessor: TXDMA_ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`txdma_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdma_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdma_addr`] module"]
pub type TXDMA_ADDR = crate::Reg<txdma_addr::TXDMA_ADDR_SPEC>;
#[doc = "TXDMA_ADDR"]
pub mod txdma_addr;
#[doc = "TXDMA_CTRL_STATE (rw) register accessor: TXDMA_CTRL_STATE\n\nYou can [`read`](crate::Reg::read) this register and get [`txdma_ctrl_state::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdma_ctrl_state::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdma_ctrl_state`] module"]
pub type TXDMA_CTRL_STATE = crate::Reg<txdma_ctrl_state::TXDMA_CTRL_STATE_SPEC>;
#[doc = "TXDMA_CTRL_STATE"]
pub mod txdma_ctrl_state;
#[doc = "TXDMA_ERR (rw) register accessor: TXDMA_ERR\n\nYou can [`read`](crate::Reg::read) this register and get [`txdma_err::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdma_err::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdma_err`] module"]
pub type TXDMA_ERR = crate::Reg<txdma_err::TXDMA_ERR_SPEC>;
#[doc = "TXDMA_ERR"]
pub mod txdma_err;
#[doc = "RXDMA_ADDR (rw) register accessor: RXDMA_ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdma_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdma_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdma_addr`] module"]
pub type RXDMA_ADDR = crate::Reg<rxdma_addr::RXDMA_ADDR_SPEC>;
#[doc = "RXDMA_ADDR"]
pub mod rxdma_addr;
#[doc = "RXDMA_CTRL_STATE (rw) register accessor: RXDMA_CTRL_STATE\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdma_ctrl_state::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdma_ctrl_state::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdma_ctrl_state`] module"]
pub type RXDMA_CTRL_STATE = crate::Reg<rxdma_ctrl_state::RXDMA_CTRL_STATE_SPEC>;
#[doc = "RXDMA_CTRL_STATE"]
pub mod rxdma_ctrl_state;
#[doc = "RXDMA_ERR (rw) register accessor: RXDMA_ERR\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdma_err::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdma_err::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdma_err`] module"]
pub type RXDMA_ERR = crate::Reg<rxdma_err::RXDMA_ERR_SPEC>;
#[doc = "RXDMA_ERR"]
pub mod rxdma_err;
#[doc = "DMA_GCK_CFG (rw) register accessor: DMA_GCK_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_gck_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_gck_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_gck_cfg`] module"]
pub type DMA_GCK_CFG = crate::Reg<dma_gck_cfg::DMA_GCK_CFG_SPEC>;
#[doc = "DMA_GCK_CFG"]
pub mod dma_gck_cfg;
#[doc = "DMA_DUMMY (rw) register accessor: DMA_DUMMY\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_dummy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_dummy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_dummy`] module"]
pub type DMA_DUMMY = crate::Reg<dma_dummy::DMA_DUMMY_SPEC>;
#[doc = "DMA_DUMMY"]
pub mod dma_dummy;
#[doc = "PAON_DELAY (rw) register accessor: PAON_DELAY\n\nYou can [`read`](crate::Reg::read) this register and get [`paon_delay::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`paon_delay::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@paon_delay`] module"]
pub type PAON_DELAY = crate::Reg<paon_delay::PAON_DELAY_SPEC>;
#[doc = "PAON_DELAY"]
pub mod paon_delay;
#[doc = "TXON_DELAY (rw) register accessor: TXON_DELAY\n\nYou can [`read`](crate::Reg::read) this register and get [`txon_delay::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txon_delay::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txon_delay`] module"]
pub type TXON_DELAY = crate::Reg<txon_delay::TXON_DELAY_SPEC>;
#[doc = "TXON_DELAY"]
pub mod txon_delay;
#[doc = "TXEN_STOP_DELAY (rw) register accessor: TXEN_STOP_DELAY\n\nYou can [`read`](crate::Reg::read) this register and get [`txen_stop_delay::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txen_stop_delay::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txen_stop_delay`] module"]
pub type TXEN_STOP_DELAY = crate::Reg<txen_stop_delay::TXEN_STOP_DELAY_SPEC>;
#[doc = "TXEN_STOP_DELAY"]
pub mod txen_stop_delay;
#[doc = "TXOFF_DELAY (rw) register accessor: TXOFF_DELAY\n\nYou can [`read`](crate::Reg::read) this register and get [`txoff_delay::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txoff_delay::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txoff_delay`] module"]
pub type TXOFF_DELAY = crate::Reg<txoff_delay::TXOFF_DELAY_SPEC>;
#[doc = "TXOFF_DELAY"]
pub mod txoff_delay;
#[doc = "RXON_DELAY (rw) register accessor: RXON_DELAY\n\nYou can [`read`](crate::Reg::read) this register and get [`rxon_delay::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxon_delay::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxon_delay`] module"]
pub type RXON_DELAY = crate::Reg<rxon_delay::RXON_DELAY_SPEC>;
#[doc = "RXON_DELAY"]
pub mod rxon_delay;
#[doc = "TXRX_SWITCH_DELAY (rw) register accessor: TXRX_SWITCH_DELAY\n\nYou can [`read`](crate::Reg::read) this register and get [`txrx_switch_delay::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txrx_switch_delay::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txrx_switch_delay`] module"]
pub type TXRX_SWITCH_DELAY = crate::Reg<txrx_switch_delay::TXRX_SWITCH_DELAY_SPEC>;
#[doc = "TXRX_SWITCH_DELAY"]
pub mod txrx_switch_delay;
#[doc = "CONT_RX_DELAY (rw) register accessor: CONT_RX_DELAY\n\nYou can [`read`](crate::Reg::read) this register and get [`cont_rx_delay::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cont_rx_delay::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cont_rx_delay`] module"]
pub type CONT_RX_DELAY = crate::Reg<cont_rx_delay::CONT_RX_DELAY_SPEC>;
#[doc = "CONT_RX_DELAY"]
pub mod cont_rx_delay;
#[doc = "DCDC_CTRL (rw) register accessor: DCDC_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdc_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdc_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcdc_ctrl`] module"]
pub type DCDC_CTRL = crate::Reg<dcdc_ctrl::DCDC_CTRL_SPEC>;
#[doc = "DCDC_CTRL"]
pub mod dcdc_ctrl;
#[doc = "DEBUG_CTRL (rw) register accessor: DEBUG_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_ctrl`] module"]
pub type DEBUG_CTRL = crate::Reg<debug_ctrl::DEBUG_CTRL_SPEC>;
#[doc = "DEBUG_CTRL"]
pub mod debug_ctrl;
#[doc = "SEC_CTRL (rw) register accessor: SEC_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_ctrl`] module"]
pub type SEC_CTRL = crate::Reg<sec_ctrl::SEC_CTRL_SPEC>;
#[doc = "SEC_CTRL"]
pub mod sec_ctrl;
#[doc = "SEC_EXTEND_ADDRESS0 (rw) register accessor: SEC_EXTEND_ADDRESS0\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_extend_address0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_extend_address0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_extend_address0`] module"]
pub type SEC_EXTEND_ADDRESS0 = crate::Reg<sec_extend_address0::SEC_EXTEND_ADDRESS0_SPEC>;
#[doc = "SEC_EXTEND_ADDRESS0"]
pub mod sec_extend_address0;
#[doc = "SEC_EXTEND_ADDRESS1 (rw) register accessor: SEC_EXTEND_ADDRESS1\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_extend_address1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_extend_address1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_extend_address1`] module"]
pub type SEC_EXTEND_ADDRESS1 = crate::Reg<sec_extend_address1::SEC_EXTEND_ADDRESS1_SPEC>;
#[doc = "SEC_EXTEND_ADDRESS1"]
pub mod sec_extend_address1;
#[doc = "SEC_KEY (rw) register accessor: SEC_KEY%s\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_key::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_key::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec_key`] module"]
pub type SEC_KEY = crate::Reg<sec_key::SEC_KEY_SPEC>;
#[doc = "SEC_KEY%s"]
pub mod sec_key;
#[doc = "SFD_TIMEOUT_CNT (rw) register accessor: SFD_TIMEOUT_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`sfd_timeout_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfd_timeout_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfd_timeout_cnt`] module"]
pub type SFD_TIMEOUT_CNT = crate::Reg<sfd_timeout_cnt::SFD_TIMEOUT_CNT_SPEC>;
#[doc = "SFD_TIMEOUT_CNT"]
pub mod sfd_timeout_cnt;
#[doc = "CRC_ERROR_CNT (rw) register accessor: CRC_ERROR_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_error_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_error_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_error_cnt`] module"]
pub type CRC_ERROR_CNT = crate::Reg<crc_error_cnt::CRC_ERROR_CNT_SPEC>;
#[doc = "CRC_ERROR_CNT"]
pub mod crc_error_cnt;
#[doc = "ED_ABORT_CNT (rw) register accessor: ED_ABORT_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`ed_abort_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ed_abort_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ed_abort_cnt`] module"]
pub type ED_ABORT_CNT = crate::Reg<ed_abort_cnt::ED_ABORT_CNT_SPEC>;
#[doc = "ED_ABORT_CNT"]
pub mod ed_abort_cnt;
#[doc = "CCA_FAIL_CNT (rw) register accessor: CCA_FAIL_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`cca_fail_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cca_fail_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cca_fail_cnt`] module"]
pub type CCA_FAIL_CNT = crate::Reg<cca_fail_cnt::CCA_FAIL_CNT_SPEC>;
#[doc = "CCA_FAIL_CNT"]
pub mod cca_fail_cnt;
#[doc = "RX_FILTER_FAIL_CNT (rw) register accessor: RX_FILTER_FAIL_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_filter_fail_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_filter_fail_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_filter_fail_cnt`] module"]
pub type RX_FILTER_FAIL_CNT = crate::Reg<rx_filter_fail_cnt::RX_FILTER_FAIL_CNT_SPEC>;
#[doc = "RX_FILTER_FAIL_CNT"]
pub mod rx_filter_fail_cnt;
#[doc = "NO_RSS_DETECT_CNT (rw) register accessor: NO_RSS_DETECT_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`no_rss_detect_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`no_rss_detect_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@no_rss_detect_cnt`] module"]
pub type NO_RSS_DETECT_CNT = crate::Reg<no_rss_detect_cnt::NO_RSS_DETECT_CNT_SPEC>;
#[doc = "NO_RSS_DETECT_CNT"]
pub mod no_rss_detect_cnt;
#[doc = "RX_ABORT_COEX_CNT (rw) register accessor: RX_ABORT_COEX_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_abort_coex_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_abort_coex_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_abort_coex_cnt`] module"]
pub type RX_ABORT_COEX_CNT = crate::Reg<rx_abort_coex_cnt::RX_ABORT_COEX_CNT_SPEC>;
#[doc = "RX_ABORT_COEX_CNT"]
pub mod rx_abort_coex_cnt;
#[doc = "RX_RESTART_CNT (rw) register accessor: RX_RESTART_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_restart_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_restart_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_restart_cnt`] module"]
pub type RX_RESTART_CNT = crate::Reg<rx_restart_cnt::RX_RESTART_CNT_SPEC>;
#[doc = "RX_RESTART_CNT"]
pub mod rx_restart_cnt;
#[doc = "TX_ACK_ABORT_COEX_CNT (rw) register accessor: TX_ACK_ABORT_COEX_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_ack_abort_coex_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_ack_abort_coex_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_ack_abort_coex_cnt`] module"]
pub type TX_ACK_ABORT_COEX_CNT = crate::Reg<tx_ack_abort_coex_cnt::TX_ACK_ABORT_COEX_CNT_SPEC>;
#[doc = "TX_ACK_ABORT_COEX_CNT"]
pub mod tx_ack_abort_coex_cnt;
#[doc = "ED_SCAN_COEX_CNT (rw) register accessor: ED_SCAN_COEX_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`ed_scan_coex_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ed_scan_coex_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ed_scan_coex_cnt`] module"]
pub type ED_SCAN_COEX_CNT = crate::Reg<ed_scan_coex_cnt::ED_SCAN_COEX_CNT_SPEC>;
#[doc = "ED_SCAN_COEX_CNT"]
pub mod ed_scan_coex_cnt;
#[doc = "RX_ACK_ABORT_COEX_CNT (rw) register accessor: RX_ACK_ABORT_COEX_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ack_abort_coex_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ack_abort_coex_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ack_abort_coex_cnt`] module"]
pub type RX_ACK_ABORT_COEX_CNT = crate::Reg<rx_ack_abort_coex_cnt::RX_ACK_ABORT_COEX_CNT_SPEC>;
#[doc = "RX_ACK_ABORT_COEX_CNT"]
pub mod rx_ack_abort_coex_cnt;
#[doc = "RX_ACK_TIMEOUT_CNT (rw) register accessor: RX_ACK_TIMEOUT_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ack_timeout_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ack_timeout_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_ack_timeout_cnt`] module"]
pub type RX_ACK_TIMEOUT_CNT = crate::Reg<rx_ack_timeout_cnt::RX_ACK_TIMEOUT_CNT_SPEC>;
#[doc = "RX_ACK_TIMEOUT_CNT"]
pub mod rx_ack_timeout_cnt;
#[doc = "TX_BREAK_COEX_CNT (rw) register accessor: TX_BREAK_COEX_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_break_coex_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_break_coex_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_break_coex_cnt`] module"]
pub type TX_BREAK_COEX_CNT = crate::Reg<tx_break_coex_cnt::TX_BREAK_COEX_CNT_SPEC>;
#[doc = "TX_BREAK_COEX_CNT"]
pub mod tx_break_coex_cnt;
#[doc = "TX_SECURITY_ERROR_CNT (rw) register accessor: TX_SECURITY_ERROR_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_security_error_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_security_error_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_security_error_cnt`] module"]
pub type TX_SECURITY_ERROR_CNT = crate::Reg<tx_security_error_cnt::TX_SECURITY_ERROR_CNT_SPEC>;
#[doc = "TX_SECURITY_ERROR_CNT"]
pub mod tx_security_error_cnt;
#[doc = "CCA_BUSY_CNT (rw) register accessor: CCA_BUSY_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`cca_busy_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cca_busy_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cca_busy_cnt`] module"]
pub type CCA_BUSY_CNT = crate::Reg<cca_busy_cnt::CCA_BUSY_CNT_SPEC>;
#[doc = "CCA_BUSY_CNT"]
pub mod cca_busy_cnt;
#[doc = "ERROR_CNT_CLEAR (rw) register accessor: ERROR_CNT_CLEAR\n\nYou can [`read`](crate::Reg::read) this register and get [`error_cnt_clear::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`error_cnt_clear::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@error_cnt_clear`] module"]
pub type ERROR_CNT_CLEAR = crate::Reg<error_cnt_clear::ERROR_CNT_CLEAR_SPEC>;
#[doc = "ERROR_CNT_CLEAR"]
pub mod error_cnt_clear;
#[doc = "DEBUG_SEL_CFG0 (rw) register accessor: DEBUG_SEL_CFG0\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_sel_cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_sel_cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_sel_cfg0`] module"]
pub type DEBUG_SEL_CFG0 = crate::Reg<debug_sel_cfg0::DEBUG_SEL_CFG0_SPEC>;
#[doc = "DEBUG_SEL_CFG0"]
pub mod debug_sel_cfg0;
#[doc = "DEBUG_SEL_CFG1 (rw) register accessor: DEBUG_SEL_CFG1\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_sel_cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_sel_cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug_sel_cfg1`] module"]
pub type DEBUG_SEL_CFG1 = crate::Reg<debug_sel_cfg1::DEBUG_SEL_CFG1_SPEC>;
#[doc = "DEBUG_SEL_CFG1"]
pub mod debug_sel_cfg1;
#[doc = "RX_TIMEOUT_TARGET_CNT (rw) register accessor: RX_TIMEOUT_TARGET_CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_timeout_target_cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_timeout_target_cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_timeout_target_cnt`] module"]
pub type RX_TIMEOUT_TARGET_CNT = crate::Reg<rx_timeout_target_cnt::RX_TIMEOUT_TARGET_CNT_SPEC>;
#[doc = "RX_TIMEOUT_TARGET_CNT"]
pub mod rx_timeout_target_cnt;
#[doc = "MODSLP_CONFIG (rw) register accessor: MODSLP_CONFIG\n\nYou can [`read`](crate::Reg::read) this register and get [`modslp_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modslp_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modslp_config`] module"]
pub type MODSLP_CONFIG = crate::Reg<modslp_config::MODSLP_CONFIG_SPEC>;
#[doc = "MODSLP_CONFIG"]
pub mod modslp_config;
#[doc = "MODSLP_STATUS (rw) register accessor: MODSLP_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`modslp_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modslp_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modslp_status`] module"]
pub type MODSLP_STATUS = crate::Reg<modslp_status::MODSLP_STATUS_SPEC>;
#[doc = "MODSLP_STATUS"]
pub mod modslp_status;
#[doc = "MODSLP_WW (rw) register accessor: MODSLP_WW\n\nYou can [`read`](crate::Reg::read) this register and get [`modslp_ww::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modslp_ww::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modslp_ww`] module"]
pub type MODSLP_WW = crate::Reg<modslp_ww::MODSLP_WW_SPEC>;
#[doc = "MODSLP_WW"]
pub mod modslp_ww;
#[doc = "MODSLP_INTR_WAKE_CONFIG (rw) register accessor: MODSLP_INTR_WAKE_CONFIG\n\nYou can [`read`](crate::Reg::read) this register and get [`modslp_intr_wake_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modslp_intr_wake_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modslp_intr_wake_config`] module"]
pub type MODSLP_INTR_WAKE_CONFIG =
    crate::Reg<modslp_intr_wake_config::MODSLP_INTR_WAKE_CONFIG_SPEC>;
#[doc = "MODSLP_INTR_WAKE_CONFIG"]
pub mod modslp_intr_wake_config;
#[doc = "MAC_DATE (rw) register accessor: MAC_DATE\n\nYou can [`read`](crate::Reg::read) this register and get [`mac_date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac_date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac_date`] module"]
pub type MAC_DATE = crate::Reg<mac_date::MAC_DATE_SPEC>;
#[doc = "MAC_DATE"]
pub mod mac_date;
