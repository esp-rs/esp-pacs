#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    hp_active_dig_power: HP_ACTIVE_DIG_POWER,
    hp_active_icg_hp_func: HP_ACTIVE_ICG_HP_FUNC,
    hp_active_icg_hp_apb: HP_ACTIVE_ICG_HP_APB,
    hp_active_icg_modem: HP_ACTIVE_ICG_MODEM,
    hp_active_hp_sys_cntl: HP_ACTIVE_HP_SYS_CNTL,
    hp_active_hp_ck_power: HP_ACTIVE_HP_CK_POWER,
    hp_active_bias: HP_ACTIVE_BIAS,
    hp_active_backup: HP_ACTIVE_BACKUP,
    hp_active_backup_clk: HP_ACTIVE_BACKUP_CLK,
    hp_active_sysclk: HP_ACTIVE_SYSCLK,
    hp_active_hp_regulator0: HP_ACTIVE_HP_REGULATOR0,
    hp_active_hp_regulator1: HP_ACTIVE_HP_REGULATOR1,
    hp_active_xtal: HP_ACTIVE_XTAL,
    hp_modem_dig_power: HP_MODEM_DIG_POWER,
    hp_modem_icg_hp_func: HP_MODEM_ICG_HP_FUNC,
    hp_modem_icg_hp_apb: HP_MODEM_ICG_HP_APB,
    hp_modem_icg_modem: HP_MODEM_ICG_MODEM,
    hp_modem_hp_sys_cntl: HP_MODEM_HP_SYS_CNTL,
    hp_modem_hp_ck_power: HP_MODEM_HP_CK_POWER,
    hp_modem_bias: HP_MODEM_BIAS,
    hp_modem_backup: HP_MODEM_BACKUP,
    hp_modem_backup_clk: HP_MODEM_BACKUP_CLK,
    hp_modem_sysclk: HP_MODEM_SYSCLK,
    hp_modem_hp_regulator0: HP_MODEM_HP_REGULATOR0,
    hp_modem_hp_regulator1: HP_MODEM_HP_REGULATOR1,
    hp_modem_xtal: HP_MODEM_XTAL,
    hp_sleep_dig_power: HP_SLEEP_DIG_POWER,
    hp_sleep_icg_hp_func: HP_SLEEP_ICG_HP_FUNC,
    hp_sleep_icg_hp_apb: HP_SLEEP_ICG_HP_APB,
    hp_sleep_icg_modem: HP_SLEEP_ICG_MODEM,
    hp_sleep_hp_sys_cntl: HP_SLEEP_HP_SYS_CNTL,
    hp_sleep_hp_ck_power: HP_SLEEP_HP_CK_POWER,
    hp_sleep_bias: HP_SLEEP_BIAS,
    hp_sleep_backup: HP_SLEEP_BACKUP,
    hp_sleep_backup_clk: HP_SLEEP_BACKUP_CLK,
    hp_sleep_sysclk: HP_SLEEP_SYSCLK,
    hp_sleep_hp_regulator0: HP_SLEEP_HP_REGULATOR0,
    hp_sleep_hp_regulator1: HP_SLEEP_HP_REGULATOR1,
    hp_sleep_xtal: HP_SLEEP_XTAL,
    hp_sleep_lp_regulator0: HP_SLEEP_LP_REGULATOR0,
    hp_sleep_lp_regulator1: HP_SLEEP_LP_REGULATOR1,
    _reserved41: [u8; 0x04],
    hp_sleep_lp_dig_power: HP_SLEEP_LP_DIG_POWER,
    hp_sleep_lp_ck_power: HP_SLEEP_LP_CK_POWER,
    _reserved43: [u8; 0x04],
    lp_sleep_lp_regulator0: LP_SLEEP_LP_REGULATOR0,
    lp_sleep_lp_regulator1: LP_SLEEP_LP_REGULATOR1,
    lp_sleep_xtal: LP_SLEEP_XTAL,
    lp_sleep_lp_dig_power: LP_SLEEP_LP_DIG_POWER,
    lp_sleep_lp_ck_power: LP_SLEEP_LP_CK_POWER,
    lp_sleep_bias: LP_SLEEP_BIAS,
    imm_hp_ck_power: IMM_HP_CK_POWER,
    imm_sleep_sysclk: IMM_SLEEP_SYSCLK,
    imm_hp_func_icg: IMM_HP_FUNC_ICG,
    imm_hp_apb_icg: IMM_HP_APB_ICG,
    imm_modem_icg: IMM_MODEM_ICG,
    imm_lp_icg: IMM_LP_ICG,
    imm_pad_hold_all: IMM_PAD_HOLD_ALL,
    imm_i2c_iso: IMM_I2C_ISO,
    power_wait_timer0: POWER_WAIT_TIMER0,
    power_wait_timer1: POWER_WAIT_TIMER1,
    power_pd_top_cntl: POWER_PD_TOP_CNTL,
    power_pd_cnnt_cntl: POWER_PD_CNNT_CNTL,
    power_pd_hpmem_cntl: POWER_PD_HPMEM_CNTL,
    power_pd_top_mask: POWER_PD_TOP_MASK,
    power_pd_cnnt_mask: POWER_PD_CNNT_MASK,
    power_pd_hpmem_mask: POWER_PD_HPMEM_MASK,
    power_dcdc_switch: POWER_DCDC_SWITCH,
    power_pd_lpperi_cntl: POWER_PD_LPPERI_CNTL,
    power_pd_lpperi_mask: POWER_PD_LPPERI_MASK,
    power_hp_pad: POWER_HP_PAD,
    power_ck_wait_cntl: POWER_CK_WAIT_CNTL,
    slp_wakeup_cntl0: SLP_WAKEUP_CNTL0,
    slp_wakeup_cntl1: SLP_WAKEUP_CNTL1,
    slp_wakeup_cntl2: SLP_WAKEUP_CNTL2,
    slp_wakeup_cntl3: SLP_WAKEUP_CNTL3,
    slp_wakeup_cntl4: SLP_WAKEUP_CNTL4,
    slp_wakeup_cntl5: SLP_WAKEUP_CNTL5,
    slp_wakeup_cntl6: SLP_WAKEUP_CNTL6,
    slp_wakeup_cntl7: SLP_WAKEUP_CNTL7,
    slp_wakeup_cntl8: SLP_WAKEUP_CNTL8,
    slp_wakeup_status0: SLP_WAKEUP_STATUS0,
    slp_wakeup_status1: SLP_WAKEUP_STATUS1,
    slp_wakeup_status2: SLP_WAKEUP_STATUS2,
    hp_ck_poweron: HP_CK_POWERON,
    hp_ck_cntl: HP_CK_CNTL,
    por_status: POR_STATUS,
    rf_pwc: RF_PWC,
    backup_cfg: BACKUP_CFG,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    lp_int_raw: LP_INT_RAW,
    lp_int_st: LP_INT_ST,
    lp_int_ena: LP_INT_ENA,
    lp_int_clr: LP_INT_CLR,
    lp_cpu_pwr0: LP_CPU_PWR0,
    lp_cpu_pwr1: LP_CPU_PWR1,
    lp_cpu_pwr2: LP_CPU_PWR2,
    lp_cpu_pwr3: LP_CPU_PWR3,
    lp_cpu_pwr4: LP_CPU_PWR4,
    lp_cpu_pwr5: LP_CPU_PWR5,
    hp_lp_cpu_comm: HP_LP_CPU_COMM,
    hp_regulator_cfg: HP_REGULATOR_CFG,
    main_state: MAIN_STATE,
    pwr_state: PWR_STATE,
    clk_state0: CLK_STATE0,
    clk_state1: CLK_STATE1,
    clk_state2: CLK_STATE2,
    ext_ldo_p0_0p1a: EXT_LDO_P0_0P1A,
    ext_ldo_p0_0p1a_ana: EXT_LDO_P0_0P1A_ANA,
    ext_ldo_p0_0p2a: EXT_LDO_P0_0P2A,
    ext_ldo_p0_0p2a_ana: EXT_LDO_P0_0P2A_ANA,
    ext_ldo_p0_0p3a: EXT_LDO_P0_0P3A,
    ext_ldo_p0_0p3a_ana: EXT_LDO_P0_0P3A_ANA,
    ext_ldo_p1_0p1a: EXT_LDO_P1_0P1A,
    ext_ldo_p1_0p1a_ana: EXT_LDO_P1_0P1A_ANA,
    ext_ldo_p1_0p2a: EXT_LDO_P1_0P2A,
    ext_ldo_p1_0p2a_ana: EXT_LDO_P1_0P2A_ANA,
    ext_ldo_p1_0p3a: EXT_LDO_P1_0P3A,
    ext_ldo_p1_0p3a_ana: EXT_LDO_P1_0P3A_ANA,
    ext_wakeup_lv: EXT_WAKEUP_LV,
    ext_wakeup_sel: EXT_WAKEUP_SEL,
    ext_wakeup_st: EXT_WAKEUP_ST,
    ext_wakeup_cntl: EXT_WAKEUP_CNTL,
    sdio_wakeup_cntl: SDIO_WAKEUP_CNTL,
    xtal_slp: XTAL_SLP,
    cpu_sw_stall: CPU_SW_STALL,
    dcm_ctrl: DCM_CTRL,
    dcm_wait_delay: DCM_WAIT_DELAY,
    vddbat_cfg: VDDBAT_CFG,
    touch_pwr_cntl: TOUCH_PWR_CNTL,
    rdn_eco: RDN_ECO,
    power_pd_hp_cpu_cntl: POWER_PD_HP_CPU_CNTL,
    power_pd_hp_cpu_mask: POWER_PD_HP_CPU_MASK,
    _reserved134: [u8; 0x01dc],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - PMU_HP_ACTIVE_DIG_POWER"]
    #[inline(always)]
    pub const fn hp_active_dig_power(&self) -> &HP_ACTIVE_DIG_POWER {
        &self.hp_active_dig_power
    }
    #[doc = "0x04 - PMU_HP_ACTIVE_ICG_HP_FUNC"]
    #[inline(always)]
    pub const fn hp_active_icg_hp_func(&self) -> &HP_ACTIVE_ICG_HP_FUNC {
        &self.hp_active_icg_hp_func
    }
    #[doc = "0x08 - PMU_HP_ACTIVE_ICG_HP_APB"]
    #[inline(always)]
    pub const fn hp_active_icg_hp_apb(&self) -> &HP_ACTIVE_ICG_HP_APB {
        &self.hp_active_icg_hp_apb
    }
    #[doc = "0x0c - PMU_HP_ACTIVE_ICG_MODEM"]
    #[inline(always)]
    pub const fn hp_active_icg_modem(&self) -> &HP_ACTIVE_ICG_MODEM {
        &self.hp_active_icg_modem
    }
    #[doc = "0x10 - PMU_HP_ACTIVE_HP_SYS_CNTL"]
    #[inline(always)]
    pub const fn hp_active_hp_sys_cntl(&self) -> &HP_ACTIVE_HP_SYS_CNTL {
        &self.hp_active_hp_sys_cntl
    }
    #[doc = "0x14 - PMU_HP_ACTIVE_HP_CK_POWER"]
    #[inline(always)]
    pub const fn hp_active_hp_ck_power(&self) -> &HP_ACTIVE_HP_CK_POWER {
        &self.hp_active_hp_ck_power
    }
    #[doc = "0x18 - PMU_HP_ACTIVE_BIAS"]
    #[inline(always)]
    pub const fn hp_active_bias(&self) -> &HP_ACTIVE_BIAS {
        &self.hp_active_bias
    }
    #[doc = "0x1c - PMU_HP_ACTIVE_BACKUP"]
    #[inline(always)]
    pub const fn hp_active_backup(&self) -> &HP_ACTIVE_BACKUP {
        &self.hp_active_backup
    }
    #[doc = "0x20 - PMU_HP_ACTIVE_BACKUP_CLK"]
    #[inline(always)]
    pub const fn hp_active_backup_clk(&self) -> &HP_ACTIVE_BACKUP_CLK {
        &self.hp_active_backup_clk
    }
    #[doc = "0x24 - PMU_HP_ACTIVE_SYSCLK"]
    #[inline(always)]
    pub const fn hp_active_sysclk(&self) -> &HP_ACTIVE_SYSCLK {
        &self.hp_active_sysclk
    }
    #[doc = "0x28 - PMU_HP_ACTIVE_HP_REGULATOR0"]
    #[inline(always)]
    pub const fn hp_active_hp_regulator0(&self) -> &HP_ACTIVE_HP_REGULATOR0 {
        &self.hp_active_hp_regulator0
    }
    #[doc = "0x2c - PMU_HP_ACTIVE_HP_REGULATOR1"]
    #[inline(always)]
    pub const fn hp_active_hp_regulator1(&self) -> &HP_ACTIVE_HP_REGULATOR1 {
        &self.hp_active_hp_regulator1
    }
    #[doc = "0x30 - PMU_HP_ACTIVE_XTAL"]
    #[inline(always)]
    pub const fn hp_active_xtal(&self) -> &HP_ACTIVE_XTAL {
        &self.hp_active_xtal
    }
    #[doc = "0x34 - PMU_HP_MODEM_DIG_POWER"]
    #[inline(always)]
    pub const fn hp_modem_dig_power(&self) -> &HP_MODEM_DIG_POWER {
        &self.hp_modem_dig_power
    }
    #[doc = "0x38 - PMU_HP_MODEM_ICG_HP_FUNC"]
    #[inline(always)]
    pub const fn hp_modem_icg_hp_func(&self) -> &HP_MODEM_ICG_HP_FUNC {
        &self.hp_modem_icg_hp_func
    }
    #[doc = "0x3c - PMU_HP_MODEM_ICG_HP_APB"]
    #[inline(always)]
    pub const fn hp_modem_icg_hp_apb(&self) -> &HP_MODEM_ICG_HP_APB {
        &self.hp_modem_icg_hp_apb
    }
    #[doc = "0x40 - PMU_HP_MODEM_ICG_MODEM"]
    #[inline(always)]
    pub const fn hp_modem_icg_modem(&self) -> &HP_MODEM_ICG_MODEM {
        &self.hp_modem_icg_modem
    }
    #[doc = "0x44 - PMU_HP_MODEM_HP_SYS_CNTL"]
    #[inline(always)]
    pub const fn hp_modem_hp_sys_cntl(&self) -> &HP_MODEM_HP_SYS_CNTL {
        &self.hp_modem_hp_sys_cntl
    }
    #[doc = "0x48 - PMU_HP_MODEM_HP_CK_POWER"]
    #[inline(always)]
    pub const fn hp_modem_hp_ck_power(&self) -> &HP_MODEM_HP_CK_POWER {
        &self.hp_modem_hp_ck_power
    }
    #[doc = "0x4c - PMU_HP_MODEM_BIAS"]
    #[inline(always)]
    pub const fn hp_modem_bias(&self) -> &HP_MODEM_BIAS {
        &self.hp_modem_bias
    }
    #[doc = "0x50 - PMU_HP_MODEM_BACKUP"]
    #[inline(always)]
    pub const fn hp_modem_backup(&self) -> &HP_MODEM_BACKUP {
        &self.hp_modem_backup
    }
    #[doc = "0x54 - PMU_HP_MODEM_BACKUP_CLK"]
    #[inline(always)]
    pub const fn hp_modem_backup_clk(&self) -> &HP_MODEM_BACKUP_CLK {
        &self.hp_modem_backup_clk
    }
    #[doc = "0x58 - PMU_HP_MODEM_SYSCLK"]
    #[inline(always)]
    pub const fn hp_modem_sysclk(&self) -> &HP_MODEM_SYSCLK {
        &self.hp_modem_sysclk
    }
    #[doc = "0x5c - PMU_HP_MODEM_HP_REGULATOR0"]
    #[inline(always)]
    pub const fn hp_modem_hp_regulator0(&self) -> &HP_MODEM_HP_REGULATOR0 {
        &self.hp_modem_hp_regulator0
    }
    #[doc = "0x60 - PMU_HP_MODEM_HP_REGULATOR1"]
    #[inline(always)]
    pub const fn hp_modem_hp_regulator1(&self) -> &HP_MODEM_HP_REGULATOR1 {
        &self.hp_modem_hp_regulator1
    }
    #[doc = "0x64 - PMU_HP_MODEM_XTAL"]
    #[inline(always)]
    pub const fn hp_modem_xtal(&self) -> &HP_MODEM_XTAL {
        &self.hp_modem_xtal
    }
    #[doc = "0x68 - PMU_HP_SLEEP_DIG_POWER"]
    #[inline(always)]
    pub const fn hp_sleep_dig_power(&self) -> &HP_SLEEP_DIG_POWER {
        &self.hp_sleep_dig_power
    }
    #[doc = "0x6c - PMU_HP_SLEEP_ICG_HP_FUNC"]
    #[inline(always)]
    pub const fn hp_sleep_icg_hp_func(&self) -> &HP_SLEEP_ICG_HP_FUNC {
        &self.hp_sleep_icg_hp_func
    }
    #[doc = "0x70 - PMU_HP_SLEEP_ICG_HP_APB"]
    #[inline(always)]
    pub const fn hp_sleep_icg_hp_apb(&self) -> &HP_SLEEP_ICG_HP_APB {
        &self.hp_sleep_icg_hp_apb
    }
    #[doc = "0x74 - PMU_HP_SLEEP_ICG_MODEM"]
    #[inline(always)]
    pub const fn hp_sleep_icg_modem(&self) -> &HP_SLEEP_ICG_MODEM {
        &self.hp_sleep_icg_modem
    }
    #[doc = "0x78 - PMU_HP_SLEEP_HP_SYS_CNTL"]
    #[inline(always)]
    pub const fn hp_sleep_hp_sys_cntl(&self) -> &HP_SLEEP_HP_SYS_CNTL {
        &self.hp_sleep_hp_sys_cntl
    }
    #[doc = "0x7c - PMU_HP_SLEEP_HP_CK_POWER"]
    #[inline(always)]
    pub const fn hp_sleep_hp_ck_power(&self) -> &HP_SLEEP_HP_CK_POWER {
        &self.hp_sleep_hp_ck_power
    }
    #[doc = "0x80 - PMU_HP_SLEEP_BIAS"]
    #[inline(always)]
    pub const fn hp_sleep_bias(&self) -> &HP_SLEEP_BIAS {
        &self.hp_sleep_bias
    }
    #[doc = "0x84 - PMU_HP_SLEEP_BACKUP"]
    #[inline(always)]
    pub const fn hp_sleep_backup(&self) -> &HP_SLEEP_BACKUP {
        &self.hp_sleep_backup
    }
    #[doc = "0x88 - PMU_HP_SLEEP_BACKUP_CLK"]
    #[inline(always)]
    pub const fn hp_sleep_backup_clk(&self) -> &HP_SLEEP_BACKUP_CLK {
        &self.hp_sleep_backup_clk
    }
    #[doc = "0x8c - PMU_HP_SLEEP_SYSCLK"]
    #[inline(always)]
    pub const fn hp_sleep_sysclk(&self) -> &HP_SLEEP_SYSCLK {
        &self.hp_sleep_sysclk
    }
    #[doc = "0x90 - PMU_HP_SLEEP_HP_REGULATOR0"]
    #[inline(always)]
    pub const fn hp_sleep_hp_regulator0(&self) -> &HP_SLEEP_HP_REGULATOR0 {
        &self.hp_sleep_hp_regulator0
    }
    #[doc = "0x94 - PMU_HP_SLEEP_HP_REGULATOR1"]
    #[inline(always)]
    pub const fn hp_sleep_hp_regulator1(&self) -> &HP_SLEEP_HP_REGULATOR1 {
        &self.hp_sleep_hp_regulator1
    }
    #[doc = "0x98 - PMU_HP_SLEEP_XTAL"]
    #[inline(always)]
    pub const fn hp_sleep_xtal(&self) -> &HP_SLEEP_XTAL {
        &self.hp_sleep_xtal
    }
    #[doc = "0x9c - PMU_HP_SLEEP_LP_REGULATOR0"]
    #[inline(always)]
    pub const fn hp_sleep_lp_regulator0(&self) -> &HP_SLEEP_LP_REGULATOR0 {
        &self.hp_sleep_lp_regulator0
    }
    #[doc = "0xa0 - PMU_HP_SLEEP_LP_REGULATOR1"]
    #[inline(always)]
    pub const fn hp_sleep_lp_regulator1(&self) -> &HP_SLEEP_LP_REGULATOR1 {
        &self.hp_sleep_lp_regulator1
    }
    #[doc = "0xa8 - PMU_HP_SLEEP_LP_DIG_POWER"]
    #[inline(always)]
    pub const fn hp_sleep_lp_dig_power(&self) -> &HP_SLEEP_LP_DIG_POWER {
        &self.hp_sleep_lp_dig_power
    }
    #[doc = "0xac - PMU_HP_SLEEP_LP_CK_POWER"]
    #[inline(always)]
    pub const fn hp_sleep_lp_ck_power(&self) -> &HP_SLEEP_LP_CK_POWER {
        &self.hp_sleep_lp_ck_power
    }
    #[doc = "0xb4 - PMU_LP_SLEEP_LP_REGULATOR0"]
    #[inline(always)]
    pub const fn lp_sleep_lp_regulator0(&self) -> &LP_SLEEP_LP_REGULATOR0 {
        &self.lp_sleep_lp_regulator0
    }
    #[doc = "0xb8 - PMU_LP_SLEEP_LP_REGULATOR1"]
    #[inline(always)]
    pub const fn lp_sleep_lp_regulator1(&self) -> &LP_SLEEP_LP_REGULATOR1 {
        &self.lp_sleep_lp_regulator1
    }
    #[doc = "0xbc - PMU_LP_SLEEP_XTAL"]
    #[inline(always)]
    pub const fn lp_sleep_xtal(&self) -> &LP_SLEEP_XTAL {
        &self.lp_sleep_xtal
    }
    #[doc = "0xc0 - PMU_LP_SLEEP_LP_DIG_POWER"]
    #[inline(always)]
    pub const fn lp_sleep_lp_dig_power(&self) -> &LP_SLEEP_LP_DIG_POWER {
        &self.lp_sleep_lp_dig_power
    }
    #[doc = "0xc4 - PMU_LP_SLEEP_LP_CK_POWER"]
    #[inline(always)]
    pub const fn lp_sleep_lp_ck_power(&self) -> &LP_SLEEP_LP_CK_POWER {
        &self.lp_sleep_lp_ck_power
    }
    #[doc = "0xc8 - PMU_LP_SLEEP_BIAS"]
    #[inline(always)]
    pub const fn lp_sleep_bias(&self) -> &LP_SLEEP_BIAS {
        &self.lp_sleep_bias
    }
    #[doc = "0xcc - PMU_IMM_HP_CK_POWER"]
    #[inline(always)]
    pub const fn imm_hp_ck_power(&self) -> &IMM_HP_CK_POWER {
        &self.imm_hp_ck_power
    }
    #[doc = "0xd0 - PMU_IMM_SLEEP_SYSCLK"]
    #[inline(always)]
    pub const fn imm_sleep_sysclk(&self) -> &IMM_SLEEP_SYSCLK {
        &self.imm_sleep_sysclk
    }
    #[doc = "0xd4 - PMU_IMM_HP_FUNC_ICG"]
    #[inline(always)]
    pub const fn imm_hp_func_icg(&self) -> &IMM_HP_FUNC_ICG {
        &self.imm_hp_func_icg
    }
    #[doc = "0xd8 - PMU_IMM_HP_APB_ICG"]
    #[inline(always)]
    pub const fn imm_hp_apb_icg(&self) -> &IMM_HP_APB_ICG {
        &self.imm_hp_apb_icg
    }
    #[doc = "0xdc - PMU_IMM_MODEM_ICG"]
    #[inline(always)]
    pub const fn imm_modem_icg(&self) -> &IMM_MODEM_ICG {
        &self.imm_modem_icg
    }
    #[doc = "0xe0 - PMU_IMM_LP_ICG"]
    #[inline(always)]
    pub const fn imm_lp_icg(&self) -> &IMM_LP_ICG {
        &self.imm_lp_icg
    }
    #[doc = "0xe4 - PMU_IMM_PAD_HOLD_ALL"]
    #[inline(always)]
    pub const fn imm_pad_hold_all(&self) -> &IMM_PAD_HOLD_ALL {
        &self.imm_pad_hold_all
    }
    #[doc = "0xe8 - PMU_IMM_I2C_ISO"]
    #[inline(always)]
    pub const fn imm_i2c_iso(&self) -> &IMM_I2C_ISO {
        &self.imm_i2c_iso
    }
    #[doc = "0xec - PMU_POWER_WAIT_TIMER0"]
    #[inline(always)]
    pub const fn power_wait_timer0(&self) -> &POWER_WAIT_TIMER0 {
        &self.power_wait_timer0
    }
    #[doc = "0xf0 - PMU_POWER_WAIT_TIMER1"]
    #[inline(always)]
    pub const fn power_wait_timer1(&self) -> &POWER_WAIT_TIMER1 {
        &self.power_wait_timer1
    }
    #[doc = "0xf4 - PMU_POWER_PD_TOP_CNTL"]
    #[inline(always)]
    pub const fn power_pd_top_cntl(&self) -> &POWER_PD_TOP_CNTL {
        &self.power_pd_top_cntl
    }
    #[doc = "0xf8 - PMU_POWER_PD_CNNT_CNTL"]
    #[inline(always)]
    pub const fn power_pd_cnnt_cntl(&self) -> &POWER_PD_CNNT_CNTL {
        &self.power_pd_cnnt_cntl
    }
    #[doc = "0xfc - PMU_POWER_PD_HPMEM_CNTL"]
    #[inline(always)]
    pub const fn power_pd_hpmem_cntl(&self) -> &POWER_PD_HPMEM_CNTL {
        &self.power_pd_hpmem_cntl
    }
    #[doc = "0x100 - PMU_POWER_PD_TOP_MASK"]
    #[inline(always)]
    pub const fn power_pd_top_mask(&self) -> &POWER_PD_TOP_MASK {
        &self.power_pd_top_mask
    }
    #[doc = "0x104 - PMU_POWER_PD_CNNT_MASK"]
    #[inline(always)]
    pub const fn power_pd_cnnt_mask(&self) -> &POWER_PD_CNNT_MASK {
        &self.power_pd_cnnt_mask
    }
    #[doc = "0x108 - PMU_POWER_PD_HPMEM_MASK"]
    #[inline(always)]
    pub const fn power_pd_hpmem_mask(&self) -> &POWER_PD_HPMEM_MASK {
        &self.power_pd_hpmem_mask
    }
    #[doc = "0x10c - PMU_POWER_DCDC_SWITCH"]
    #[inline(always)]
    pub const fn power_dcdc_switch(&self) -> &POWER_DCDC_SWITCH {
        &self.power_dcdc_switch
    }
    #[doc = "0x110 - PMU_POWER_PD_LPPERI_CNTL"]
    #[inline(always)]
    pub const fn power_pd_lpperi_cntl(&self) -> &POWER_PD_LPPERI_CNTL {
        &self.power_pd_lpperi_cntl
    }
    #[doc = "0x114 - PMU_POWER_PD_LPPERI_MASK"]
    #[inline(always)]
    pub const fn power_pd_lpperi_mask(&self) -> &POWER_PD_LPPERI_MASK {
        &self.power_pd_lpperi_mask
    }
    #[doc = "0x118 - PMU_POWER_HP_PAD"]
    #[inline(always)]
    pub const fn power_hp_pad(&self) -> &POWER_HP_PAD {
        &self.power_hp_pad
    }
    #[doc = "0x11c - PMU_POWER_CK_WAIT_CNTL"]
    #[inline(always)]
    pub const fn power_ck_wait_cntl(&self) -> &POWER_CK_WAIT_CNTL {
        &self.power_ck_wait_cntl
    }
    #[doc = "0x120 - PMU_SLP_WAKEUP_CNTL0"]
    #[inline(always)]
    pub const fn slp_wakeup_cntl0(&self) -> &SLP_WAKEUP_CNTL0 {
        &self.slp_wakeup_cntl0
    }
    #[doc = "0x124 - PMU_SLP_WAKEUP_CNTL1"]
    #[inline(always)]
    pub const fn slp_wakeup_cntl1(&self) -> &SLP_WAKEUP_CNTL1 {
        &self.slp_wakeup_cntl1
    }
    #[doc = "0x128 - PMU_SLP_WAKEUP_CNTL2"]
    #[inline(always)]
    pub const fn slp_wakeup_cntl2(&self) -> &SLP_WAKEUP_CNTL2 {
        &self.slp_wakeup_cntl2
    }
    #[doc = "0x12c - PMU_SLP_WAKEUP_CNTL3"]
    #[inline(always)]
    pub const fn slp_wakeup_cntl3(&self) -> &SLP_WAKEUP_CNTL3 {
        &self.slp_wakeup_cntl3
    }
    #[doc = "0x130 - PMU_SLP_WAKEUP_CNTL4"]
    #[inline(always)]
    pub const fn slp_wakeup_cntl4(&self) -> &SLP_WAKEUP_CNTL4 {
        &self.slp_wakeup_cntl4
    }
    #[doc = "0x134 - PMU_SLP_WAKEUP_CNTL5"]
    #[inline(always)]
    pub const fn slp_wakeup_cntl5(&self) -> &SLP_WAKEUP_CNTL5 {
        &self.slp_wakeup_cntl5
    }
    #[doc = "0x138 - PMU_SLP_WAKEUP_CNTL6"]
    #[inline(always)]
    pub const fn slp_wakeup_cntl6(&self) -> &SLP_WAKEUP_CNTL6 {
        &self.slp_wakeup_cntl6
    }
    #[doc = "0x13c - PMU_SLP_WAKEUP_CNTL7"]
    #[inline(always)]
    pub const fn slp_wakeup_cntl7(&self) -> &SLP_WAKEUP_CNTL7 {
        &self.slp_wakeup_cntl7
    }
    #[doc = "0x140 - PMU_SLP_WAKEUP_CNTL8"]
    #[inline(always)]
    pub const fn slp_wakeup_cntl8(&self) -> &SLP_WAKEUP_CNTL8 {
        &self.slp_wakeup_cntl8
    }
    #[doc = "0x144 - PMU_SLP_WAKEUP_STATUS0"]
    #[inline(always)]
    pub const fn slp_wakeup_status0(&self) -> &SLP_WAKEUP_STATUS0 {
        &self.slp_wakeup_status0
    }
    #[doc = "0x148 - PMU_SLP_WAKEUP_STATUS1"]
    #[inline(always)]
    pub const fn slp_wakeup_status1(&self) -> &SLP_WAKEUP_STATUS1 {
        &self.slp_wakeup_status1
    }
    #[doc = "0x14c - PMU_SLP_WAKEUP_STATUS2"]
    #[inline(always)]
    pub const fn slp_wakeup_status2(&self) -> &SLP_WAKEUP_STATUS2 {
        &self.slp_wakeup_status2
    }
    #[doc = "0x150 - PMU_HP_CK_POWERON"]
    #[inline(always)]
    pub const fn hp_ck_poweron(&self) -> &HP_CK_POWERON {
        &self.hp_ck_poweron
    }
    #[doc = "0x154 - PMU_HP_CK_CNTL"]
    #[inline(always)]
    pub const fn hp_ck_cntl(&self) -> &HP_CK_CNTL {
        &self.hp_ck_cntl
    }
    #[doc = "0x158 - PMU_POR_STATUS"]
    #[inline(always)]
    pub const fn por_status(&self) -> &POR_STATUS {
        &self.por_status
    }
    #[doc = "0x15c - PMU_RF_PWC"]
    #[inline(always)]
    pub const fn rf_pwc(&self) -> &RF_PWC {
        &self.rf_pwc
    }
    #[doc = "0x160 - PMU_BACKUP_CFG"]
    #[inline(always)]
    pub const fn backup_cfg(&self) -> &BACKUP_CFG {
        &self.backup_cfg
    }
    #[doc = "0x164 - PMU_INT_RAW"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x168 - PMU_HP_INT_ST"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x16c - PMU_HP_INT_ENA"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x170 - PMU_HP_INT_CLR"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x174 - PMU_LP_INT_RAW"]
    #[inline(always)]
    pub const fn lp_int_raw(&self) -> &LP_INT_RAW {
        &self.lp_int_raw
    }
    #[doc = "0x178 - PMU_LP_INT_ST"]
    #[inline(always)]
    pub const fn lp_int_st(&self) -> &LP_INT_ST {
        &self.lp_int_st
    }
    #[doc = "0x17c - PMU_LP_INT_ENA"]
    #[inline(always)]
    pub const fn lp_int_ena(&self) -> &LP_INT_ENA {
        &self.lp_int_ena
    }
    #[doc = "0x180 - PMU_LP_INT_CLR"]
    #[inline(always)]
    pub const fn lp_int_clr(&self) -> &LP_INT_CLR {
        &self.lp_int_clr
    }
    #[doc = "0x184 - PMU_LP_CPU_PWR0"]
    #[inline(always)]
    pub const fn lp_cpu_pwr0(&self) -> &LP_CPU_PWR0 {
        &self.lp_cpu_pwr0
    }
    #[doc = "0x188 - PMU_LP_CPU_PWR1"]
    #[inline(always)]
    pub const fn lp_cpu_pwr1(&self) -> &LP_CPU_PWR1 {
        &self.lp_cpu_pwr1
    }
    #[doc = "0x18c - PMU_LP_CPU_PWR2"]
    #[inline(always)]
    pub const fn lp_cpu_pwr2(&self) -> &LP_CPU_PWR2 {
        &self.lp_cpu_pwr2
    }
    #[doc = "0x190 - PMU_LP_CPU_PWR3"]
    #[inline(always)]
    pub const fn lp_cpu_pwr3(&self) -> &LP_CPU_PWR3 {
        &self.lp_cpu_pwr3
    }
    #[doc = "0x194 - PMU_LP_CPU_PWR4"]
    #[inline(always)]
    pub const fn lp_cpu_pwr4(&self) -> &LP_CPU_PWR4 {
        &self.lp_cpu_pwr4
    }
    #[doc = "0x198 - PMU_LP_CPU_PWR5"]
    #[inline(always)]
    pub const fn lp_cpu_pwr5(&self) -> &LP_CPU_PWR5 {
        &self.lp_cpu_pwr5
    }
    #[doc = "0x19c - PMU_HP_LP_CPU_COMM"]
    #[inline(always)]
    pub const fn hp_lp_cpu_comm(&self) -> &HP_LP_CPU_COMM {
        &self.hp_lp_cpu_comm
    }
    #[doc = "0x1a0 - PMU_HP_REGULATOR_CFG"]
    #[inline(always)]
    pub const fn hp_regulator_cfg(&self) -> &HP_REGULATOR_CFG {
        &self.hp_regulator_cfg
    }
    #[doc = "0x1a4 - PMU_MAIN_STATE"]
    #[inline(always)]
    pub const fn main_state(&self) -> &MAIN_STATE {
        &self.main_state
    }
    #[doc = "0x1a8 - PMU_PWR_STATE"]
    #[inline(always)]
    pub const fn pwr_state(&self) -> &PWR_STATE {
        &self.pwr_state
    }
    #[doc = "0x1ac - PMU_CLK_STATE0"]
    #[inline(always)]
    pub const fn clk_state0(&self) -> &CLK_STATE0 {
        &self.clk_state0
    }
    #[doc = "0x1b0 - PMU_CLK_STATE1"]
    #[inline(always)]
    pub const fn clk_state1(&self) -> &CLK_STATE1 {
        &self.clk_state1
    }
    #[doc = "0x1b4 - PMU_CLK_STATE2"]
    #[inline(always)]
    pub const fn clk_state2(&self) -> &CLK_STATE2 {
        &self.clk_state2
    }
    #[doc = "0x1b8 - PMU_EXT_LDO_P0_0P1A"]
    #[inline(always)]
    pub const fn ext_ldo_p0_0p1a(&self) -> &EXT_LDO_P0_0P1A {
        &self.ext_ldo_p0_0p1a
    }
    #[doc = "0x1bc - PMU_EXT_LDO_P0_0P1A_ANA"]
    #[inline(always)]
    pub const fn ext_ldo_p0_0p1a_ana(&self) -> &EXT_LDO_P0_0P1A_ANA {
        &self.ext_ldo_p0_0p1a_ana
    }
    #[doc = "0x1c0 - PMU_EXT_LDO_P0_0P2A"]
    #[inline(always)]
    pub const fn ext_ldo_p0_0p2a(&self) -> &EXT_LDO_P0_0P2A {
        &self.ext_ldo_p0_0p2a
    }
    #[doc = "0x1c4 - PMU_EXT_LDO_P0_0P2A_ANA"]
    #[inline(always)]
    pub const fn ext_ldo_p0_0p2a_ana(&self) -> &EXT_LDO_P0_0P2A_ANA {
        &self.ext_ldo_p0_0p2a_ana
    }
    #[doc = "0x1c8 - PMU_EXT_LDO_P0_0P3A"]
    #[inline(always)]
    pub const fn ext_ldo_p0_0p3a(&self) -> &EXT_LDO_P0_0P3A {
        &self.ext_ldo_p0_0p3a
    }
    #[doc = "0x1cc - PMU_EXT_LDO_P0_0P3A_ANA"]
    #[inline(always)]
    pub const fn ext_ldo_p0_0p3a_ana(&self) -> &EXT_LDO_P0_0P3A_ANA {
        &self.ext_ldo_p0_0p3a_ana
    }
    #[doc = "0x1d0 - PMU_EXT_LDO_P1_0P1A"]
    #[inline(always)]
    pub const fn ext_ldo_p1_0p1a(&self) -> &EXT_LDO_P1_0P1A {
        &self.ext_ldo_p1_0p1a
    }
    #[doc = "0x1d4 - PMU_EXT_LDO_P1_0P1A_ANA"]
    #[inline(always)]
    pub const fn ext_ldo_p1_0p1a_ana(&self) -> &EXT_LDO_P1_0P1A_ANA {
        &self.ext_ldo_p1_0p1a_ana
    }
    #[doc = "0x1d8 - PMU_EXT_LDO_P1_0P2A"]
    #[inline(always)]
    pub const fn ext_ldo_p1_0p2a(&self) -> &EXT_LDO_P1_0P2A {
        &self.ext_ldo_p1_0p2a
    }
    #[doc = "0x1dc - PMU_EXT_LDO_P1_0P2A_ANA"]
    #[inline(always)]
    pub const fn ext_ldo_p1_0p2a_ana(&self) -> &EXT_LDO_P1_0P2A_ANA {
        &self.ext_ldo_p1_0p2a_ana
    }
    #[doc = "0x1e0 - PMU_EXT_LDO_P1_0P3A"]
    #[inline(always)]
    pub const fn ext_ldo_p1_0p3a(&self) -> &EXT_LDO_P1_0P3A {
        &self.ext_ldo_p1_0p3a
    }
    #[doc = "0x1e4 - PMU_EXT_LDO_P1_0P3A_ANA"]
    #[inline(always)]
    pub const fn ext_ldo_p1_0p3a_ana(&self) -> &EXT_LDO_P1_0P3A_ANA {
        &self.ext_ldo_p1_0p3a_ana
    }
    #[doc = "0x1e8 - PMU_EXT_WAKEUP_LV"]
    #[inline(always)]
    pub const fn ext_wakeup_lv(&self) -> &EXT_WAKEUP_LV {
        &self.ext_wakeup_lv
    }
    #[doc = "0x1ec - PMU_EXT_WAKEUP_SEL"]
    #[inline(always)]
    pub const fn ext_wakeup_sel(&self) -> &EXT_WAKEUP_SEL {
        &self.ext_wakeup_sel
    }
    #[doc = "0x1f0 - PMU_EXT_WAKEUP_ST"]
    #[inline(always)]
    pub const fn ext_wakeup_st(&self) -> &EXT_WAKEUP_ST {
        &self.ext_wakeup_st
    }
    #[doc = "0x1f4 - PMU_EXT_WAKEUP_CNTL"]
    #[inline(always)]
    pub const fn ext_wakeup_cntl(&self) -> &EXT_WAKEUP_CNTL {
        &self.ext_wakeup_cntl
    }
    #[doc = "0x1f8 - PMU_SDIO_WAKEUP_CNTL"]
    #[inline(always)]
    pub const fn sdio_wakeup_cntl(&self) -> &SDIO_WAKEUP_CNTL {
        &self.sdio_wakeup_cntl
    }
    #[doc = "0x1fc - PMU_XTAL_SLP"]
    #[inline(always)]
    pub const fn xtal_slp(&self) -> &XTAL_SLP {
        &self.xtal_slp
    }
    #[doc = "0x200 - PMU_CPU_SW_STALL"]
    #[inline(always)]
    pub const fn cpu_sw_stall(&self) -> &CPU_SW_STALL {
        &self.cpu_sw_stall
    }
    #[doc = "0x204 - PMU_DCM_CTRL"]
    #[inline(always)]
    pub const fn dcm_ctrl(&self) -> &DCM_CTRL {
        &self.dcm_ctrl
    }
    #[doc = "0x208 - PMU_DCM_WAIT_DELAY"]
    #[inline(always)]
    pub const fn dcm_wait_delay(&self) -> &DCM_WAIT_DELAY {
        &self.dcm_wait_delay
    }
    #[doc = "0x20c - PMU_VDDBAT_CFG"]
    #[inline(always)]
    pub const fn vddbat_cfg(&self) -> &VDDBAT_CFG {
        &self.vddbat_cfg
    }
    #[doc = "0x210 - PMU_TOUCH_PWR_CNTL"]
    #[inline(always)]
    pub const fn touch_pwr_cntl(&self) -> &TOUCH_PWR_CNTL {
        &self.touch_pwr_cntl
    }
    #[doc = "0x214 - PMU_RDN_ECO"]
    #[inline(always)]
    pub const fn rdn_eco(&self) -> &RDN_ECO {
        &self.rdn_eco
    }
    #[doc = "0x218 - PMU_POWER_PD_HP_CPU_CNTL"]
    #[inline(always)]
    pub const fn power_pd_hp_cpu_cntl(&self) -> &POWER_PD_HP_CPU_CNTL {
        &self.power_pd_hp_cpu_cntl
    }
    #[doc = "0x21c - PMU_POWER_PD_HP_CPU_MASK"]
    #[inline(always)]
    pub const fn power_pd_hp_cpu_mask(&self) -> &POWER_PD_HP_CPU_MASK {
        &self.power_pd_hp_cpu_mask
    }
    #[doc = "0x3fc - PMU_DATE"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "HP_ACTIVE_DIG_POWER (rw) register accessor: PMU_HP_ACTIVE_DIG_POWER\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_dig_power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_dig_power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_active_dig_power`] module"]
pub type HP_ACTIVE_DIG_POWER = crate::Reg<hp_active_dig_power::HP_ACTIVE_DIG_POWER_SPEC>;
#[doc = "PMU_HP_ACTIVE_DIG_POWER"]
pub mod hp_active_dig_power;
#[doc = "HP_ACTIVE_ICG_HP_FUNC (rw) register accessor: PMU_HP_ACTIVE_ICG_HP_FUNC\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_icg_hp_func::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_icg_hp_func::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_active_icg_hp_func`] module"]
pub type HP_ACTIVE_ICG_HP_FUNC = crate::Reg<hp_active_icg_hp_func::HP_ACTIVE_ICG_HP_FUNC_SPEC>;
#[doc = "PMU_HP_ACTIVE_ICG_HP_FUNC"]
pub mod hp_active_icg_hp_func;
#[doc = "HP_ACTIVE_ICG_HP_APB (rw) register accessor: PMU_HP_ACTIVE_ICG_HP_APB\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_icg_hp_apb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_icg_hp_apb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_active_icg_hp_apb`] module"]
pub type HP_ACTIVE_ICG_HP_APB = crate::Reg<hp_active_icg_hp_apb::HP_ACTIVE_ICG_HP_APB_SPEC>;
#[doc = "PMU_HP_ACTIVE_ICG_HP_APB"]
pub mod hp_active_icg_hp_apb;
#[doc = "HP_ACTIVE_ICG_MODEM (rw) register accessor: PMU_HP_ACTIVE_ICG_MODEM\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_icg_modem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_icg_modem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_active_icg_modem`] module"]
pub type HP_ACTIVE_ICG_MODEM = crate::Reg<hp_active_icg_modem::HP_ACTIVE_ICG_MODEM_SPEC>;
#[doc = "PMU_HP_ACTIVE_ICG_MODEM"]
pub mod hp_active_icg_modem;
#[doc = "HP_ACTIVE_HP_SYS_CNTL (rw) register accessor: PMU_HP_ACTIVE_HP_SYS_CNTL\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_hp_sys_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_hp_sys_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_active_hp_sys_cntl`] module"]
pub type HP_ACTIVE_HP_SYS_CNTL = crate::Reg<hp_active_hp_sys_cntl::HP_ACTIVE_HP_SYS_CNTL_SPEC>;
#[doc = "PMU_HP_ACTIVE_HP_SYS_CNTL"]
pub mod hp_active_hp_sys_cntl;
#[doc = "HP_ACTIVE_HP_CK_POWER (rw) register accessor: PMU_HP_ACTIVE_HP_CK_POWER\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_hp_ck_power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_hp_ck_power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_active_hp_ck_power`] module"]
pub type HP_ACTIVE_HP_CK_POWER = crate::Reg<hp_active_hp_ck_power::HP_ACTIVE_HP_CK_POWER_SPEC>;
#[doc = "PMU_HP_ACTIVE_HP_CK_POWER"]
pub mod hp_active_hp_ck_power;
#[doc = "HP_ACTIVE_BIAS (rw) register accessor: PMU_HP_ACTIVE_BIAS\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_bias::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_bias::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_active_bias`] module"]
pub type HP_ACTIVE_BIAS = crate::Reg<hp_active_bias::HP_ACTIVE_BIAS_SPEC>;
#[doc = "PMU_HP_ACTIVE_BIAS"]
pub mod hp_active_bias;
#[doc = "HP_ACTIVE_BACKUP (rw) register accessor: PMU_HP_ACTIVE_BACKUP\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_backup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_backup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_active_backup`] module"]
pub type HP_ACTIVE_BACKUP = crate::Reg<hp_active_backup::HP_ACTIVE_BACKUP_SPEC>;
#[doc = "PMU_HP_ACTIVE_BACKUP"]
pub mod hp_active_backup;
#[doc = "HP_ACTIVE_BACKUP_CLK (rw) register accessor: PMU_HP_ACTIVE_BACKUP_CLK\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_backup_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_backup_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_active_backup_clk`] module"]
pub type HP_ACTIVE_BACKUP_CLK = crate::Reg<hp_active_backup_clk::HP_ACTIVE_BACKUP_CLK_SPEC>;
#[doc = "PMU_HP_ACTIVE_BACKUP_CLK"]
pub mod hp_active_backup_clk;
#[doc = "HP_ACTIVE_SYSCLK (rw) register accessor: PMU_HP_ACTIVE_SYSCLK\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_sysclk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_sysclk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_active_sysclk`] module"]
pub type HP_ACTIVE_SYSCLK = crate::Reg<hp_active_sysclk::HP_ACTIVE_SYSCLK_SPEC>;
#[doc = "PMU_HP_ACTIVE_SYSCLK"]
pub mod hp_active_sysclk;
#[doc = "HP_ACTIVE_HP_REGULATOR0 (rw) register accessor: PMU_HP_ACTIVE_HP_REGULATOR0\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_hp_regulator0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_hp_regulator0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_active_hp_regulator0`] module"]
pub type HP_ACTIVE_HP_REGULATOR0 =
    crate::Reg<hp_active_hp_regulator0::HP_ACTIVE_HP_REGULATOR0_SPEC>;
#[doc = "PMU_HP_ACTIVE_HP_REGULATOR0"]
pub mod hp_active_hp_regulator0;
#[doc = "HP_ACTIVE_HP_REGULATOR1 (rw) register accessor: PMU_HP_ACTIVE_HP_REGULATOR1\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_hp_regulator1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_hp_regulator1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_active_hp_regulator1`] module"]
pub type HP_ACTIVE_HP_REGULATOR1 =
    crate::Reg<hp_active_hp_regulator1::HP_ACTIVE_HP_REGULATOR1_SPEC>;
#[doc = "PMU_HP_ACTIVE_HP_REGULATOR1"]
pub mod hp_active_hp_regulator1;
#[doc = "HP_ACTIVE_XTAL (rw) register accessor: PMU_HP_ACTIVE_XTAL\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_active_xtal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_active_xtal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_active_xtal`] module"]
pub type HP_ACTIVE_XTAL = crate::Reg<hp_active_xtal::HP_ACTIVE_XTAL_SPEC>;
#[doc = "PMU_HP_ACTIVE_XTAL"]
pub mod hp_active_xtal;
#[doc = "HP_MODEM_DIG_POWER (rw) register accessor: PMU_HP_MODEM_DIG_POWER\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_modem_dig_power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_dig_power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_modem_dig_power`] module"]
pub type HP_MODEM_DIG_POWER = crate::Reg<hp_modem_dig_power::HP_MODEM_DIG_POWER_SPEC>;
#[doc = "PMU_HP_MODEM_DIG_POWER"]
pub mod hp_modem_dig_power;
#[doc = "HP_MODEM_ICG_HP_FUNC (w) register accessor: PMU_HP_MODEM_ICG_HP_FUNC\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_icg_hp_func::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_modem_icg_hp_func`] module"]
pub type HP_MODEM_ICG_HP_FUNC = crate::Reg<hp_modem_icg_hp_func::HP_MODEM_ICG_HP_FUNC_SPEC>;
#[doc = "PMU_HP_MODEM_ICG_HP_FUNC"]
pub mod hp_modem_icg_hp_func;
#[doc = "HP_MODEM_ICG_HP_APB (w) register accessor: PMU_HP_MODEM_ICG_HP_APB\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_icg_hp_apb::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_modem_icg_hp_apb`] module"]
pub type HP_MODEM_ICG_HP_APB = crate::Reg<hp_modem_icg_hp_apb::HP_MODEM_ICG_HP_APB_SPEC>;
#[doc = "PMU_HP_MODEM_ICG_HP_APB"]
pub mod hp_modem_icg_hp_apb;
#[doc = "HP_MODEM_ICG_MODEM (w) register accessor: PMU_HP_MODEM_ICG_MODEM\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_icg_modem::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_modem_icg_modem`] module"]
pub type HP_MODEM_ICG_MODEM = crate::Reg<hp_modem_icg_modem::HP_MODEM_ICG_MODEM_SPEC>;
#[doc = "PMU_HP_MODEM_ICG_MODEM"]
pub mod hp_modem_icg_modem;
#[doc = "HP_MODEM_HP_SYS_CNTL (w) register accessor: PMU_HP_MODEM_HP_SYS_CNTL\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_hp_sys_cntl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_modem_hp_sys_cntl`] module"]
pub type HP_MODEM_HP_SYS_CNTL = crate::Reg<hp_modem_hp_sys_cntl::HP_MODEM_HP_SYS_CNTL_SPEC>;
#[doc = "PMU_HP_MODEM_HP_SYS_CNTL"]
pub mod hp_modem_hp_sys_cntl;
#[doc = "HP_MODEM_HP_CK_POWER (w) register accessor: PMU_HP_MODEM_HP_CK_POWER\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_hp_ck_power::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_modem_hp_ck_power`] module"]
pub type HP_MODEM_HP_CK_POWER = crate::Reg<hp_modem_hp_ck_power::HP_MODEM_HP_CK_POWER_SPEC>;
#[doc = "PMU_HP_MODEM_HP_CK_POWER"]
pub mod hp_modem_hp_ck_power;
#[doc = "HP_MODEM_BIAS (w) register accessor: PMU_HP_MODEM_BIAS\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_bias::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_modem_bias`] module"]
pub type HP_MODEM_BIAS = crate::Reg<hp_modem_bias::HP_MODEM_BIAS_SPEC>;
#[doc = "PMU_HP_MODEM_BIAS"]
pub mod hp_modem_bias;
#[doc = "HP_MODEM_BACKUP (w) register accessor: PMU_HP_MODEM_BACKUP\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_backup::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_modem_backup`] module"]
pub type HP_MODEM_BACKUP = crate::Reg<hp_modem_backup::HP_MODEM_BACKUP_SPEC>;
#[doc = "PMU_HP_MODEM_BACKUP"]
pub mod hp_modem_backup;
#[doc = "HP_MODEM_BACKUP_CLK (w) register accessor: PMU_HP_MODEM_BACKUP_CLK\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_backup_clk::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_modem_backup_clk`] module"]
pub type HP_MODEM_BACKUP_CLK = crate::Reg<hp_modem_backup_clk::HP_MODEM_BACKUP_CLK_SPEC>;
#[doc = "PMU_HP_MODEM_BACKUP_CLK"]
pub mod hp_modem_backup_clk;
#[doc = "HP_MODEM_SYSCLK (w) register accessor: PMU_HP_MODEM_SYSCLK\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_sysclk::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_modem_sysclk`] module"]
pub type HP_MODEM_SYSCLK = crate::Reg<hp_modem_sysclk::HP_MODEM_SYSCLK_SPEC>;
#[doc = "PMU_HP_MODEM_SYSCLK"]
pub mod hp_modem_sysclk;
#[doc = "HP_MODEM_HP_REGULATOR0 (w) register accessor: PMU_HP_MODEM_HP_REGULATOR0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_hp_regulator0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_modem_hp_regulator0`] module"]
pub type HP_MODEM_HP_REGULATOR0 = crate::Reg<hp_modem_hp_regulator0::HP_MODEM_HP_REGULATOR0_SPEC>;
#[doc = "PMU_HP_MODEM_HP_REGULATOR0"]
pub mod hp_modem_hp_regulator0;
#[doc = "HP_MODEM_HP_REGULATOR1 (w) register accessor: PMU_HP_MODEM_HP_REGULATOR1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_hp_regulator1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_modem_hp_regulator1`] module"]
pub type HP_MODEM_HP_REGULATOR1 = crate::Reg<hp_modem_hp_regulator1::HP_MODEM_HP_REGULATOR1_SPEC>;
#[doc = "PMU_HP_MODEM_HP_REGULATOR1"]
pub mod hp_modem_hp_regulator1;
#[doc = "HP_MODEM_XTAL (w) register accessor: PMU_HP_MODEM_XTAL\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_modem_xtal::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_modem_xtal`] module"]
pub type HP_MODEM_XTAL = crate::Reg<hp_modem_xtal::HP_MODEM_XTAL_SPEC>;
#[doc = "PMU_HP_MODEM_XTAL"]
pub mod hp_modem_xtal;
#[doc = "HP_SLEEP_DIG_POWER (rw) register accessor: PMU_HP_SLEEP_DIG_POWER\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_dig_power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_dig_power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_dig_power`] module"]
pub type HP_SLEEP_DIG_POWER = crate::Reg<hp_sleep_dig_power::HP_SLEEP_DIG_POWER_SPEC>;
#[doc = "PMU_HP_SLEEP_DIG_POWER"]
pub mod hp_sleep_dig_power;
#[doc = "HP_SLEEP_ICG_HP_FUNC (rw) register accessor: PMU_HP_SLEEP_ICG_HP_FUNC\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_icg_hp_func::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_icg_hp_func::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_icg_hp_func`] module"]
pub type HP_SLEEP_ICG_HP_FUNC = crate::Reg<hp_sleep_icg_hp_func::HP_SLEEP_ICG_HP_FUNC_SPEC>;
#[doc = "PMU_HP_SLEEP_ICG_HP_FUNC"]
pub mod hp_sleep_icg_hp_func;
#[doc = "HP_SLEEP_ICG_HP_APB (rw) register accessor: PMU_HP_SLEEP_ICG_HP_APB\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_icg_hp_apb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_icg_hp_apb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_icg_hp_apb`] module"]
pub type HP_SLEEP_ICG_HP_APB = crate::Reg<hp_sleep_icg_hp_apb::HP_SLEEP_ICG_HP_APB_SPEC>;
#[doc = "PMU_HP_SLEEP_ICG_HP_APB"]
pub mod hp_sleep_icg_hp_apb;
#[doc = "HP_SLEEP_ICG_MODEM (rw) register accessor: PMU_HP_SLEEP_ICG_MODEM\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_icg_modem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_icg_modem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_icg_modem`] module"]
pub type HP_SLEEP_ICG_MODEM = crate::Reg<hp_sleep_icg_modem::HP_SLEEP_ICG_MODEM_SPEC>;
#[doc = "PMU_HP_SLEEP_ICG_MODEM"]
pub mod hp_sleep_icg_modem;
#[doc = "HP_SLEEP_HP_SYS_CNTL (rw) register accessor: PMU_HP_SLEEP_HP_SYS_CNTL\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_hp_sys_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_hp_sys_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_hp_sys_cntl`] module"]
pub type HP_SLEEP_HP_SYS_CNTL = crate::Reg<hp_sleep_hp_sys_cntl::HP_SLEEP_HP_SYS_CNTL_SPEC>;
#[doc = "PMU_HP_SLEEP_HP_SYS_CNTL"]
pub mod hp_sleep_hp_sys_cntl;
#[doc = "HP_SLEEP_HP_CK_POWER (rw) register accessor: PMU_HP_SLEEP_HP_CK_POWER\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_hp_ck_power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_hp_ck_power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_hp_ck_power`] module"]
pub type HP_SLEEP_HP_CK_POWER = crate::Reg<hp_sleep_hp_ck_power::HP_SLEEP_HP_CK_POWER_SPEC>;
#[doc = "PMU_HP_SLEEP_HP_CK_POWER"]
pub mod hp_sleep_hp_ck_power;
#[doc = "HP_SLEEP_BIAS (rw) register accessor: PMU_HP_SLEEP_BIAS\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_bias::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_bias::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_bias`] module"]
pub type HP_SLEEP_BIAS = crate::Reg<hp_sleep_bias::HP_SLEEP_BIAS_SPEC>;
#[doc = "PMU_HP_SLEEP_BIAS"]
pub mod hp_sleep_bias;
#[doc = "HP_SLEEP_BACKUP (rw) register accessor: PMU_HP_SLEEP_BACKUP\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_backup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_backup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_backup`] module"]
pub type HP_SLEEP_BACKUP = crate::Reg<hp_sleep_backup::HP_SLEEP_BACKUP_SPEC>;
#[doc = "PMU_HP_SLEEP_BACKUP"]
pub mod hp_sleep_backup;
#[doc = "HP_SLEEP_BACKUP_CLK (rw) register accessor: PMU_HP_SLEEP_BACKUP_CLK\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_backup_clk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_backup_clk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_backup_clk`] module"]
pub type HP_SLEEP_BACKUP_CLK = crate::Reg<hp_sleep_backup_clk::HP_SLEEP_BACKUP_CLK_SPEC>;
#[doc = "PMU_HP_SLEEP_BACKUP_CLK"]
pub mod hp_sleep_backup_clk;
#[doc = "HP_SLEEP_SYSCLK (rw) register accessor: PMU_HP_SLEEP_SYSCLK\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_sysclk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_sysclk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_sysclk`] module"]
pub type HP_SLEEP_SYSCLK = crate::Reg<hp_sleep_sysclk::HP_SLEEP_SYSCLK_SPEC>;
#[doc = "PMU_HP_SLEEP_SYSCLK"]
pub mod hp_sleep_sysclk;
#[doc = "HP_SLEEP_HP_REGULATOR0 (rw) register accessor: PMU_HP_SLEEP_HP_REGULATOR0\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_hp_regulator0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_hp_regulator0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_hp_regulator0`] module"]
pub type HP_SLEEP_HP_REGULATOR0 = crate::Reg<hp_sleep_hp_regulator0::HP_SLEEP_HP_REGULATOR0_SPEC>;
#[doc = "PMU_HP_SLEEP_HP_REGULATOR0"]
pub mod hp_sleep_hp_regulator0;
#[doc = "HP_SLEEP_HP_REGULATOR1 (rw) register accessor: PMU_HP_SLEEP_HP_REGULATOR1\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_hp_regulator1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_hp_regulator1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_hp_regulator1`] module"]
pub type HP_SLEEP_HP_REGULATOR1 = crate::Reg<hp_sleep_hp_regulator1::HP_SLEEP_HP_REGULATOR1_SPEC>;
#[doc = "PMU_HP_SLEEP_HP_REGULATOR1"]
pub mod hp_sleep_hp_regulator1;
#[doc = "HP_SLEEP_XTAL (rw) register accessor: PMU_HP_SLEEP_XTAL\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_xtal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_xtal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_xtal`] module"]
pub type HP_SLEEP_XTAL = crate::Reg<hp_sleep_xtal::HP_SLEEP_XTAL_SPEC>;
#[doc = "PMU_HP_SLEEP_XTAL"]
pub mod hp_sleep_xtal;
#[doc = "HP_SLEEP_LP_REGULATOR0 (rw) register accessor: PMU_HP_SLEEP_LP_REGULATOR0\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_lp_regulator0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_lp_regulator0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_lp_regulator0`] module"]
pub type HP_SLEEP_LP_REGULATOR0 = crate::Reg<hp_sleep_lp_regulator0::HP_SLEEP_LP_REGULATOR0_SPEC>;
#[doc = "PMU_HP_SLEEP_LP_REGULATOR0"]
pub mod hp_sleep_lp_regulator0;
#[doc = "HP_SLEEP_LP_REGULATOR1 (rw) register accessor: PMU_HP_SLEEP_LP_REGULATOR1\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_lp_regulator1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_lp_regulator1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_lp_regulator1`] module"]
pub type HP_SLEEP_LP_REGULATOR1 = crate::Reg<hp_sleep_lp_regulator1::HP_SLEEP_LP_REGULATOR1_SPEC>;
#[doc = "PMU_HP_SLEEP_LP_REGULATOR1"]
pub mod hp_sleep_lp_regulator1;
#[doc = "HP_SLEEP_LP_DIG_POWER (rw) register accessor: PMU_HP_SLEEP_LP_DIG_POWER\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_lp_dig_power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_lp_dig_power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_lp_dig_power`] module"]
pub type HP_SLEEP_LP_DIG_POWER = crate::Reg<hp_sleep_lp_dig_power::HP_SLEEP_LP_DIG_POWER_SPEC>;
#[doc = "PMU_HP_SLEEP_LP_DIG_POWER"]
pub mod hp_sleep_lp_dig_power;
#[doc = "HP_SLEEP_LP_CK_POWER (rw) register accessor: PMU_HP_SLEEP_LP_CK_POWER\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_sleep_lp_ck_power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_sleep_lp_ck_power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_sleep_lp_ck_power`] module"]
pub type HP_SLEEP_LP_CK_POWER = crate::Reg<hp_sleep_lp_ck_power::HP_SLEEP_LP_CK_POWER_SPEC>;
#[doc = "PMU_HP_SLEEP_LP_CK_POWER"]
pub mod hp_sleep_lp_ck_power;
#[doc = "LP_SLEEP_LP_REGULATOR0 (rw) register accessor: PMU_LP_SLEEP_LP_REGULATOR0\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_sleep_lp_regulator0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_sleep_lp_regulator0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_sleep_lp_regulator0`] module"]
pub type LP_SLEEP_LP_REGULATOR0 = crate::Reg<lp_sleep_lp_regulator0::LP_SLEEP_LP_REGULATOR0_SPEC>;
#[doc = "PMU_LP_SLEEP_LP_REGULATOR0"]
pub mod lp_sleep_lp_regulator0;
#[doc = "LP_SLEEP_LP_REGULATOR1 (rw) register accessor: PMU_LP_SLEEP_LP_REGULATOR1\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_sleep_lp_regulator1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_sleep_lp_regulator1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_sleep_lp_regulator1`] module"]
pub type LP_SLEEP_LP_REGULATOR1 = crate::Reg<lp_sleep_lp_regulator1::LP_SLEEP_LP_REGULATOR1_SPEC>;
#[doc = "PMU_LP_SLEEP_LP_REGULATOR1"]
pub mod lp_sleep_lp_regulator1;
#[doc = "LP_SLEEP_XTAL (rw) register accessor: PMU_LP_SLEEP_XTAL\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_sleep_xtal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_sleep_xtal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_sleep_xtal`] module"]
pub type LP_SLEEP_XTAL = crate::Reg<lp_sleep_xtal::LP_SLEEP_XTAL_SPEC>;
#[doc = "PMU_LP_SLEEP_XTAL"]
pub mod lp_sleep_xtal;
#[doc = "LP_SLEEP_LP_DIG_POWER (rw) register accessor: PMU_LP_SLEEP_LP_DIG_POWER\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_sleep_lp_dig_power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_sleep_lp_dig_power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_sleep_lp_dig_power`] module"]
pub type LP_SLEEP_LP_DIG_POWER = crate::Reg<lp_sleep_lp_dig_power::LP_SLEEP_LP_DIG_POWER_SPEC>;
#[doc = "PMU_LP_SLEEP_LP_DIG_POWER"]
pub mod lp_sleep_lp_dig_power;
#[doc = "LP_SLEEP_LP_CK_POWER (rw) register accessor: PMU_LP_SLEEP_LP_CK_POWER\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_sleep_lp_ck_power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_sleep_lp_ck_power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_sleep_lp_ck_power`] module"]
pub type LP_SLEEP_LP_CK_POWER = crate::Reg<lp_sleep_lp_ck_power::LP_SLEEP_LP_CK_POWER_SPEC>;
#[doc = "PMU_LP_SLEEP_LP_CK_POWER"]
pub mod lp_sleep_lp_ck_power;
#[doc = "LP_SLEEP_BIAS (rw) register accessor: PMU_LP_SLEEP_BIAS\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_sleep_bias::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_sleep_bias::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_sleep_bias`] module"]
pub type LP_SLEEP_BIAS = crate::Reg<lp_sleep_bias::LP_SLEEP_BIAS_SPEC>;
#[doc = "PMU_LP_SLEEP_BIAS"]
pub mod lp_sleep_bias;
#[doc = "IMM_HP_CK_POWER (rw) register accessor: PMU_IMM_HP_CK_POWER\n\nYou can [`read`](crate::Reg::read) this register and get [`imm_hp_ck_power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imm_hp_ck_power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imm_hp_ck_power`] module"]
pub type IMM_HP_CK_POWER = crate::Reg<imm_hp_ck_power::IMM_HP_CK_POWER_SPEC>;
#[doc = "PMU_IMM_HP_CK_POWER"]
pub mod imm_hp_ck_power;
#[doc = "IMM_SLEEP_SYSCLK (w) register accessor: PMU_IMM_SLEEP_SYSCLK\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imm_sleep_sysclk::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imm_sleep_sysclk`] module"]
pub type IMM_SLEEP_SYSCLK = crate::Reg<imm_sleep_sysclk::IMM_SLEEP_SYSCLK_SPEC>;
#[doc = "PMU_IMM_SLEEP_SYSCLK"]
pub mod imm_sleep_sysclk;
#[doc = "IMM_HP_FUNC_ICG (w) register accessor: PMU_IMM_HP_FUNC_ICG\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imm_hp_func_icg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imm_hp_func_icg`] module"]
pub type IMM_HP_FUNC_ICG = crate::Reg<imm_hp_func_icg::IMM_HP_FUNC_ICG_SPEC>;
#[doc = "PMU_IMM_HP_FUNC_ICG"]
pub mod imm_hp_func_icg;
#[doc = "IMM_HP_APB_ICG (w) register accessor: PMU_IMM_HP_APB_ICG\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imm_hp_apb_icg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imm_hp_apb_icg`] module"]
pub type IMM_HP_APB_ICG = crate::Reg<imm_hp_apb_icg::IMM_HP_APB_ICG_SPEC>;
#[doc = "PMU_IMM_HP_APB_ICG"]
pub mod imm_hp_apb_icg;
#[doc = "IMM_MODEM_ICG (w) register accessor: PMU_IMM_MODEM_ICG\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imm_modem_icg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imm_modem_icg`] module"]
pub type IMM_MODEM_ICG = crate::Reg<imm_modem_icg::IMM_MODEM_ICG_SPEC>;
#[doc = "PMU_IMM_MODEM_ICG"]
pub mod imm_modem_icg;
#[doc = "IMM_LP_ICG (w) register accessor: PMU_IMM_LP_ICG\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imm_lp_icg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imm_lp_icg`] module"]
pub type IMM_LP_ICG = crate::Reg<imm_lp_icg::IMM_LP_ICG_SPEC>;
#[doc = "PMU_IMM_LP_ICG"]
pub mod imm_lp_icg;
#[doc = "IMM_PAD_HOLD_ALL (rw) register accessor: PMU_IMM_PAD_HOLD_ALL\n\nYou can [`read`](crate::Reg::read) this register and get [`imm_pad_hold_all::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imm_pad_hold_all::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imm_pad_hold_all`] module"]
pub type IMM_PAD_HOLD_ALL = crate::Reg<imm_pad_hold_all::IMM_PAD_HOLD_ALL_SPEC>;
#[doc = "PMU_IMM_PAD_HOLD_ALL"]
pub mod imm_pad_hold_all;
#[doc = "IMM_I2C_ISO (w) register accessor: PMU_IMM_I2C_ISO\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imm_i2c_iso::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imm_i2c_iso`] module"]
pub type IMM_I2C_ISO = crate::Reg<imm_i2c_iso::IMM_I2C_ISO_SPEC>;
#[doc = "PMU_IMM_I2C_ISO"]
pub mod imm_i2c_iso;
#[doc = "POWER_WAIT_TIMER0 (rw) register accessor: PMU_POWER_WAIT_TIMER0\n\nYou can [`read`](crate::Reg::read) this register and get [`power_wait_timer0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_wait_timer0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_wait_timer0`] module"]
pub type POWER_WAIT_TIMER0 = crate::Reg<power_wait_timer0::POWER_WAIT_TIMER0_SPEC>;
#[doc = "PMU_POWER_WAIT_TIMER0"]
pub mod power_wait_timer0;
#[doc = "POWER_WAIT_TIMER1 (rw) register accessor: PMU_POWER_WAIT_TIMER1\n\nYou can [`read`](crate::Reg::read) this register and get [`power_wait_timer1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_wait_timer1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_wait_timer1`] module"]
pub type POWER_WAIT_TIMER1 = crate::Reg<power_wait_timer1::POWER_WAIT_TIMER1_SPEC>;
#[doc = "PMU_POWER_WAIT_TIMER1"]
pub mod power_wait_timer1;
#[doc = "POWER_PD_TOP_CNTL (rw) register accessor: PMU_POWER_PD_TOP_CNTL\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_top_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_top_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_pd_top_cntl`] module"]
pub type POWER_PD_TOP_CNTL = crate::Reg<power_pd_top_cntl::POWER_PD_TOP_CNTL_SPEC>;
#[doc = "PMU_POWER_PD_TOP_CNTL"]
pub mod power_pd_top_cntl;
#[doc = "POWER_PD_CNNT_CNTL (rw) register accessor: PMU_POWER_PD_CNNT_CNTL\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_cnnt_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_cnnt_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_pd_cnnt_cntl`] module"]
pub type POWER_PD_CNNT_CNTL = crate::Reg<power_pd_cnnt_cntl::POWER_PD_CNNT_CNTL_SPEC>;
#[doc = "PMU_POWER_PD_CNNT_CNTL"]
pub mod power_pd_cnnt_cntl;
#[doc = "POWER_PD_HPMEM_CNTL (rw) register accessor: PMU_POWER_PD_HPMEM_CNTL\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_hpmem_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_hpmem_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_pd_hpmem_cntl`] module"]
pub type POWER_PD_HPMEM_CNTL = crate::Reg<power_pd_hpmem_cntl::POWER_PD_HPMEM_CNTL_SPEC>;
#[doc = "PMU_POWER_PD_HPMEM_CNTL"]
pub mod power_pd_hpmem_cntl;
#[doc = "POWER_PD_TOP_MASK (rw) register accessor: PMU_POWER_PD_TOP_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_top_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_top_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_pd_top_mask`] module"]
pub type POWER_PD_TOP_MASK = crate::Reg<power_pd_top_mask::POWER_PD_TOP_MASK_SPEC>;
#[doc = "PMU_POWER_PD_TOP_MASK"]
pub mod power_pd_top_mask;
#[doc = "POWER_PD_CNNT_MASK (rw) register accessor: PMU_POWER_PD_CNNT_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_cnnt_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_cnnt_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_pd_cnnt_mask`] module"]
pub type POWER_PD_CNNT_MASK = crate::Reg<power_pd_cnnt_mask::POWER_PD_CNNT_MASK_SPEC>;
#[doc = "PMU_POWER_PD_CNNT_MASK"]
pub mod power_pd_cnnt_mask;
#[doc = "POWER_PD_HPMEM_MASK (rw) register accessor: PMU_POWER_PD_HPMEM_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_hpmem_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_hpmem_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_pd_hpmem_mask`] module"]
pub type POWER_PD_HPMEM_MASK = crate::Reg<power_pd_hpmem_mask::POWER_PD_HPMEM_MASK_SPEC>;
#[doc = "PMU_POWER_PD_HPMEM_MASK"]
pub mod power_pd_hpmem_mask;
#[doc = "POWER_DCDC_SWITCH (rw) register accessor: PMU_POWER_DCDC_SWITCH\n\nYou can [`read`](crate::Reg::read) this register and get [`power_dcdc_switch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_dcdc_switch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_dcdc_switch`] module"]
pub type POWER_DCDC_SWITCH = crate::Reg<power_dcdc_switch::POWER_DCDC_SWITCH_SPEC>;
#[doc = "PMU_POWER_DCDC_SWITCH"]
pub mod power_dcdc_switch;
#[doc = "POWER_PD_LPPERI_CNTL (rw) register accessor: PMU_POWER_PD_LPPERI_CNTL\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_lpperi_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_lpperi_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_pd_lpperi_cntl`] module"]
pub type POWER_PD_LPPERI_CNTL = crate::Reg<power_pd_lpperi_cntl::POWER_PD_LPPERI_CNTL_SPEC>;
#[doc = "PMU_POWER_PD_LPPERI_CNTL"]
pub mod power_pd_lpperi_cntl;
#[doc = "POWER_PD_LPPERI_MASK (rw) register accessor: PMU_POWER_PD_LPPERI_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_lpperi_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_lpperi_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_pd_lpperi_mask`] module"]
pub type POWER_PD_LPPERI_MASK = crate::Reg<power_pd_lpperi_mask::POWER_PD_LPPERI_MASK_SPEC>;
#[doc = "PMU_POWER_PD_LPPERI_MASK"]
pub mod power_pd_lpperi_mask;
#[doc = "POWER_HP_PAD (rw) register accessor: PMU_POWER_HP_PAD\n\nYou can [`read`](crate::Reg::read) this register and get [`power_hp_pad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_hp_pad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_hp_pad`] module"]
pub type POWER_HP_PAD = crate::Reg<power_hp_pad::POWER_HP_PAD_SPEC>;
#[doc = "PMU_POWER_HP_PAD"]
pub mod power_hp_pad;
#[doc = "POWER_CK_WAIT_CNTL (rw) register accessor: PMU_POWER_CK_WAIT_CNTL\n\nYou can [`read`](crate::Reg::read) this register and get [`power_ck_wait_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_ck_wait_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_ck_wait_cntl`] module"]
pub type POWER_CK_WAIT_CNTL = crate::Reg<power_ck_wait_cntl::POWER_CK_WAIT_CNTL_SPEC>;
#[doc = "PMU_POWER_CK_WAIT_CNTL"]
pub mod power_ck_wait_cntl;
#[doc = "SLP_WAKEUP_CNTL0 (w) register accessor: PMU_SLP_WAKEUP_CNTL0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_wakeup_cntl0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_wakeup_cntl0`] module"]
pub type SLP_WAKEUP_CNTL0 = crate::Reg<slp_wakeup_cntl0::SLP_WAKEUP_CNTL0_SPEC>;
#[doc = "PMU_SLP_WAKEUP_CNTL0"]
pub mod slp_wakeup_cntl0;
#[doc = "SLP_WAKEUP_CNTL1 (rw) register accessor: PMU_SLP_WAKEUP_CNTL1\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_cntl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_wakeup_cntl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_wakeup_cntl1`] module"]
pub type SLP_WAKEUP_CNTL1 = crate::Reg<slp_wakeup_cntl1::SLP_WAKEUP_CNTL1_SPEC>;
#[doc = "PMU_SLP_WAKEUP_CNTL1"]
pub mod slp_wakeup_cntl1;
#[doc = "SLP_WAKEUP_CNTL2 (rw) register accessor: PMU_SLP_WAKEUP_CNTL2\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_cntl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_wakeup_cntl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_wakeup_cntl2`] module"]
pub type SLP_WAKEUP_CNTL2 = crate::Reg<slp_wakeup_cntl2::SLP_WAKEUP_CNTL2_SPEC>;
#[doc = "PMU_SLP_WAKEUP_CNTL2"]
pub mod slp_wakeup_cntl2;
#[doc = "SLP_WAKEUP_CNTL3 (rw) register accessor: PMU_SLP_WAKEUP_CNTL3\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_cntl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_wakeup_cntl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_wakeup_cntl3`] module"]
pub type SLP_WAKEUP_CNTL3 = crate::Reg<slp_wakeup_cntl3::SLP_WAKEUP_CNTL3_SPEC>;
#[doc = "PMU_SLP_WAKEUP_CNTL3"]
pub mod slp_wakeup_cntl3;
#[doc = "SLP_WAKEUP_CNTL4 (w) register accessor: PMU_SLP_WAKEUP_CNTL4\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_wakeup_cntl4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_wakeup_cntl4`] module"]
pub type SLP_WAKEUP_CNTL4 = crate::Reg<slp_wakeup_cntl4::SLP_WAKEUP_CNTL4_SPEC>;
#[doc = "PMU_SLP_WAKEUP_CNTL4"]
pub mod slp_wakeup_cntl4;
#[doc = "SLP_WAKEUP_CNTL5 (rw) register accessor: PMU_SLP_WAKEUP_CNTL5\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_cntl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_wakeup_cntl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_wakeup_cntl5`] module"]
pub type SLP_WAKEUP_CNTL5 = crate::Reg<slp_wakeup_cntl5::SLP_WAKEUP_CNTL5_SPEC>;
#[doc = "PMU_SLP_WAKEUP_CNTL5"]
pub mod slp_wakeup_cntl5;
#[doc = "SLP_WAKEUP_CNTL6 (rw) register accessor: PMU_SLP_WAKEUP_CNTL6\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_cntl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_wakeup_cntl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_wakeup_cntl6`] module"]
pub type SLP_WAKEUP_CNTL6 = crate::Reg<slp_wakeup_cntl6::SLP_WAKEUP_CNTL6_SPEC>;
#[doc = "PMU_SLP_WAKEUP_CNTL6"]
pub mod slp_wakeup_cntl6;
#[doc = "SLP_WAKEUP_CNTL7 (rw) register accessor: PMU_SLP_WAKEUP_CNTL7\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_cntl7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_wakeup_cntl7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_wakeup_cntl7`] module"]
pub type SLP_WAKEUP_CNTL7 = crate::Reg<slp_wakeup_cntl7::SLP_WAKEUP_CNTL7_SPEC>;
#[doc = "PMU_SLP_WAKEUP_CNTL7"]
pub mod slp_wakeup_cntl7;
#[doc = "SLP_WAKEUP_CNTL8 (rw) register accessor: PMU_SLP_WAKEUP_CNTL8\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_cntl8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_wakeup_cntl8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_wakeup_cntl8`] module"]
pub type SLP_WAKEUP_CNTL8 = crate::Reg<slp_wakeup_cntl8::SLP_WAKEUP_CNTL8_SPEC>;
#[doc = "PMU_SLP_WAKEUP_CNTL8"]
pub mod slp_wakeup_cntl8;
#[doc = "SLP_WAKEUP_STATUS0 (r) register accessor: PMU_SLP_WAKEUP_STATUS0\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_status0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_wakeup_status0`] module"]
pub type SLP_WAKEUP_STATUS0 = crate::Reg<slp_wakeup_status0::SLP_WAKEUP_STATUS0_SPEC>;
#[doc = "PMU_SLP_WAKEUP_STATUS0"]
pub mod slp_wakeup_status0;
#[doc = "SLP_WAKEUP_STATUS1 (r) register accessor: PMU_SLP_WAKEUP_STATUS1\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_status1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_wakeup_status1`] module"]
pub type SLP_WAKEUP_STATUS1 = crate::Reg<slp_wakeup_status1::SLP_WAKEUP_STATUS1_SPEC>;
#[doc = "PMU_SLP_WAKEUP_STATUS1"]
pub mod slp_wakeup_status1;
#[doc = "SLP_WAKEUP_STATUS2 (r) register accessor: PMU_SLP_WAKEUP_STATUS2\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_status2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slp_wakeup_status2`] module"]
pub type SLP_WAKEUP_STATUS2 = crate::Reg<slp_wakeup_status2::SLP_WAKEUP_STATUS2_SPEC>;
#[doc = "PMU_SLP_WAKEUP_STATUS2"]
pub mod slp_wakeup_status2;
#[doc = "HP_CK_POWERON (rw) register accessor: PMU_HP_CK_POWERON\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_ck_poweron::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_ck_poweron::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_ck_poweron`] module"]
pub type HP_CK_POWERON = crate::Reg<hp_ck_poweron::HP_CK_POWERON_SPEC>;
#[doc = "PMU_HP_CK_POWERON"]
pub mod hp_ck_poweron;
#[doc = "HP_CK_CNTL (rw) register accessor: PMU_HP_CK_CNTL\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_ck_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_ck_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_ck_cntl`] module"]
pub type HP_CK_CNTL = crate::Reg<hp_ck_cntl::HP_CK_CNTL_SPEC>;
#[doc = "PMU_HP_CK_CNTL"]
pub mod hp_ck_cntl;
#[doc = "POR_STATUS (r) register accessor: PMU_POR_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`por_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@por_status`] module"]
pub type POR_STATUS = crate::Reg<por_status::POR_STATUS_SPEC>;
#[doc = "PMU_POR_STATUS"]
pub mod por_status;
#[doc = "RF_PWC (rw) register accessor: PMU_RF_PWC\n\nYou can [`read`](crate::Reg::read) this register and get [`rf_pwc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rf_pwc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf_pwc`] module"]
pub type RF_PWC = crate::Reg<rf_pwc::RF_PWC_SPEC>;
#[doc = "PMU_RF_PWC"]
pub mod rf_pwc;
#[doc = "BACKUP_CFG (rw) register accessor: PMU_BACKUP_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`backup_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`backup_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@backup_cfg`] module"]
pub type BACKUP_CFG = crate::Reg<backup_cfg::BACKUP_CFG_SPEC>;
#[doc = "PMU_BACKUP_CFG"]
pub mod backup_cfg;
#[doc = "INT_RAW (rw) register accessor: PMU_INT_RAW\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "PMU_INT_RAW"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: PMU_HP_INT_ST\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "PMU_HP_INT_ST"]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: PMU_HP_INT_ENA\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "PMU_HP_INT_ENA"]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: PMU_HP_INT_CLR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "PMU_HP_INT_CLR"]
pub mod int_clr;
#[doc = "LP_INT_RAW (rw) register accessor: PMU_LP_INT_RAW\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_int_raw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_int_raw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_int_raw`] module"]
pub type LP_INT_RAW = crate::Reg<lp_int_raw::LP_INT_RAW_SPEC>;
#[doc = "PMU_LP_INT_RAW"]
pub mod lp_int_raw;
#[doc = "LP_INT_ST (r) register accessor: PMU_LP_INT_ST\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_int_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_int_st`] module"]
pub type LP_INT_ST = crate::Reg<lp_int_st::LP_INT_ST_SPEC>;
#[doc = "PMU_LP_INT_ST"]
pub mod lp_int_st;
#[doc = "LP_INT_ENA (rw) register accessor: PMU_LP_INT_ENA\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_int_ena::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_int_ena::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_int_ena`] module"]
pub type LP_INT_ENA = crate::Reg<lp_int_ena::LP_INT_ENA_SPEC>;
#[doc = "PMU_LP_INT_ENA"]
pub mod lp_int_ena;
#[doc = "LP_INT_CLR (w) register accessor: PMU_LP_INT_CLR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_int_clr`] module"]
pub type LP_INT_CLR = crate::Reg<lp_int_clr::LP_INT_CLR_SPEC>;
#[doc = "PMU_LP_INT_CLR"]
pub mod lp_int_clr;
#[doc = "LP_CPU_PWR0 (rw) register accessor: PMU_LP_CPU_PWR0\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_cpu_pwr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_cpu_pwr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_cpu_pwr0`] module"]
pub type LP_CPU_PWR0 = crate::Reg<lp_cpu_pwr0::LP_CPU_PWR0_SPEC>;
#[doc = "PMU_LP_CPU_PWR0"]
pub mod lp_cpu_pwr0;
#[doc = "LP_CPU_PWR1 (w) register accessor: PMU_LP_CPU_PWR1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_cpu_pwr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_cpu_pwr1`] module"]
pub type LP_CPU_PWR1 = crate::Reg<lp_cpu_pwr1::LP_CPU_PWR1_SPEC>;
#[doc = "PMU_LP_CPU_PWR1"]
pub mod lp_cpu_pwr1;
#[doc = "LP_CPU_PWR2 (rw) register accessor: PMU_LP_CPU_PWR2\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_cpu_pwr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_cpu_pwr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_cpu_pwr2`] module"]
pub type LP_CPU_PWR2 = crate::Reg<lp_cpu_pwr2::LP_CPU_PWR2_SPEC>;
#[doc = "PMU_LP_CPU_PWR2"]
pub mod lp_cpu_pwr2;
#[doc = "LP_CPU_PWR3 (r) register accessor: PMU_LP_CPU_PWR3\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_cpu_pwr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_cpu_pwr3`] module"]
pub type LP_CPU_PWR3 = crate::Reg<lp_cpu_pwr3::LP_CPU_PWR3_SPEC>;
#[doc = "PMU_LP_CPU_PWR3"]
pub mod lp_cpu_pwr3;
#[doc = "LP_CPU_PWR4 (rw) register accessor: PMU_LP_CPU_PWR4\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_cpu_pwr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_cpu_pwr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_cpu_pwr4`] module"]
pub type LP_CPU_PWR4 = crate::Reg<lp_cpu_pwr4::LP_CPU_PWR4_SPEC>;
#[doc = "PMU_LP_CPU_PWR4"]
pub mod lp_cpu_pwr4;
#[doc = "LP_CPU_PWR5 (r) register accessor: PMU_LP_CPU_PWR5\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_cpu_pwr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lp_cpu_pwr5`] module"]
pub type LP_CPU_PWR5 = crate::Reg<lp_cpu_pwr5::LP_CPU_PWR5_SPEC>;
#[doc = "PMU_LP_CPU_PWR5"]
pub mod lp_cpu_pwr5;
#[doc = "HP_LP_CPU_COMM (w) register accessor: PMU_HP_LP_CPU_COMM\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_lp_cpu_comm::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_lp_cpu_comm`] module"]
pub type HP_LP_CPU_COMM = crate::Reg<hp_lp_cpu_comm::HP_LP_CPU_COMM_SPEC>;
#[doc = "PMU_HP_LP_CPU_COMM"]
pub mod hp_lp_cpu_comm;
#[doc = "HP_REGULATOR_CFG (rw) register accessor: PMU_HP_REGULATOR_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_regulator_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_regulator_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hp_regulator_cfg`] module"]
pub type HP_REGULATOR_CFG = crate::Reg<hp_regulator_cfg::HP_REGULATOR_CFG_SPEC>;
#[doc = "PMU_HP_REGULATOR_CFG"]
pub mod hp_regulator_cfg;
#[doc = "MAIN_STATE (rw) register accessor: PMU_MAIN_STATE\n\nYou can [`read`](crate::Reg::read) this register and get [`main_state::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`main_state::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@main_state`] module"]
pub type MAIN_STATE = crate::Reg<main_state::MAIN_STATE_SPEC>;
#[doc = "PMU_MAIN_STATE"]
pub mod main_state;
#[doc = "PWR_STATE (r) register accessor: PMU_PWR_STATE\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr_state`] module"]
pub type PWR_STATE = crate::Reg<pwr_state::PWR_STATE_SPEC>;
#[doc = "PMU_PWR_STATE"]
pub mod pwr_state;
#[doc = "CLK_STATE0 (r) register accessor: PMU_CLK_STATE0\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_state0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_state0`] module"]
pub type CLK_STATE0 = crate::Reg<clk_state0::CLK_STATE0_SPEC>;
#[doc = "PMU_CLK_STATE0"]
pub mod clk_state0;
#[doc = "CLK_STATE1 (r) register accessor: PMU_CLK_STATE1\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_state1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_state1`] module"]
pub type CLK_STATE1 = crate::Reg<clk_state1::CLK_STATE1_SPEC>;
#[doc = "PMU_CLK_STATE1"]
pub mod clk_state1;
#[doc = "CLK_STATE2 (r) register accessor: PMU_CLK_STATE2\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_state2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_state2`] module"]
pub type CLK_STATE2 = crate::Reg<clk_state2::CLK_STATE2_SPEC>;
#[doc = "PMU_CLK_STATE2"]
pub mod clk_state2;
#[doc = "EXT_LDO_P0_0P1A (rw) register accessor: PMU_EXT_LDO_P0_0P1A\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p0_0p1a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p0_0p1a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ldo_p0_0p1a`] module"]
pub type EXT_LDO_P0_0P1A = crate::Reg<ext_ldo_p0_0p1a::EXT_LDO_P0_0P1A_SPEC>;
#[doc = "PMU_EXT_LDO_P0_0P1A"]
pub mod ext_ldo_p0_0p1a;
#[doc = "EXT_LDO_P0_0P1A_ANA (rw) register accessor: PMU_EXT_LDO_P0_0P1A_ANA\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p0_0p1a_ana::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p0_0p1a_ana::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ldo_p0_0p1a_ana`] module"]
pub type EXT_LDO_P0_0P1A_ANA = crate::Reg<ext_ldo_p0_0p1a_ana::EXT_LDO_P0_0P1A_ANA_SPEC>;
#[doc = "PMU_EXT_LDO_P0_0P1A_ANA"]
pub mod ext_ldo_p0_0p1a_ana;
#[doc = "EXT_LDO_P0_0P2A (rw) register accessor: PMU_EXT_LDO_P0_0P2A\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p0_0p2a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p0_0p2a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ldo_p0_0p2a`] module"]
pub type EXT_LDO_P0_0P2A = crate::Reg<ext_ldo_p0_0p2a::EXT_LDO_P0_0P2A_SPEC>;
#[doc = "PMU_EXT_LDO_P0_0P2A"]
pub mod ext_ldo_p0_0p2a;
#[doc = "EXT_LDO_P0_0P2A_ANA (rw) register accessor: PMU_EXT_LDO_P0_0P2A_ANA\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p0_0p2a_ana::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p0_0p2a_ana::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ldo_p0_0p2a_ana`] module"]
pub type EXT_LDO_P0_0P2A_ANA = crate::Reg<ext_ldo_p0_0p2a_ana::EXT_LDO_P0_0P2A_ANA_SPEC>;
#[doc = "PMU_EXT_LDO_P0_0P2A_ANA"]
pub mod ext_ldo_p0_0p2a_ana;
#[doc = "EXT_LDO_P0_0P3A (rw) register accessor: PMU_EXT_LDO_P0_0P3A\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p0_0p3a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p0_0p3a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ldo_p0_0p3a`] module"]
pub type EXT_LDO_P0_0P3A = crate::Reg<ext_ldo_p0_0p3a::EXT_LDO_P0_0P3A_SPEC>;
#[doc = "PMU_EXT_LDO_P0_0P3A"]
pub mod ext_ldo_p0_0p3a;
#[doc = "EXT_LDO_P0_0P3A_ANA (rw) register accessor: PMU_EXT_LDO_P0_0P3A_ANA\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p0_0p3a_ana::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p0_0p3a_ana::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ldo_p0_0p3a_ana`] module"]
pub type EXT_LDO_P0_0P3A_ANA = crate::Reg<ext_ldo_p0_0p3a_ana::EXT_LDO_P0_0P3A_ANA_SPEC>;
#[doc = "PMU_EXT_LDO_P0_0P3A_ANA"]
pub mod ext_ldo_p0_0p3a_ana;
#[doc = "EXT_LDO_P1_0P1A (rw) register accessor: PMU_EXT_LDO_P1_0P1A\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p1_0p1a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p1_0p1a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ldo_p1_0p1a`] module"]
pub type EXT_LDO_P1_0P1A = crate::Reg<ext_ldo_p1_0p1a::EXT_LDO_P1_0P1A_SPEC>;
#[doc = "PMU_EXT_LDO_P1_0P1A"]
pub mod ext_ldo_p1_0p1a;
#[doc = "EXT_LDO_P1_0P1A_ANA (rw) register accessor: PMU_EXT_LDO_P1_0P1A_ANA\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p1_0p1a_ana::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p1_0p1a_ana::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ldo_p1_0p1a_ana`] module"]
pub type EXT_LDO_P1_0P1A_ANA = crate::Reg<ext_ldo_p1_0p1a_ana::EXT_LDO_P1_0P1A_ANA_SPEC>;
#[doc = "PMU_EXT_LDO_P1_0P1A_ANA"]
pub mod ext_ldo_p1_0p1a_ana;
#[doc = "EXT_LDO_P1_0P2A (rw) register accessor: PMU_EXT_LDO_P1_0P2A\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p1_0p2a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p1_0p2a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ldo_p1_0p2a`] module"]
pub type EXT_LDO_P1_0P2A = crate::Reg<ext_ldo_p1_0p2a::EXT_LDO_P1_0P2A_SPEC>;
#[doc = "PMU_EXT_LDO_P1_0P2A"]
pub mod ext_ldo_p1_0p2a;
#[doc = "EXT_LDO_P1_0P2A_ANA (rw) register accessor: PMU_EXT_LDO_P1_0P2A_ANA\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p1_0p2a_ana::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p1_0p2a_ana::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ldo_p1_0p2a_ana`] module"]
pub type EXT_LDO_P1_0P2A_ANA = crate::Reg<ext_ldo_p1_0p2a_ana::EXT_LDO_P1_0P2A_ANA_SPEC>;
#[doc = "PMU_EXT_LDO_P1_0P2A_ANA"]
pub mod ext_ldo_p1_0p2a_ana;
#[doc = "EXT_LDO_P1_0P3A (rw) register accessor: PMU_EXT_LDO_P1_0P3A\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p1_0p3a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p1_0p3a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ldo_p1_0p3a`] module"]
pub type EXT_LDO_P1_0P3A = crate::Reg<ext_ldo_p1_0p3a::EXT_LDO_P1_0P3A_SPEC>;
#[doc = "PMU_EXT_LDO_P1_0P3A"]
pub mod ext_ldo_p1_0p3a;
#[doc = "EXT_LDO_P1_0P3A_ANA (rw) register accessor: PMU_EXT_LDO_P1_0P3A_ANA\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p1_0p3a_ana::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p1_0p3a_ana::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_ldo_p1_0p3a_ana`] module"]
pub type EXT_LDO_P1_0P3A_ANA = crate::Reg<ext_ldo_p1_0p3a_ana::EXT_LDO_P1_0P3A_ANA_SPEC>;
#[doc = "PMU_EXT_LDO_P1_0P3A_ANA"]
pub mod ext_ldo_p1_0p3a_ana;
#[doc = "EXT_WAKEUP_LV (rw) register accessor: PMU_EXT_WAKEUP_LV\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_wakeup_lv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_wakeup_lv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_wakeup_lv`] module"]
pub type EXT_WAKEUP_LV = crate::Reg<ext_wakeup_lv::EXT_WAKEUP_LV_SPEC>;
#[doc = "PMU_EXT_WAKEUP_LV"]
pub mod ext_wakeup_lv;
#[doc = "EXT_WAKEUP_SEL (rw) register accessor: PMU_EXT_WAKEUP_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_wakeup_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_wakeup_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_wakeup_sel`] module"]
pub type EXT_WAKEUP_SEL = crate::Reg<ext_wakeup_sel::EXT_WAKEUP_SEL_SPEC>;
#[doc = "PMU_EXT_WAKEUP_SEL"]
pub mod ext_wakeup_sel;
#[doc = "EXT_WAKEUP_ST (r) register accessor: PMU_EXT_WAKEUP_ST\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_wakeup_st::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_wakeup_st`] module"]
pub type EXT_WAKEUP_ST = crate::Reg<ext_wakeup_st::EXT_WAKEUP_ST_SPEC>;
#[doc = "PMU_EXT_WAKEUP_ST"]
pub mod ext_wakeup_st;
#[doc = "EXT_WAKEUP_CNTL (rw) register accessor: PMU_EXT_WAKEUP_CNTL\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_wakeup_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_wakeup_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_wakeup_cntl`] module"]
pub type EXT_WAKEUP_CNTL = crate::Reg<ext_wakeup_cntl::EXT_WAKEUP_CNTL_SPEC>;
#[doc = "PMU_EXT_WAKEUP_CNTL"]
pub mod ext_wakeup_cntl;
#[doc = "SDIO_WAKEUP_CNTL (rw) register accessor: PMU_SDIO_WAKEUP_CNTL\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_wakeup_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_wakeup_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_wakeup_cntl`] module"]
pub type SDIO_WAKEUP_CNTL = crate::Reg<sdio_wakeup_cntl::SDIO_WAKEUP_CNTL_SPEC>;
#[doc = "PMU_SDIO_WAKEUP_CNTL"]
pub mod sdio_wakeup_cntl;
#[doc = "XTAL_SLP (rw) register accessor: PMU_XTAL_SLP\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal_slp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal_slp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtal_slp`] module"]
pub type XTAL_SLP = crate::Reg<xtal_slp::XTAL_SLP_SPEC>;
#[doc = "PMU_XTAL_SLP"]
pub mod xtal_slp;
#[doc = "CPU_SW_STALL (rw) register accessor: PMU_CPU_SW_STALL\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_sw_stall::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_sw_stall::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpu_sw_stall`] module"]
pub type CPU_SW_STALL = crate::Reg<cpu_sw_stall::CPU_SW_STALL_SPEC>;
#[doc = "PMU_CPU_SW_STALL"]
pub mod cpu_sw_stall;
#[doc = "DCM_CTRL (rw) register accessor: PMU_DCM_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`dcm_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcm_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcm_ctrl`] module"]
pub type DCM_CTRL = crate::Reg<dcm_ctrl::DCM_CTRL_SPEC>;
#[doc = "PMU_DCM_CTRL"]
pub mod dcm_ctrl;
#[doc = "DCM_WAIT_DELAY (rw) register accessor: PMU_DCM_WAIT_DELAY\n\nYou can [`read`](crate::Reg::read) this register and get [`dcm_wait_delay::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcm_wait_delay::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcm_wait_delay`] module"]
pub type DCM_WAIT_DELAY = crate::Reg<dcm_wait_delay::DCM_WAIT_DELAY_SPEC>;
#[doc = "PMU_DCM_WAIT_DELAY"]
pub mod dcm_wait_delay;
#[doc = "VDDBAT_CFG (rw) register accessor: PMU_VDDBAT_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`vddbat_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vddbat_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vddbat_cfg`] module"]
pub type VDDBAT_CFG = crate::Reg<vddbat_cfg::VDDBAT_CFG_SPEC>;
#[doc = "PMU_VDDBAT_CFG"]
pub mod vddbat_cfg;
#[doc = "TOUCH_PWR_CNTL (rw) register accessor: PMU_TOUCH_PWR_CNTL\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_pwr_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_pwr_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@touch_pwr_cntl`] module"]
pub type TOUCH_PWR_CNTL = crate::Reg<touch_pwr_cntl::TOUCH_PWR_CNTL_SPEC>;
#[doc = "PMU_TOUCH_PWR_CNTL"]
pub mod touch_pwr_cntl;
#[doc = "RDN_ECO (rw) register accessor: PMU_RDN_ECO\n\nYou can [`read`](crate::Reg::read) this register and get [`rdn_eco::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdn_eco::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdn_eco`] module"]
pub type RDN_ECO = crate::Reg<rdn_eco::RDN_ECO_SPEC>;
#[doc = "PMU_RDN_ECO"]
pub mod rdn_eco;
#[doc = "POWER_PD_HP_CPU_CNTL (rw) register accessor: PMU_POWER_PD_HP_CPU_CNTL\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_hp_cpu_cntl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_hp_cpu_cntl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_pd_hp_cpu_cntl`] module"]
pub type POWER_PD_HP_CPU_CNTL = crate::Reg<power_pd_hp_cpu_cntl::POWER_PD_HP_CPU_CNTL_SPEC>;
#[doc = "PMU_POWER_PD_HP_CPU_CNTL"]
pub mod power_pd_hp_cpu_cntl;
#[doc = "POWER_PD_HP_CPU_MASK (rw) register accessor: PMU_POWER_PD_HP_CPU_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_hp_cpu_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_hp_cpu_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power_pd_hp_cpu_mask`] module"]
pub type POWER_PD_HP_CPU_MASK = crate::Reg<power_pd_hp_cpu_mask::POWER_PD_HP_CPU_MASK_SPEC>;
#[doc = "PMU_POWER_PD_HP_CPU_MASK"]
pub mod power_pd_hp_cpu_mask;
#[doc = "DATE (rw) register accessor: PMU_DATE\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "PMU_DATE"]
pub mod date;
