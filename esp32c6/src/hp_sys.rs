#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - EXTERNAL DEVICE ENCRYPTION/DECRYPTION configuration register"]
    pub external_device_encrypt_decrypt_control: EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL,
    #[doc = "0x04 - HP memory usage configuration register"]
    pub sram_usage_conf: SRAM_USAGE_CONF,
    #[doc = "0x08 - HP anti-DPA security configuration register"]
    pub sec_dpa_conf: SEC_DPA_CONF,
    #[doc = "0x0c - CPU_PERI_TIMEOUT configuration register"]
    pub cpu_peri_timeout_conf: CPU_PERI_TIMEOUT_CONF,
    #[doc = "0x10 - CPU_PERI_TIMEOUT_ADDR register"]
    pub cpu_peri_timeout_addr: CPU_PERI_TIMEOUT_ADDR,
    #[doc = "0x14 - CPU_PERI_TIMEOUT_UID register"]
    pub cpu_peri_timeout_uid: CPU_PERI_TIMEOUT_UID,
    #[doc = "0x18 - HP_PERI_TIMEOUT configuration register"]
    pub hp_peri_timeout_conf: HP_PERI_TIMEOUT_CONF,
    #[doc = "0x1c - HP_PERI_TIMEOUT_ADDR register"]
    pub hp_peri_timeout_addr: HP_PERI_TIMEOUT_ADDR,
    #[doc = "0x20 - HP_PERI_TIMEOUT_UID register"]
    pub hp_peri_timeout_uid: HP_PERI_TIMEOUT_UID,
    #[doc = "0x24 - MODEM_PERI_TIMEOUT configuration register"]
    pub modem_peri_timeout_conf: MODEM_PERI_TIMEOUT_CONF,
    #[doc = "0x28 - MODEM_PERI_TIMEOUT_ADDR register"]
    pub modem_peri_timeout_addr: MODEM_PERI_TIMEOUT_ADDR,
    #[doc = "0x2c - MODEM_PERI_TIMEOUT_UID register"]
    pub modem_peri_timeout_uid: MODEM_PERI_TIMEOUT_UID,
    #[doc = "0x30 - SDIO Control configuration register"]
    pub sdio_ctrl: SDIO_CTRL,
    #[doc = "0x34 - Retention configuration register"]
    pub retention_conf: RETENTION_CONF,
    #[doc = "0x38 - Rom-Table lock register"]
    pub rom_table_lock: ROM_TABLE_LOCK,
    #[doc = "0x3c - Rom-Table register"]
    pub rom_table: ROM_TABLE,
    #[doc = "0x40 - Core Debug runstall configure register"]
    pub core_debug_runstall_conf: CORE_DEBUG_RUNSTALL_CONF,
    #[doc = "0x44 - MEM_TEST configuration register"]
    pub mem_test_conf: MEM_TEST_CONF,
    _reserved18: [u8; 0x0398],
    #[doc = "0x3e0 - redcy eco register."]
    pub rnd_eco: RND_ECO,
    #[doc = "0x3e4 - redcy eco low register."]
    pub rnd_eco_low: RND_ECO_LOW,
    #[doc = "0x3e8 - redcy eco high register."]
    pub rnd_eco_high: RND_ECO_HIGH,
    _reserved21: [u8; 0x0c],
    #[doc = "0x3f8 - HP-SYSTEM clock gating configure register"]
    pub clock_gate: CLOCK_GATE,
    #[doc = "0x3fc - Date register."]
    pub date: DATE,
}
#[doc = "EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL (rw) register accessor: an alias for `Reg<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC>`"]
pub type EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL = crate::Reg<
    external_device_encrypt_decrypt_control::EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC,
>;
#[doc = "EXTERNAL DEVICE ENCRYPTION/DECRYPTION configuration register"]
pub mod external_device_encrypt_decrypt_control;
#[doc = "SRAM_USAGE_CONF (rw) register accessor: an alias for `Reg<SRAM_USAGE_CONF_SPEC>`"]
pub type SRAM_USAGE_CONF = crate::Reg<sram_usage_conf::SRAM_USAGE_CONF_SPEC>;
#[doc = "HP memory usage configuration register"]
pub mod sram_usage_conf;
#[doc = "SEC_DPA_CONF (rw) register accessor: an alias for `Reg<SEC_DPA_CONF_SPEC>`"]
pub type SEC_DPA_CONF = crate::Reg<sec_dpa_conf::SEC_DPA_CONF_SPEC>;
#[doc = "HP anti-DPA security configuration register"]
pub mod sec_dpa_conf;
#[doc = "CPU_PERI_TIMEOUT_CONF (rw) register accessor: an alias for `Reg<CPU_PERI_TIMEOUT_CONF_SPEC>`"]
pub type CPU_PERI_TIMEOUT_CONF = crate::Reg<cpu_peri_timeout_conf::CPU_PERI_TIMEOUT_CONF_SPEC>;
#[doc = "CPU_PERI_TIMEOUT configuration register"]
pub mod cpu_peri_timeout_conf;
#[doc = "CPU_PERI_TIMEOUT_ADDR (r) register accessor: an alias for `Reg<CPU_PERI_TIMEOUT_ADDR_SPEC>`"]
pub type CPU_PERI_TIMEOUT_ADDR = crate::Reg<cpu_peri_timeout_addr::CPU_PERI_TIMEOUT_ADDR_SPEC>;
#[doc = "CPU_PERI_TIMEOUT_ADDR register"]
pub mod cpu_peri_timeout_addr;
#[doc = "CPU_PERI_TIMEOUT_UID (r) register accessor: an alias for `Reg<CPU_PERI_TIMEOUT_UID_SPEC>`"]
pub type CPU_PERI_TIMEOUT_UID = crate::Reg<cpu_peri_timeout_uid::CPU_PERI_TIMEOUT_UID_SPEC>;
#[doc = "CPU_PERI_TIMEOUT_UID register"]
pub mod cpu_peri_timeout_uid;
#[doc = "HP_PERI_TIMEOUT_CONF (rw) register accessor: an alias for `Reg<HP_PERI_TIMEOUT_CONF_SPEC>`"]
pub type HP_PERI_TIMEOUT_CONF = crate::Reg<hp_peri_timeout_conf::HP_PERI_TIMEOUT_CONF_SPEC>;
#[doc = "HP_PERI_TIMEOUT configuration register"]
pub mod hp_peri_timeout_conf;
#[doc = "HP_PERI_TIMEOUT_ADDR (r) register accessor: an alias for `Reg<HP_PERI_TIMEOUT_ADDR_SPEC>`"]
pub type HP_PERI_TIMEOUT_ADDR = crate::Reg<hp_peri_timeout_addr::HP_PERI_TIMEOUT_ADDR_SPEC>;
#[doc = "HP_PERI_TIMEOUT_ADDR register"]
pub mod hp_peri_timeout_addr;
#[doc = "HP_PERI_TIMEOUT_UID (r) register accessor: an alias for `Reg<HP_PERI_TIMEOUT_UID_SPEC>`"]
pub type HP_PERI_TIMEOUT_UID = crate::Reg<hp_peri_timeout_uid::HP_PERI_TIMEOUT_UID_SPEC>;
#[doc = "HP_PERI_TIMEOUT_UID register"]
pub mod hp_peri_timeout_uid;
#[doc = "MODEM_PERI_TIMEOUT_CONF (rw) register accessor: an alias for `Reg<MODEM_PERI_TIMEOUT_CONF_SPEC>`"]
pub type MODEM_PERI_TIMEOUT_CONF =
    crate::Reg<modem_peri_timeout_conf::MODEM_PERI_TIMEOUT_CONF_SPEC>;
#[doc = "MODEM_PERI_TIMEOUT configuration register"]
pub mod modem_peri_timeout_conf;
#[doc = "MODEM_PERI_TIMEOUT_ADDR (r) register accessor: an alias for `Reg<MODEM_PERI_TIMEOUT_ADDR_SPEC>`"]
pub type MODEM_PERI_TIMEOUT_ADDR =
    crate::Reg<modem_peri_timeout_addr::MODEM_PERI_TIMEOUT_ADDR_SPEC>;
#[doc = "MODEM_PERI_TIMEOUT_ADDR register"]
pub mod modem_peri_timeout_addr;
#[doc = "MODEM_PERI_TIMEOUT_UID (r) register accessor: an alias for `Reg<MODEM_PERI_TIMEOUT_UID_SPEC>`"]
pub type MODEM_PERI_TIMEOUT_UID = crate::Reg<modem_peri_timeout_uid::MODEM_PERI_TIMEOUT_UID_SPEC>;
#[doc = "MODEM_PERI_TIMEOUT_UID register"]
pub mod modem_peri_timeout_uid;
#[doc = "SDIO_CTRL (rw) register accessor: an alias for `Reg<SDIO_CTRL_SPEC>`"]
pub type SDIO_CTRL = crate::Reg<sdio_ctrl::SDIO_CTRL_SPEC>;
#[doc = "SDIO Control configuration register"]
pub mod sdio_ctrl;
#[doc = "RETENTION_CONF (rw) register accessor: an alias for `Reg<RETENTION_CONF_SPEC>`"]
pub type RETENTION_CONF = crate::Reg<retention_conf::RETENTION_CONF_SPEC>;
#[doc = "Retention configuration register"]
pub mod retention_conf;
#[doc = "ROM_TABLE_LOCK (rw) register accessor: an alias for `Reg<ROM_TABLE_LOCK_SPEC>`"]
pub type ROM_TABLE_LOCK = crate::Reg<rom_table_lock::ROM_TABLE_LOCK_SPEC>;
#[doc = "Rom-Table lock register"]
pub mod rom_table_lock;
#[doc = "ROM_TABLE (rw) register accessor: an alias for `Reg<ROM_TABLE_SPEC>`"]
pub type ROM_TABLE = crate::Reg<rom_table::ROM_TABLE_SPEC>;
#[doc = "Rom-Table register"]
pub mod rom_table;
#[doc = "CORE_DEBUG_RUNSTALL_CONF (rw) register accessor: an alias for `Reg<CORE_DEBUG_RUNSTALL_CONF_SPEC>`"]
pub type CORE_DEBUG_RUNSTALL_CONF =
    crate::Reg<core_debug_runstall_conf::CORE_DEBUG_RUNSTALL_CONF_SPEC>;
#[doc = "Core Debug runstall configure register"]
pub mod core_debug_runstall_conf;
#[doc = "MEM_TEST_CONF (rw) register accessor: an alias for `Reg<MEM_TEST_CONF_SPEC>`"]
pub type MEM_TEST_CONF = crate::Reg<mem_test_conf::MEM_TEST_CONF_SPEC>;
#[doc = "MEM_TEST configuration register"]
pub mod mem_test_conf;
#[doc = "RND_ECO (rw) register accessor: an alias for `Reg<RND_ECO_SPEC>`"]
pub type RND_ECO = crate::Reg<rnd_eco::RND_ECO_SPEC>;
#[doc = "redcy eco register."]
pub mod rnd_eco;
#[doc = "RND_ECO_LOW (rw) register accessor: an alias for `Reg<RND_ECO_LOW_SPEC>`"]
pub type RND_ECO_LOW = crate::Reg<rnd_eco_low::RND_ECO_LOW_SPEC>;
#[doc = "redcy eco low register."]
pub mod rnd_eco_low;
#[doc = "RND_ECO_HIGH (rw) register accessor: an alias for `Reg<RND_ECO_HIGH_SPEC>`"]
pub type RND_ECO_HIGH = crate::Reg<rnd_eco_high::RND_ECO_HIGH_SPEC>;
#[doc = "redcy eco high register."]
pub mod rnd_eco_high;
#[doc = "CLOCK_GATE (rw) register accessor: an alias for `Reg<CLOCK_GATE_SPEC>`"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "HP-SYSTEM clock gating configure register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "Date register."]
pub mod date;
