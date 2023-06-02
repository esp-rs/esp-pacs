#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C_SCL_LOW_PERIOD_REG"]
    pub scl_low_period: SCL_LOW_PERIOD,
    #[doc = "0x04 - I2C_CTR_REG"]
    pub ctr: CTR,
    #[doc = "0x08 - I2C_SR_REG"]
    pub sr: SR,
    #[doc = "0x0c - I2C_TO_REG"]
    pub to: TO,
    #[doc = "0x10 - I2C_SLAVE_ADDR_REG"]
    pub slave_addr: SLAVE_ADDR,
    #[doc = "0x14 - I2C_FIFO_ST_REG"]
    pub fifo_st: FIFO_ST,
    #[doc = "0x18 - I2C_FIFO_CONF_REG"]
    pub fifo_conf: FIFO_CONF,
    #[doc = "0x1c - I2C_FIFO_DATA_REG"]
    pub data: DATA,
    #[doc = "0x20 - I2C_INT_RAW_REG"]
    pub int_raw: INT_RAW,
    #[doc = "0x24 - I2C_INT_CLR_REG"]
    pub int_clr: INT_CLR,
    #[doc = "0x28 - I2C_INT_ENA_REG"]
    pub int_ena: INT_ENA,
    #[doc = "0x2c - I2C_INT_STATUS_REG"]
    pub int_status: INT_STATUS,
    #[doc = "0x30 - I2C_SDA_HOLD_REG"]
    pub sda_hold: SDA_HOLD,
    #[doc = "0x34 - I2C_SDA_SAMPLE_REG"]
    pub sda_sample: SDA_SAMPLE,
    #[doc = "0x38 - I2C_SCL_HIGH_PERIOD_REG"]
    pub scl_high_period: SCL_HIGH_PERIOD,
    _reserved15: [u8; 0x04],
    #[doc = "0x40 - I2C_SCL_START_HOLD_REG"]
    pub scl_start_hold: SCL_START_HOLD,
    #[doc = "0x44 - I2C_SCL_RSTART_SETUP_REG"]
    pub scl_rstart_setup: SCL_RSTART_SETUP,
    #[doc = "0x48 - I2C_SCL_STOP_HOLD_REG"]
    pub scl_stop_hold: SCL_STOP_HOLD,
    #[doc = "0x4c - I2C_SCL_STOP_SETUP_REG"]
    pub scl_stop_setup: SCL_STOP_SETUP,
    #[doc = "0x50 - I2C_FILTER_CFG_REG"]
    pub filter_cfg: FILTER_CFG,
    #[doc = "0x54 - I2C_CLK_CONF_REG"]
    pub clk_conf: CLK_CONF,
    #[doc = "0x58..0x78 - I2C_COMD%s_REG"]
    pub comd: [COMD; 8],
    #[doc = "0x78 - I2C_SCL_ST_TIME_OUT_REG"]
    pub scl_st_time_out: SCL_ST_TIME_OUT,
    #[doc = "0x7c - I2C_SCL_MAIN_ST_TIME_OUT_REG"]
    pub scl_main_st_time_out: SCL_MAIN_ST_TIME_OUT,
    #[doc = "0x80 - I2C_SCL_SP_CONF_REG"]
    pub scl_sp_conf: SCL_SP_CONF,
    #[doc = "0x84 - I2C_SCL_STRETCH_CONF_REG"]
    pub scl_stretch_conf: SCL_STRETCH_CONF,
    _reserved26: [u8; 0x70],
    #[doc = "0xf8 - I2C_DATE_REG"]
    pub date: DATE,
    _reserved27: [u8; 0x04],
    #[doc = "0x100 - I2C_TXFIFO_START_ADDR_REG"]
    pub txfifo_start_addr: TXFIFO_START_ADDR,
    _reserved28: [u8; 0x7c],
    #[doc = "0x180 - I2C_RXFIFO_START_ADDR_REG"]
    pub rxfifo_start_addr: RXFIFO_START_ADDR,
}
#[doc = "SCL_LOW_PERIOD (rw) register accessor: an alias for `Reg<SCL_LOW_PERIOD_SPEC>`"]
pub type SCL_LOW_PERIOD = crate::Reg<scl_low_period::SCL_LOW_PERIOD_SPEC>;
#[doc = "I2C_SCL_LOW_PERIOD_REG"]
pub mod scl_low_period;
#[doc = "CTR (rw) register accessor: an alias for `Reg<CTR_SPEC>`"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "I2C_CTR_REG"]
pub mod ctr;
#[doc = "SR (r) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "I2C_SR_REG"]
pub mod sr;
#[doc = "TO (rw) register accessor: an alias for `Reg<TO_SPEC>`"]
pub type TO = crate::Reg<to::TO_SPEC>;
#[doc = "I2C_TO_REG"]
pub mod to;
#[doc = "SLAVE_ADDR (rw) register accessor: an alias for `Reg<SLAVE_ADDR_SPEC>`"]
pub type SLAVE_ADDR = crate::Reg<slave_addr::SLAVE_ADDR_SPEC>;
#[doc = "I2C_SLAVE_ADDR_REG"]
pub mod slave_addr;
#[doc = "FIFO_ST (r) register accessor: an alias for `Reg<FIFO_ST_SPEC>`"]
pub type FIFO_ST = crate::Reg<fifo_st::FIFO_ST_SPEC>;
#[doc = "I2C_FIFO_ST_REG"]
pub mod fifo_st;
#[doc = "FIFO_CONF (rw) register accessor: an alias for `Reg<FIFO_CONF_SPEC>`"]
pub type FIFO_CONF = crate::Reg<fifo_conf::FIFO_CONF_SPEC>;
#[doc = "I2C_FIFO_CONF_REG"]
pub mod fifo_conf;
#[doc = "DATA (rw) register accessor: an alias for `Reg<DATA_SPEC>`"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "I2C_FIFO_DATA_REG"]
pub mod data;
#[doc = "INT_RAW (r) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "I2C_INT_RAW_REG"]
pub mod int_raw;
#[doc = "INT_CLR (w) register accessor: an alias for `Reg<INT_CLR_SPEC>`"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "I2C_INT_CLR_REG"]
pub mod int_clr;
#[doc = "INT_ENA (rw) register accessor: an alias for `Reg<INT_ENA_SPEC>`"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "I2C_INT_ENA_REG"]
pub mod int_ena;
#[doc = "INT_STATUS (r) register accessor: an alias for `Reg<INT_STATUS_SPEC>`"]
pub type INT_STATUS = crate::Reg<int_status::INT_STATUS_SPEC>;
#[doc = "I2C_INT_STATUS_REG"]
pub mod int_status;
#[doc = "SDA_HOLD (rw) register accessor: an alias for `Reg<SDA_HOLD_SPEC>`"]
pub type SDA_HOLD = crate::Reg<sda_hold::SDA_HOLD_SPEC>;
#[doc = "I2C_SDA_HOLD_REG"]
pub mod sda_hold;
#[doc = "SDA_SAMPLE (rw) register accessor: an alias for `Reg<SDA_SAMPLE_SPEC>`"]
pub type SDA_SAMPLE = crate::Reg<sda_sample::SDA_SAMPLE_SPEC>;
#[doc = "I2C_SDA_SAMPLE_REG"]
pub mod sda_sample;
#[doc = "SCL_HIGH_PERIOD (rw) register accessor: an alias for `Reg<SCL_HIGH_PERIOD_SPEC>`"]
pub type SCL_HIGH_PERIOD = crate::Reg<scl_high_period::SCL_HIGH_PERIOD_SPEC>;
#[doc = "I2C_SCL_HIGH_PERIOD_REG"]
pub mod scl_high_period;
#[doc = "SCL_START_HOLD (rw) register accessor: an alias for `Reg<SCL_START_HOLD_SPEC>`"]
pub type SCL_START_HOLD = crate::Reg<scl_start_hold::SCL_START_HOLD_SPEC>;
#[doc = "I2C_SCL_START_HOLD_REG"]
pub mod scl_start_hold;
#[doc = "SCL_RSTART_SETUP (rw) register accessor: an alias for `Reg<SCL_RSTART_SETUP_SPEC>`"]
pub type SCL_RSTART_SETUP = crate::Reg<scl_rstart_setup::SCL_RSTART_SETUP_SPEC>;
#[doc = "I2C_SCL_RSTART_SETUP_REG"]
pub mod scl_rstart_setup;
#[doc = "SCL_STOP_HOLD (rw) register accessor: an alias for `Reg<SCL_STOP_HOLD_SPEC>`"]
pub type SCL_STOP_HOLD = crate::Reg<scl_stop_hold::SCL_STOP_HOLD_SPEC>;
#[doc = "I2C_SCL_STOP_HOLD_REG"]
pub mod scl_stop_hold;
#[doc = "SCL_STOP_SETUP (rw) register accessor: an alias for `Reg<SCL_STOP_SETUP_SPEC>`"]
pub type SCL_STOP_SETUP = crate::Reg<scl_stop_setup::SCL_STOP_SETUP_SPEC>;
#[doc = "I2C_SCL_STOP_SETUP_REG"]
pub mod scl_stop_setup;
#[doc = "FILTER_CFG (rw) register accessor: an alias for `Reg<FILTER_CFG_SPEC>`"]
pub type FILTER_CFG = crate::Reg<filter_cfg::FILTER_CFG_SPEC>;
#[doc = "I2C_FILTER_CFG_REG"]
pub mod filter_cfg;
#[doc = "CLK_CONF (rw) register accessor: an alias for `Reg<CLK_CONF_SPEC>`"]
pub type CLK_CONF = crate::Reg<clk_conf::CLK_CONF_SPEC>;
#[doc = "I2C_CLK_CONF_REG"]
pub mod clk_conf;
#[doc = "COMD (rw) register accessor: an alias for `Reg<COMD_SPEC>`"]
pub type COMD = crate::Reg<comd::COMD_SPEC>;
#[doc = "I2C_COMD%s_REG"]
pub mod comd;
#[doc = "SCL_ST_TIME_OUT (rw) register accessor: an alias for `Reg<SCL_ST_TIME_OUT_SPEC>`"]
pub type SCL_ST_TIME_OUT = crate::Reg<scl_st_time_out::SCL_ST_TIME_OUT_SPEC>;
#[doc = "I2C_SCL_ST_TIME_OUT_REG"]
pub mod scl_st_time_out;
#[doc = "SCL_MAIN_ST_TIME_OUT (rw) register accessor: an alias for `Reg<SCL_MAIN_ST_TIME_OUT_SPEC>`"]
pub type SCL_MAIN_ST_TIME_OUT = crate::Reg<scl_main_st_time_out::SCL_MAIN_ST_TIME_OUT_SPEC>;
#[doc = "I2C_SCL_MAIN_ST_TIME_OUT_REG"]
pub mod scl_main_st_time_out;
#[doc = "SCL_SP_CONF (rw) register accessor: an alias for `Reg<SCL_SP_CONF_SPEC>`"]
pub type SCL_SP_CONF = crate::Reg<scl_sp_conf::SCL_SP_CONF_SPEC>;
#[doc = "I2C_SCL_SP_CONF_REG"]
pub mod scl_sp_conf;
#[doc = "SCL_STRETCH_CONF (rw) register accessor: an alias for `Reg<SCL_STRETCH_CONF_SPEC>`"]
pub type SCL_STRETCH_CONF = crate::Reg<scl_stretch_conf::SCL_STRETCH_CONF_SPEC>;
#[doc = "I2C_SCL_STRETCH_CONF_REG"]
pub mod scl_stretch_conf;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "I2C_DATE_REG"]
pub mod date;
#[doc = "TXFIFO_START_ADDR (r) register accessor: an alias for `Reg<TXFIFO_START_ADDR_SPEC>`"]
pub type TXFIFO_START_ADDR = crate::Reg<txfifo_start_addr::TXFIFO_START_ADDR_SPEC>;
#[doc = "I2C_TXFIFO_START_ADDR_REG"]
pub mod txfifo_start_addr;
#[doc = "RXFIFO_START_ADDR (r) register accessor: an alias for `Reg<RXFIFO_START_ADDR_SPEC>`"]
pub type RXFIFO_START_ADDR = crate::Reg<rxfifo_start_addr::RXFIFO_START_ADDR_SPEC>;
#[doc = "I2C_RXFIFO_START_ADDR_REG"]
pub mod rxfifo_start_addr;
