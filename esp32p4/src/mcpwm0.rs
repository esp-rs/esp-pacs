#[doc = r"Register block"]
#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
pub struct RegisterBlock {
    clk_cfg: CLK_CFG,
    timer_cfg0: (),
    _reserved2: [u8; 0x04],
    timer_cfg1: (),
    _reserved3: [u8; 0x04],
    timer_sync: (),
    _reserved4: [u8; 0x04],
    timer_status: (),
    _reserved5: [u8; 0x24],
    timer_synci_cfg: TIMER_SYNCI_CFG,
    operator_timersel: OPERATOR_TIMERSEL,
    gen_stmp_cfg: (),
    _reserved8: [u8; 0x04],
    gen_tstmp_a: (),
    _reserved9: [u8; 0x04],
    gen_tstmp_b: (),
    _reserved10: [u8; 0x04],
    gen_cfg0: (),
    _reserved11: [u8; 0x04],
    gen_force: (),
    _reserved12: [u8; 0x04],
    gen_a: (),
    _reserved13: [u8; 0x04],
    gen_b: (),
    _reserved14: [u8; 0x04],
    dt_cfg: (),
    _reserved15: [u8; 0x04],
    dt_fed_cfg: (),
    _reserved16: [u8; 0x04],
    dt_red_cfg: (),
    _reserved17: [u8; 0x04],
    carrier_cfg: (),
    _reserved18: [u8; 0x04],
    fh_cfg0: (),
    _reserved19: [u8; 0x04],
    fh_cfg1: (),
    _reserved20: [u8; 0x04],
    fh_status: (),
    _reserved21: [u8; 0x74],
    fault_detect: FAULT_DETECT,
    cap_timer_cfg: CAP_TIMER_CFG,
    cap_timer_phase: CAP_TIMER_PHASE,
    cap_ch_cfg: [CAP_CH_CFG; 3],
    cap_ch: [CAP_CH; 3],
    cap_status: CAP_STATUS,
    update_cfg: UPDATE_CFG,
    int_ena: INT_ENA,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_clr: INT_CLR,
    evt_en: EVT_EN,
    task_en: TASK_EN,
    evt_en2: EVT_EN2,
    op_tstmp_e1: (),
    _reserved36: [u8; 0x04],
    op_tstmp_e2: (),
    _reserved37: [u8; 0x14],
    clk: CLK,
    version: VERSION,
}
impl RegisterBlock {
    #[doc = "0x00 - PWM clock prescaler register."]
    #[inline(always)]
    pub const fn clk_cfg(&self) -> &CLK_CFG {
        &self.clk_cfg
    }
    #[doc = "0x04..0x10 - PWM timer%s period and update method configuration register."]
    #[inline(always)]
    pub const fn timer_cfg0(&self, n: usize) -> &TIMER_CFG0 {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(4).add(16 * n).cast() }
    }
    #[doc = "0x04 - PWM timer0 period and update method configuration register."]
    #[inline(always)]
    pub const fn timer0_cfg0(&self) -> &TIMER_CFG0 {
        &self.timer_cfg0(0)
    }
    #[doc = "0x14 - PWM timer1 period and update method configuration register."]
    #[inline(always)]
    pub const fn timer1_cfg0(&self) -> &TIMER_CFG0 {
        &self.timer_cfg0(1)
    }
    #[doc = "0x24 - PWM timer2 period and update method configuration register."]
    #[inline(always)]
    pub const fn timer2_cfg0(&self) -> &TIMER_CFG0 {
        &self.timer_cfg0(2)
    }
    #[doc = "0x08..0x14 - PWM timer%s working mode and start/stop control register."]
    #[inline(always)]
    pub const fn timer_cfg1(&self, n: usize) -> &TIMER_CFG1 {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(8).add(16 * n).cast() }
    }
    #[doc = "0x08 - PWM timer0 working mode and start/stop control register."]
    #[inline(always)]
    pub const fn timer0_cfg1(&self) -> &TIMER_CFG1 {
        &self.timer_cfg1(0)
    }
    #[doc = "0x18 - PWM timer1 working mode and start/stop control register."]
    #[inline(always)]
    pub const fn timer1_cfg1(&self) -> &TIMER_CFG1 {
        &self.timer_cfg1(1)
    }
    #[doc = "0x28 - PWM timer2 working mode and start/stop control register."]
    #[inline(always)]
    pub const fn timer2_cfg1(&self) -> &TIMER_CFG1 {
        &self.timer_cfg1(2)
    }
    #[doc = "0x0c..0x18 - PWM timer%s sync function configuration register."]
    #[inline(always)]
    pub const fn timer_sync(&self, n: usize) -> &TIMER_SYNC {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(12)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "0x0c - PWM timer0 sync function configuration register."]
    #[inline(always)]
    pub const fn timer0_sync(&self) -> &TIMER_SYNC {
        &self.timer_sync(0)
    }
    #[doc = "0x1c - PWM timer1 sync function configuration register."]
    #[inline(always)]
    pub const fn timer1_sync(&self) -> &TIMER_SYNC {
        &self.timer_sync(1)
    }
    #[doc = "0x2c - PWM timer2 sync function configuration register."]
    #[inline(always)]
    pub const fn timer2_sync(&self) -> &TIMER_SYNC {
        &self.timer_sync(2)
    }
    #[doc = "0x10..0x1c - PWM timer%s status register."]
    #[inline(always)]
    pub const fn timer_status(&self, n: usize) -> &TIMER_STATUS {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(16)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "0x10 - PWM timer0 status register."]
    #[inline(always)]
    pub const fn timer0_status(&self) -> &TIMER_STATUS {
        &self.timer_status(0)
    }
    #[doc = "0x20 - PWM timer1 status register."]
    #[inline(always)]
    pub const fn timer1_status(&self) -> &TIMER_STATUS {
        &self.timer_status(1)
    }
    #[doc = "0x30 - PWM timer2 status register."]
    #[inline(always)]
    pub const fn timer2_status(&self) -> &TIMER_STATUS {
        &self.timer_status(2)
    }
    #[doc = "0x34 - Synchronization input selection register for PWM timers."]
    #[inline(always)]
    pub const fn timer_synci_cfg(&self) -> &TIMER_SYNCI_CFG {
        &self.timer_synci_cfg
    }
    #[doc = "0x38 - PWM operator's timer select register"]
    #[inline(always)]
    pub const fn operator_timersel(&self) -> &OPERATOR_TIMERSEL {
        &self.operator_timersel
    }
    #[doc = "0x3c..0x48 - Generator%s time stamp registers A and B transfer status and update method register"]
    #[inline(always)]
    pub const fn gen_stmp_cfg(&self, n: usize) -> &GEN_STMP_CFG {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(60)
                .add(56 * n)
                .cast()
        }
    }
    #[doc = "0x3c - Generator0 time stamp registers A and B transfer status and update method register"]
    #[inline(always)]
    pub const fn gen0_stmp_cfg(&self) -> &GEN_STMP_CFG {
        &self.gen_stmp_cfg(0)
    }
    #[doc = "0x74 - Generator1 time stamp registers A and B transfer status and update method register"]
    #[inline(always)]
    pub const fn gen1_stmp_cfg(&self) -> &GEN_STMP_CFG {
        &self.gen_stmp_cfg(1)
    }
    #[doc = "0xac - Generator2 time stamp registers A and B transfer status and update method register"]
    #[inline(always)]
    pub const fn gen2_stmp_cfg(&self) -> &GEN_STMP_CFG {
        &self.gen_stmp_cfg(2)
    }
    #[doc = "0x40..0x4c - Generator%s time stamp A's shadow register"]
    #[inline(always)]
    pub const fn gen_tstmp_a(&self, n: usize) -> &GEN_TSTMP_A {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(64)
                .add(56 * n)
                .cast()
        }
    }
    #[doc = "0x40 - Generator0 time stamp A's shadow register"]
    #[inline(always)]
    pub const fn gen0_tstmp_a(&self) -> &GEN_TSTMP_A {
        &self.gen_tstmp_a(0)
    }
    #[doc = "0x78 - Generator1 time stamp A's shadow register"]
    #[inline(always)]
    pub const fn gen1_tstmp_a(&self) -> &GEN_TSTMP_A {
        &self.gen_tstmp_a(1)
    }
    #[doc = "0xb0 - Generator2 time stamp A's shadow register"]
    #[inline(always)]
    pub const fn gen2_tstmp_a(&self) -> &GEN_TSTMP_A {
        &self.gen_tstmp_a(2)
    }
    #[doc = "0x44..0x50 - Generator%s time stamp B's shadow register"]
    #[inline(always)]
    pub const fn gen_tstmp_b(&self, n: usize) -> &GEN_TSTMP_B {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(68)
                .add(56 * n)
                .cast()
        }
    }
    #[doc = "0x44 - Generator0 time stamp B's shadow register"]
    #[inline(always)]
    pub const fn gen0_tstmp_b(&self) -> &GEN_TSTMP_B {
        &self.gen_tstmp_b(0)
    }
    #[doc = "0x7c - Generator1 time stamp B's shadow register"]
    #[inline(always)]
    pub const fn gen1_tstmp_b(&self) -> &GEN_TSTMP_B {
        &self.gen_tstmp_b(1)
    }
    #[doc = "0xb4 - Generator2 time stamp B's shadow register"]
    #[inline(always)]
    pub const fn gen2_tstmp_b(&self) -> &GEN_TSTMP_B {
        &self.gen_tstmp_b(2)
    }
    #[doc = "0x48..0x54 - Generator%s fault event T0 and T1 configuration register"]
    #[inline(always)]
    pub const fn gen_cfg0(&self, n: usize) -> &GEN_CFG0 {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(72)
                .add(56 * n)
                .cast()
        }
    }
    #[doc = "0x48 - Generator0 fault event T0 and T1 configuration register"]
    #[inline(always)]
    pub const fn gen0_cfg0(&self) -> &GEN_CFG0 {
        &self.gen_cfg0(0)
    }
    #[doc = "0x80 - Generator1 fault event T0 and T1 configuration register"]
    #[inline(always)]
    pub const fn gen1_cfg0(&self) -> &GEN_CFG0 {
        &self.gen_cfg0(1)
    }
    #[doc = "0xb8 - Generator2 fault event T0 and T1 configuration register"]
    #[inline(always)]
    pub const fn gen2_cfg0(&self) -> &GEN_CFG0 {
        &self.gen_cfg0(2)
    }
    #[doc = "0x4c..0x58 - Generator%s output signal force mode register."]
    #[inline(always)]
    pub const fn gen_force(&self, n: usize) -> &GEN_FORCE {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(76)
                .add(56 * n)
                .cast()
        }
    }
    #[doc = "0x4c - Generator0 output signal force mode register."]
    #[inline(always)]
    pub const fn gen0_force(&self) -> &GEN_FORCE {
        &self.gen_force(0)
    }
    #[doc = "0x84 - Generator1 output signal force mode register."]
    #[inline(always)]
    pub const fn gen1_force(&self) -> &GEN_FORCE {
        &self.gen_force(1)
    }
    #[doc = "0xbc - Generator2 output signal force mode register."]
    #[inline(always)]
    pub const fn gen2_force(&self) -> &GEN_FORCE {
        &self.gen_force(2)
    }
    #[doc = "0x50..0x5c - PWM%s output signal A actions configuration register"]
    #[inline(always)]
    pub const fn gen_a(&self, n: usize) -> &GEN_A {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(80)
                .add(56 * n)
                .cast()
        }
    }
    #[doc = "0x50 - PWM0 output signal A actions configuration register"]
    #[inline(always)]
    pub const fn gen0_a(&self) -> &GEN_A {
        &self.gen_a(0)
    }
    #[doc = "0x88 - PWM1 output signal A actions configuration register"]
    #[inline(always)]
    pub const fn gen1_a(&self) -> &GEN_A {
        &self.gen_a(1)
    }
    #[doc = "0xc0 - PWM2 output signal A actions configuration register"]
    #[inline(always)]
    pub const fn gen2_a(&self) -> &GEN_A {
        &self.gen_a(2)
    }
    #[doc = "0x54..0x60 - PWM%s output signal B actions configuration register"]
    #[inline(always)]
    pub const fn gen_b(&self, n: usize) -> &GEN_B {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(84)
                .add(56 * n)
                .cast()
        }
    }
    #[doc = "0x54 - PWM0 output signal B actions configuration register"]
    #[inline(always)]
    pub const fn gen0_b(&self) -> &GEN_B {
        &self.gen_b(0)
    }
    #[doc = "0x8c - PWM1 output signal B actions configuration register"]
    #[inline(always)]
    pub const fn gen1_b(&self) -> &GEN_B {
        &self.gen_b(1)
    }
    #[doc = "0xc4 - PWM2 output signal B actions configuration register"]
    #[inline(always)]
    pub const fn gen2_b(&self) -> &GEN_B {
        &self.gen_b(2)
    }
    #[doc = "0x58..0x64 - Dead time configuration register"]
    #[inline(always)]
    pub const fn dt_cfg(&self, n: usize) -> &DT_CFG {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(88)
                .add(56 * n)
                .cast()
        }
    }
    #[doc = "0x58 - Dead time configuration register"]
    #[inline(always)]
    pub const fn dt0_cfg(&self) -> &DT_CFG {
        &self.dt_cfg(0)
    }
    #[doc = "0x90 - Dead time configuration register"]
    #[inline(always)]
    pub const fn dt1_cfg(&self) -> &DT_CFG {
        &self.dt_cfg(1)
    }
    #[doc = "0xc8 - Dead time configuration register"]
    #[inline(always)]
    pub const fn dt2_cfg(&self) -> &DT_CFG {
        &self.dt_cfg(2)
    }
    #[doc = "0x5c..0x68 - Falling edge delay (FED) shadow register"]
    #[inline(always)]
    pub const fn dt_fed_cfg(&self, n: usize) -> &DT_FED_CFG {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(92)
                .add(56 * n)
                .cast()
        }
    }
    #[doc = "0x5c - Falling edge delay (FED) shadow register"]
    #[inline(always)]
    pub const fn dt0_fed_cfg(&self) -> &DT_FED_CFG {
        &self.dt_fed_cfg(0)
    }
    #[doc = "0x94 - Falling edge delay (FED) shadow register"]
    #[inline(always)]
    pub const fn dt1_fed_cfg(&self) -> &DT_FED_CFG {
        &self.dt_fed_cfg(1)
    }
    #[doc = "0xcc - Falling edge delay (FED) shadow register"]
    #[inline(always)]
    pub const fn dt2_fed_cfg(&self) -> &DT_FED_CFG {
        &self.dt_fed_cfg(2)
    }
    #[doc = "0x60..0x6c - Rising edge delay (RED) shadow register"]
    #[inline(always)]
    pub const fn dt_red_cfg(&self, n: usize) -> &DT_RED_CFG {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(96)
                .add(56 * n)
                .cast()
        }
    }
    #[doc = "0x60 - Rising edge delay (RED) shadow register"]
    #[inline(always)]
    pub const fn dt0_red_cfg(&self) -> &DT_RED_CFG {
        &self.dt_red_cfg(0)
    }
    #[doc = "0x98 - Rising edge delay (RED) shadow register"]
    #[inline(always)]
    pub const fn dt1_red_cfg(&self) -> &DT_RED_CFG {
        &self.dt_red_cfg(1)
    }
    #[doc = "0xd0 - Rising edge delay (RED) shadow register"]
    #[inline(always)]
    pub const fn dt2_red_cfg(&self) -> &DT_RED_CFG {
        &self.dt_red_cfg(2)
    }
    #[doc = "0x64..0x70 - Carrier%s configuration register"]
    #[inline(always)]
    pub const fn carrier_cfg(&self, n: usize) -> &CARRIER_CFG {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(100)
                .add(56 * n)
                .cast()
        }
    }
    #[doc = "0x64 - Carrier0 configuration register"]
    #[inline(always)]
    pub const fn carrier0_cfg(&self) -> &CARRIER_CFG {
        &self.carrier_cfg(0)
    }
    #[doc = "0x9c - Carrier1 configuration register"]
    #[inline(always)]
    pub const fn carrier1_cfg(&self) -> &CARRIER_CFG {
        &self.carrier_cfg(1)
    }
    #[doc = "0xd4 - Carrier2 configuration register"]
    #[inline(always)]
    pub const fn carrier2_cfg(&self) -> &CARRIER_CFG {
        &self.carrier_cfg(2)
    }
    #[doc = "0x68..0x74 - PWM%s A and PWM%s B trip events actions configuration register"]
    #[inline(always)]
    pub const fn fh_cfg0(&self, n: usize) -> &FH_CFG0 {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(104)
                .add(56 * n)
                .cast()
        }
    }
    #[doc = "0x68 - PWM0 A and PWM0 B trip events actions configuration register"]
    #[inline(always)]
    pub const fn fh0_cfg0(&self) -> &FH_CFG0 {
        &self.fh_cfg0(0)
    }
    #[doc = "0xa0 - PWM1 A and PWM1 B trip events actions configuration register"]
    #[inline(always)]
    pub const fn fh1_cfg0(&self) -> &FH_CFG0 {
        &self.fh_cfg0(1)
    }
    #[doc = "0xd8 - PWM2 A and PWM2 B trip events actions configuration register"]
    #[inline(always)]
    pub const fn fh2_cfg0(&self) -> &FH_CFG0 {
        &self.fh_cfg0(2)
    }
    #[doc = "0x6c..0x78 - Software triggers for fault handler actions configuration register"]
    #[inline(always)]
    pub const fn fh_cfg1(&self, n: usize) -> &FH_CFG1 {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(108)
                .add(56 * n)
                .cast()
        }
    }
    #[doc = "0x6c - Software triggers for fault handler actions configuration register"]
    #[inline(always)]
    pub const fn fh0_cfg1(&self) -> &FH_CFG1 {
        &self.fh_cfg1(0)
    }
    #[doc = "0xa4 - Software triggers for fault handler actions configuration register"]
    #[inline(always)]
    pub const fn fh1_cfg1(&self) -> &FH_CFG1 {
        &self.fh_cfg1(1)
    }
    #[doc = "0xdc - Software triggers for fault handler actions configuration register"]
    #[inline(always)]
    pub const fn fh2_cfg1(&self) -> &FH_CFG1 {
        &self.fh_cfg1(2)
    }
    #[doc = "0x70..0x7c - Fault events status register"]
    #[inline(always)]
    pub const fn fh_status(&self, n: usize) -> &FH_STATUS {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(112)
                .add(56 * n)
                .cast()
        }
    }
    #[doc = "0x70 - Fault events status register"]
    #[inline(always)]
    pub const fn fh0_status(&self) -> &FH_STATUS {
        &self.fh_status(0)
    }
    #[doc = "0xa8 - Fault events status register"]
    #[inline(always)]
    pub const fn fh1_status(&self) -> &FH_STATUS {
        &self.fh_status(1)
    }
    #[doc = "0xe0 - Fault events status register"]
    #[inline(always)]
    pub const fn fh2_status(&self) -> &FH_STATUS {
        &self.fh_status(2)
    }
    #[doc = "0xe4 - Fault detection configuration and status register"]
    #[inline(always)]
    pub const fn fault_detect(&self) -> &FAULT_DETECT {
        &self.fault_detect
    }
    #[doc = "0xe8 - Capture timer configuration register"]
    #[inline(always)]
    pub const fn cap_timer_cfg(&self) -> &CAP_TIMER_CFG {
        &self.cap_timer_cfg
    }
    #[doc = "0xec - Capture timer sync phase register"]
    #[inline(always)]
    pub const fn cap_timer_phase(&self) -> &CAP_TIMER_PHASE {
        &self.cap_timer_phase
    }
    #[doc = "0xf0..0xfc - Capture channel %s configuration register"]
    #[inline(always)]
    pub const fn cap_ch_cfg(&self, n: usize) -> &CAP_CH_CFG {
        &self.cap_ch_cfg[n]
    }
    #[doc = "0xf0 - Capture channel 0 configuration register"]
    #[inline(always)]
    pub const fn cap_ch0_cfg(&self) -> &CAP_CH_CFG {
        &self.cap_ch_cfg(0)
    }
    #[doc = "0xf4 - Capture channel 1 configuration register"]
    #[inline(always)]
    pub const fn cap_ch1_cfg(&self) -> &CAP_CH_CFG {
        &self.cap_ch_cfg(1)
    }
    #[doc = "0xf8 - Capture channel 2 configuration register"]
    #[inline(always)]
    pub const fn cap_ch2_cfg(&self) -> &CAP_CH_CFG {
        &self.cap_ch_cfg(2)
    }
    #[doc = "0xfc..0x108 - CAP%s capture value register"]
    #[inline(always)]
    pub const fn cap_ch(&self, n: usize) -> &CAP_CH {
        &self.cap_ch[n]
    }
    #[doc = "0x108 - Last capture trigger edge information register"]
    #[inline(always)]
    pub const fn cap_status(&self) -> &CAP_STATUS {
        &self.cap_status
    }
    #[doc = "0x10c - Generator Update configuration register"]
    #[inline(always)]
    pub const fn update_cfg(&self) -> &UPDATE_CFG {
        &self.update_cfg
    }
    #[doc = "0x110 - Interrupt enable register"]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x114 - Interrupt raw status register"]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x118 - Interrupt masked status register"]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x11c - Interrupt clear register"]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x120 - Event enable register"]
    #[inline(always)]
    pub const fn evt_en(&self) -> &EVT_EN {
        &self.evt_en
    }
    #[doc = "0x124 - Task enable register"]
    #[inline(always)]
    pub const fn task_en(&self) -> &TASK_EN {
        &self.task_en
    }
    #[doc = "0x128 - Event enable register2"]
    #[inline(always)]
    pub const fn evt_en2(&self) -> &EVT_EN2 {
        &self.evt_en2
    }
    #[doc = "0x12c..0x138 - Generator%s timer stamp E1 value register"]
    #[inline(always)]
    pub const fn op_tstmp_e1(&self, n: usize) -> &OP_TSTMP_E1 {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(300)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "0x12c - Generator0 timer stamp E1 value register"]
    #[inline(always)]
    pub const fn op0_tstmp_e1(&self) -> &OP_TSTMP_E1 {
        &self.op_tstmp_e1(0)
    }
    #[doc = "0x134 - Generator1 timer stamp E1 value register"]
    #[inline(always)]
    pub const fn op1_tstmp_e1(&self) -> &OP_TSTMP_E1 {
        &self.op_tstmp_e1(1)
    }
    #[doc = "0x13c - Generator2 timer stamp E1 value register"]
    #[inline(always)]
    pub const fn op2_tstmp_e1(&self) -> &OP_TSTMP_E1 {
        &self.op_tstmp_e1(2)
    }
    #[doc = "0x130..0x13c - Generator%s timer stamp E2 value register"]
    #[inline(always)]
    pub const fn op_tstmp_e2(&self, n: usize) -> &OP_TSTMP_E2 {
        #[allow(clippy::no_effect)]
        [(); 3][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(304)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "0x130 - Generator0 timer stamp E2 value register"]
    #[inline(always)]
    pub const fn op0_tstmp_e2(&self) -> &OP_TSTMP_E2 {
        &self.op_tstmp_e2(0)
    }
    #[doc = "0x138 - Generator1 timer stamp E2 value register"]
    #[inline(always)]
    pub const fn op1_tstmp_e2(&self) -> &OP_TSTMP_E2 {
        &self.op_tstmp_e2(1)
    }
    #[doc = "0x140 - Generator2 timer stamp E2 value register"]
    #[inline(always)]
    pub const fn op2_tstmp_e2(&self) -> &OP_TSTMP_E2 {
        &self.op_tstmp_e2(2)
    }
    #[doc = "0x144 - Global configuration register"]
    #[inline(always)]
    pub const fn clk(&self) -> &CLK {
        &self.clk
    }
    #[doc = "0x148 - Version register."]
    #[inline(always)]
    pub const fn version(&self) -> &VERSION {
        &self.version
    }
}
#[doc = "CLK_CFG (rw) register accessor: PWM clock prescaler register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_cfg`] module"]
pub type CLK_CFG = crate::Reg<clk_cfg::CLK_CFG_SPEC>;
#[doc = "PWM clock prescaler register."]
pub mod clk_cfg;
#[doc = "TIMER_CFG0 (rw) register accessor: PWM timer%s period and update method configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_cfg0`] module"]
pub type TIMER_CFG0 = crate::Reg<timer_cfg0::TIMER_CFG0_SPEC>;
#[doc = "PWM timer%s period and update method configuration register."]
pub mod timer_cfg0;
#[doc = "TIMER_CFG1 (rw) register accessor: PWM timer%s working mode and start/stop control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_cfg1`] module"]
pub type TIMER_CFG1 = crate::Reg<timer_cfg1::TIMER_CFG1_SPEC>;
#[doc = "PWM timer%s working mode and start/stop control register."]
pub mod timer_cfg1;
#[doc = "TIMER_SYNC (rw) register accessor: PWM timer%s sync function configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_sync`] module"]
pub type TIMER_SYNC = crate::Reg<timer_sync::TIMER_SYNC_SPEC>;
#[doc = "PWM timer%s sync function configuration register."]
pub mod timer_sync;
#[doc = "TIMER_STATUS (r) register accessor: PWM timer%s status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_status`] module"]
pub type TIMER_STATUS = crate::Reg<timer_status::TIMER_STATUS_SPEC>;
#[doc = "PWM timer%s status register."]
pub mod timer_status;
#[doc = "TIMER_SYNCI_CFG (rw) register accessor: Synchronization input selection register for PWM timers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_synci_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_synci_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_synci_cfg`] module"]
pub type TIMER_SYNCI_CFG = crate::Reg<timer_synci_cfg::TIMER_SYNCI_CFG_SPEC>;
#[doc = "Synchronization input selection register for PWM timers."]
pub mod timer_synci_cfg;
#[doc = "OPERATOR_TIMERSEL (rw) register accessor: PWM operator's timer select register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`operator_timersel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`operator_timersel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@operator_timersel`] module"]
pub type OPERATOR_TIMERSEL = crate::Reg<operator_timersel::OPERATOR_TIMERSEL_SPEC>;
#[doc = "PWM operator's timer select register"]
pub mod operator_timersel;
#[doc = "GEN_STMP_CFG (rw) register accessor: Generator%s time stamp registers A and B transfer status and update method register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen_stmp_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_stmp_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen_stmp_cfg`] module"]
pub type GEN_STMP_CFG = crate::Reg<gen_stmp_cfg::GEN_STMP_CFG_SPEC>;
#[doc = "Generator%s time stamp registers A and B transfer status and update method register"]
pub mod gen_stmp_cfg;
#[doc = "GEN_TSTMP_A (rw) register accessor: Generator%s time stamp A's shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen_tstmp_a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_tstmp_a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen_tstmp_a`] module"]
pub type GEN_TSTMP_A = crate::Reg<gen_tstmp_a::GEN_TSTMP_A_SPEC>;
#[doc = "Generator%s time stamp A's shadow register"]
pub mod gen_tstmp_a;
#[doc = "GEN_TSTMP_B (rw) register accessor: Generator%s time stamp B's shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen_tstmp_b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_tstmp_b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen_tstmp_b`] module"]
pub type GEN_TSTMP_B = crate::Reg<gen_tstmp_b::GEN_TSTMP_B_SPEC>;
#[doc = "Generator%s time stamp B's shadow register"]
pub mod gen_tstmp_b;
#[doc = "GEN_CFG0 (rw) register accessor: Generator%s fault event T0 and T1 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen_cfg0`] module"]
pub type GEN_CFG0 = crate::Reg<gen_cfg0::GEN_CFG0_SPEC>;
#[doc = "Generator%s fault event T0 and T1 configuration register"]
pub mod gen_cfg0;
#[doc = "GEN_FORCE (rw) register accessor: Generator%s output signal force mode register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen_force::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_force::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen_force`] module"]
pub type GEN_FORCE = crate::Reg<gen_force::GEN_FORCE_SPEC>;
#[doc = "Generator%s output signal force mode register."]
pub mod gen_force;
#[doc = "GEN_A (rw) register accessor: PWM%s output signal A actions configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen_a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen_a`] module"]
pub type GEN_A = crate::Reg<gen_a::GEN_A_SPEC>;
#[doc = "PWM%s output signal A actions configuration register"]
pub mod gen_a;
#[doc = "GEN_B (rw) register accessor: PWM%s output signal B actions configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen_b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen_b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen_b`] module"]
pub type GEN_B = crate::Reg<gen_b::GEN_B_SPEC>;
#[doc = "PWM%s output signal B actions configuration register"]
pub mod gen_b;
#[doc = "DT_CFG (rw) register accessor: Dead time configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt_cfg`] module"]
pub type DT_CFG = crate::Reg<dt_cfg::DT_CFG_SPEC>;
#[doc = "Dead time configuration register"]
pub mod dt_cfg;
#[doc = "DT_FED_CFG (rw) register accessor: Falling edge delay (FED) shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt_fed_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt_fed_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt_fed_cfg`] module"]
pub type DT_FED_CFG = crate::Reg<dt_fed_cfg::DT_FED_CFG_SPEC>;
#[doc = "Falling edge delay (FED) shadow register"]
pub mod dt_fed_cfg;
#[doc = "DT_RED_CFG (rw) register accessor: Rising edge delay (RED) shadow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt_red_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt_red_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt_red_cfg`] module"]
pub type DT_RED_CFG = crate::Reg<dt_red_cfg::DT_RED_CFG_SPEC>;
#[doc = "Rising edge delay (RED) shadow register"]
pub mod dt_red_cfg;
#[doc = "CARRIER_CFG (rw) register accessor: Carrier%s configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`carrier_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`carrier_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@carrier_cfg`] module"]
pub type CARRIER_CFG = crate::Reg<carrier_cfg::CARRIER_CFG_SPEC>;
#[doc = "Carrier%s configuration register"]
pub mod carrier_cfg;
#[doc = "FH_CFG0 (rw) register accessor: PWM%s A and PWM%s B trip events actions configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fh_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fh_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fh_cfg0`] module"]
pub type FH_CFG0 = crate::Reg<fh_cfg0::FH_CFG0_SPEC>;
#[doc = "PWM%s A and PWM%s B trip events actions configuration register"]
pub mod fh_cfg0;
#[doc = "FH_CFG1 (rw) register accessor: Software triggers for fault handler actions configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fh_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fh_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fh_cfg1`] module"]
pub type FH_CFG1 = crate::Reg<fh_cfg1::FH_CFG1_SPEC>;
#[doc = "Software triggers for fault handler actions configuration register"]
pub mod fh_cfg1;
#[doc = "FH_STATUS (r) register accessor: Fault events status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fh_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fh_status`] module"]
pub type FH_STATUS = crate::Reg<fh_status::FH_STATUS_SPEC>;
#[doc = "Fault events status register"]
pub mod fh_status;
#[doc = "FAULT_DETECT (rw) register accessor: Fault detection configuration and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fault_detect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fault_detect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fault_detect`] module"]
pub type FAULT_DETECT = crate::Reg<fault_detect::FAULT_DETECT_SPEC>;
#[doc = "Fault detection configuration and status register"]
pub mod fault_detect;
#[doc = "CAP_TIMER_CFG (rw) register accessor: Capture timer configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_timer_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap_timer_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_timer_cfg`] module"]
pub type CAP_TIMER_CFG = crate::Reg<cap_timer_cfg::CAP_TIMER_CFG_SPEC>;
#[doc = "Capture timer configuration register"]
pub mod cap_timer_cfg;
#[doc = "CAP_TIMER_PHASE (rw) register accessor: Capture timer sync phase register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_timer_phase::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap_timer_phase::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_timer_phase`] module"]
pub type CAP_TIMER_PHASE = crate::Reg<cap_timer_phase::CAP_TIMER_PHASE_SPEC>;
#[doc = "Capture timer sync phase register"]
pub mod cap_timer_phase;
#[doc = "CAP_CH_CFG (rw) register accessor: Capture channel %s configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_ch_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap_ch_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_ch_cfg`] module"]
pub type CAP_CH_CFG = crate::Reg<cap_ch_cfg::CAP_CH_CFG_SPEC>;
#[doc = "Capture channel %s configuration register"]
pub mod cap_ch_cfg;
#[doc = "CAP_CH (r) register accessor: CAP%s capture value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_ch::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_ch`] module"]
pub type CAP_CH = crate::Reg<cap_ch::CAP_CH_SPEC>;
#[doc = "CAP%s capture value register"]
pub mod cap_ch;
#[doc = "CAP_STATUS (r) register accessor: Last capture trigger edge information register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_status`] module"]
pub type CAP_STATUS = crate::Reg<cap_status::CAP_STATUS_SPEC>;
#[doc = "Last capture trigger edge information register"]
pub mod cap_status;
#[doc = "UPDATE_CFG (rw) register accessor: Generator Update configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`update_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`update_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@update_cfg`] module"]
pub type UPDATE_CFG = crate::Reg<update_cfg::UPDATE_CFG_SPEC>;
#[doc = "Generator Update configuration register"]
pub mod update_cfg;
#[doc = "INT_ENA (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = "Interrupt enable register"]
pub mod int_ena;
#[doc = "INT_RAW (rw) register accessor: Interrupt raw status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = "Interrupt raw status register"]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: Interrupt masked status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = "Interrupt masked status register"]
pub mod int_st;
#[doc = "INT_CLR (w) register accessor: Interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = "Interrupt clear register"]
pub mod int_clr;
#[doc = "EVT_EN (rw) register accessor: Event enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evt_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_en`] module"]
pub type EVT_EN = crate::Reg<evt_en::EVT_EN_SPEC>;
#[doc = "Event enable register"]
pub mod evt_en;
#[doc = "TASK_EN (rw) register accessor: Task enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`task_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`task_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_en`] module"]
pub type TASK_EN = crate::Reg<task_en::TASK_EN_SPEC>;
#[doc = "Task enable register"]
pub mod task_en;
#[doc = "EVT_EN2 (rw) register accessor: Event enable register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evt_en2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_en2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_en2`] module"]
pub type EVT_EN2 = crate::Reg<evt_en2::EVT_EN2_SPEC>;
#[doc = "Event enable register2"]
pub mod evt_en2;
#[doc = "OP_TSTMP_E1 (rw) register accessor: Generator%s timer stamp E1 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`op_tstmp_e1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`op_tstmp_e1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@op_tstmp_e1`] module"]
pub type OP_TSTMP_E1 = crate::Reg<op_tstmp_e1::OP_TSTMP_E1_SPEC>;
#[doc = "Generator%s timer stamp E1 value register"]
pub mod op_tstmp_e1;
#[doc = "OP_TSTMP_E2 (rw) register accessor: Generator%s timer stamp E2 value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`op_tstmp_e2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`op_tstmp_e2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@op_tstmp_e2`] module"]
pub type OP_TSTMP_E2 = crate::Reg<op_tstmp_e2::OP_TSTMP_E2_SPEC>;
#[doc = "Generator%s timer stamp E2 value register"]
pub mod op_tstmp_e2;
#[doc = "CLK (rw) register accessor: Global configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk`] module"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = "Global configuration register"]
pub mod clk;
#[doc = "VERSION (rw) register accessor: Version register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`version::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version`] module"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = "Version register."]
pub mod version;
