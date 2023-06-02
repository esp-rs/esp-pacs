#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    #[doc = "0x00 - need_des"]
    pub hp_active_dig_power: HP_ACTIVE_DIG_POWER,
    #[doc = "0x04 - need_des"]
    pub hp_active_icg_hp_func: HP_ACTIVE_ICG_HP_FUNC,
    #[doc = "0x08 - need_des"]
    pub hp_active_icg_hp_apb: HP_ACTIVE_ICG_HP_APB,
    #[doc = "0x0c - need_des"]
    pub hp_active_icg_modem: HP_ACTIVE_ICG_MODEM,
    #[doc = "0x10 - need_des"]
    pub hp_active_hp_sys_cntl: HP_ACTIVE_HP_SYS_CNTL,
    #[doc = "0x14 - need_des"]
    pub hp_active_hp_ck_power: HP_ACTIVE_HP_CK_POWER,
    #[doc = "0x18 - need_des"]
    pub hp_active_bias: HP_ACTIVE_BIAS,
    #[doc = "0x1c - need_des"]
    pub hp_active_backup: HP_ACTIVE_BACKUP,
    #[doc = "0x20 - need_des"]
    pub hp_active_backup_clk: HP_ACTIVE_BACKUP_CLK,
    #[doc = "0x24 - need_des"]
    pub hp_active_sysclk: HP_ACTIVE_SYSCLK,
    #[doc = "0x28 - need_des"]
    pub hp_active_hp_regulator0: HP_ACTIVE_HP_REGULATOR0,
    #[doc = "0x2c - need_des"]
    pub hp_active_hp_regulator1: HP_ACTIVE_HP_REGULATOR1,
    #[doc = "0x30 - need_des"]
    pub hp_active_xtal: HP_ACTIVE_XTAL,
    #[doc = "0x34 - need_des"]
    pub hp_modem_dig_power: HP_MODEM_DIG_POWER,
    #[doc = "0x38 - need_des"]
    pub hp_modem_icg_hp_func: HP_MODEM_ICG_HP_FUNC,
    #[doc = "0x3c - need_des"]
    pub hp_modem_icg_hp_apb: HP_MODEM_ICG_HP_APB,
    #[doc = "0x40 - need_des"]
    pub hp_modem_icg_modem: HP_MODEM_ICG_MODEM,
    #[doc = "0x44 - need_des"]
    pub hp_modem_hp_sys_cntl: HP_MODEM_HP_SYS_CNTL,
    #[doc = "0x48 - need_des"]
    pub hp_modem_hp_ck_power: HP_MODEM_HP_CK_POWER,
    #[doc = "0x4c - need_des"]
    pub hp_modem_bias: HP_MODEM_BIAS,
    #[doc = "0x50 - need_des"]
    pub hp_modem_backup: HP_MODEM_BACKUP,
    #[doc = "0x54 - need_des"]
    pub hp_modem_backup_clk: HP_MODEM_BACKUP_CLK,
    #[doc = "0x58 - need_des"]
    pub hp_modem_sysclk: HP_MODEM_SYSCLK,
    #[doc = "0x5c - need_des"]
    pub hp_modem_hp_regulator0: HP_MODEM_HP_REGULATOR0,
    #[doc = "0x60 - need_des"]
    pub hp_modem_hp_regulator1: HP_MODEM_HP_REGULATOR1,
    #[doc = "0x64 - need_des"]
    pub hp_modem_xtal: HP_MODEM_XTAL,
    #[doc = "0x68 - need_des"]
    pub hp_sleep_dig_power: HP_SLEEP_DIG_POWER,
    #[doc = "0x6c - need_des"]
    pub hp_sleep_icg_hp_func: HP_SLEEP_ICG_HP_FUNC,
    #[doc = "0x70 - need_des"]
    pub hp_sleep_icg_hp_apb: HP_SLEEP_ICG_HP_APB,
    #[doc = "0x74 - need_des"]
    pub hp_sleep_icg_modem: HP_SLEEP_ICG_MODEM,
    #[doc = "0x78 - need_des"]
    pub hp_sleep_hp_sys_cntl: HP_SLEEP_HP_SYS_CNTL,
    #[doc = "0x7c - need_des"]
    pub hp_sleep_hp_ck_power: HP_SLEEP_HP_CK_POWER,
    #[doc = "0x80 - need_des"]
    pub hp_sleep_bias: HP_SLEEP_BIAS,
    #[doc = "0x84 - need_des"]
    pub hp_sleep_backup: HP_SLEEP_BACKUP,
    #[doc = "0x88 - need_des"]
    pub hp_sleep_backup_clk: HP_SLEEP_BACKUP_CLK,
    #[doc = "0x8c - need_des"]
    pub hp_sleep_sysclk: HP_SLEEP_SYSCLK,
    #[doc = "0x90 - need_des"]
    pub hp_sleep_hp_regulator0: HP_SLEEP_HP_REGULATOR0,
    #[doc = "0x94 - need_des"]
    pub hp_sleep_hp_regulator1: HP_SLEEP_HP_REGULATOR1,
    #[doc = "0x98 - need_des"]
    pub hp_sleep_xtal: HP_SLEEP_XTAL,
    #[doc = "0x9c - need_des"]
    pub hp_sleep_lp_regulator0: HP_SLEEP_LP_REGULATOR0,
    #[doc = "0xa0 - need_des"]
    pub hp_sleep_lp_regulator1: HP_SLEEP_LP_REGULATOR1,
    #[doc = "0xa4 - need_des"]
    pub hp_sleep_lp_dcdc_reserve: HP_SLEEP_LP_DCDC_RESERVE,
    #[doc = "0xa8 - need_des"]
    pub hp_sleep_lp_dig_power: HP_SLEEP_LP_DIG_POWER,
    #[doc = "0xac - need_des"]
    pub hp_sleep_lp_ck_power: HP_SLEEP_LP_CK_POWER,
    #[doc = "0xb0 - need_des"]
    pub lp_sleep_lp_bias_reserve: LP_SLEEP_LP_BIAS_RESERVE,
    #[doc = "0xb4 - need_des"]
    pub lp_sleep_lp_regulator0: LP_SLEEP_LP_REGULATOR0,
    #[doc = "0xb8 - need_des"]
    pub lp_sleep_lp_regulator1: LP_SLEEP_LP_REGULATOR1,
    #[doc = "0xbc - need_des"]
    pub lp_sleep_xtal: LP_SLEEP_XTAL,
    #[doc = "0xc0 - need_des"]
    pub lp_sleep_lp_dig_power: LP_SLEEP_LP_DIG_POWER,
    #[doc = "0xc4 - need_des"]
    pub lp_sleep_lp_ck_power: LP_SLEEP_LP_CK_POWER,
    #[doc = "0xc8 - need_des"]
    pub lp_sleep_bias: LP_SLEEP_BIAS,
    #[doc = "0xcc - need_des"]
    pub imm_hp_ck_power: IMM_HP_CK_POWER,
    #[doc = "0xd0 - need_des"]
    pub imm_sleep_sysclk: IMM_SLEEP_SYSCLK,
    #[doc = "0xd4 - need_des"]
    pub imm_hp_func_icg: IMM_HP_FUNC_ICG,
    #[doc = "0xd8 - need_des"]
    pub imm_hp_apb_icg: IMM_HP_APB_ICG,
    #[doc = "0xdc - need_des"]
    pub imm_modem_icg: IMM_MODEM_ICG,
    #[doc = "0xe0 - need_des"]
    pub imm_lp_icg: IMM_LP_ICG,
    #[doc = "0xe4 - need_des"]
    pub imm_pad_hold_all: IMM_PAD_HOLD_ALL,
    #[doc = "0xe8 - need_des"]
    pub imm_i2c_iso: IMM_I2C_ISO,
    #[doc = "0xec - need_des"]
    pub power_wait_timer0: POWER_WAIT_TIMER0,
    #[doc = "0xf0 - need_des"]
    pub power_wait_timer1: POWER_WAIT_TIMER1,
    #[doc = "0xf4 - need_des"]
    pub power_pd_top_cntl: POWER_PD_TOP_CNTL,
    #[doc = "0xf8 - need_des"]
    pub power_pd_hpaon_cntl: POWER_PD_HPAON_CNTL,
    #[doc = "0xfc - need_des"]
    pub power_pd_hpcpu_cntl: POWER_PD_HPCPU_CNTL,
    #[doc = "0x100 - need_des"]
    pub power_pd_hpperi_reserve: POWER_PD_HPPERI_RESERVE,
    #[doc = "0x104 - need_des"]
    pub power_pd_hpwifi_cntl: POWER_PD_HPWIFI_CNTL,
    #[doc = "0x108 - need_des"]
    pub power_pd_lpperi_cntl: POWER_PD_LPPERI_CNTL,
    #[doc = "0x10c - need_des"]
    pub power_pd_mem_cntl: POWER_PD_MEM_CNTL,
    #[doc = "0x110 - need_des"]
    pub power_pd_mem_mask: POWER_PD_MEM_MASK,
    #[doc = "0x114 - need_des"]
    pub power_hp_pad: POWER_HP_PAD,
    #[doc = "0x118 - need_des"]
    pub power_vdd_spi_cntl: POWER_VDD_SPI_CNTL,
    #[doc = "0x11c - need_des"]
    pub power_ck_wait_cntl: POWER_CK_WAIT_CNTL,
    #[doc = "0x120 - need_des"]
    pub slp_wakeup_cntl0: SLP_WAKEUP_CNTL0,
    #[doc = "0x124 - need_des"]
    pub slp_wakeup_cntl1: SLP_WAKEUP_CNTL1,
    #[doc = "0x128 - need_des"]
    pub slp_wakeup_cntl2: SLP_WAKEUP_CNTL2,
    #[doc = "0x12c - need_des"]
    pub slp_wakeup_cntl3: SLP_WAKEUP_CNTL3,
    #[doc = "0x130 - need_des"]
    pub slp_wakeup_cntl4: SLP_WAKEUP_CNTL4,
    #[doc = "0x134 - need_des"]
    pub slp_wakeup_cntl5: SLP_WAKEUP_CNTL5,
    #[doc = "0x138 - need_des"]
    pub slp_wakeup_cntl6: SLP_WAKEUP_CNTL6,
    #[doc = "0x13c - need_des"]
    pub slp_wakeup_cntl7: SLP_WAKEUP_CNTL7,
    #[doc = "0x140 - need_des"]
    pub slp_wakeup_status0: SLP_WAKEUP_STATUS0,
    #[doc = "0x144 - need_des"]
    pub slp_wakeup_status1: SLP_WAKEUP_STATUS1,
    #[doc = "0x148 - need_des"]
    pub hp_ck_poweron: HP_CK_POWERON,
    #[doc = "0x14c - need_des"]
    pub hp_ck_cntl: HP_CK_CNTL,
    #[doc = "0x150 - need_des"]
    pub por_status: POR_STATUS,
    #[doc = "0x154 - need_des"]
    pub rf_pwc: RF_PWC,
    #[doc = "0x158 - need_des"]
    pub backup_cfg: BACKUP_CFG,
    #[doc = "0x15c - need_des"]
    pub int_raw: INT_RAW,
    #[doc = "0x160 - need_des"]
    pub hp_int_st: HP_INT_ST,
    #[doc = "0x164 - need_des"]
    pub hp_int_ena: HP_INT_ENA,
    #[doc = "0x168 - need_des"]
    pub hp_int_clr: HP_INT_CLR,
    #[doc = "0x16c - need_des"]
    pub lp_int_raw: LP_INT_RAW,
    #[doc = "0x170 - need_des"]
    pub lp_int_st: LP_INT_ST,
    #[doc = "0x174 - need_des"]
    pub lp_int_ena: LP_INT_ENA,
    #[doc = "0x178 - need_des"]
    pub lp_int_clr: LP_INT_CLR,
    #[doc = "0x17c - need_des"]
    pub lp_cpu_pwr0: LP_CPU_PWR0,
    #[doc = "0x180 - need_des"]
    pub lp_cpu_pwr1: LP_CPU_PWR1,
    #[doc = "0x184 - need_des"]
    pub hp_lp_cpu_comm: HP_LP_CPU_COMM,
    #[doc = "0x188 - need_des"]
    pub hp_regulator_cfg: HP_REGULATOR_CFG,
    #[doc = "0x18c - need_des"]
    pub main_state: MAIN_STATE,
    #[doc = "0x190 - need_des"]
    pub pwr_state: PWR_STATE,
    #[doc = "0x194 - need_des"]
    pub clk_state0: CLK_STATE0,
    #[doc = "0x198 - need_des"]
    pub clk_state1: CLK_STATE1,
    #[doc = "0x19c - need_des"]
    pub clk_state2: CLK_STATE2,
    #[doc = "0x1a0 - need_des"]
    pub vdd_spi_status: VDD_SPI_STATUS,
    _reserved105: [u8; 0x0258],
    #[doc = "0x3fc - need_des"]
    pub date: DATE,
}
#[doc = "HP_ACTIVE_DIG_POWER (rw) register accessor: an alias for `Reg<HP_ACTIVE_DIG_POWER_SPEC>`"]
pub type HP_ACTIVE_DIG_POWER = crate::Reg<hp_active_dig_power::HP_ACTIVE_DIG_POWER_SPEC>;
#[doc = "need_des"]
pub mod hp_active_dig_power;
#[doc = "HP_ACTIVE_ICG_HP_FUNC (rw) register accessor: an alias for `Reg<HP_ACTIVE_ICG_HP_FUNC_SPEC>`"]
pub type HP_ACTIVE_ICG_HP_FUNC = crate::Reg<hp_active_icg_hp_func::HP_ACTIVE_ICG_HP_FUNC_SPEC>;
#[doc = "need_des"]
pub mod hp_active_icg_hp_func;
#[doc = "HP_ACTIVE_ICG_HP_APB (rw) register accessor: an alias for `Reg<HP_ACTIVE_ICG_HP_APB_SPEC>`"]
pub type HP_ACTIVE_ICG_HP_APB = crate::Reg<hp_active_icg_hp_apb::HP_ACTIVE_ICG_HP_APB_SPEC>;
#[doc = "need_des"]
pub mod hp_active_icg_hp_apb;
#[doc = "HP_ACTIVE_ICG_MODEM (rw) register accessor: an alias for `Reg<HP_ACTIVE_ICG_MODEM_SPEC>`"]
pub type HP_ACTIVE_ICG_MODEM = crate::Reg<hp_active_icg_modem::HP_ACTIVE_ICG_MODEM_SPEC>;
#[doc = "need_des"]
pub mod hp_active_icg_modem;
#[doc = "HP_ACTIVE_HP_SYS_CNTL (rw) register accessor: an alias for `Reg<HP_ACTIVE_HP_SYS_CNTL_SPEC>`"]
pub type HP_ACTIVE_HP_SYS_CNTL = crate::Reg<hp_active_hp_sys_cntl::HP_ACTIVE_HP_SYS_CNTL_SPEC>;
#[doc = "need_des"]
pub mod hp_active_hp_sys_cntl;
#[doc = "HP_ACTIVE_HP_CK_POWER (rw) register accessor: an alias for `Reg<HP_ACTIVE_HP_CK_POWER_SPEC>`"]
pub type HP_ACTIVE_HP_CK_POWER = crate::Reg<hp_active_hp_ck_power::HP_ACTIVE_HP_CK_POWER_SPEC>;
#[doc = "need_des"]
pub mod hp_active_hp_ck_power;
#[doc = "HP_ACTIVE_BIAS (rw) register accessor: an alias for `Reg<HP_ACTIVE_BIAS_SPEC>`"]
pub type HP_ACTIVE_BIAS = crate::Reg<hp_active_bias::HP_ACTIVE_BIAS_SPEC>;
#[doc = "need_des"]
pub mod hp_active_bias;
#[doc = "HP_ACTIVE_BACKUP (rw) register accessor: an alias for `Reg<HP_ACTIVE_BACKUP_SPEC>`"]
pub type HP_ACTIVE_BACKUP = crate::Reg<hp_active_backup::HP_ACTIVE_BACKUP_SPEC>;
#[doc = "need_des"]
pub mod hp_active_backup;
#[doc = "HP_ACTIVE_BACKUP_CLK (rw) register accessor: an alias for `Reg<HP_ACTIVE_BACKUP_CLK_SPEC>`"]
pub type HP_ACTIVE_BACKUP_CLK = crate::Reg<hp_active_backup_clk::HP_ACTIVE_BACKUP_CLK_SPEC>;
#[doc = "need_des"]
pub mod hp_active_backup_clk;
#[doc = "HP_ACTIVE_SYSCLK (rw) register accessor: an alias for `Reg<HP_ACTIVE_SYSCLK_SPEC>`"]
pub type HP_ACTIVE_SYSCLK = crate::Reg<hp_active_sysclk::HP_ACTIVE_SYSCLK_SPEC>;
#[doc = "need_des"]
pub mod hp_active_sysclk;
#[doc = "HP_ACTIVE_HP_REGULATOR0 (rw) register accessor: an alias for `Reg<HP_ACTIVE_HP_REGULATOR0_SPEC>`"]
pub type HP_ACTIVE_HP_REGULATOR0 =
    crate::Reg<hp_active_hp_regulator0::HP_ACTIVE_HP_REGULATOR0_SPEC>;
#[doc = "need_des"]
pub mod hp_active_hp_regulator0;
#[doc = "HP_ACTIVE_HP_REGULATOR1 (rw) register accessor: an alias for `Reg<HP_ACTIVE_HP_REGULATOR1_SPEC>`"]
pub type HP_ACTIVE_HP_REGULATOR1 =
    crate::Reg<hp_active_hp_regulator1::HP_ACTIVE_HP_REGULATOR1_SPEC>;
#[doc = "need_des"]
pub mod hp_active_hp_regulator1;
#[doc = "HP_ACTIVE_XTAL (rw) register accessor: an alias for `Reg<HP_ACTIVE_XTAL_SPEC>`"]
pub type HP_ACTIVE_XTAL = crate::Reg<hp_active_xtal::HP_ACTIVE_XTAL_SPEC>;
#[doc = "need_des"]
pub mod hp_active_xtal;
#[doc = "HP_MODEM_DIG_POWER (rw) register accessor: an alias for `Reg<HP_MODEM_DIG_POWER_SPEC>`"]
pub type HP_MODEM_DIG_POWER = crate::Reg<hp_modem_dig_power::HP_MODEM_DIG_POWER_SPEC>;
#[doc = "need_des"]
pub mod hp_modem_dig_power;
#[doc = "HP_MODEM_ICG_HP_FUNC (rw) register accessor: an alias for `Reg<HP_MODEM_ICG_HP_FUNC_SPEC>`"]
pub type HP_MODEM_ICG_HP_FUNC = crate::Reg<hp_modem_icg_hp_func::HP_MODEM_ICG_HP_FUNC_SPEC>;
#[doc = "need_des"]
pub mod hp_modem_icg_hp_func;
#[doc = "HP_MODEM_ICG_HP_APB (rw) register accessor: an alias for `Reg<HP_MODEM_ICG_HP_APB_SPEC>`"]
pub type HP_MODEM_ICG_HP_APB = crate::Reg<hp_modem_icg_hp_apb::HP_MODEM_ICG_HP_APB_SPEC>;
#[doc = "need_des"]
pub mod hp_modem_icg_hp_apb;
#[doc = "HP_MODEM_ICG_MODEM (rw) register accessor: an alias for `Reg<HP_MODEM_ICG_MODEM_SPEC>`"]
pub type HP_MODEM_ICG_MODEM = crate::Reg<hp_modem_icg_modem::HP_MODEM_ICG_MODEM_SPEC>;
#[doc = "need_des"]
pub mod hp_modem_icg_modem;
#[doc = "HP_MODEM_HP_SYS_CNTL (rw) register accessor: an alias for `Reg<HP_MODEM_HP_SYS_CNTL_SPEC>`"]
pub type HP_MODEM_HP_SYS_CNTL = crate::Reg<hp_modem_hp_sys_cntl::HP_MODEM_HP_SYS_CNTL_SPEC>;
#[doc = "need_des"]
pub mod hp_modem_hp_sys_cntl;
#[doc = "HP_MODEM_HP_CK_POWER (rw) register accessor: an alias for `Reg<HP_MODEM_HP_CK_POWER_SPEC>`"]
pub type HP_MODEM_HP_CK_POWER = crate::Reg<hp_modem_hp_ck_power::HP_MODEM_HP_CK_POWER_SPEC>;
#[doc = "need_des"]
pub mod hp_modem_hp_ck_power;
#[doc = "HP_MODEM_BIAS (rw) register accessor: an alias for `Reg<HP_MODEM_BIAS_SPEC>`"]
pub type HP_MODEM_BIAS = crate::Reg<hp_modem_bias::HP_MODEM_BIAS_SPEC>;
#[doc = "need_des"]
pub mod hp_modem_bias;
#[doc = "HP_MODEM_BACKUP (rw) register accessor: an alias for `Reg<HP_MODEM_BACKUP_SPEC>`"]
pub type HP_MODEM_BACKUP = crate::Reg<hp_modem_backup::HP_MODEM_BACKUP_SPEC>;
#[doc = "need_des"]
pub mod hp_modem_backup;
#[doc = "HP_MODEM_BACKUP_CLK (rw) register accessor: an alias for `Reg<HP_MODEM_BACKUP_CLK_SPEC>`"]
pub type HP_MODEM_BACKUP_CLK = crate::Reg<hp_modem_backup_clk::HP_MODEM_BACKUP_CLK_SPEC>;
#[doc = "need_des"]
pub mod hp_modem_backup_clk;
#[doc = "HP_MODEM_SYSCLK (rw) register accessor: an alias for `Reg<HP_MODEM_SYSCLK_SPEC>`"]
pub type HP_MODEM_SYSCLK = crate::Reg<hp_modem_sysclk::HP_MODEM_SYSCLK_SPEC>;
#[doc = "need_des"]
pub mod hp_modem_sysclk;
#[doc = "HP_MODEM_HP_REGULATOR0 (rw) register accessor: an alias for `Reg<HP_MODEM_HP_REGULATOR0_SPEC>`"]
pub type HP_MODEM_HP_REGULATOR0 = crate::Reg<hp_modem_hp_regulator0::HP_MODEM_HP_REGULATOR0_SPEC>;
#[doc = "need_des"]
pub mod hp_modem_hp_regulator0;
#[doc = "HP_MODEM_HP_REGULATOR1 (rw) register accessor: an alias for `Reg<HP_MODEM_HP_REGULATOR1_SPEC>`"]
pub type HP_MODEM_HP_REGULATOR1 = crate::Reg<hp_modem_hp_regulator1::HP_MODEM_HP_REGULATOR1_SPEC>;
#[doc = "need_des"]
pub mod hp_modem_hp_regulator1;
#[doc = "HP_MODEM_XTAL (rw) register accessor: an alias for `Reg<HP_MODEM_XTAL_SPEC>`"]
pub type HP_MODEM_XTAL = crate::Reg<hp_modem_xtal::HP_MODEM_XTAL_SPEC>;
#[doc = "need_des"]
pub mod hp_modem_xtal;
#[doc = "HP_SLEEP_DIG_POWER (rw) register accessor: an alias for `Reg<HP_SLEEP_DIG_POWER_SPEC>`"]
pub type HP_SLEEP_DIG_POWER = crate::Reg<hp_sleep_dig_power::HP_SLEEP_DIG_POWER_SPEC>;
#[doc = "need_des"]
pub mod hp_sleep_dig_power;
#[doc = "HP_SLEEP_ICG_HP_FUNC (rw) register accessor: an alias for `Reg<HP_SLEEP_ICG_HP_FUNC_SPEC>`"]
pub type HP_SLEEP_ICG_HP_FUNC = crate::Reg<hp_sleep_icg_hp_func::HP_SLEEP_ICG_HP_FUNC_SPEC>;
#[doc = "need_des"]
pub mod hp_sleep_icg_hp_func;
#[doc = "HP_SLEEP_ICG_HP_APB (rw) register accessor: an alias for `Reg<HP_SLEEP_ICG_HP_APB_SPEC>`"]
pub type HP_SLEEP_ICG_HP_APB = crate::Reg<hp_sleep_icg_hp_apb::HP_SLEEP_ICG_HP_APB_SPEC>;
#[doc = "need_des"]
pub mod hp_sleep_icg_hp_apb;
#[doc = "HP_SLEEP_ICG_MODEM (rw) register accessor: an alias for `Reg<HP_SLEEP_ICG_MODEM_SPEC>`"]
pub type HP_SLEEP_ICG_MODEM = crate::Reg<hp_sleep_icg_modem::HP_SLEEP_ICG_MODEM_SPEC>;
#[doc = "need_des"]
pub mod hp_sleep_icg_modem;
#[doc = "HP_SLEEP_HP_SYS_CNTL (rw) register accessor: an alias for `Reg<HP_SLEEP_HP_SYS_CNTL_SPEC>`"]
pub type HP_SLEEP_HP_SYS_CNTL = crate::Reg<hp_sleep_hp_sys_cntl::HP_SLEEP_HP_SYS_CNTL_SPEC>;
#[doc = "need_des"]
pub mod hp_sleep_hp_sys_cntl;
#[doc = "HP_SLEEP_HP_CK_POWER (rw) register accessor: an alias for `Reg<HP_SLEEP_HP_CK_POWER_SPEC>`"]
pub type HP_SLEEP_HP_CK_POWER = crate::Reg<hp_sleep_hp_ck_power::HP_SLEEP_HP_CK_POWER_SPEC>;
#[doc = "need_des"]
pub mod hp_sleep_hp_ck_power;
#[doc = "HP_SLEEP_BIAS (rw) register accessor: an alias for `Reg<HP_SLEEP_BIAS_SPEC>`"]
pub type HP_SLEEP_BIAS = crate::Reg<hp_sleep_bias::HP_SLEEP_BIAS_SPEC>;
#[doc = "need_des"]
pub mod hp_sleep_bias;
#[doc = "HP_SLEEP_BACKUP (rw) register accessor: an alias for `Reg<HP_SLEEP_BACKUP_SPEC>`"]
pub type HP_SLEEP_BACKUP = crate::Reg<hp_sleep_backup::HP_SLEEP_BACKUP_SPEC>;
#[doc = "need_des"]
pub mod hp_sleep_backup;
#[doc = "HP_SLEEP_BACKUP_CLK (rw) register accessor: an alias for `Reg<HP_SLEEP_BACKUP_CLK_SPEC>`"]
pub type HP_SLEEP_BACKUP_CLK = crate::Reg<hp_sleep_backup_clk::HP_SLEEP_BACKUP_CLK_SPEC>;
#[doc = "need_des"]
pub mod hp_sleep_backup_clk;
#[doc = "HP_SLEEP_SYSCLK (rw) register accessor: an alias for `Reg<HP_SLEEP_SYSCLK_SPEC>`"]
pub type HP_SLEEP_SYSCLK = crate::Reg<hp_sleep_sysclk::HP_SLEEP_SYSCLK_SPEC>;
#[doc = "need_des"]
pub mod hp_sleep_sysclk;
#[doc = "HP_SLEEP_HP_REGULATOR0 (rw) register accessor: an alias for `Reg<HP_SLEEP_HP_REGULATOR0_SPEC>`"]
pub type HP_SLEEP_HP_REGULATOR0 = crate::Reg<hp_sleep_hp_regulator0::HP_SLEEP_HP_REGULATOR0_SPEC>;
#[doc = "need_des"]
pub mod hp_sleep_hp_regulator0;
#[doc = "HP_SLEEP_HP_REGULATOR1 (rw) register accessor: an alias for `Reg<HP_SLEEP_HP_REGULATOR1_SPEC>`"]
pub type HP_SLEEP_HP_REGULATOR1 = crate::Reg<hp_sleep_hp_regulator1::HP_SLEEP_HP_REGULATOR1_SPEC>;
#[doc = "need_des"]
pub mod hp_sleep_hp_regulator1;
#[doc = "HP_SLEEP_XTAL (rw) register accessor: an alias for `Reg<HP_SLEEP_XTAL_SPEC>`"]
pub type HP_SLEEP_XTAL = crate::Reg<hp_sleep_xtal::HP_SLEEP_XTAL_SPEC>;
#[doc = "need_des"]
pub mod hp_sleep_xtal;
#[doc = "HP_SLEEP_LP_REGULATOR0 (rw) register accessor: an alias for `Reg<HP_SLEEP_LP_REGULATOR0_SPEC>`"]
pub type HP_SLEEP_LP_REGULATOR0 = crate::Reg<hp_sleep_lp_regulator0::HP_SLEEP_LP_REGULATOR0_SPEC>;
#[doc = "need_des"]
pub mod hp_sleep_lp_regulator0;
#[doc = "HP_SLEEP_LP_REGULATOR1 (rw) register accessor: an alias for `Reg<HP_SLEEP_LP_REGULATOR1_SPEC>`"]
pub type HP_SLEEP_LP_REGULATOR1 = crate::Reg<hp_sleep_lp_regulator1::HP_SLEEP_LP_REGULATOR1_SPEC>;
#[doc = "need_des"]
pub mod hp_sleep_lp_regulator1;
#[doc = "HP_SLEEP_LP_DCDC_RESERVE (w) register accessor: an alias for `Reg<HP_SLEEP_LP_DCDC_RESERVE_SPEC>`"]
pub type HP_SLEEP_LP_DCDC_RESERVE =
    crate::Reg<hp_sleep_lp_dcdc_reserve::HP_SLEEP_LP_DCDC_RESERVE_SPEC>;
#[doc = "need_des"]
pub mod hp_sleep_lp_dcdc_reserve;
#[doc = "HP_SLEEP_LP_DIG_POWER (rw) register accessor: an alias for `Reg<HP_SLEEP_LP_DIG_POWER_SPEC>`"]
pub type HP_SLEEP_LP_DIG_POWER = crate::Reg<hp_sleep_lp_dig_power::HP_SLEEP_LP_DIG_POWER_SPEC>;
#[doc = "need_des"]
pub mod hp_sleep_lp_dig_power;
#[doc = "HP_SLEEP_LP_CK_POWER (rw) register accessor: an alias for `Reg<HP_SLEEP_LP_CK_POWER_SPEC>`"]
pub type HP_SLEEP_LP_CK_POWER = crate::Reg<hp_sleep_lp_ck_power::HP_SLEEP_LP_CK_POWER_SPEC>;
#[doc = "need_des"]
pub mod hp_sleep_lp_ck_power;
#[doc = "LP_SLEEP_LP_BIAS_RESERVE (w) register accessor: an alias for `Reg<LP_SLEEP_LP_BIAS_RESERVE_SPEC>`"]
pub type LP_SLEEP_LP_BIAS_RESERVE =
    crate::Reg<lp_sleep_lp_bias_reserve::LP_SLEEP_LP_BIAS_RESERVE_SPEC>;
#[doc = "need_des"]
pub mod lp_sleep_lp_bias_reserve;
#[doc = "LP_SLEEP_LP_REGULATOR0 (rw) register accessor: an alias for `Reg<LP_SLEEP_LP_REGULATOR0_SPEC>`"]
pub type LP_SLEEP_LP_REGULATOR0 = crate::Reg<lp_sleep_lp_regulator0::LP_SLEEP_LP_REGULATOR0_SPEC>;
#[doc = "need_des"]
pub mod lp_sleep_lp_regulator0;
#[doc = "LP_SLEEP_LP_REGULATOR1 (rw) register accessor: an alias for `Reg<LP_SLEEP_LP_REGULATOR1_SPEC>`"]
pub type LP_SLEEP_LP_REGULATOR1 = crate::Reg<lp_sleep_lp_regulator1::LP_SLEEP_LP_REGULATOR1_SPEC>;
#[doc = "need_des"]
pub mod lp_sleep_lp_regulator1;
#[doc = "LP_SLEEP_XTAL (rw) register accessor: an alias for `Reg<LP_SLEEP_XTAL_SPEC>`"]
pub type LP_SLEEP_XTAL = crate::Reg<lp_sleep_xtal::LP_SLEEP_XTAL_SPEC>;
#[doc = "need_des"]
pub mod lp_sleep_xtal;
#[doc = "LP_SLEEP_LP_DIG_POWER (rw) register accessor: an alias for `Reg<LP_SLEEP_LP_DIG_POWER_SPEC>`"]
pub type LP_SLEEP_LP_DIG_POWER = crate::Reg<lp_sleep_lp_dig_power::LP_SLEEP_LP_DIG_POWER_SPEC>;
#[doc = "need_des"]
pub mod lp_sleep_lp_dig_power;
#[doc = "LP_SLEEP_LP_CK_POWER (rw) register accessor: an alias for `Reg<LP_SLEEP_LP_CK_POWER_SPEC>`"]
pub type LP_SLEEP_LP_CK_POWER = crate::Reg<lp_sleep_lp_ck_power::LP_SLEEP_LP_CK_POWER_SPEC>;
#[doc = "need_des"]
pub mod lp_sleep_lp_ck_power;
#[doc = "LP_SLEEP_BIAS (rw) register accessor: an alias for `Reg<LP_SLEEP_BIAS_SPEC>`"]
pub type LP_SLEEP_BIAS = crate::Reg<lp_sleep_bias::LP_SLEEP_BIAS_SPEC>;
#[doc = "need_des"]
pub mod lp_sleep_bias;
#[doc = "IMM_HP_CK_POWER (rw) register accessor: an alias for `Reg<IMM_HP_CK_POWER_SPEC>`"]
pub type IMM_HP_CK_POWER = crate::Reg<imm_hp_ck_power::IMM_HP_CK_POWER_SPEC>;
#[doc = "need_des"]
pub mod imm_hp_ck_power;
#[doc = "IMM_SLEEP_SYSCLK (w) register accessor: an alias for `Reg<IMM_SLEEP_SYSCLK_SPEC>`"]
pub type IMM_SLEEP_SYSCLK = crate::Reg<imm_sleep_sysclk::IMM_SLEEP_SYSCLK_SPEC>;
#[doc = "need_des"]
pub mod imm_sleep_sysclk;
#[doc = "IMM_HP_FUNC_ICG (w) register accessor: an alias for `Reg<IMM_HP_FUNC_ICG_SPEC>`"]
pub type IMM_HP_FUNC_ICG = crate::Reg<imm_hp_func_icg::IMM_HP_FUNC_ICG_SPEC>;
#[doc = "need_des"]
pub mod imm_hp_func_icg;
#[doc = "IMM_HP_APB_ICG (w) register accessor: an alias for `Reg<IMM_HP_APB_ICG_SPEC>`"]
pub type IMM_HP_APB_ICG = crate::Reg<imm_hp_apb_icg::IMM_HP_APB_ICG_SPEC>;
#[doc = "need_des"]
pub mod imm_hp_apb_icg;
#[doc = "IMM_MODEM_ICG (w) register accessor: an alias for `Reg<IMM_MODEM_ICG_SPEC>`"]
pub type IMM_MODEM_ICG = crate::Reg<imm_modem_icg::IMM_MODEM_ICG_SPEC>;
#[doc = "need_des"]
pub mod imm_modem_icg;
#[doc = "IMM_LP_ICG (w) register accessor: an alias for `Reg<IMM_LP_ICG_SPEC>`"]
pub type IMM_LP_ICG = crate::Reg<imm_lp_icg::IMM_LP_ICG_SPEC>;
#[doc = "need_des"]
pub mod imm_lp_icg;
#[doc = "IMM_PAD_HOLD_ALL (w) register accessor: an alias for `Reg<IMM_PAD_HOLD_ALL_SPEC>`"]
pub type IMM_PAD_HOLD_ALL = crate::Reg<imm_pad_hold_all::IMM_PAD_HOLD_ALL_SPEC>;
#[doc = "need_des"]
pub mod imm_pad_hold_all;
#[doc = "IMM_I2C_ISO (w) register accessor: an alias for `Reg<IMM_I2C_ISO_SPEC>`"]
pub type IMM_I2C_ISO = crate::Reg<imm_i2c_iso::IMM_I2C_ISO_SPEC>;
#[doc = "need_des"]
pub mod imm_i2c_iso;
#[doc = "POWER_WAIT_TIMER0 (rw) register accessor: an alias for `Reg<POWER_WAIT_TIMER0_SPEC>`"]
pub type POWER_WAIT_TIMER0 = crate::Reg<power_wait_timer0::POWER_WAIT_TIMER0_SPEC>;
#[doc = "need_des"]
pub mod power_wait_timer0;
#[doc = "POWER_WAIT_TIMER1 (rw) register accessor: an alias for `Reg<POWER_WAIT_TIMER1_SPEC>`"]
pub type POWER_WAIT_TIMER1 = crate::Reg<power_wait_timer1::POWER_WAIT_TIMER1_SPEC>;
#[doc = "need_des"]
pub mod power_wait_timer1;
#[doc = "POWER_PD_TOP_CNTL (rw) register accessor: an alias for `Reg<POWER_PD_TOP_CNTL_SPEC>`"]
pub type POWER_PD_TOP_CNTL = crate::Reg<power_pd_top_cntl::POWER_PD_TOP_CNTL_SPEC>;
#[doc = "need_des"]
pub mod power_pd_top_cntl;
#[doc = "POWER_PD_HPAON_CNTL (rw) register accessor: an alias for `Reg<POWER_PD_HPAON_CNTL_SPEC>`"]
pub type POWER_PD_HPAON_CNTL = crate::Reg<power_pd_hpaon_cntl::POWER_PD_HPAON_CNTL_SPEC>;
#[doc = "need_des"]
pub mod power_pd_hpaon_cntl;
#[doc = "POWER_PD_HPCPU_CNTL (rw) register accessor: an alias for `Reg<POWER_PD_HPCPU_CNTL_SPEC>`"]
pub type POWER_PD_HPCPU_CNTL = crate::Reg<power_pd_hpcpu_cntl::POWER_PD_HPCPU_CNTL_SPEC>;
#[doc = "need_des"]
pub mod power_pd_hpcpu_cntl;
#[doc = "POWER_PD_HPPERI_RESERVE (w) register accessor: an alias for `Reg<POWER_PD_HPPERI_RESERVE_SPEC>`"]
pub type POWER_PD_HPPERI_RESERVE =
    crate::Reg<power_pd_hpperi_reserve::POWER_PD_HPPERI_RESERVE_SPEC>;
#[doc = "need_des"]
pub mod power_pd_hpperi_reserve;
#[doc = "POWER_PD_HPWIFI_CNTL (rw) register accessor: an alias for `Reg<POWER_PD_HPWIFI_CNTL_SPEC>`"]
pub type POWER_PD_HPWIFI_CNTL = crate::Reg<power_pd_hpwifi_cntl::POWER_PD_HPWIFI_CNTL_SPEC>;
#[doc = "need_des"]
pub mod power_pd_hpwifi_cntl;
#[doc = "POWER_PD_LPPERI_CNTL (rw) register accessor: an alias for `Reg<POWER_PD_LPPERI_CNTL_SPEC>`"]
pub type POWER_PD_LPPERI_CNTL = crate::Reg<power_pd_lpperi_cntl::POWER_PD_LPPERI_CNTL_SPEC>;
#[doc = "need_des"]
pub mod power_pd_lpperi_cntl;
#[doc = "POWER_PD_MEM_CNTL (rw) register accessor: an alias for `Reg<POWER_PD_MEM_CNTL_SPEC>`"]
pub type POWER_PD_MEM_CNTL = crate::Reg<power_pd_mem_cntl::POWER_PD_MEM_CNTL_SPEC>;
#[doc = "need_des"]
pub mod power_pd_mem_cntl;
#[doc = "POWER_PD_MEM_MASK (rw) register accessor: an alias for `Reg<POWER_PD_MEM_MASK_SPEC>`"]
pub type POWER_PD_MEM_MASK = crate::Reg<power_pd_mem_mask::POWER_PD_MEM_MASK_SPEC>;
#[doc = "need_des"]
pub mod power_pd_mem_mask;
#[doc = "POWER_HP_PAD (rw) register accessor: an alias for `Reg<POWER_HP_PAD_SPEC>`"]
pub type POWER_HP_PAD = crate::Reg<power_hp_pad::POWER_HP_PAD_SPEC>;
#[doc = "need_des"]
pub mod power_hp_pad;
#[doc = "POWER_VDD_SPI_CNTL (rw) register accessor: an alias for `Reg<POWER_VDD_SPI_CNTL_SPEC>`"]
pub type POWER_VDD_SPI_CNTL = crate::Reg<power_vdd_spi_cntl::POWER_VDD_SPI_CNTL_SPEC>;
#[doc = "need_des"]
pub mod power_vdd_spi_cntl;
#[doc = "POWER_CK_WAIT_CNTL (rw) register accessor: an alias for `Reg<POWER_CK_WAIT_CNTL_SPEC>`"]
pub type POWER_CK_WAIT_CNTL = crate::Reg<power_ck_wait_cntl::POWER_CK_WAIT_CNTL_SPEC>;
#[doc = "need_des"]
pub mod power_ck_wait_cntl;
#[doc = "SLP_WAKEUP_CNTL0 (w) register accessor: an alias for `Reg<SLP_WAKEUP_CNTL0_SPEC>`"]
pub type SLP_WAKEUP_CNTL0 = crate::Reg<slp_wakeup_cntl0::SLP_WAKEUP_CNTL0_SPEC>;
#[doc = "need_des"]
pub mod slp_wakeup_cntl0;
#[doc = "SLP_WAKEUP_CNTL1 (rw) register accessor: an alias for `Reg<SLP_WAKEUP_CNTL1_SPEC>`"]
pub type SLP_WAKEUP_CNTL1 = crate::Reg<slp_wakeup_cntl1::SLP_WAKEUP_CNTL1_SPEC>;
#[doc = "need_des"]
pub mod slp_wakeup_cntl1;
#[doc = "SLP_WAKEUP_CNTL2 (rw) register accessor: an alias for `Reg<SLP_WAKEUP_CNTL2_SPEC>`"]
pub type SLP_WAKEUP_CNTL2 = crate::Reg<slp_wakeup_cntl2::SLP_WAKEUP_CNTL2_SPEC>;
#[doc = "need_des"]
pub mod slp_wakeup_cntl2;
#[doc = "SLP_WAKEUP_CNTL3 (rw) register accessor: an alias for `Reg<SLP_WAKEUP_CNTL3_SPEC>`"]
pub type SLP_WAKEUP_CNTL3 = crate::Reg<slp_wakeup_cntl3::SLP_WAKEUP_CNTL3_SPEC>;
#[doc = "need_des"]
pub mod slp_wakeup_cntl3;
#[doc = "SLP_WAKEUP_CNTL4 (w) register accessor: an alias for `Reg<SLP_WAKEUP_CNTL4_SPEC>`"]
pub type SLP_WAKEUP_CNTL4 = crate::Reg<slp_wakeup_cntl4::SLP_WAKEUP_CNTL4_SPEC>;
#[doc = "need_des"]
pub mod slp_wakeup_cntl4;
#[doc = "SLP_WAKEUP_CNTL5 (rw) register accessor: an alias for `Reg<SLP_WAKEUP_CNTL5_SPEC>`"]
pub type SLP_WAKEUP_CNTL5 = crate::Reg<slp_wakeup_cntl5::SLP_WAKEUP_CNTL5_SPEC>;
#[doc = "need_des"]
pub mod slp_wakeup_cntl5;
#[doc = "SLP_WAKEUP_CNTL6 (rw) register accessor: an alias for `Reg<SLP_WAKEUP_CNTL6_SPEC>`"]
pub type SLP_WAKEUP_CNTL6 = crate::Reg<slp_wakeup_cntl6::SLP_WAKEUP_CNTL6_SPEC>;
#[doc = "need_des"]
pub mod slp_wakeup_cntl6;
#[doc = "SLP_WAKEUP_CNTL7 (rw) register accessor: an alias for `Reg<SLP_WAKEUP_CNTL7_SPEC>`"]
pub type SLP_WAKEUP_CNTL7 = crate::Reg<slp_wakeup_cntl7::SLP_WAKEUP_CNTL7_SPEC>;
#[doc = "need_des"]
pub mod slp_wakeup_cntl7;
#[doc = "SLP_WAKEUP_STATUS0 (r) register accessor: an alias for `Reg<SLP_WAKEUP_STATUS0_SPEC>`"]
pub type SLP_WAKEUP_STATUS0 = crate::Reg<slp_wakeup_status0::SLP_WAKEUP_STATUS0_SPEC>;
#[doc = "need_des"]
pub mod slp_wakeup_status0;
#[doc = "SLP_WAKEUP_STATUS1 (r) register accessor: an alias for `Reg<SLP_WAKEUP_STATUS1_SPEC>`"]
pub type SLP_WAKEUP_STATUS1 = crate::Reg<slp_wakeup_status1::SLP_WAKEUP_STATUS1_SPEC>;
#[doc = "need_des"]
pub mod slp_wakeup_status1;
#[doc = "HP_CK_POWERON (rw) register accessor: an alias for `Reg<HP_CK_POWERON_SPEC>`"]
pub type HP_CK_POWERON = crate::Reg<hp_ck_poweron::HP_CK_POWERON_SPEC>;
#[doc = "need_des"]
pub mod hp_ck_poweron;
#[doc = "HP_CK_CNTL (rw) register accessor: an alias for `Reg<HP_CK_CNTL_SPEC>`"]
pub type HP_CK_CNTL = crate::Reg<hp_ck_cntl::HP_CK_CNTL_SPEC>;
#[doc = "need_des"]
pub mod hp_ck_cntl;
#[doc = "POR_STATUS (r) register accessor: an alias for `Reg<POR_STATUS_SPEC>`"]
pub type POR_STATUS = crate::Reg<por_status::POR_STATUS_SPEC>;
#[doc = "need_des"]
pub mod por_status;
#[doc = "RF_PWC (rw) register accessor: an alias for `Reg<RF_PWC_SPEC>`"]
pub type RF_PWC = crate::Reg<rf_pwc::RF_PWC_SPEC>;
#[doc = "need_des"]
pub mod rf_pwc;
#[doc = "BACKUP_CFG (rw) register accessor: an alias for `Reg<BACKUP_CFG_SPEC>`"]
pub type BACKUP_CFG = crate::Reg<backup_cfg::BACKUP_CFG_SPEC>;
#[doc = "need_des"]
pub mod backup_cfg;
#[doc = "INT_RAW (rw) register accessor: an alias for `Reg<INT_RAW_SPEC>`"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "need_des"]
pub mod int_raw;
#[doc = "HP_INT_ST (r) register accessor: an alias for `Reg<HP_INT_ST_SPEC>`"]
pub type HP_INT_ST = crate::Reg<hp_int_st::HP_INT_ST_SPEC>;
#[doc = "need_des"]
pub mod hp_int_st;
#[doc = "HP_INT_ENA (rw) register accessor: an alias for `Reg<HP_INT_ENA_SPEC>`"]
pub type HP_INT_ENA = crate::Reg<hp_int_ena::HP_INT_ENA_SPEC>;
#[doc = "need_des"]
pub mod hp_int_ena;
#[doc = "HP_INT_CLR (w) register accessor: an alias for `Reg<HP_INT_CLR_SPEC>`"]
pub type HP_INT_CLR = crate::Reg<hp_int_clr::HP_INT_CLR_SPEC>;
#[doc = "need_des"]
pub mod hp_int_clr;
#[doc = "LP_INT_RAW (rw) register accessor: an alias for `Reg<LP_INT_RAW_SPEC>`"]
pub type LP_INT_RAW = crate::Reg<lp_int_raw::LP_INT_RAW_SPEC>;
#[doc = "need_des"]
pub mod lp_int_raw;
#[doc = "LP_INT_ST (r) register accessor: an alias for `Reg<LP_INT_ST_SPEC>`"]
pub type LP_INT_ST = crate::Reg<lp_int_st::LP_INT_ST_SPEC>;
#[doc = "need_des"]
pub mod lp_int_st;
#[doc = "LP_INT_ENA (rw) register accessor: an alias for `Reg<LP_INT_ENA_SPEC>`"]
pub type LP_INT_ENA = crate::Reg<lp_int_ena::LP_INT_ENA_SPEC>;
#[doc = "need_des"]
pub mod lp_int_ena;
#[doc = "LP_INT_CLR (w) register accessor: an alias for `Reg<LP_INT_CLR_SPEC>`"]
pub type LP_INT_CLR = crate::Reg<lp_int_clr::LP_INT_CLR_SPEC>;
#[doc = "need_des"]
pub mod lp_int_clr;
#[doc = "LP_CPU_PWR0 (rw) register accessor: an alias for `Reg<LP_CPU_PWR0_SPEC>`"]
pub type LP_CPU_PWR0 = crate::Reg<lp_cpu_pwr0::LP_CPU_PWR0_SPEC>;
#[doc = "need_des"]
pub mod lp_cpu_pwr0;
#[doc = "LP_CPU_PWR1 (rw) register accessor: an alias for `Reg<LP_CPU_PWR1_SPEC>`"]
pub type LP_CPU_PWR1 = crate::Reg<lp_cpu_pwr1::LP_CPU_PWR1_SPEC>;
#[doc = "need_des"]
pub mod lp_cpu_pwr1;
#[doc = "HP_LP_CPU_COMM (w) register accessor: an alias for `Reg<HP_LP_CPU_COMM_SPEC>`"]
pub type HP_LP_CPU_COMM = crate::Reg<hp_lp_cpu_comm::HP_LP_CPU_COMM_SPEC>;
#[doc = "need_des"]
pub mod hp_lp_cpu_comm;
#[doc = "HP_REGULATOR_CFG (rw) register accessor: an alias for `Reg<HP_REGULATOR_CFG_SPEC>`"]
pub type HP_REGULATOR_CFG = crate::Reg<hp_regulator_cfg::HP_REGULATOR_CFG_SPEC>;
#[doc = "need_des"]
pub mod hp_regulator_cfg;
#[doc = "MAIN_STATE (r) register accessor: an alias for `Reg<MAIN_STATE_SPEC>`"]
pub type MAIN_STATE = crate::Reg<main_state::MAIN_STATE_SPEC>;
#[doc = "need_des"]
pub mod main_state;
#[doc = "PWR_STATE (r) register accessor: an alias for `Reg<PWR_STATE_SPEC>`"]
pub type PWR_STATE = crate::Reg<pwr_state::PWR_STATE_SPEC>;
#[doc = "need_des"]
pub mod pwr_state;
#[doc = "CLK_STATE0 (r) register accessor: an alias for `Reg<CLK_STATE0_SPEC>`"]
pub type CLK_STATE0 = crate::Reg<clk_state0::CLK_STATE0_SPEC>;
#[doc = "need_des"]
pub mod clk_state0;
#[doc = "CLK_STATE1 (r) register accessor: an alias for `Reg<CLK_STATE1_SPEC>`"]
pub type CLK_STATE1 = crate::Reg<clk_state1::CLK_STATE1_SPEC>;
#[doc = "need_des"]
pub mod clk_state1;
#[doc = "CLK_STATE2 (r) register accessor: an alias for `Reg<CLK_STATE2_SPEC>`"]
pub type CLK_STATE2 = crate::Reg<clk_state2::CLK_STATE2_SPEC>;
#[doc = "need_des"]
pub mod clk_state2;
#[doc = "VDD_SPI_STATUS (r) register accessor: an alias for `Reg<VDD_SPI_STATUS_SPEC>`"]
pub type VDD_SPI_STATUS = crate::Reg<vdd_spi_status::VDD_SPI_STATUS_SPEC>;
#[doc = "need_des"]
pub mod vdd_spi_status;
#[doc = "DATE (rw) register accessor: an alias for `Reg<DATE_SPEC>`"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "need_des"]
pub mod date;
