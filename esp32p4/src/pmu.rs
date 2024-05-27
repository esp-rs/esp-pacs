#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
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
    hp_sleep_lp_dcdc_reserve: HP_SLEEP_LP_DCDC_RESERVE,
    hp_sleep_lp_dig_power: HP_SLEEP_LP_DIG_POWER,
    hp_sleep_lp_ck_power: HP_SLEEP_LP_CK_POWER,
    lp_sleep_lp_bias_reserve: LP_SLEEP_LP_BIAS_RESERVE,
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
    _reserved134: [u8; 0x01e4],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - need_des
    #[inline(always)]
    pub const fn hp_active_dig_power(&self) -> &HP_ACTIVE_DIG_POWER {
        &self.hp_active_dig_power
    }
    ///0x04 - need_des
    #[inline(always)]
    pub const fn hp_active_icg_hp_func(&self) -> &HP_ACTIVE_ICG_HP_FUNC {
        &self.hp_active_icg_hp_func
    }
    ///0x08 - need_des
    #[inline(always)]
    pub const fn hp_active_icg_hp_apb(&self) -> &HP_ACTIVE_ICG_HP_APB {
        &self.hp_active_icg_hp_apb
    }
    ///0x0c - need_des
    #[inline(always)]
    pub const fn hp_active_icg_modem(&self) -> &HP_ACTIVE_ICG_MODEM {
        &self.hp_active_icg_modem
    }
    ///0x10 - need_des
    #[inline(always)]
    pub const fn hp_active_hp_sys_cntl(&self) -> &HP_ACTIVE_HP_SYS_CNTL {
        &self.hp_active_hp_sys_cntl
    }
    ///0x14 - need_des
    #[inline(always)]
    pub const fn hp_active_hp_ck_power(&self) -> &HP_ACTIVE_HP_CK_POWER {
        &self.hp_active_hp_ck_power
    }
    ///0x18 - need_des
    #[inline(always)]
    pub const fn hp_active_bias(&self) -> &HP_ACTIVE_BIAS {
        &self.hp_active_bias
    }
    ///0x1c - need_des
    #[inline(always)]
    pub const fn hp_active_backup(&self) -> &HP_ACTIVE_BACKUP {
        &self.hp_active_backup
    }
    ///0x20 - need_des
    #[inline(always)]
    pub const fn hp_active_backup_clk(&self) -> &HP_ACTIVE_BACKUP_CLK {
        &self.hp_active_backup_clk
    }
    ///0x24 - need_des
    #[inline(always)]
    pub const fn hp_active_sysclk(&self) -> &HP_ACTIVE_SYSCLK {
        &self.hp_active_sysclk
    }
    ///0x28 - need_des
    #[inline(always)]
    pub const fn hp_active_hp_regulator0(&self) -> &HP_ACTIVE_HP_REGULATOR0 {
        &self.hp_active_hp_regulator0
    }
    ///0x2c - need_des
    #[inline(always)]
    pub const fn hp_active_hp_regulator1(&self) -> &HP_ACTIVE_HP_REGULATOR1 {
        &self.hp_active_hp_regulator1
    }
    ///0x30 - need_des
    #[inline(always)]
    pub const fn hp_active_xtal(&self) -> &HP_ACTIVE_XTAL {
        &self.hp_active_xtal
    }
    ///0x34 - need_des
    #[inline(always)]
    pub const fn hp_modem_dig_power(&self) -> &HP_MODEM_DIG_POWER {
        &self.hp_modem_dig_power
    }
    ///0x38 - need_des
    #[inline(always)]
    pub const fn hp_modem_icg_hp_func(&self) -> &HP_MODEM_ICG_HP_FUNC {
        &self.hp_modem_icg_hp_func
    }
    ///0x3c - need_des
    #[inline(always)]
    pub const fn hp_modem_icg_hp_apb(&self) -> &HP_MODEM_ICG_HP_APB {
        &self.hp_modem_icg_hp_apb
    }
    ///0x40 - need_des
    #[inline(always)]
    pub const fn hp_modem_icg_modem(&self) -> &HP_MODEM_ICG_MODEM {
        &self.hp_modem_icg_modem
    }
    ///0x44 - need_des
    #[inline(always)]
    pub const fn hp_modem_hp_sys_cntl(&self) -> &HP_MODEM_HP_SYS_CNTL {
        &self.hp_modem_hp_sys_cntl
    }
    ///0x48 - need_des
    #[inline(always)]
    pub const fn hp_modem_hp_ck_power(&self) -> &HP_MODEM_HP_CK_POWER {
        &self.hp_modem_hp_ck_power
    }
    ///0x4c - need_des
    #[inline(always)]
    pub const fn hp_modem_bias(&self) -> &HP_MODEM_BIAS {
        &self.hp_modem_bias
    }
    ///0x50 - need_des
    #[inline(always)]
    pub const fn hp_modem_backup(&self) -> &HP_MODEM_BACKUP {
        &self.hp_modem_backup
    }
    ///0x54 - need_des
    #[inline(always)]
    pub const fn hp_modem_backup_clk(&self) -> &HP_MODEM_BACKUP_CLK {
        &self.hp_modem_backup_clk
    }
    ///0x58 - need_des
    #[inline(always)]
    pub const fn hp_modem_sysclk(&self) -> &HP_MODEM_SYSCLK {
        &self.hp_modem_sysclk
    }
    ///0x5c - need_des
    #[inline(always)]
    pub const fn hp_modem_hp_regulator0(&self) -> &HP_MODEM_HP_REGULATOR0 {
        &self.hp_modem_hp_regulator0
    }
    ///0x60 - need_des
    #[inline(always)]
    pub const fn hp_modem_hp_regulator1(&self) -> &HP_MODEM_HP_REGULATOR1 {
        &self.hp_modem_hp_regulator1
    }
    ///0x64 - need_des
    #[inline(always)]
    pub const fn hp_modem_xtal(&self) -> &HP_MODEM_XTAL {
        &self.hp_modem_xtal
    }
    ///0x68 - need_des
    #[inline(always)]
    pub const fn hp_sleep_dig_power(&self) -> &HP_SLEEP_DIG_POWER {
        &self.hp_sleep_dig_power
    }
    ///0x6c - need_des
    #[inline(always)]
    pub const fn hp_sleep_icg_hp_func(&self) -> &HP_SLEEP_ICG_HP_FUNC {
        &self.hp_sleep_icg_hp_func
    }
    ///0x70 - need_des
    #[inline(always)]
    pub const fn hp_sleep_icg_hp_apb(&self) -> &HP_SLEEP_ICG_HP_APB {
        &self.hp_sleep_icg_hp_apb
    }
    ///0x74 - need_des
    #[inline(always)]
    pub const fn hp_sleep_icg_modem(&self) -> &HP_SLEEP_ICG_MODEM {
        &self.hp_sleep_icg_modem
    }
    ///0x78 - need_des
    #[inline(always)]
    pub const fn hp_sleep_hp_sys_cntl(&self) -> &HP_SLEEP_HP_SYS_CNTL {
        &self.hp_sleep_hp_sys_cntl
    }
    ///0x7c - need_des
    #[inline(always)]
    pub const fn hp_sleep_hp_ck_power(&self) -> &HP_SLEEP_HP_CK_POWER {
        &self.hp_sleep_hp_ck_power
    }
    ///0x80 - need_des
    #[inline(always)]
    pub const fn hp_sleep_bias(&self) -> &HP_SLEEP_BIAS {
        &self.hp_sleep_bias
    }
    ///0x84 - need_des
    #[inline(always)]
    pub const fn hp_sleep_backup(&self) -> &HP_SLEEP_BACKUP {
        &self.hp_sleep_backup
    }
    ///0x88 - need_des
    #[inline(always)]
    pub const fn hp_sleep_backup_clk(&self) -> &HP_SLEEP_BACKUP_CLK {
        &self.hp_sleep_backup_clk
    }
    ///0x8c - need_des
    #[inline(always)]
    pub const fn hp_sleep_sysclk(&self) -> &HP_SLEEP_SYSCLK {
        &self.hp_sleep_sysclk
    }
    ///0x90 - need_des
    #[inline(always)]
    pub const fn hp_sleep_hp_regulator0(&self) -> &HP_SLEEP_HP_REGULATOR0 {
        &self.hp_sleep_hp_regulator0
    }
    ///0x94 - need_des
    #[inline(always)]
    pub const fn hp_sleep_hp_regulator1(&self) -> &HP_SLEEP_HP_REGULATOR1 {
        &self.hp_sleep_hp_regulator1
    }
    ///0x98 - need_des
    #[inline(always)]
    pub const fn hp_sleep_xtal(&self) -> &HP_SLEEP_XTAL {
        &self.hp_sleep_xtal
    }
    ///0x9c - need_des
    #[inline(always)]
    pub const fn hp_sleep_lp_regulator0(&self) -> &HP_SLEEP_LP_REGULATOR0 {
        &self.hp_sleep_lp_regulator0
    }
    ///0xa0 - need_des
    #[inline(always)]
    pub const fn hp_sleep_lp_regulator1(&self) -> &HP_SLEEP_LP_REGULATOR1 {
        &self.hp_sleep_lp_regulator1
    }
    ///0xa4 - need_des
    #[inline(always)]
    pub const fn hp_sleep_lp_dcdc_reserve(&self) -> &HP_SLEEP_LP_DCDC_RESERVE {
        &self.hp_sleep_lp_dcdc_reserve
    }
    ///0xa8 - need_des
    #[inline(always)]
    pub const fn hp_sleep_lp_dig_power(&self) -> &HP_SLEEP_LP_DIG_POWER {
        &self.hp_sleep_lp_dig_power
    }
    ///0xac - need_des
    #[inline(always)]
    pub const fn hp_sleep_lp_ck_power(&self) -> &HP_SLEEP_LP_CK_POWER {
        &self.hp_sleep_lp_ck_power
    }
    ///0xb0 - need_des
    #[inline(always)]
    pub const fn lp_sleep_lp_bias_reserve(&self) -> &LP_SLEEP_LP_BIAS_RESERVE {
        &self.lp_sleep_lp_bias_reserve
    }
    ///0xb4 - need_des
    #[inline(always)]
    pub const fn lp_sleep_lp_regulator0(&self) -> &LP_SLEEP_LP_REGULATOR0 {
        &self.lp_sleep_lp_regulator0
    }
    ///0xb8 - need_des
    #[inline(always)]
    pub const fn lp_sleep_lp_regulator1(&self) -> &LP_SLEEP_LP_REGULATOR1 {
        &self.lp_sleep_lp_regulator1
    }
    ///0xbc - need_des
    #[inline(always)]
    pub const fn lp_sleep_xtal(&self) -> &LP_SLEEP_XTAL {
        &self.lp_sleep_xtal
    }
    ///0xc0 - need_des
    #[inline(always)]
    pub const fn lp_sleep_lp_dig_power(&self) -> &LP_SLEEP_LP_DIG_POWER {
        &self.lp_sleep_lp_dig_power
    }
    ///0xc4 - need_des
    #[inline(always)]
    pub const fn lp_sleep_lp_ck_power(&self) -> &LP_SLEEP_LP_CK_POWER {
        &self.lp_sleep_lp_ck_power
    }
    ///0xc8 - need_des
    #[inline(always)]
    pub const fn lp_sleep_bias(&self) -> &LP_SLEEP_BIAS {
        &self.lp_sleep_bias
    }
    ///0xcc - need_des
    #[inline(always)]
    pub const fn imm_hp_ck_power(&self) -> &IMM_HP_CK_POWER {
        &self.imm_hp_ck_power
    }
    ///0xd0 - need_des
    #[inline(always)]
    pub const fn imm_sleep_sysclk(&self) -> &IMM_SLEEP_SYSCLK {
        &self.imm_sleep_sysclk
    }
    ///0xd4 - need_des
    #[inline(always)]
    pub const fn imm_hp_func_icg(&self) -> &IMM_HP_FUNC_ICG {
        &self.imm_hp_func_icg
    }
    ///0xd8 - need_des
    #[inline(always)]
    pub const fn imm_hp_apb_icg(&self) -> &IMM_HP_APB_ICG {
        &self.imm_hp_apb_icg
    }
    ///0xdc - need_des
    #[inline(always)]
    pub const fn imm_modem_icg(&self) -> &IMM_MODEM_ICG {
        &self.imm_modem_icg
    }
    ///0xe0 - need_des
    #[inline(always)]
    pub const fn imm_lp_icg(&self) -> &IMM_LP_ICG {
        &self.imm_lp_icg
    }
    ///0xe4 - need_des
    #[inline(always)]
    pub const fn imm_pad_hold_all(&self) -> &IMM_PAD_HOLD_ALL {
        &self.imm_pad_hold_all
    }
    ///0xe8 - need_des
    #[inline(always)]
    pub const fn imm_i2c_iso(&self) -> &IMM_I2C_ISO {
        &self.imm_i2c_iso
    }
    ///0xec - need_des
    #[inline(always)]
    pub const fn power_wait_timer0(&self) -> &POWER_WAIT_TIMER0 {
        &self.power_wait_timer0
    }
    ///0xf0 - need_des
    #[inline(always)]
    pub const fn power_wait_timer1(&self) -> &POWER_WAIT_TIMER1 {
        &self.power_wait_timer1
    }
    ///0xf4 - need_des
    #[inline(always)]
    pub const fn power_pd_top_cntl(&self) -> &POWER_PD_TOP_CNTL {
        &self.power_pd_top_cntl
    }
    ///0xf8 - need_des
    #[inline(always)]
    pub const fn power_pd_cnnt_cntl(&self) -> &POWER_PD_CNNT_CNTL {
        &self.power_pd_cnnt_cntl
    }
    ///0xfc - need_des
    #[inline(always)]
    pub const fn power_pd_hpmem_cntl(&self) -> &POWER_PD_HPMEM_CNTL {
        &self.power_pd_hpmem_cntl
    }
    ///0x100 - need_des
    #[inline(always)]
    pub const fn power_pd_top_mask(&self) -> &POWER_PD_TOP_MASK {
        &self.power_pd_top_mask
    }
    ///0x104 - need_des
    #[inline(always)]
    pub const fn power_pd_cnnt_mask(&self) -> &POWER_PD_CNNT_MASK {
        &self.power_pd_cnnt_mask
    }
    ///0x108 - need_des
    #[inline(always)]
    pub const fn power_pd_hpmem_mask(&self) -> &POWER_PD_HPMEM_MASK {
        &self.power_pd_hpmem_mask
    }
    ///0x10c - need_des
    #[inline(always)]
    pub const fn power_dcdc_switch(&self) -> &POWER_DCDC_SWITCH {
        &self.power_dcdc_switch
    }
    ///0x110 - need_des
    #[inline(always)]
    pub const fn power_pd_lpperi_cntl(&self) -> &POWER_PD_LPPERI_CNTL {
        &self.power_pd_lpperi_cntl
    }
    ///0x114 - need_des
    #[inline(always)]
    pub const fn power_pd_lpperi_mask(&self) -> &POWER_PD_LPPERI_MASK {
        &self.power_pd_lpperi_mask
    }
    ///0x118 - need_des
    #[inline(always)]
    pub const fn power_hp_pad(&self) -> &POWER_HP_PAD {
        &self.power_hp_pad
    }
    ///0x11c - need_des
    #[inline(always)]
    pub const fn power_ck_wait_cntl(&self) -> &POWER_CK_WAIT_CNTL {
        &self.power_ck_wait_cntl
    }
    ///0x120 - need_des
    #[inline(always)]
    pub const fn slp_wakeup_cntl0(&self) -> &SLP_WAKEUP_CNTL0 {
        &self.slp_wakeup_cntl0
    }
    ///0x124 - need_des
    #[inline(always)]
    pub const fn slp_wakeup_cntl1(&self) -> &SLP_WAKEUP_CNTL1 {
        &self.slp_wakeup_cntl1
    }
    ///0x128 - need_des
    #[inline(always)]
    pub const fn slp_wakeup_cntl2(&self) -> &SLP_WAKEUP_CNTL2 {
        &self.slp_wakeup_cntl2
    }
    ///0x12c - need_des
    #[inline(always)]
    pub const fn slp_wakeup_cntl3(&self) -> &SLP_WAKEUP_CNTL3 {
        &self.slp_wakeup_cntl3
    }
    ///0x130 - need_des
    #[inline(always)]
    pub const fn slp_wakeup_cntl4(&self) -> &SLP_WAKEUP_CNTL4 {
        &self.slp_wakeup_cntl4
    }
    ///0x134 - need_des
    #[inline(always)]
    pub const fn slp_wakeup_cntl5(&self) -> &SLP_WAKEUP_CNTL5 {
        &self.slp_wakeup_cntl5
    }
    ///0x138 - need_des
    #[inline(always)]
    pub const fn slp_wakeup_cntl6(&self) -> &SLP_WAKEUP_CNTL6 {
        &self.slp_wakeup_cntl6
    }
    ///0x13c - need_des
    #[inline(always)]
    pub const fn slp_wakeup_cntl7(&self) -> &SLP_WAKEUP_CNTL7 {
        &self.slp_wakeup_cntl7
    }
    ///0x140 - need_des
    #[inline(always)]
    pub const fn slp_wakeup_cntl8(&self) -> &SLP_WAKEUP_CNTL8 {
        &self.slp_wakeup_cntl8
    }
    ///0x144 - need_des
    #[inline(always)]
    pub const fn slp_wakeup_status0(&self) -> &SLP_WAKEUP_STATUS0 {
        &self.slp_wakeup_status0
    }
    ///0x148 - need_des
    #[inline(always)]
    pub const fn slp_wakeup_status1(&self) -> &SLP_WAKEUP_STATUS1 {
        &self.slp_wakeup_status1
    }
    ///0x14c - need_des
    #[inline(always)]
    pub const fn slp_wakeup_status2(&self) -> &SLP_WAKEUP_STATUS2 {
        &self.slp_wakeup_status2
    }
    ///0x150 - need_des
    #[inline(always)]
    pub const fn hp_ck_poweron(&self) -> &HP_CK_POWERON {
        &self.hp_ck_poweron
    }
    ///0x154 - need_des
    #[inline(always)]
    pub const fn hp_ck_cntl(&self) -> &HP_CK_CNTL {
        &self.hp_ck_cntl
    }
    ///0x158 - need_des
    #[inline(always)]
    pub const fn por_status(&self) -> &POR_STATUS {
        &self.por_status
    }
    ///0x15c - need_des
    #[inline(always)]
    pub const fn rf_pwc(&self) -> &RF_PWC {
        &self.rf_pwc
    }
    ///0x160 - need_des
    #[inline(always)]
    pub const fn backup_cfg(&self) -> &BACKUP_CFG {
        &self.backup_cfg
    }
    ///0x164 - need_des
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x168 - need_des
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x16c - need_des
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x170 - need_des
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x174 - need_des
    #[inline(always)]
    pub const fn lp_int_raw(&self) -> &LP_INT_RAW {
        &self.lp_int_raw
    }
    ///0x178 - need_des
    #[inline(always)]
    pub const fn lp_int_st(&self) -> &LP_INT_ST {
        &self.lp_int_st
    }
    ///0x17c - need_des
    #[inline(always)]
    pub const fn lp_int_ena(&self) -> &LP_INT_ENA {
        &self.lp_int_ena
    }
    ///0x180 - need_des
    #[inline(always)]
    pub const fn lp_int_clr(&self) -> &LP_INT_CLR {
        &self.lp_int_clr
    }
    ///0x184 - need_des
    #[inline(always)]
    pub const fn lp_cpu_pwr0(&self) -> &LP_CPU_PWR0 {
        &self.lp_cpu_pwr0
    }
    ///0x188 - need_des
    #[inline(always)]
    pub const fn lp_cpu_pwr1(&self) -> &LP_CPU_PWR1 {
        &self.lp_cpu_pwr1
    }
    ///0x18c - need_des
    #[inline(always)]
    pub const fn lp_cpu_pwr2(&self) -> &LP_CPU_PWR2 {
        &self.lp_cpu_pwr2
    }
    ///0x190 - need_des
    #[inline(always)]
    pub const fn lp_cpu_pwr3(&self) -> &LP_CPU_PWR3 {
        &self.lp_cpu_pwr3
    }
    ///0x194 - need_des
    #[inline(always)]
    pub const fn lp_cpu_pwr4(&self) -> &LP_CPU_PWR4 {
        &self.lp_cpu_pwr4
    }
    ///0x198 - need_des
    #[inline(always)]
    pub const fn lp_cpu_pwr5(&self) -> &LP_CPU_PWR5 {
        &self.lp_cpu_pwr5
    }
    ///0x19c - need_des
    #[inline(always)]
    pub const fn hp_lp_cpu_comm(&self) -> &HP_LP_CPU_COMM {
        &self.hp_lp_cpu_comm
    }
    ///0x1a0 - need_des
    #[inline(always)]
    pub const fn hp_regulator_cfg(&self) -> &HP_REGULATOR_CFG {
        &self.hp_regulator_cfg
    }
    ///0x1a4 - need_des
    #[inline(always)]
    pub const fn main_state(&self) -> &MAIN_STATE {
        &self.main_state
    }
    ///0x1a8 - need_des
    #[inline(always)]
    pub const fn pwr_state(&self) -> &PWR_STATE {
        &self.pwr_state
    }
    ///0x1ac - need_des
    #[inline(always)]
    pub const fn clk_state0(&self) -> &CLK_STATE0 {
        &self.clk_state0
    }
    ///0x1b0 - need_des
    #[inline(always)]
    pub const fn clk_state1(&self) -> &CLK_STATE1 {
        &self.clk_state1
    }
    ///0x1b4 - need_des
    #[inline(always)]
    pub const fn clk_state2(&self) -> &CLK_STATE2 {
        &self.clk_state2
    }
    ///0x1b8 - need_des
    #[inline(always)]
    pub const fn ext_ldo_p0_0p1a(&self) -> &EXT_LDO_P0_0P1A {
        &self.ext_ldo_p0_0p1a
    }
    ///0x1bc - need_des
    #[inline(always)]
    pub const fn ext_ldo_p0_0p1a_ana(&self) -> &EXT_LDO_P0_0P1A_ANA {
        &self.ext_ldo_p0_0p1a_ana
    }
    ///0x1c0 - need_des
    #[inline(always)]
    pub const fn ext_ldo_p0_0p2a(&self) -> &EXT_LDO_P0_0P2A {
        &self.ext_ldo_p0_0p2a
    }
    ///0x1c4 - need_des
    #[inline(always)]
    pub const fn ext_ldo_p0_0p2a_ana(&self) -> &EXT_LDO_P0_0P2A_ANA {
        &self.ext_ldo_p0_0p2a_ana
    }
    ///0x1c8 - need_des
    #[inline(always)]
    pub const fn ext_ldo_p0_0p3a(&self) -> &EXT_LDO_P0_0P3A {
        &self.ext_ldo_p0_0p3a
    }
    ///0x1cc - need_des
    #[inline(always)]
    pub const fn ext_ldo_p0_0p3a_ana(&self) -> &EXT_LDO_P0_0P3A_ANA {
        &self.ext_ldo_p0_0p3a_ana
    }
    ///0x1d0 - need_des
    #[inline(always)]
    pub const fn ext_ldo_p1_0p1a(&self) -> &EXT_LDO_P1_0P1A {
        &self.ext_ldo_p1_0p1a
    }
    ///0x1d4 - need_des
    #[inline(always)]
    pub const fn ext_ldo_p1_0p1a_ana(&self) -> &EXT_LDO_P1_0P1A_ANA {
        &self.ext_ldo_p1_0p1a_ana
    }
    ///0x1d8 - need_des
    #[inline(always)]
    pub const fn ext_ldo_p1_0p2a(&self) -> &EXT_LDO_P1_0P2A {
        &self.ext_ldo_p1_0p2a
    }
    ///0x1dc - need_des
    #[inline(always)]
    pub const fn ext_ldo_p1_0p2a_ana(&self) -> &EXT_LDO_P1_0P2A_ANA {
        &self.ext_ldo_p1_0p2a_ana
    }
    ///0x1e0 - need_des
    #[inline(always)]
    pub const fn ext_ldo_p1_0p3a(&self) -> &EXT_LDO_P1_0P3A {
        &self.ext_ldo_p1_0p3a
    }
    ///0x1e4 - need_des
    #[inline(always)]
    pub const fn ext_ldo_p1_0p3a_ana(&self) -> &EXT_LDO_P1_0P3A_ANA {
        &self.ext_ldo_p1_0p3a_ana
    }
    ///0x1e8 - need_des
    #[inline(always)]
    pub const fn ext_wakeup_lv(&self) -> &EXT_WAKEUP_LV {
        &self.ext_wakeup_lv
    }
    ///0x1ec - need_des
    #[inline(always)]
    pub const fn ext_wakeup_sel(&self) -> &EXT_WAKEUP_SEL {
        &self.ext_wakeup_sel
    }
    ///0x1f0 - need_des
    #[inline(always)]
    pub const fn ext_wakeup_st(&self) -> &EXT_WAKEUP_ST {
        &self.ext_wakeup_st
    }
    ///0x1f4 - need_des
    #[inline(always)]
    pub const fn ext_wakeup_cntl(&self) -> &EXT_WAKEUP_CNTL {
        &self.ext_wakeup_cntl
    }
    ///0x1f8 - need_des
    #[inline(always)]
    pub const fn sdio_wakeup_cntl(&self) -> &SDIO_WAKEUP_CNTL {
        &self.sdio_wakeup_cntl
    }
    ///0x1fc - need_des
    #[inline(always)]
    pub const fn xtal_slp(&self) -> &XTAL_SLP {
        &self.xtal_slp
    }
    ///0x200 - need_des
    #[inline(always)]
    pub const fn cpu_sw_stall(&self) -> &CPU_SW_STALL {
        &self.cpu_sw_stall
    }
    ///0x204 - need_des
    #[inline(always)]
    pub const fn dcm_ctrl(&self) -> &DCM_CTRL {
        &self.dcm_ctrl
    }
    ///0x208 - need_des
    #[inline(always)]
    pub const fn dcm_wait_delay(&self) -> &DCM_WAIT_DELAY {
        &self.dcm_wait_delay
    }
    ///0x20c - need_des
    #[inline(always)]
    pub const fn vddbat_cfg(&self) -> &VDDBAT_CFG {
        &self.vddbat_cfg
    }
    ///0x210 - need_des
    #[inline(always)]
    pub const fn touch_pwr_cntl(&self) -> &TOUCH_PWR_CNTL {
        &self.touch_pwr_cntl
    }
    ///0x214 - need_des
    #[inline(always)]
    pub const fn rdn_eco(&self) -> &RDN_ECO {
        &self.rdn_eco
    }
    ///0x3fc - need_des
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**HP_ACTIVE_DIG_POWER (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_active_dig_power::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_active_dig_power::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_active_dig_power`] module*/
pub type HP_ACTIVE_DIG_POWER = crate::Reg<hp_active_dig_power::HP_ACTIVE_DIG_POWER_SPEC>;
///need_des
pub mod hp_active_dig_power;
/**HP_ACTIVE_ICG_HP_FUNC (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_active_icg_hp_func::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_active_icg_hp_func::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_active_icg_hp_func`] module*/
pub type HP_ACTIVE_ICG_HP_FUNC = crate::Reg<hp_active_icg_hp_func::HP_ACTIVE_ICG_HP_FUNC_SPEC>;
///need_des
pub mod hp_active_icg_hp_func;
/**HP_ACTIVE_ICG_HP_APB (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_active_icg_hp_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_active_icg_hp_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_active_icg_hp_apb`] module*/
pub type HP_ACTIVE_ICG_HP_APB = crate::Reg<hp_active_icg_hp_apb::HP_ACTIVE_ICG_HP_APB_SPEC>;
///need_des
pub mod hp_active_icg_hp_apb;
/**HP_ACTIVE_ICG_MODEM (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_active_icg_modem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_active_icg_modem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_active_icg_modem`] module*/
pub type HP_ACTIVE_ICG_MODEM = crate::Reg<hp_active_icg_modem::HP_ACTIVE_ICG_MODEM_SPEC>;
///need_des
pub mod hp_active_icg_modem;
/**HP_ACTIVE_HP_SYS_CNTL (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_active_hp_sys_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_active_hp_sys_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_active_hp_sys_cntl`] module*/
pub type HP_ACTIVE_HP_SYS_CNTL = crate::Reg<hp_active_hp_sys_cntl::HP_ACTIVE_HP_SYS_CNTL_SPEC>;
///need_des
pub mod hp_active_hp_sys_cntl;
/**HP_ACTIVE_HP_CK_POWER (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_active_hp_ck_power::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_active_hp_ck_power::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_active_hp_ck_power`] module*/
pub type HP_ACTIVE_HP_CK_POWER = crate::Reg<hp_active_hp_ck_power::HP_ACTIVE_HP_CK_POWER_SPEC>;
///need_des
pub mod hp_active_hp_ck_power;
/**HP_ACTIVE_BIAS (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_active_bias::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_active_bias::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_active_bias`] module*/
pub type HP_ACTIVE_BIAS = crate::Reg<hp_active_bias::HP_ACTIVE_BIAS_SPEC>;
///need_des
pub mod hp_active_bias;
/**HP_ACTIVE_BACKUP (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_active_backup::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_active_backup::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_active_backup`] module*/
pub type HP_ACTIVE_BACKUP = crate::Reg<hp_active_backup::HP_ACTIVE_BACKUP_SPEC>;
///need_des
pub mod hp_active_backup;
/**HP_ACTIVE_BACKUP_CLK (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_active_backup_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_active_backup_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_active_backup_clk`] module*/
pub type HP_ACTIVE_BACKUP_CLK = crate::Reg<hp_active_backup_clk::HP_ACTIVE_BACKUP_CLK_SPEC>;
///need_des
pub mod hp_active_backup_clk;
/**HP_ACTIVE_SYSCLK (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_active_sysclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_active_sysclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_active_sysclk`] module*/
pub type HP_ACTIVE_SYSCLK = crate::Reg<hp_active_sysclk::HP_ACTIVE_SYSCLK_SPEC>;
///need_des
pub mod hp_active_sysclk;
/**HP_ACTIVE_HP_REGULATOR0 (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_active_hp_regulator0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_active_hp_regulator0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_active_hp_regulator0`] module*/
pub type HP_ACTIVE_HP_REGULATOR0 =
    crate::Reg<hp_active_hp_regulator0::HP_ACTIVE_HP_REGULATOR0_SPEC>;
///need_des
pub mod hp_active_hp_regulator0;
/**HP_ACTIVE_HP_REGULATOR1 (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_active_hp_regulator1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_active_hp_regulator1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_active_hp_regulator1`] module*/
pub type HP_ACTIVE_HP_REGULATOR1 =
    crate::Reg<hp_active_hp_regulator1::HP_ACTIVE_HP_REGULATOR1_SPEC>;
///need_des
pub mod hp_active_hp_regulator1;
/**HP_ACTIVE_XTAL (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_active_xtal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_active_xtal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_active_xtal`] module*/
pub type HP_ACTIVE_XTAL = crate::Reg<hp_active_xtal::HP_ACTIVE_XTAL_SPEC>;
///need_des
pub mod hp_active_xtal;
/**HP_MODEM_DIG_POWER (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_modem_dig_power::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_modem_dig_power::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_modem_dig_power`] module*/
pub type HP_MODEM_DIG_POWER = crate::Reg<hp_modem_dig_power::HP_MODEM_DIG_POWER_SPEC>;
///need_des
pub mod hp_modem_dig_power;
/**HP_MODEM_ICG_HP_FUNC (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_modem_icg_hp_func::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_modem_icg_hp_func`] module*/
pub type HP_MODEM_ICG_HP_FUNC = crate::Reg<hp_modem_icg_hp_func::HP_MODEM_ICG_HP_FUNC_SPEC>;
///need_des
pub mod hp_modem_icg_hp_func;
/**HP_MODEM_ICG_HP_APB (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_modem_icg_hp_apb::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_modem_icg_hp_apb`] module*/
pub type HP_MODEM_ICG_HP_APB = crate::Reg<hp_modem_icg_hp_apb::HP_MODEM_ICG_HP_APB_SPEC>;
///need_des
pub mod hp_modem_icg_hp_apb;
/**HP_MODEM_ICG_MODEM (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_modem_icg_modem::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_modem_icg_modem`] module*/
pub type HP_MODEM_ICG_MODEM = crate::Reg<hp_modem_icg_modem::HP_MODEM_ICG_MODEM_SPEC>;
///need_des
pub mod hp_modem_icg_modem;
/**HP_MODEM_HP_SYS_CNTL (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_modem_hp_sys_cntl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_modem_hp_sys_cntl`] module*/
pub type HP_MODEM_HP_SYS_CNTL = crate::Reg<hp_modem_hp_sys_cntl::HP_MODEM_HP_SYS_CNTL_SPEC>;
///need_des
pub mod hp_modem_hp_sys_cntl;
/**HP_MODEM_HP_CK_POWER (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_modem_hp_ck_power::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_modem_hp_ck_power`] module*/
pub type HP_MODEM_HP_CK_POWER = crate::Reg<hp_modem_hp_ck_power::HP_MODEM_HP_CK_POWER_SPEC>;
///need_des
pub mod hp_modem_hp_ck_power;
/**HP_MODEM_BIAS (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_modem_bias::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_modem_bias`] module*/
pub type HP_MODEM_BIAS = crate::Reg<hp_modem_bias::HP_MODEM_BIAS_SPEC>;
///need_des
pub mod hp_modem_bias;
/**HP_MODEM_BACKUP (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_modem_backup::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_modem_backup`] module*/
pub type HP_MODEM_BACKUP = crate::Reg<hp_modem_backup::HP_MODEM_BACKUP_SPEC>;
///need_des
pub mod hp_modem_backup;
/**HP_MODEM_BACKUP_CLK (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_modem_backup_clk::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_modem_backup_clk`] module*/
pub type HP_MODEM_BACKUP_CLK = crate::Reg<hp_modem_backup_clk::HP_MODEM_BACKUP_CLK_SPEC>;
///need_des
pub mod hp_modem_backup_clk;
/**HP_MODEM_SYSCLK (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_modem_sysclk::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_modem_sysclk`] module*/
pub type HP_MODEM_SYSCLK = crate::Reg<hp_modem_sysclk::HP_MODEM_SYSCLK_SPEC>;
///need_des
pub mod hp_modem_sysclk;
/**HP_MODEM_HP_REGULATOR0 (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_modem_hp_regulator0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_modem_hp_regulator0`] module*/
pub type HP_MODEM_HP_REGULATOR0 = crate::Reg<hp_modem_hp_regulator0::HP_MODEM_HP_REGULATOR0_SPEC>;
///need_des
pub mod hp_modem_hp_regulator0;
/**HP_MODEM_HP_REGULATOR1 (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_modem_hp_regulator1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_modem_hp_regulator1`] module*/
pub type HP_MODEM_HP_REGULATOR1 = crate::Reg<hp_modem_hp_regulator1::HP_MODEM_HP_REGULATOR1_SPEC>;
///need_des
pub mod hp_modem_hp_regulator1;
/**HP_MODEM_XTAL (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_modem_xtal::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_modem_xtal`] module*/
pub type HP_MODEM_XTAL = crate::Reg<hp_modem_xtal::HP_MODEM_XTAL_SPEC>;
///need_des
pub mod hp_modem_xtal;
/**HP_SLEEP_DIG_POWER (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_sleep_dig_power::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_sleep_dig_power::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_sleep_dig_power`] module*/
pub type HP_SLEEP_DIG_POWER = crate::Reg<hp_sleep_dig_power::HP_SLEEP_DIG_POWER_SPEC>;
///need_des
pub mod hp_sleep_dig_power;
/**HP_SLEEP_ICG_HP_FUNC (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_sleep_icg_hp_func::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_sleep_icg_hp_func::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_sleep_icg_hp_func`] module*/
pub type HP_SLEEP_ICG_HP_FUNC = crate::Reg<hp_sleep_icg_hp_func::HP_SLEEP_ICG_HP_FUNC_SPEC>;
///need_des
pub mod hp_sleep_icg_hp_func;
/**HP_SLEEP_ICG_HP_APB (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_sleep_icg_hp_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_sleep_icg_hp_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_sleep_icg_hp_apb`] module*/
pub type HP_SLEEP_ICG_HP_APB = crate::Reg<hp_sleep_icg_hp_apb::HP_SLEEP_ICG_HP_APB_SPEC>;
///need_des
pub mod hp_sleep_icg_hp_apb;
/**HP_SLEEP_ICG_MODEM (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_sleep_icg_modem::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_sleep_icg_modem::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_sleep_icg_modem`] module*/
pub type HP_SLEEP_ICG_MODEM = crate::Reg<hp_sleep_icg_modem::HP_SLEEP_ICG_MODEM_SPEC>;
///need_des
pub mod hp_sleep_icg_modem;
/**HP_SLEEP_HP_SYS_CNTL (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_sleep_hp_sys_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_sleep_hp_sys_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_sleep_hp_sys_cntl`] module*/
pub type HP_SLEEP_HP_SYS_CNTL = crate::Reg<hp_sleep_hp_sys_cntl::HP_SLEEP_HP_SYS_CNTL_SPEC>;
///need_des
pub mod hp_sleep_hp_sys_cntl;
/**HP_SLEEP_HP_CK_POWER (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_sleep_hp_ck_power::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_sleep_hp_ck_power::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_sleep_hp_ck_power`] module*/
pub type HP_SLEEP_HP_CK_POWER = crate::Reg<hp_sleep_hp_ck_power::HP_SLEEP_HP_CK_POWER_SPEC>;
///need_des
pub mod hp_sleep_hp_ck_power;
/**HP_SLEEP_BIAS (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_sleep_bias::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_sleep_bias::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_sleep_bias`] module*/
pub type HP_SLEEP_BIAS = crate::Reg<hp_sleep_bias::HP_SLEEP_BIAS_SPEC>;
///need_des
pub mod hp_sleep_bias;
/**HP_SLEEP_BACKUP (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_sleep_backup::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_sleep_backup::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_sleep_backup`] module*/
pub type HP_SLEEP_BACKUP = crate::Reg<hp_sleep_backup::HP_SLEEP_BACKUP_SPEC>;
///need_des
pub mod hp_sleep_backup;
/**HP_SLEEP_BACKUP_CLK (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_sleep_backup_clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_sleep_backup_clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_sleep_backup_clk`] module*/
pub type HP_SLEEP_BACKUP_CLK = crate::Reg<hp_sleep_backup_clk::HP_SLEEP_BACKUP_CLK_SPEC>;
///need_des
pub mod hp_sleep_backup_clk;
/**HP_SLEEP_SYSCLK (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_sleep_sysclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_sleep_sysclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_sleep_sysclk`] module*/
pub type HP_SLEEP_SYSCLK = crate::Reg<hp_sleep_sysclk::HP_SLEEP_SYSCLK_SPEC>;
///need_des
pub mod hp_sleep_sysclk;
/**HP_SLEEP_HP_REGULATOR0 (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_sleep_hp_regulator0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_sleep_hp_regulator0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_sleep_hp_regulator0`] module*/
pub type HP_SLEEP_HP_REGULATOR0 = crate::Reg<hp_sleep_hp_regulator0::HP_SLEEP_HP_REGULATOR0_SPEC>;
///need_des
pub mod hp_sleep_hp_regulator0;
/**HP_SLEEP_HP_REGULATOR1 (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_sleep_hp_regulator1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_sleep_hp_regulator1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_sleep_hp_regulator1`] module*/
pub type HP_SLEEP_HP_REGULATOR1 = crate::Reg<hp_sleep_hp_regulator1::HP_SLEEP_HP_REGULATOR1_SPEC>;
///need_des
pub mod hp_sleep_hp_regulator1;
/**HP_SLEEP_XTAL (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_sleep_xtal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_sleep_xtal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_sleep_xtal`] module*/
pub type HP_SLEEP_XTAL = crate::Reg<hp_sleep_xtal::HP_SLEEP_XTAL_SPEC>;
///need_des
pub mod hp_sleep_xtal;
/**HP_SLEEP_LP_REGULATOR0 (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_sleep_lp_regulator0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_sleep_lp_regulator0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_sleep_lp_regulator0`] module*/
pub type HP_SLEEP_LP_REGULATOR0 = crate::Reg<hp_sleep_lp_regulator0::HP_SLEEP_LP_REGULATOR0_SPEC>;
///need_des
pub mod hp_sleep_lp_regulator0;
/**HP_SLEEP_LP_REGULATOR1 (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_sleep_lp_regulator1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_sleep_lp_regulator1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_sleep_lp_regulator1`] module*/
pub type HP_SLEEP_LP_REGULATOR1 = crate::Reg<hp_sleep_lp_regulator1::HP_SLEEP_LP_REGULATOR1_SPEC>;
///need_des
pub mod hp_sleep_lp_regulator1;
/**HP_SLEEP_LP_DCDC_RESERVE (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_sleep_lp_dcdc_reserve::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_sleep_lp_dcdc_reserve`] module*/
pub type HP_SLEEP_LP_DCDC_RESERVE =
    crate::Reg<hp_sleep_lp_dcdc_reserve::HP_SLEEP_LP_DCDC_RESERVE_SPEC>;
///need_des
pub mod hp_sleep_lp_dcdc_reserve;
/**HP_SLEEP_LP_DIG_POWER (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_sleep_lp_dig_power::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_sleep_lp_dig_power::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_sleep_lp_dig_power`] module*/
pub type HP_SLEEP_LP_DIG_POWER = crate::Reg<hp_sleep_lp_dig_power::HP_SLEEP_LP_DIG_POWER_SPEC>;
///need_des
pub mod hp_sleep_lp_dig_power;
/**HP_SLEEP_LP_CK_POWER (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_sleep_lp_ck_power::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_sleep_lp_ck_power::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_sleep_lp_ck_power`] module*/
pub type HP_SLEEP_LP_CK_POWER = crate::Reg<hp_sleep_lp_ck_power::HP_SLEEP_LP_CK_POWER_SPEC>;
///need_des
pub mod hp_sleep_lp_ck_power;
/**LP_SLEEP_LP_BIAS_RESERVE (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_sleep_lp_bias_reserve::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_sleep_lp_bias_reserve`] module*/
pub type LP_SLEEP_LP_BIAS_RESERVE =
    crate::Reg<lp_sleep_lp_bias_reserve::LP_SLEEP_LP_BIAS_RESERVE_SPEC>;
///need_des
pub mod lp_sleep_lp_bias_reserve;
/**LP_SLEEP_LP_REGULATOR0 (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_sleep_lp_regulator0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_sleep_lp_regulator0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_sleep_lp_regulator0`] module*/
pub type LP_SLEEP_LP_REGULATOR0 = crate::Reg<lp_sleep_lp_regulator0::LP_SLEEP_LP_REGULATOR0_SPEC>;
///need_des
pub mod lp_sleep_lp_regulator0;
/**LP_SLEEP_LP_REGULATOR1 (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_sleep_lp_regulator1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_sleep_lp_regulator1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_sleep_lp_regulator1`] module*/
pub type LP_SLEEP_LP_REGULATOR1 = crate::Reg<lp_sleep_lp_regulator1::LP_SLEEP_LP_REGULATOR1_SPEC>;
///need_des
pub mod lp_sleep_lp_regulator1;
/**LP_SLEEP_XTAL (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_sleep_xtal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_sleep_xtal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_sleep_xtal`] module*/
pub type LP_SLEEP_XTAL = crate::Reg<lp_sleep_xtal::LP_SLEEP_XTAL_SPEC>;
///need_des
pub mod lp_sleep_xtal;
/**LP_SLEEP_LP_DIG_POWER (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_sleep_lp_dig_power::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_sleep_lp_dig_power::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_sleep_lp_dig_power`] module*/
pub type LP_SLEEP_LP_DIG_POWER = crate::Reg<lp_sleep_lp_dig_power::LP_SLEEP_LP_DIG_POWER_SPEC>;
///need_des
pub mod lp_sleep_lp_dig_power;
/**LP_SLEEP_LP_CK_POWER (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_sleep_lp_ck_power::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_sleep_lp_ck_power::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_sleep_lp_ck_power`] module*/
pub type LP_SLEEP_LP_CK_POWER = crate::Reg<lp_sleep_lp_ck_power::LP_SLEEP_LP_CK_POWER_SPEC>;
///need_des
pub mod lp_sleep_lp_ck_power;
/**LP_SLEEP_BIAS (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_sleep_bias::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_sleep_bias::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_sleep_bias`] module*/
pub type LP_SLEEP_BIAS = crate::Reg<lp_sleep_bias::LP_SLEEP_BIAS_SPEC>;
///need_des
pub mod lp_sleep_bias;
/**IMM_HP_CK_POWER (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`imm_hp_ck_power::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imm_hp_ck_power::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@imm_hp_ck_power`] module*/
pub type IMM_HP_CK_POWER = crate::Reg<imm_hp_ck_power::IMM_HP_CK_POWER_SPEC>;
///need_des
pub mod imm_hp_ck_power;
/**IMM_SLEEP_SYSCLK (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imm_sleep_sysclk::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@imm_sleep_sysclk`] module*/
pub type IMM_SLEEP_SYSCLK = crate::Reg<imm_sleep_sysclk::IMM_SLEEP_SYSCLK_SPEC>;
///need_des
pub mod imm_sleep_sysclk;
/**IMM_HP_FUNC_ICG (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imm_hp_func_icg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@imm_hp_func_icg`] module*/
pub type IMM_HP_FUNC_ICG = crate::Reg<imm_hp_func_icg::IMM_HP_FUNC_ICG_SPEC>;
///need_des
pub mod imm_hp_func_icg;
/**IMM_HP_APB_ICG (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imm_hp_apb_icg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@imm_hp_apb_icg`] module*/
pub type IMM_HP_APB_ICG = crate::Reg<imm_hp_apb_icg::IMM_HP_APB_ICG_SPEC>;
///need_des
pub mod imm_hp_apb_icg;
/**IMM_MODEM_ICG (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imm_modem_icg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@imm_modem_icg`] module*/
pub type IMM_MODEM_ICG = crate::Reg<imm_modem_icg::IMM_MODEM_ICG_SPEC>;
///need_des
pub mod imm_modem_icg;
/**IMM_LP_ICG (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imm_lp_icg::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@imm_lp_icg`] module*/
pub type IMM_LP_ICG = crate::Reg<imm_lp_icg::IMM_LP_ICG_SPEC>;
///need_des
pub mod imm_lp_icg;
/**IMM_PAD_HOLD_ALL (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`imm_pad_hold_all::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imm_pad_hold_all::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@imm_pad_hold_all`] module*/
pub type IMM_PAD_HOLD_ALL = crate::Reg<imm_pad_hold_all::IMM_PAD_HOLD_ALL_SPEC>;
///need_des
pub mod imm_pad_hold_all;
/**IMM_I2C_ISO (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imm_i2c_iso::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@imm_i2c_iso`] module*/
pub type IMM_I2C_ISO = crate::Reg<imm_i2c_iso::IMM_I2C_ISO_SPEC>;
///need_des
pub mod imm_i2c_iso;
/**POWER_WAIT_TIMER0 (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`power_wait_timer0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_wait_timer0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@power_wait_timer0`] module*/
pub type POWER_WAIT_TIMER0 = crate::Reg<power_wait_timer0::POWER_WAIT_TIMER0_SPEC>;
///need_des
pub mod power_wait_timer0;
/**POWER_WAIT_TIMER1 (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`power_wait_timer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_wait_timer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@power_wait_timer1`] module*/
pub type POWER_WAIT_TIMER1 = crate::Reg<power_wait_timer1::POWER_WAIT_TIMER1_SPEC>;
///need_des
pub mod power_wait_timer1;
/**POWER_PD_TOP_CNTL (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`power_pd_top_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_pd_top_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@power_pd_top_cntl`] module*/
pub type POWER_PD_TOP_CNTL = crate::Reg<power_pd_top_cntl::POWER_PD_TOP_CNTL_SPEC>;
///need_des
pub mod power_pd_top_cntl;
/**POWER_PD_CNNT_CNTL (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`power_pd_cnnt_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_pd_cnnt_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@power_pd_cnnt_cntl`] module*/
pub type POWER_PD_CNNT_CNTL = crate::Reg<power_pd_cnnt_cntl::POWER_PD_CNNT_CNTL_SPEC>;
///need_des
pub mod power_pd_cnnt_cntl;
/**POWER_PD_HPMEM_CNTL (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`power_pd_hpmem_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_pd_hpmem_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@power_pd_hpmem_cntl`] module*/
pub type POWER_PD_HPMEM_CNTL = crate::Reg<power_pd_hpmem_cntl::POWER_PD_HPMEM_CNTL_SPEC>;
///need_des
pub mod power_pd_hpmem_cntl;
/**POWER_PD_TOP_MASK (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`power_pd_top_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_pd_top_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@power_pd_top_mask`] module*/
pub type POWER_PD_TOP_MASK = crate::Reg<power_pd_top_mask::POWER_PD_TOP_MASK_SPEC>;
///need_des
pub mod power_pd_top_mask;
/**POWER_PD_CNNT_MASK (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`power_pd_cnnt_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_pd_cnnt_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@power_pd_cnnt_mask`] module*/
pub type POWER_PD_CNNT_MASK = crate::Reg<power_pd_cnnt_mask::POWER_PD_CNNT_MASK_SPEC>;
///need_des
pub mod power_pd_cnnt_mask;
/**POWER_PD_HPMEM_MASK (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`power_pd_hpmem_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_pd_hpmem_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@power_pd_hpmem_mask`] module*/
pub type POWER_PD_HPMEM_MASK = crate::Reg<power_pd_hpmem_mask::POWER_PD_HPMEM_MASK_SPEC>;
///need_des
pub mod power_pd_hpmem_mask;
/**POWER_DCDC_SWITCH (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`power_dcdc_switch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_dcdc_switch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@power_dcdc_switch`] module*/
pub type POWER_DCDC_SWITCH = crate::Reg<power_dcdc_switch::POWER_DCDC_SWITCH_SPEC>;
///need_des
pub mod power_dcdc_switch;
/**POWER_PD_LPPERI_CNTL (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`power_pd_lpperi_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_pd_lpperi_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@power_pd_lpperi_cntl`] module*/
pub type POWER_PD_LPPERI_CNTL = crate::Reg<power_pd_lpperi_cntl::POWER_PD_LPPERI_CNTL_SPEC>;
///need_des
pub mod power_pd_lpperi_cntl;
/**POWER_PD_LPPERI_MASK (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`power_pd_lpperi_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_pd_lpperi_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@power_pd_lpperi_mask`] module*/
pub type POWER_PD_LPPERI_MASK = crate::Reg<power_pd_lpperi_mask::POWER_PD_LPPERI_MASK_SPEC>;
///need_des
pub mod power_pd_lpperi_mask;
/**POWER_HP_PAD (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`power_hp_pad::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_hp_pad::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@power_hp_pad`] module*/
pub type POWER_HP_PAD = crate::Reg<power_hp_pad::POWER_HP_PAD_SPEC>;
///need_des
pub mod power_hp_pad;
/**POWER_CK_WAIT_CNTL (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`power_ck_wait_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`power_ck_wait_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@power_ck_wait_cntl`] module*/
pub type POWER_CK_WAIT_CNTL = crate::Reg<power_ck_wait_cntl::POWER_CK_WAIT_CNTL_SPEC>;
///need_des
pub mod power_ck_wait_cntl;
/**SLP_WAKEUP_CNTL0 (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_wakeup_cntl0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slp_wakeup_cntl0`] module*/
pub type SLP_WAKEUP_CNTL0 = crate::Reg<slp_wakeup_cntl0::SLP_WAKEUP_CNTL0_SPEC>;
///need_des
pub mod slp_wakeup_cntl0;
/**SLP_WAKEUP_CNTL1 (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`slp_wakeup_cntl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_wakeup_cntl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slp_wakeup_cntl1`] module*/
pub type SLP_WAKEUP_CNTL1 = crate::Reg<slp_wakeup_cntl1::SLP_WAKEUP_CNTL1_SPEC>;
///need_des
pub mod slp_wakeup_cntl1;
/**SLP_WAKEUP_CNTL2 (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`slp_wakeup_cntl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_wakeup_cntl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slp_wakeup_cntl2`] module*/
pub type SLP_WAKEUP_CNTL2 = crate::Reg<slp_wakeup_cntl2::SLP_WAKEUP_CNTL2_SPEC>;
///need_des
pub mod slp_wakeup_cntl2;
/**SLP_WAKEUP_CNTL3 (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`slp_wakeup_cntl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_wakeup_cntl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slp_wakeup_cntl3`] module*/
pub type SLP_WAKEUP_CNTL3 = crate::Reg<slp_wakeup_cntl3::SLP_WAKEUP_CNTL3_SPEC>;
///need_des
pub mod slp_wakeup_cntl3;
/**SLP_WAKEUP_CNTL4 (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_wakeup_cntl4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slp_wakeup_cntl4`] module*/
pub type SLP_WAKEUP_CNTL4 = crate::Reg<slp_wakeup_cntl4::SLP_WAKEUP_CNTL4_SPEC>;
///need_des
pub mod slp_wakeup_cntl4;
/**SLP_WAKEUP_CNTL5 (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`slp_wakeup_cntl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_wakeup_cntl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slp_wakeup_cntl5`] module*/
pub type SLP_WAKEUP_CNTL5 = crate::Reg<slp_wakeup_cntl5::SLP_WAKEUP_CNTL5_SPEC>;
///need_des
pub mod slp_wakeup_cntl5;
/**SLP_WAKEUP_CNTL6 (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`slp_wakeup_cntl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_wakeup_cntl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slp_wakeup_cntl6`] module*/
pub type SLP_WAKEUP_CNTL6 = crate::Reg<slp_wakeup_cntl6::SLP_WAKEUP_CNTL6_SPEC>;
///need_des
pub mod slp_wakeup_cntl6;
/**SLP_WAKEUP_CNTL7 (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`slp_wakeup_cntl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_wakeup_cntl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slp_wakeup_cntl7`] module*/
pub type SLP_WAKEUP_CNTL7 = crate::Reg<slp_wakeup_cntl7::SLP_WAKEUP_CNTL7_SPEC>;
///need_des
pub mod slp_wakeup_cntl7;
/**SLP_WAKEUP_CNTL8 (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`slp_wakeup_cntl8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slp_wakeup_cntl8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slp_wakeup_cntl8`] module*/
pub type SLP_WAKEUP_CNTL8 = crate::Reg<slp_wakeup_cntl8::SLP_WAKEUP_CNTL8_SPEC>;
///need_des
pub mod slp_wakeup_cntl8;
/**SLP_WAKEUP_STATUS0 (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`slp_wakeup_status0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slp_wakeup_status0`] module*/
pub type SLP_WAKEUP_STATUS0 = crate::Reg<slp_wakeup_status0::SLP_WAKEUP_STATUS0_SPEC>;
///need_des
pub mod slp_wakeup_status0;
/**SLP_WAKEUP_STATUS1 (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`slp_wakeup_status1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slp_wakeup_status1`] module*/
pub type SLP_WAKEUP_STATUS1 = crate::Reg<slp_wakeup_status1::SLP_WAKEUP_STATUS1_SPEC>;
///need_des
pub mod slp_wakeup_status1;
/**SLP_WAKEUP_STATUS2 (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`slp_wakeup_status2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@slp_wakeup_status2`] module*/
pub type SLP_WAKEUP_STATUS2 = crate::Reg<slp_wakeup_status2::SLP_WAKEUP_STATUS2_SPEC>;
///need_des
pub mod slp_wakeup_status2;
/**HP_CK_POWERON (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_ck_poweron::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_ck_poweron::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_ck_poweron`] module*/
pub type HP_CK_POWERON = crate::Reg<hp_ck_poweron::HP_CK_POWERON_SPEC>;
///need_des
pub mod hp_ck_poweron;
/**HP_CK_CNTL (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_ck_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_ck_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_ck_cntl`] module*/
pub type HP_CK_CNTL = crate::Reg<hp_ck_cntl::HP_CK_CNTL_SPEC>;
///need_des
pub mod hp_ck_cntl;
/**POR_STATUS (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`por_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@por_status`] module*/
pub type POR_STATUS = crate::Reg<por_status::POR_STATUS_SPEC>;
///need_des
pub mod por_status;
/**RF_PWC (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`rf_pwc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rf_pwc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rf_pwc`] module*/
pub type RF_PWC = crate::Reg<rf_pwc::RF_PWC_SPEC>;
///need_des
pub mod rf_pwc;
/**BACKUP_CFG (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`backup_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`backup_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@backup_cfg`] module*/
pub type BACKUP_CFG = crate::Reg<backup_cfg::BACKUP_CFG_SPEC>;
///need_des
pub mod backup_cfg;
/**INT_RAW (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///need_des
pub mod int_raw;
/**INT_ST (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///need_des
pub mod int_st;
/**INT_ENA (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///need_des
pub mod int_ena;
/**INT_CLR (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///need_des
pub mod int_clr;
/**LP_INT_RAW (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_int_raw`] module*/
pub type LP_INT_RAW = crate::Reg<lp_int_raw::LP_INT_RAW_SPEC>;
///need_des
pub mod lp_int_raw;
/**LP_INT_ST (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_int_st`] module*/
pub type LP_INT_ST = crate::Reg<lp_int_st::LP_INT_ST_SPEC>;
///need_des
pub mod lp_int_st;
/**LP_INT_ENA (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_int_ena`] module*/
pub type LP_INT_ENA = crate::Reg<lp_int_ena::LP_INT_ENA_SPEC>;
///need_des
pub mod lp_int_ena;
/**LP_INT_CLR (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_int_clr`] module*/
pub type LP_INT_CLR = crate::Reg<lp_int_clr::LP_INT_CLR_SPEC>;
///need_des
pub mod lp_int_clr;
/**LP_CPU_PWR0 (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_cpu_pwr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_cpu_pwr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_cpu_pwr0`] module*/
pub type LP_CPU_PWR0 = crate::Reg<lp_cpu_pwr0::LP_CPU_PWR0_SPEC>;
///need_des
pub mod lp_cpu_pwr0;
/**LP_CPU_PWR1 (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_cpu_pwr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_cpu_pwr1`] module*/
pub type LP_CPU_PWR1 = crate::Reg<lp_cpu_pwr1::LP_CPU_PWR1_SPEC>;
///need_des
pub mod lp_cpu_pwr1;
/**LP_CPU_PWR2 (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_cpu_pwr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_cpu_pwr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_cpu_pwr2`] module*/
pub type LP_CPU_PWR2 = crate::Reg<lp_cpu_pwr2::LP_CPU_PWR2_SPEC>;
///need_des
pub mod lp_cpu_pwr2;
/**LP_CPU_PWR3 (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_cpu_pwr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_cpu_pwr3`] module*/
pub type LP_CPU_PWR3 = crate::Reg<lp_cpu_pwr3::LP_CPU_PWR3_SPEC>;
///need_des
pub mod lp_cpu_pwr3;
/**LP_CPU_PWR4 (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_cpu_pwr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_cpu_pwr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_cpu_pwr4`] module*/
pub type LP_CPU_PWR4 = crate::Reg<lp_cpu_pwr4::LP_CPU_PWR4_SPEC>;
///need_des
pub mod lp_cpu_pwr4;
/**LP_CPU_PWR5 (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_cpu_pwr5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lp_cpu_pwr5`] module*/
pub type LP_CPU_PWR5 = crate::Reg<lp_cpu_pwr5::LP_CPU_PWR5_SPEC>;
///need_des
pub mod lp_cpu_pwr5;
/**HP_LP_CPU_COMM (w) register accessor: need_des

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_lp_cpu_comm::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_lp_cpu_comm`] module*/
pub type HP_LP_CPU_COMM = crate::Reg<hp_lp_cpu_comm::HP_LP_CPU_COMM_SPEC>;
///need_des
pub mod hp_lp_cpu_comm;
/**HP_REGULATOR_CFG (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`hp_regulator_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_regulator_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hp_regulator_cfg`] module*/
pub type HP_REGULATOR_CFG = crate::Reg<hp_regulator_cfg::HP_REGULATOR_CFG_SPEC>;
///need_des
pub mod hp_regulator_cfg;
/**MAIN_STATE (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`main_state::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`main_state::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@main_state`] module*/
pub type MAIN_STATE = crate::Reg<main_state::MAIN_STATE_SPEC>;
///need_des
pub mod main_state;
/**PWR_STATE (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`pwr_state::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pwr_state`] module*/
pub type PWR_STATE = crate::Reg<pwr_state::PWR_STATE_SPEC>;
///need_des
pub mod pwr_state;
/**CLK_STATE0 (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`clk_state0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk_state0`] module*/
pub type CLK_STATE0 = crate::Reg<clk_state0::CLK_STATE0_SPEC>;
///need_des
pub mod clk_state0;
/**CLK_STATE1 (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`clk_state1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk_state1`] module*/
pub type CLK_STATE1 = crate::Reg<clk_state1::CLK_STATE1_SPEC>;
///need_des
pub mod clk_state1;
/**CLK_STATE2 (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`clk_state2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clk_state2`] module*/
pub type CLK_STATE2 = crate::Reg<clk_state2::CLK_STATE2_SPEC>;
///need_des
pub mod clk_state2;
/**EXT_LDO_P0_0P1A (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`ext_ldo_p0_0p1a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_ldo_p0_0p1a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_ldo_p0_0p1a`] module*/
pub type EXT_LDO_P0_0P1A = crate::Reg<ext_ldo_p0_0p1a::EXT_LDO_P0_0P1A_SPEC>;
///need_des
pub mod ext_ldo_p0_0p1a;
/**EXT_LDO_P0_0P1A_ANA (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`ext_ldo_p0_0p1a_ana::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_ldo_p0_0p1a_ana::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_ldo_p0_0p1a_ana`] module*/
pub type EXT_LDO_P0_0P1A_ANA = crate::Reg<ext_ldo_p0_0p1a_ana::EXT_LDO_P0_0P1A_ANA_SPEC>;
///need_des
pub mod ext_ldo_p0_0p1a_ana;
/**EXT_LDO_P0_0P2A (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`ext_ldo_p0_0p2a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_ldo_p0_0p2a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_ldo_p0_0p2a`] module*/
pub type EXT_LDO_P0_0P2A = crate::Reg<ext_ldo_p0_0p2a::EXT_LDO_P0_0P2A_SPEC>;
///need_des
pub mod ext_ldo_p0_0p2a;
/**EXT_LDO_P0_0P2A_ANA (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`ext_ldo_p0_0p2a_ana::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_ldo_p0_0p2a_ana::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_ldo_p0_0p2a_ana`] module*/
pub type EXT_LDO_P0_0P2A_ANA = crate::Reg<ext_ldo_p0_0p2a_ana::EXT_LDO_P0_0P2A_ANA_SPEC>;
///need_des
pub mod ext_ldo_p0_0p2a_ana;
/**EXT_LDO_P0_0P3A (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`ext_ldo_p0_0p3a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_ldo_p0_0p3a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_ldo_p0_0p3a`] module*/
pub type EXT_LDO_P0_0P3A = crate::Reg<ext_ldo_p0_0p3a::EXT_LDO_P0_0P3A_SPEC>;
///need_des
pub mod ext_ldo_p0_0p3a;
/**EXT_LDO_P0_0P3A_ANA (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`ext_ldo_p0_0p3a_ana::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_ldo_p0_0p3a_ana::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_ldo_p0_0p3a_ana`] module*/
pub type EXT_LDO_P0_0P3A_ANA = crate::Reg<ext_ldo_p0_0p3a_ana::EXT_LDO_P0_0P3A_ANA_SPEC>;
///need_des
pub mod ext_ldo_p0_0p3a_ana;
/**EXT_LDO_P1_0P1A (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`ext_ldo_p1_0p1a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_ldo_p1_0p1a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_ldo_p1_0p1a`] module*/
pub type EXT_LDO_P1_0P1A = crate::Reg<ext_ldo_p1_0p1a::EXT_LDO_P1_0P1A_SPEC>;
///need_des
pub mod ext_ldo_p1_0p1a;
/**EXT_LDO_P1_0P1A_ANA (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`ext_ldo_p1_0p1a_ana::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_ldo_p1_0p1a_ana::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_ldo_p1_0p1a_ana`] module*/
pub type EXT_LDO_P1_0P1A_ANA = crate::Reg<ext_ldo_p1_0p1a_ana::EXT_LDO_P1_0P1A_ANA_SPEC>;
///need_des
pub mod ext_ldo_p1_0p1a_ana;
/**EXT_LDO_P1_0P2A (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`ext_ldo_p1_0p2a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_ldo_p1_0p2a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_ldo_p1_0p2a`] module*/
pub type EXT_LDO_P1_0P2A = crate::Reg<ext_ldo_p1_0p2a::EXT_LDO_P1_0P2A_SPEC>;
///need_des
pub mod ext_ldo_p1_0p2a;
/**EXT_LDO_P1_0P2A_ANA (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`ext_ldo_p1_0p2a_ana::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_ldo_p1_0p2a_ana::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_ldo_p1_0p2a_ana`] module*/
pub type EXT_LDO_P1_0P2A_ANA = crate::Reg<ext_ldo_p1_0p2a_ana::EXT_LDO_P1_0P2A_ANA_SPEC>;
///need_des
pub mod ext_ldo_p1_0p2a_ana;
/**EXT_LDO_P1_0P3A (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`ext_ldo_p1_0p3a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_ldo_p1_0p3a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_ldo_p1_0p3a`] module*/
pub type EXT_LDO_P1_0P3A = crate::Reg<ext_ldo_p1_0p3a::EXT_LDO_P1_0P3A_SPEC>;
///need_des
pub mod ext_ldo_p1_0p3a;
/**EXT_LDO_P1_0P3A_ANA (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`ext_ldo_p1_0p3a_ana::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_ldo_p1_0p3a_ana::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_ldo_p1_0p3a_ana`] module*/
pub type EXT_LDO_P1_0P3A_ANA = crate::Reg<ext_ldo_p1_0p3a_ana::EXT_LDO_P1_0P3A_ANA_SPEC>;
///need_des
pub mod ext_ldo_p1_0p3a_ana;
/**EXT_WAKEUP_LV (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup_lv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup_lv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_wakeup_lv`] module*/
pub type EXT_WAKEUP_LV = crate::Reg<ext_wakeup_lv::EXT_WAKEUP_LV_SPEC>;
///need_des
pub mod ext_wakeup_lv;
/**EXT_WAKEUP_SEL (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_wakeup_sel`] module*/
pub type EXT_WAKEUP_SEL = crate::Reg<ext_wakeup_sel::EXT_WAKEUP_SEL_SPEC>;
///need_des
pub mod ext_wakeup_sel;
/**EXT_WAKEUP_ST (r) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_wakeup_st`] module*/
pub type EXT_WAKEUP_ST = crate::Reg<ext_wakeup_st::EXT_WAKEUP_ST_SPEC>;
///need_des
pub mod ext_wakeup_st;
/**EXT_WAKEUP_CNTL (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ext_wakeup_cntl`] module*/
pub type EXT_WAKEUP_CNTL = crate::Reg<ext_wakeup_cntl::EXT_WAKEUP_CNTL_SPEC>;
///need_des
pub mod ext_wakeup_cntl;
/**SDIO_WAKEUP_CNTL (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`sdio_wakeup_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_wakeup_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdio_wakeup_cntl`] module*/
pub type SDIO_WAKEUP_CNTL = crate::Reg<sdio_wakeup_cntl::SDIO_WAKEUP_CNTL_SPEC>;
///need_des
pub mod sdio_wakeup_cntl;
/**XTAL_SLP (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`xtal_slp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xtal_slp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@xtal_slp`] module*/
pub type XTAL_SLP = crate::Reg<xtal_slp::XTAL_SLP_SPEC>;
///need_des
pub mod xtal_slp;
/**CPU_SW_STALL (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_sw_stall::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_sw_stall::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_sw_stall`] module*/
pub type CPU_SW_STALL = crate::Reg<cpu_sw_stall::CPU_SW_STALL_SPEC>;
///need_des
pub mod cpu_sw_stall;
/**DCM_CTRL (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`dcm_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcm_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dcm_ctrl`] module*/
pub type DCM_CTRL = crate::Reg<dcm_ctrl::DCM_CTRL_SPEC>;
///need_des
pub mod dcm_ctrl;
/**DCM_WAIT_DELAY (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`dcm_wait_delay::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcm_wait_delay::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dcm_wait_delay`] module*/
pub type DCM_WAIT_DELAY = crate::Reg<dcm_wait_delay::DCM_WAIT_DELAY_SPEC>;
///need_des
pub mod dcm_wait_delay;
/**VDDBAT_CFG (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`vddbat_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vddbat_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vddbat_cfg`] module*/
pub type VDDBAT_CFG = crate::Reg<vddbat_cfg::VDDBAT_CFG_SPEC>;
///need_des
pub mod vddbat_cfg;
/**TOUCH_PWR_CNTL (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`touch_pwr_cntl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pwr_cntl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@touch_pwr_cntl`] module*/
pub type TOUCH_PWR_CNTL = crate::Reg<touch_pwr_cntl::TOUCH_PWR_CNTL_SPEC>;
///need_des
pub mod touch_pwr_cntl;
/**RDN_ECO (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`rdn_eco::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdn_eco::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdn_eco`] module*/
pub type RDN_ECO = crate::Reg<rdn_eco::RDN_ECO_SPEC>;
///need_des
pub mod rdn_eco;
/**DATE (rw) register accessor: need_des

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///need_des
pub mod date;
