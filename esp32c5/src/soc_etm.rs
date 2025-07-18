#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    ch_ena_ad0: CH_ENA_AD0,
    ch_ena_ad0_set: CH_ENA_AD0_SET,
    ch_ena_ad0_clr: CH_ENA_AD0_CLR,
    ch_ena_ad1: CH_ENA_AD1,
    ch_ena_ad1_set: CH_ENA_AD1_SET,
    ch_ena_ad1_clr: CH_ENA_AD1_CLR,
    ch_evt_id: (),
    _reserved7: [u8; 0x04],
    ch_task_id: (),
    _reserved8: [u8; 0x018c],
    evt_st0: EVT_ST0,
    evt_st0_clr: EVT_ST0_CLR,
    evt_st1: EVT_ST1,
    evt_st1_clr: EVT_ST1_CLR,
    evt_st2: EVT_ST2,
    evt_st2_clr: EVT_ST2_CLR,
    evt_st3: EVT_ST3,
    evt_st3_clr: EVT_ST3_CLR,
    evt_st4: EVT_ST4,
    evt_st4_clr: EVT_ST4_CLR,
    task_st0: TASK_ST0,
    task_st0_clr: TASK_ST0_CLR,
    task_st1: TASK_ST1,
    task_st1_clr: TASK_ST1_CLR,
    task_st2: TASK_ST2,
    task_st2_clr: TASK_ST2_CLR,
    task_st3: TASK_ST3,
    task_st3_clr: TASK_ST3_CLR,
    task_st4: TASK_ST4,
    task_st4_clr: TASK_ST4_CLR,
    clk_en: CLK_EN,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - Channel enable status register"]
    #[inline(always)]
    pub const fn ch_ena_ad0(&self) -> &CH_ENA_AD0 {
        &self.ch_ena_ad0
    }
    #[doc = "0x04 - Channel enable set register"]
    #[inline(always)]
    pub const fn ch_ena_ad0_set(&self) -> &CH_ENA_AD0_SET {
        &self.ch_ena_ad0_set
    }
    #[doc = "0x08 - Channel enable clear register"]
    #[inline(always)]
    pub const fn ch_ena_ad0_clr(&self) -> &CH_ENA_AD0_CLR {
        &self.ch_ena_ad0_clr
    }
    #[doc = "0x0c - Channel enable status register"]
    #[inline(always)]
    pub const fn ch_ena_ad1(&self) -> &CH_ENA_AD1 {
        &self.ch_ena_ad1
    }
    #[doc = "0x10 - Channel enable set register"]
    #[inline(always)]
    pub const fn ch_ena_ad1_set(&self) -> &CH_ENA_AD1_SET {
        &self.ch_ena_ad1_set
    }
    #[doc = "0x14 - Channel enable clear register"]
    #[inline(always)]
    pub const fn ch_ena_ad1_clr(&self) -> &CH_ENA_AD1_CLR {
        &self.ch_ena_ad1_clr
    }
    #[doc = "0x18..0xe0 - Channel%s event id register"]
    #[inline(always)]
    pub const fn ch_evt_id(&self, n: usize) -> &CH_EVT_ID {
        #[allow(clippy::no_effect)]
        [(); 50][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(24)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x18..0xe0 - Channel%s event id register"]
    #[inline(always)]
    pub fn ch_evt_id_iter(&self) -> impl Iterator<Item = &CH_EVT_ID> {
        (0..50).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(24)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x18 - Channel0 event id register"]
    #[inline(always)]
    pub const fn ch0_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(0)
    }
    #[doc = "0x20 - Channel1 event id register"]
    #[inline(always)]
    pub const fn ch1_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(1)
    }
    #[doc = "0x28 - Channel2 event id register"]
    #[inline(always)]
    pub const fn ch2_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(2)
    }
    #[doc = "0x30 - Channel3 event id register"]
    #[inline(always)]
    pub const fn ch3_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(3)
    }
    #[doc = "0x38 - Channel4 event id register"]
    #[inline(always)]
    pub const fn ch4_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(4)
    }
    #[doc = "0x40 - Channel5 event id register"]
    #[inline(always)]
    pub const fn ch5_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(5)
    }
    #[doc = "0x48 - Channel6 event id register"]
    #[inline(always)]
    pub const fn ch6_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(6)
    }
    #[doc = "0x50 - Channel7 event id register"]
    #[inline(always)]
    pub const fn ch7_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(7)
    }
    #[doc = "0x58 - Channel8 event id register"]
    #[inline(always)]
    pub const fn ch8_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(8)
    }
    #[doc = "0x60 - Channel9 event id register"]
    #[inline(always)]
    pub const fn ch9_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(9)
    }
    #[doc = "0x68 - Channel10 event id register"]
    #[inline(always)]
    pub const fn ch10_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(10)
    }
    #[doc = "0x70 - Channel11 event id register"]
    #[inline(always)]
    pub const fn ch11_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(11)
    }
    #[doc = "0x78 - Channel12 event id register"]
    #[inline(always)]
    pub const fn ch12_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(12)
    }
    #[doc = "0x80 - Channel13 event id register"]
    #[inline(always)]
    pub const fn ch13_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(13)
    }
    #[doc = "0x88 - Channel14 event id register"]
    #[inline(always)]
    pub const fn ch14_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(14)
    }
    #[doc = "0x90 - Channel15 event id register"]
    #[inline(always)]
    pub const fn ch15_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(15)
    }
    #[doc = "0x98 - Channel16 event id register"]
    #[inline(always)]
    pub const fn ch16_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(16)
    }
    #[doc = "0xa0 - Channel17 event id register"]
    #[inline(always)]
    pub const fn ch17_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(17)
    }
    #[doc = "0xa8 - Channel18 event id register"]
    #[inline(always)]
    pub const fn ch18_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(18)
    }
    #[doc = "0xb0 - Channel19 event id register"]
    #[inline(always)]
    pub const fn ch19_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(19)
    }
    #[doc = "0xb8 - Channel20 event id register"]
    #[inline(always)]
    pub const fn ch20_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(20)
    }
    #[doc = "0xc0 - Channel21 event id register"]
    #[inline(always)]
    pub const fn ch21_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(21)
    }
    #[doc = "0xc8 - Channel22 event id register"]
    #[inline(always)]
    pub const fn ch22_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(22)
    }
    #[doc = "0xd0 - Channel23 event id register"]
    #[inline(always)]
    pub const fn ch23_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(23)
    }
    #[doc = "0xd8 - Channel24 event id register"]
    #[inline(always)]
    pub const fn ch24_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(24)
    }
    #[doc = "0xe0 - Channel25 event id register"]
    #[inline(always)]
    pub const fn ch25_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(25)
    }
    #[doc = "0xe8 - Channel26 event id register"]
    #[inline(always)]
    pub const fn ch26_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(26)
    }
    #[doc = "0xf0 - Channel27 event id register"]
    #[inline(always)]
    pub const fn ch27_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(27)
    }
    #[doc = "0xf8 - Channel28 event id register"]
    #[inline(always)]
    pub const fn ch28_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(28)
    }
    #[doc = "0x100 - Channel29 event id register"]
    #[inline(always)]
    pub const fn ch29_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(29)
    }
    #[doc = "0x108 - Channel30 event id register"]
    #[inline(always)]
    pub const fn ch30_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(30)
    }
    #[doc = "0x110 - Channel31 event id register"]
    #[inline(always)]
    pub const fn ch31_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(31)
    }
    #[doc = "0x118 - Channel32 event id register"]
    #[inline(always)]
    pub const fn ch32_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(32)
    }
    #[doc = "0x120 - Channel33 event id register"]
    #[inline(always)]
    pub const fn ch33_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(33)
    }
    #[doc = "0x128 - Channel34 event id register"]
    #[inline(always)]
    pub const fn ch34_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(34)
    }
    #[doc = "0x130 - Channel35 event id register"]
    #[inline(always)]
    pub const fn ch35_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(35)
    }
    #[doc = "0x138 - Channel36 event id register"]
    #[inline(always)]
    pub const fn ch36_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(36)
    }
    #[doc = "0x140 - Channel37 event id register"]
    #[inline(always)]
    pub const fn ch37_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(37)
    }
    #[doc = "0x148 - Channel38 event id register"]
    #[inline(always)]
    pub const fn ch38_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(38)
    }
    #[doc = "0x150 - Channel39 event id register"]
    #[inline(always)]
    pub const fn ch39_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(39)
    }
    #[doc = "0x158 - Channel40 event id register"]
    #[inline(always)]
    pub const fn ch40_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(40)
    }
    #[doc = "0x160 - Channel41 event id register"]
    #[inline(always)]
    pub const fn ch41_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(41)
    }
    #[doc = "0x168 - Channel42 event id register"]
    #[inline(always)]
    pub const fn ch42_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(42)
    }
    #[doc = "0x170 - Channel43 event id register"]
    #[inline(always)]
    pub const fn ch43_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(43)
    }
    #[doc = "0x178 - Channel44 event id register"]
    #[inline(always)]
    pub const fn ch44_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(44)
    }
    #[doc = "0x180 - Channel45 event id register"]
    #[inline(always)]
    pub const fn ch45_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(45)
    }
    #[doc = "0x188 - Channel46 event id register"]
    #[inline(always)]
    pub const fn ch46_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(46)
    }
    #[doc = "0x190 - Channel47 event id register"]
    #[inline(always)]
    pub const fn ch47_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(47)
    }
    #[doc = "0x198 - Channel48 event id register"]
    #[inline(always)]
    pub const fn ch48_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(48)
    }
    #[doc = "0x1a0 - Channel49 event id register"]
    #[inline(always)]
    pub const fn ch49_evt_id(&self) -> &CH_EVT_ID {
        self.ch_evt_id(49)
    }
    #[doc = "0x1c..0xe4 - Channel%s task id register"]
    #[inline(always)]
    pub const fn ch_task_id(&self, n: usize) -> &CH_TASK_ID {
        #[allow(clippy::no_effect)]
        [(); 50][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(28)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c..0xe4 - Channel%s task id register"]
    #[inline(always)]
    pub fn ch_task_id_iter(&self) -> impl Iterator<Item = &CH_TASK_ID> {
        (0..50).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(28)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x1c - Channel0 task id register"]
    #[inline(always)]
    pub const fn ch0_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(0)
    }
    #[doc = "0x24 - Channel1 task id register"]
    #[inline(always)]
    pub const fn ch1_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(1)
    }
    #[doc = "0x2c - Channel2 task id register"]
    #[inline(always)]
    pub const fn ch2_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(2)
    }
    #[doc = "0x34 - Channel3 task id register"]
    #[inline(always)]
    pub const fn ch3_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(3)
    }
    #[doc = "0x3c - Channel4 task id register"]
    #[inline(always)]
    pub const fn ch4_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(4)
    }
    #[doc = "0x44 - Channel5 task id register"]
    #[inline(always)]
    pub const fn ch5_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(5)
    }
    #[doc = "0x4c - Channel6 task id register"]
    #[inline(always)]
    pub const fn ch6_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(6)
    }
    #[doc = "0x54 - Channel7 task id register"]
    #[inline(always)]
    pub const fn ch7_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(7)
    }
    #[doc = "0x5c - Channel8 task id register"]
    #[inline(always)]
    pub const fn ch8_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(8)
    }
    #[doc = "0x64 - Channel9 task id register"]
    #[inline(always)]
    pub const fn ch9_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(9)
    }
    #[doc = "0x6c - Channel10 task id register"]
    #[inline(always)]
    pub const fn ch10_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(10)
    }
    #[doc = "0x74 - Channel11 task id register"]
    #[inline(always)]
    pub const fn ch11_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(11)
    }
    #[doc = "0x7c - Channel12 task id register"]
    #[inline(always)]
    pub const fn ch12_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(12)
    }
    #[doc = "0x84 - Channel13 task id register"]
    #[inline(always)]
    pub const fn ch13_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(13)
    }
    #[doc = "0x8c - Channel14 task id register"]
    #[inline(always)]
    pub const fn ch14_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(14)
    }
    #[doc = "0x94 - Channel15 task id register"]
    #[inline(always)]
    pub const fn ch15_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(15)
    }
    #[doc = "0x9c - Channel16 task id register"]
    #[inline(always)]
    pub const fn ch16_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(16)
    }
    #[doc = "0xa4 - Channel17 task id register"]
    #[inline(always)]
    pub const fn ch17_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(17)
    }
    #[doc = "0xac - Channel18 task id register"]
    #[inline(always)]
    pub const fn ch18_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(18)
    }
    #[doc = "0xb4 - Channel19 task id register"]
    #[inline(always)]
    pub const fn ch19_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(19)
    }
    #[doc = "0xbc - Channel20 task id register"]
    #[inline(always)]
    pub const fn ch20_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(20)
    }
    #[doc = "0xc4 - Channel21 task id register"]
    #[inline(always)]
    pub const fn ch21_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(21)
    }
    #[doc = "0xcc - Channel22 task id register"]
    #[inline(always)]
    pub const fn ch22_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(22)
    }
    #[doc = "0xd4 - Channel23 task id register"]
    #[inline(always)]
    pub const fn ch23_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(23)
    }
    #[doc = "0xdc - Channel24 task id register"]
    #[inline(always)]
    pub const fn ch24_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(24)
    }
    #[doc = "0xe4 - Channel25 task id register"]
    #[inline(always)]
    pub const fn ch25_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(25)
    }
    #[doc = "0xec - Channel26 task id register"]
    #[inline(always)]
    pub const fn ch26_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(26)
    }
    #[doc = "0xf4 - Channel27 task id register"]
    #[inline(always)]
    pub const fn ch27_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(27)
    }
    #[doc = "0xfc - Channel28 task id register"]
    #[inline(always)]
    pub const fn ch28_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(28)
    }
    #[doc = "0x104 - Channel29 task id register"]
    #[inline(always)]
    pub const fn ch29_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(29)
    }
    #[doc = "0x10c - Channel30 task id register"]
    #[inline(always)]
    pub const fn ch30_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(30)
    }
    #[doc = "0x114 - Channel31 task id register"]
    #[inline(always)]
    pub const fn ch31_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(31)
    }
    #[doc = "0x11c - Channel32 task id register"]
    #[inline(always)]
    pub const fn ch32_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(32)
    }
    #[doc = "0x124 - Channel33 task id register"]
    #[inline(always)]
    pub const fn ch33_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(33)
    }
    #[doc = "0x12c - Channel34 task id register"]
    #[inline(always)]
    pub const fn ch34_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(34)
    }
    #[doc = "0x134 - Channel35 task id register"]
    #[inline(always)]
    pub const fn ch35_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(35)
    }
    #[doc = "0x13c - Channel36 task id register"]
    #[inline(always)]
    pub const fn ch36_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(36)
    }
    #[doc = "0x144 - Channel37 task id register"]
    #[inline(always)]
    pub const fn ch37_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(37)
    }
    #[doc = "0x14c - Channel38 task id register"]
    #[inline(always)]
    pub const fn ch38_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(38)
    }
    #[doc = "0x154 - Channel39 task id register"]
    #[inline(always)]
    pub const fn ch39_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(39)
    }
    #[doc = "0x15c - Channel40 task id register"]
    #[inline(always)]
    pub const fn ch40_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(40)
    }
    #[doc = "0x164 - Channel41 task id register"]
    #[inline(always)]
    pub const fn ch41_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(41)
    }
    #[doc = "0x16c - Channel42 task id register"]
    #[inline(always)]
    pub const fn ch42_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(42)
    }
    #[doc = "0x174 - Channel43 task id register"]
    #[inline(always)]
    pub const fn ch43_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(43)
    }
    #[doc = "0x17c - Channel44 task id register"]
    #[inline(always)]
    pub const fn ch44_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(44)
    }
    #[doc = "0x184 - Channel45 task id register"]
    #[inline(always)]
    pub const fn ch45_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(45)
    }
    #[doc = "0x18c - Channel46 task id register"]
    #[inline(always)]
    pub const fn ch46_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(46)
    }
    #[doc = "0x194 - Channel47 task id register"]
    #[inline(always)]
    pub const fn ch47_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(47)
    }
    #[doc = "0x19c - Channel48 task id register"]
    #[inline(always)]
    pub const fn ch48_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(48)
    }
    #[doc = "0x1a4 - Channel49 task id register"]
    #[inline(always)]
    pub const fn ch49_task_id(&self) -> &CH_TASK_ID {
        self.ch_task_id(49)
    }
    #[doc = "0x1a8 - Events trigger status register"]
    #[inline(always)]
    pub const fn evt_st0(&self) -> &EVT_ST0 {
        &self.evt_st0
    }
    #[doc = "0x1ac - Events trigger status clear register"]
    #[inline(always)]
    pub const fn evt_st0_clr(&self) -> &EVT_ST0_CLR {
        &self.evt_st0_clr
    }
    #[doc = "0x1b0 - Events trigger status register"]
    #[inline(always)]
    pub const fn evt_st1(&self) -> &EVT_ST1 {
        &self.evt_st1
    }
    #[doc = "0x1b4 - Events trigger status clear register"]
    #[inline(always)]
    pub const fn evt_st1_clr(&self) -> &EVT_ST1_CLR {
        &self.evt_st1_clr
    }
    #[doc = "0x1b8 - Events trigger status register"]
    #[inline(always)]
    pub const fn evt_st2(&self) -> &EVT_ST2 {
        &self.evt_st2
    }
    #[doc = "0x1bc - Events trigger status clear register"]
    #[inline(always)]
    pub const fn evt_st2_clr(&self) -> &EVT_ST2_CLR {
        &self.evt_st2_clr
    }
    #[doc = "0x1c0 - Events trigger status register"]
    #[inline(always)]
    pub const fn evt_st3(&self) -> &EVT_ST3 {
        &self.evt_st3
    }
    #[doc = "0x1c4 - Events trigger status clear register"]
    #[inline(always)]
    pub const fn evt_st3_clr(&self) -> &EVT_ST3_CLR {
        &self.evt_st3_clr
    }
    #[doc = "0x1c8 - Events trigger status register"]
    #[inline(always)]
    pub const fn evt_st4(&self) -> &EVT_ST4 {
        &self.evt_st4
    }
    #[doc = "0x1cc - Events trigger status clear register"]
    #[inline(always)]
    pub const fn evt_st4_clr(&self) -> &EVT_ST4_CLR {
        &self.evt_st4_clr
    }
    #[doc = "0x1d0 - Tasks trigger status register"]
    #[inline(always)]
    pub const fn task_st0(&self) -> &TASK_ST0 {
        &self.task_st0
    }
    #[doc = "0x1d4 - Tasks trigger status clear register"]
    #[inline(always)]
    pub const fn task_st0_clr(&self) -> &TASK_ST0_CLR {
        &self.task_st0_clr
    }
    #[doc = "0x1d8 - Tasks trigger status register"]
    #[inline(always)]
    pub const fn task_st1(&self) -> &TASK_ST1 {
        &self.task_st1
    }
    #[doc = "0x1dc - Tasks trigger status clear register"]
    #[inline(always)]
    pub const fn task_st1_clr(&self) -> &TASK_ST1_CLR {
        &self.task_st1_clr
    }
    #[doc = "0x1e0 - Tasks trigger status register"]
    #[inline(always)]
    pub const fn task_st2(&self) -> &TASK_ST2 {
        &self.task_st2
    }
    #[doc = "0x1e4 - Tasks trigger status clear register"]
    #[inline(always)]
    pub const fn task_st2_clr(&self) -> &TASK_ST2_CLR {
        &self.task_st2_clr
    }
    #[doc = "0x1e8 - Tasks trigger status register"]
    #[inline(always)]
    pub const fn task_st3(&self) -> &TASK_ST3 {
        &self.task_st3
    }
    #[doc = "0x1ec - Tasks trigger status clear register"]
    #[inline(always)]
    pub const fn task_st3_clr(&self) -> &TASK_ST3_CLR {
        &self.task_st3_clr
    }
    #[doc = "0x1f0 - Tasks trigger status register"]
    #[inline(always)]
    pub const fn task_st4(&self) -> &TASK_ST4 {
        &self.task_st4
    }
    #[doc = "0x1f4 - Tasks trigger status clear register"]
    #[inline(always)]
    pub const fn task_st4_clr(&self) -> &TASK_ST4_CLR {
        &self.task_st4_clr
    }
    #[doc = "0x1f8 - ETM clock enable register"]
    #[inline(always)]
    pub const fn clk_en(&self) -> &CLK_EN {
        &self.clk_en
    }
    #[doc = "0x1fc - ETM date register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "CH_ENA_AD0 (rw) register accessor: Channel enable status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_ena_ad0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ena_ad0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ena_ad0`] module"]
pub type CH_ENA_AD0 = crate::Reg<ch_ena_ad0::CH_ENA_AD0_SPEC>;
#[doc = "Channel enable status register"]
pub mod ch_ena_ad0;
#[doc = "CH_ENA_AD0_SET (w) register accessor: Channel enable set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ena_ad0_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ena_ad0_set`] module"]
pub type CH_ENA_AD0_SET = crate::Reg<ch_ena_ad0_set::CH_ENA_AD0_SET_SPEC>;
#[doc = "Channel enable set register"]
pub mod ch_ena_ad0_set;
#[doc = "CH_ENA_AD0_CLR (w) register accessor: Channel enable clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ena_ad0_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ena_ad0_clr`] module"]
pub type CH_ENA_AD0_CLR = crate::Reg<ch_ena_ad0_clr::CH_ENA_AD0_CLR_SPEC>;
#[doc = "Channel enable clear register"]
pub mod ch_ena_ad0_clr;
#[doc = "CH_ENA_AD1 (rw) register accessor: Channel enable status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_ena_ad1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ena_ad1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ena_ad1`] module"]
pub type CH_ENA_AD1 = crate::Reg<ch_ena_ad1::CH_ENA_AD1_SPEC>;
#[doc = "Channel enable status register"]
pub mod ch_ena_ad1;
#[doc = "CH_ENA_AD1_SET (w) register accessor: Channel enable set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ena_ad1_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ena_ad1_set`] module"]
pub type CH_ENA_AD1_SET = crate::Reg<ch_ena_ad1_set::CH_ENA_AD1_SET_SPEC>;
#[doc = "Channel enable set register"]
pub mod ch_ena_ad1_set;
#[doc = "CH_ENA_AD1_CLR (w) register accessor: Channel enable clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ena_ad1_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_ena_ad1_clr`] module"]
pub type CH_ENA_AD1_CLR = crate::Reg<ch_ena_ad1_clr::CH_ENA_AD1_CLR_SPEC>;
#[doc = "Channel enable clear register"]
pub mod ch_ena_ad1_clr;
#[doc = "CH_EVT_ID (rw) register accessor: Channel%s event id register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_evt_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_evt_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_evt_id`] module"]
pub type CH_EVT_ID = crate::Reg<ch_evt_id::CH_EVT_ID_SPEC>;
#[doc = "Channel%s event id register"]
pub mod ch_evt_id;
#[doc = "CH_TASK_ID (rw) register accessor: Channel%s task id register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_task_id::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_task_id::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_task_id`] module"]
pub type CH_TASK_ID = crate::Reg<ch_task_id::CH_TASK_ID_SPEC>;
#[doc = "Channel%s task id register"]
pub mod ch_task_id;
#[doc = "EVT_ST0 (rw) register accessor: Events trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_st0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st0`] module"]
pub type EVT_ST0 = crate::Reg<evt_st0::EVT_ST0_SPEC>;
#[doc = "Events trigger status register"]
pub mod evt_st0;
#[doc = "EVT_ST0_CLR (w) register accessor: Events trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st0_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st0_clr`] module"]
pub type EVT_ST0_CLR = crate::Reg<evt_st0_clr::EVT_ST0_CLR_SPEC>;
#[doc = "Events trigger status clear register"]
pub mod evt_st0_clr;
#[doc = "EVT_ST1 (rw) register accessor: Events trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_st1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st1`] module"]
pub type EVT_ST1 = crate::Reg<evt_st1::EVT_ST1_SPEC>;
#[doc = "Events trigger status register"]
pub mod evt_st1;
#[doc = "EVT_ST1_CLR (w) register accessor: Events trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st1_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st1_clr`] module"]
pub type EVT_ST1_CLR = crate::Reg<evt_st1_clr::EVT_ST1_CLR_SPEC>;
#[doc = "Events trigger status clear register"]
pub mod evt_st1_clr;
#[doc = "EVT_ST2 (rw) register accessor: Events trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_st2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st2`] module"]
pub type EVT_ST2 = crate::Reg<evt_st2::EVT_ST2_SPEC>;
#[doc = "Events trigger status register"]
pub mod evt_st2;
#[doc = "EVT_ST2_CLR (w) register accessor: Events trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st2_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st2_clr`] module"]
pub type EVT_ST2_CLR = crate::Reg<evt_st2_clr::EVT_ST2_CLR_SPEC>;
#[doc = "Events trigger status clear register"]
pub mod evt_st2_clr;
#[doc = "EVT_ST3 (rw) register accessor: Events trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_st3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st3`] module"]
pub type EVT_ST3 = crate::Reg<evt_st3::EVT_ST3_SPEC>;
#[doc = "Events trigger status register"]
pub mod evt_st3;
#[doc = "EVT_ST3_CLR (w) register accessor: Events trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st3_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st3_clr`] module"]
pub type EVT_ST3_CLR = crate::Reg<evt_st3_clr::EVT_ST3_CLR_SPEC>;
#[doc = "Events trigger status clear register"]
pub mod evt_st3_clr;
#[doc = "EVT_ST4 (rw) register accessor: Events trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_st4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st4`] module"]
pub type EVT_ST4 = crate::Reg<evt_st4::EVT_ST4_SPEC>;
#[doc = "Events trigger status register"]
pub mod evt_st4;
#[doc = "EVT_ST4_CLR (w) register accessor: Events trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st4_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evt_st4_clr`] module"]
pub type EVT_ST4_CLR = crate::Reg<evt_st4_clr::EVT_ST4_CLR_SPEC>;
#[doc = "Events trigger status clear register"]
pub mod evt_st4_clr;
#[doc = "TASK_ST0 (rw) register accessor: Tasks trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`task_st0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st0`] module"]
pub type TASK_ST0 = crate::Reg<task_st0::TASK_ST0_SPEC>;
#[doc = "Tasks trigger status register"]
pub mod task_st0;
#[doc = "TASK_ST0_CLR (w) register accessor: Tasks trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st0_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st0_clr`] module"]
pub type TASK_ST0_CLR = crate::Reg<task_st0_clr::TASK_ST0_CLR_SPEC>;
#[doc = "Tasks trigger status clear register"]
pub mod task_st0_clr;
#[doc = "TASK_ST1 (rw) register accessor: Tasks trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`task_st1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st1`] module"]
pub type TASK_ST1 = crate::Reg<task_st1::TASK_ST1_SPEC>;
#[doc = "Tasks trigger status register"]
pub mod task_st1;
#[doc = "TASK_ST1_CLR (w) register accessor: Tasks trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st1_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st1_clr`] module"]
pub type TASK_ST1_CLR = crate::Reg<task_st1_clr::TASK_ST1_CLR_SPEC>;
#[doc = "Tasks trigger status clear register"]
pub mod task_st1_clr;
#[doc = "TASK_ST2 (rw) register accessor: Tasks trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`task_st2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st2`] module"]
pub type TASK_ST2 = crate::Reg<task_st2::TASK_ST2_SPEC>;
#[doc = "Tasks trigger status register"]
pub mod task_st2;
#[doc = "TASK_ST2_CLR (w) register accessor: Tasks trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st2_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st2_clr`] module"]
pub type TASK_ST2_CLR = crate::Reg<task_st2_clr::TASK_ST2_CLR_SPEC>;
#[doc = "Tasks trigger status clear register"]
pub mod task_st2_clr;
#[doc = "TASK_ST3 (rw) register accessor: Tasks trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`task_st3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st3`] module"]
pub type TASK_ST3 = crate::Reg<task_st3::TASK_ST3_SPEC>;
#[doc = "Tasks trigger status register"]
pub mod task_st3;
#[doc = "TASK_ST3_CLR (w) register accessor: Tasks trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st3_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st3_clr`] module"]
pub type TASK_ST3_CLR = crate::Reg<task_st3_clr::TASK_ST3_CLR_SPEC>;
#[doc = "Tasks trigger status clear register"]
pub mod task_st3_clr;
#[doc = "TASK_ST4 (rw) register accessor: Tasks trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`task_st4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st4`] module"]
pub type TASK_ST4 = crate::Reg<task_st4::TASK_ST4_SPEC>;
#[doc = "Tasks trigger status register"]
pub mod task_st4;
#[doc = "TASK_ST4_CLR (w) register accessor: Tasks trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st4_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@task_st4_clr`] module"]
pub type TASK_ST4_CLR = crate::Reg<task_st4_clr::TASK_ST4_CLR_SPEC>;
#[doc = "Tasks trigger status clear register"]
pub mod task_st4_clr;
#[doc = "CLK_EN (rw) register accessor: ETM clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_en`] module"]
pub type CLK_EN = crate::Reg<clk_en::CLK_EN_SPEC>;
#[doc = "ETM clock enable register"]
pub mod clk_en;
pub use crate::aes::date;
pub use crate::aes::DATE;
