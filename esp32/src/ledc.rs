#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    hsch_conf0: (),
    _reserved1: [u8; 0x04],
    hsch_hpoint: (),
    _reserved2: [u8; 0x04],
    hsch_duty: (),
    _reserved3: [u8; 0x04],
    hsch_conf1: (),
    _reserved4: [u8; 0x04],
    hsch_duty_r: (),
    _reserved5: [u8; 0x90],
    lsch_conf0: (),
    _reserved6: [u8; 0x04],
    lsch_hpoint: (),
    _reserved7: [u8; 0x04],
    lsch_duty: (),
    _reserved8: [u8; 0x04],
    lsch_conf1: (),
    _reserved9: [u8; 0x04],
    lsch_duty_r: (),
    _reserved10: [u8; 0x90],
    hstimer_conf: (),
    _reserved11: [u8; 0x04],
    hstimer_value: (),
    _reserved12: [u8; 0x1c],
    lstimer_conf: (),
    _reserved13: [u8; 0x04],
    lstimer_value: (),
    _reserved14: [u8; 0x1c],
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    conf: CONF,
    _reserved19: [u8; 0x68],
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - "]
    #[inline(always)]
    pub const fn hsch_conf0(&self, n: usize) -> &HSCH_CONF0 {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(0).add(20 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - "]
    #[inline(always)]
    pub fn hsch_conf0_iter(&self) -> impl Iterator<Item = &HSCH_CONF0> {
        (0..8)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(0).add(20 * n).cast() })
    }
    #[doc = "0x00 - HSCH0_CONF0"]
    #[inline(always)]
    pub const fn hsch0_conf0(&self) -> &HSCH_CONF0 {
        self.hsch_conf0(0)
    }
    #[doc = "0x14 - HSCH1_CONF0"]
    #[inline(always)]
    pub const fn hsch1_conf0(&self) -> &HSCH_CONF0 {
        self.hsch_conf0(1)
    }
    #[doc = "0x28 - HSCH2_CONF0"]
    #[inline(always)]
    pub const fn hsch2_conf0(&self) -> &HSCH_CONF0 {
        self.hsch_conf0(2)
    }
    #[doc = "0x3c - HSCH3_CONF0"]
    #[inline(always)]
    pub const fn hsch3_conf0(&self) -> &HSCH_CONF0 {
        self.hsch_conf0(3)
    }
    #[doc = "0x50 - HSCH4_CONF0"]
    #[inline(always)]
    pub const fn hsch4_conf0(&self) -> &HSCH_CONF0 {
        self.hsch_conf0(4)
    }
    #[doc = "0x64 - HSCH5_CONF0"]
    #[inline(always)]
    pub const fn hsch5_conf0(&self) -> &HSCH_CONF0 {
        self.hsch_conf0(5)
    }
    #[doc = "0x78 - HSCH6_CONF0"]
    #[inline(always)]
    pub const fn hsch6_conf0(&self) -> &HSCH_CONF0 {
        self.hsch_conf0(6)
    }
    #[doc = "0x8c - HSCH7_CONF0"]
    #[inline(always)]
    pub const fn hsch7_conf0(&self) -> &HSCH_CONF0 {
        self.hsch_conf0(7)
    }
    #[doc = "0x04..0x24 - "]
    #[inline(always)]
    pub const fn hsch_hpoint(&self, n: usize) -> &HSCH_HPOINT {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(4).add(20 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x24 - "]
    #[inline(always)]
    pub fn hsch_hpoint_iter(&self) -> impl Iterator<Item = &HSCH_HPOINT> {
        (0..8)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(4).add(20 * n).cast() })
    }
    #[doc = "0x04 - HSCH0_HPOINT"]
    #[inline(always)]
    pub const fn hsch0_hpoint(&self) -> &HSCH_HPOINT {
        self.hsch_hpoint(0)
    }
    #[doc = "0x18 - HSCH1_HPOINT"]
    #[inline(always)]
    pub const fn hsch1_hpoint(&self) -> &HSCH_HPOINT {
        self.hsch_hpoint(1)
    }
    #[doc = "0x2c - HSCH2_HPOINT"]
    #[inline(always)]
    pub const fn hsch2_hpoint(&self) -> &HSCH_HPOINT {
        self.hsch_hpoint(2)
    }
    #[doc = "0x40 - HSCH3_HPOINT"]
    #[inline(always)]
    pub const fn hsch3_hpoint(&self) -> &HSCH_HPOINT {
        self.hsch_hpoint(3)
    }
    #[doc = "0x54 - HSCH4_HPOINT"]
    #[inline(always)]
    pub const fn hsch4_hpoint(&self) -> &HSCH_HPOINT {
        self.hsch_hpoint(4)
    }
    #[doc = "0x68 - HSCH5_HPOINT"]
    #[inline(always)]
    pub const fn hsch5_hpoint(&self) -> &HSCH_HPOINT {
        self.hsch_hpoint(5)
    }
    #[doc = "0x7c - HSCH6_HPOINT"]
    #[inline(always)]
    pub const fn hsch6_hpoint(&self) -> &HSCH_HPOINT {
        self.hsch_hpoint(6)
    }
    #[doc = "0x90 - HSCH7_HPOINT"]
    #[inline(always)]
    pub const fn hsch7_hpoint(&self) -> &HSCH_HPOINT {
        self.hsch_hpoint(7)
    }
    #[doc = "0x08..0x28 - "]
    #[inline(always)]
    pub const fn hsch_duty(&self, n: usize) -> &HSCH_DUTY {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(8).add(20 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x28 - "]
    #[inline(always)]
    pub fn hsch_duty_iter(&self) -> impl Iterator<Item = &HSCH_DUTY> {
        (0..8)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(8).add(20 * n).cast() })
    }
    #[doc = "0x08 - HSCH0_DUTY"]
    #[inline(always)]
    pub const fn hsch0_duty(&self) -> &HSCH_DUTY {
        self.hsch_duty(0)
    }
    #[doc = "0x1c - HSCH1_DUTY"]
    #[inline(always)]
    pub const fn hsch1_duty(&self) -> &HSCH_DUTY {
        self.hsch_duty(1)
    }
    #[doc = "0x30 - HSCH2_DUTY"]
    #[inline(always)]
    pub const fn hsch2_duty(&self) -> &HSCH_DUTY {
        self.hsch_duty(2)
    }
    #[doc = "0x44 - HSCH3_DUTY"]
    #[inline(always)]
    pub const fn hsch3_duty(&self) -> &HSCH_DUTY {
        self.hsch_duty(3)
    }
    #[doc = "0x58 - HSCH4_DUTY"]
    #[inline(always)]
    pub const fn hsch4_duty(&self) -> &HSCH_DUTY {
        self.hsch_duty(4)
    }
    #[doc = "0x6c - HSCH5_DUTY"]
    #[inline(always)]
    pub const fn hsch5_duty(&self) -> &HSCH_DUTY {
        self.hsch_duty(5)
    }
    #[doc = "0x80 - HSCH6_DUTY"]
    #[inline(always)]
    pub const fn hsch6_duty(&self) -> &HSCH_DUTY {
        self.hsch_duty(6)
    }
    #[doc = "0x94 - HSCH7_DUTY"]
    #[inline(always)]
    pub const fn hsch7_duty(&self) -> &HSCH_DUTY {
        self.hsch_duty(7)
    }
    #[doc = "0x0c..0x2c - "]
    #[inline(always)]
    pub const fn hsch_conf1(&self, n: usize) -> &HSCH_CONF1 {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(12)
                .add(20 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x2c - "]
    #[inline(always)]
    pub fn hsch_conf1_iter(&self) -> impl Iterator<Item = &HSCH_CONF1> {
        (0..8).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(12)
                .add(20 * n)
                .cast()
        })
    }
    #[doc = "0x0c - HSCH0_CONF1"]
    #[inline(always)]
    pub const fn hsch0_conf1(&self) -> &HSCH_CONF1 {
        self.hsch_conf1(0)
    }
    #[doc = "0x20 - HSCH1_CONF1"]
    #[inline(always)]
    pub const fn hsch1_conf1(&self) -> &HSCH_CONF1 {
        self.hsch_conf1(1)
    }
    #[doc = "0x34 - HSCH2_CONF1"]
    #[inline(always)]
    pub const fn hsch2_conf1(&self) -> &HSCH_CONF1 {
        self.hsch_conf1(2)
    }
    #[doc = "0x48 - HSCH3_CONF1"]
    #[inline(always)]
    pub const fn hsch3_conf1(&self) -> &HSCH_CONF1 {
        self.hsch_conf1(3)
    }
    #[doc = "0x5c - HSCH4_CONF1"]
    #[inline(always)]
    pub const fn hsch4_conf1(&self) -> &HSCH_CONF1 {
        self.hsch_conf1(4)
    }
    #[doc = "0x70 - HSCH5_CONF1"]
    #[inline(always)]
    pub const fn hsch5_conf1(&self) -> &HSCH_CONF1 {
        self.hsch_conf1(5)
    }
    #[doc = "0x84 - HSCH6_CONF1"]
    #[inline(always)]
    pub const fn hsch6_conf1(&self) -> &HSCH_CONF1 {
        self.hsch_conf1(6)
    }
    #[doc = "0x98 - HSCH7_CONF1"]
    #[inline(always)]
    pub const fn hsch7_conf1(&self) -> &HSCH_CONF1 {
        self.hsch_conf1(7)
    }
    #[doc = "0x10..0x30 - "]
    #[inline(always)]
    pub const fn hsch_duty_r(&self, n: usize) -> &HSCH_DUTY_R {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(16)
                .add(20 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x30 - "]
    #[inline(always)]
    pub fn hsch_duty_r_iter(&self) -> impl Iterator<Item = &HSCH_DUTY_R> {
        (0..8).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(16)
                .add(20 * n)
                .cast()
        })
    }
    #[doc = "0x10 - HSCH0_DUTY_R"]
    #[inline(always)]
    pub const fn hsch0_duty_r(&self) -> &HSCH_DUTY_R {
        self.hsch_duty_r(0)
    }
    #[doc = "0x24 - HSCH1_DUTY_R"]
    #[inline(always)]
    pub const fn hsch1_duty_r(&self) -> &HSCH_DUTY_R {
        self.hsch_duty_r(1)
    }
    #[doc = "0x38 - HSCH2_DUTY_R"]
    #[inline(always)]
    pub const fn hsch2_duty_r(&self) -> &HSCH_DUTY_R {
        self.hsch_duty_r(2)
    }
    #[doc = "0x4c - HSCH3_DUTY_R"]
    #[inline(always)]
    pub const fn hsch3_duty_r(&self) -> &HSCH_DUTY_R {
        self.hsch_duty_r(3)
    }
    #[doc = "0x60 - HSCH4_DUTY_R"]
    #[inline(always)]
    pub const fn hsch4_duty_r(&self) -> &HSCH_DUTY_R {
        self.hsch_duty_r(4)
    }
    #[doc = "0x74 - HSCH5_DUTY_R"]
    #[inline(always)]
    pub const fn hsch5_duty_r(&self) -> &HSCH_DUTY_R {
        self.hsch_duty_r(5)
    }
    #[doc = "0x88 - HSCH6_DUTY_R"]
    #[inline(always)]
    pub const fn hsch6_duty_r(&self) -> &HSCH_DUTY_R {
        self.hsch_duty_r(6)
    }
    #[doc = "0x9c - HSCH7_DUTY_R"]
    #[inline(always)]
    pub const fn hsch7_duty_r(&self) -> &HSCH_DUTY_R {
        self.hsch_duty_r(7)
    }
    #[doc = "0xa0..0xc0 - "]
    #[inline(always)]
    pub const fn lsch_conf0(&self, n: usize) -> &LSCH_CONF0 {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(160)
                .add(20 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa0..0xc0 - "]
    #[inline(always)]
    pub fn lsch_conf0_iter(&self) -> impl Iterator<Item = &LSCH_CONF0> {
        (0..8).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(160)
                .add(20 * n)
                .cast()
        })
    }
    #[doc = "0xa0 - LSCH0_CONF0"]
    #[inline(always)]
    pub const fn lsch0_conf0(&self) -> &LSCH_CONF0 {
        self.lsch_conf0(0)
    }
    #[doc = "0xb4 - LSCH1_CONF0"]
    #[inline(always)]
    pub const fn lsch1_conf0(&self) -> &LSCH_CONF0 {
        self.lsch_conf0(1)
    }
    #[doc = "0xc8 - LSCH2_CONF0"]
    #[inline(always)]
    pub const fn lsch2_conf0(&self) -> &LSCH_CONF0 {
        self.lsch_conf0(2)
    }
    #[doc = "0xdc - LSCH3_CONF0"]
    #[inline(always)]
    pub const fn lsch3_conf0(&self) -> &LSCH_CONF0 {
        self.lsch_conf0(3)
    }
    #[doc = "0xf0 - LSCH4_CONF0"]
    #[inline(always)]
    pub const fn lsch4_conf0(&self) -> &LSCH_CONF0 {
        self.lsch_conf0(4)
    }
    #[doc = "0x104 - LSCH5_CONF0"]
    #[inline(always)]
    pub const fn lsch5_conf0(&self) -> &LSCH_CONF0 {
        self.lsch_conf0(5)
    }
    #[doc = "0x118 - LSCH6_CONF0"]
    #[inline(always)]
    pub const fn lsch6_conf0(&self) -> &LSCH_CONF0 {
        self.lsch_conf0(6)
    }
    #[doc = "0x12c - LSCH7_CONF0"]
    #[inline(always)]
    pub const fn lsch7_conf0(&self) -> &LSCH_CONF0 {
        self.lsch_conf0(7)
    }
    #[doc = "0xa4..0xc4 - "]
    #[inline(always)]
    pub const fn lsch_hpoint(&self, n: usize) -> &LSCH_HPOINT {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(164)
                .add(20 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa4..0xc4 - "]
    #[inline(always)]
    pub fn lsch_hpoint_iter(&self) -> impl Iterator<Item = &LSCH_HPOINT> {
        (0..8).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(164)
                .add(20 * n)
                .cast()
        })
    }
    #[doc = "0xa4 - LSCH0_HPOINT"]
    #[inline(always)]
    pub const fn lsch0_hpoint(&self) -> &LSCH_HPOINT {
        self.lsch_hpoint(0)
    }
    #[doc = "0xb8 - LSCH1_HPOINT"]
    #[inline(always)]
    pub const fn lsch1_hpoint(&self) -> &LSCH_HPOINT {
        self.lsch_hpoint(1)
    }
    #[doc = "0xcc - LSCH2_HPOINT"]
    #[inline(always)]
    pub const fn lsch2_hpoint(&self) -> &LSCH_HPOINT {
        self.lsch_hpoint(2)
    }
    #[doc = "0xe0 - LSCH3_HPOINT"]
    #[inline(always)]
    pub const fn lsch3_hpoint(&self) -> &LSCH_HPOINT {
        self.lsch_hpoint(3)
    }
    #[doc = "0xf4 - LSCH4_HPOINT"]
    #[inline(always)]
    pub const fn lsch4_hpoint(&self) -> &LSCH_HPOINT {
        self.lsch_hpoint(4)
    }
    #[doc = "0x108 - LSCH5_HPOINT"]
    #[inline(always)]
    pub const fn lsch5_hpoint(&self) -> &LSCH_HPOINT {
        self.lsch_hpoint(5)
    }
    #[doc = "0x11c - LSCH6_HPOINT"]
    #[inline(always)]
    pub const fn lsch6_hpoint(&self) -> &LSCH_HPOINT {
        self.lsch_hpoint(6)
    }
    #[doc = "0x130 - LSCH7_HPOINT"]
    #[inline(always)]
    pub const fn lsch7_hpoint(&self) -> &LSCH_HPOINT {
        self.lsch_hpoint(7)
    }
    #[doc = "0xa8..0xc8 - "]
    #[inline(always)]
    pub const fn lsch_duty(&self, n: usize) -> &LSCH_DUTY {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(168)
                .add(20 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa8..0xc8 - "]
    #[inline(always)]
    pub fn lsch_duty_iter(&self) -> impl Iterator<Item = &LSCH_DUTY> {
        (0..8).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(168)
                .add(20 * n)
                .cast()
        })
    }
    #[doc = "0xa8 - LSCH0_DUTY"]
    #[inline(always)]
    pub const fn lsch0_duty(&self) -> &LSCH_DUTY {
        self.lsch_duty(0)
    }
    #[doc = "0xbc - LSCH1_DUTY"]
    #[inline(always)]
    pub const fn lsch1_duty(&self) -> &LSCH_DUTY {
        self.lsch_duty(1)
    }
    #[doc = "0xd0 - LSCH2_DUTY"]
    #[inline(always)]
    pub const fn lsch2_duty(&self) -> &LSCH_DUTY {
        self.lsch_duty(2)
    }
    #[doc = "0xe4 - LSCH3_DUTY"]
    #[inline(always)]
    pub const fn lsch3_duty(&self) -> &LSCH_DUTY {
        self.lsch_duty(3)
    }
    #[doc = "0xf8 - LSCH4_DUTY"]
    #[inline(always)]
    pub const fn lsch4_duty(&self) -> &LSCH_DUTY {
        self.lsch_duty(4)
    }
    #[doc = "0x10c - LSCH5_DUTY"]
    #[inline(always)]
    pub const fn lsch5_duty(&self) -> &LSCH_DUTY {
        self.lsch_duty(5)
    }
    #[doc = "0x120 - LSCH6_DUTY"]
    #[inline(always)]
    pub const fn lsch6_duty(&self) -> &LSCH_DUTY {
        self.lsch_duty(6)
    }
    #[doc = "0x134 - LSCH7_DUTY"]
    #[inline(always)]
    pub const fn lsch7_duty(&self) -> &LSCH_DUTY {
        self.lsch_duty(7)
    }
    #[doc = "0xac..0xcc - "]
    #[inline(always)]
    pub const fn lsch_conf1(&self, n: usize) -> &LSCH_CONF1 {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(172)
                .add(20 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xac..0xcc - "]
    #[inline(always)]
    pub fn lsch_conf1_iter(&self) -> impl Iterator<Item = &LSCH_CONF1> {
        (0..8).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(172)
                .add(20 * n)
                .cast()
        })
    }
    #[doc = "0xac - LSCH0_CONF1"]
    #[inline(always)]
    pub const fn lsch0_conf1(&self) -> &LSCH_CONF1 {
        self.lsch_conf1(0)
    }
    #[doc = "0xc0 - LSCH1_CONF1"]
    #[inline(always)]
    pub const fn lsch1_conf1(&self) -> &LSCH_CONF1 {
        self.lsch_conf1(1)
    }
    #[doc = "0xd4 - LSCH2_CONF1"]
    #[inline(always)]
    pub const fn lsch2_conf1(&self) -> &LSCH_CONF1 {
        self.lsch_conf1(2)
    }
    #[doc = "0xe8 - LSCH3_CONF1"]
    #[inline(always)]
    pub const fn lsch3_conf1(&self) -> &LSCH_CONF1 {
        self.lsch_conf1(3)
    }
    #[doc = "0xfc - LSCH4_CONF1"]
    #[inline(always)]
    pub const fn lsch4_conf1(&self) -> &LSCH_CONF1 {
        self.lsch_conf1(4)
    }
    #[doc = "0x110 - LSCH5_CONF1"]
    #[inline(always)]
    pub const fn lsch5_conf1(&self) -> &LSCH_CONF1 {
        self.lsch_conf1(5)
    }
    #[doc = "0x124 - LSCH6_CONF1"]
    #[inline(always)]
    pub const fn lsch6_conf1(&self) -> &LSCH_CONF1 {
        self.lsch_conf1(6)
    }
    #[doc = "0x138 - LSCH7_CONF1"]
    #[inline(always)]
    pub const fn lsch7_conf1(&self) -> &LSCH_CONF1 {
        self.lsch_conf1(7)
    }
    #[doc = "0xb0..0xd0 - "]
    #[inline(always)]
    pub const fn lsch_duty_r(&self, n: usize) -> &LSCH_DUTY_R {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(176)
                .add(20 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xb0..0xd0 - "]
    #[inline(always)]
    pub fn lsch_duty_r_iter(&self) -> impl Iterator<Item = &LSCH_DUTY_R> {
        (0..8).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(176)
                .add(20 * n)
                .cast()
        })
    }
    #[doc = "0xb0 - LSCH0_DUTY_R"]
    #[inline(always)]
    pub const fn lsch0_duty_r(&self) -> &LSCH_DUTY_R {
        self.lsch_duty_r(0)
    }
    #[doc = "0xc4 - LSCH1_DUTY_R"]
    #[inline(always)]
    pub const fn lsch1_duty_r(&self) -> &LSCH_DUTY_R {
        self.lsch_duty_r(1)
    }
    #[doc = "0xd8 - LSCH2_DUTY_R"]
    #[inline(always)]
    pub const fn lsch2_duty_r(&self) -> &LSCH_DUTY_R {
        self.lsch_duty_r(2)
    }
    #[doc = "0xec - LSCH3_DUTY_R"]
    #[inline(always)]
    pub const fn lsch3_duty_r(&self) -> &LSCH_DUTY_R {
        self.lsch_duty_r(3)
    }
    #[doc = "0x100 - LSCH4_DUTY_R"]
    #[inline(always)]
    pub const fn lsch4_duty_r(&self) -> &LSCH_DUTY_R {
        self.lsch_duty_r(4)
    }
    #[doc = "0x114 - LSCH5_DUTY_R"]
    #[inline(always)]
    pub const fn lsch5_duty_r(&self) -> &LSCH_DUTY_R {
        self.lsch_duty_r(5)
    }
    #[doc = "0x128 - LSCH6_DUTY_R"]
    #[inline(always)]
    pub const fn lsch6_duty_r(&self) -> &LSCH_DUTY_R {
        self.lsch_duty_r(6)
    }
    #[doc = "0x13c - LSCH7_DUTY_R"]
    #[inline(always)]
    pub const fn lsch7_duty_r(&self) -> &LSCH_DUTY_R {
        self.lsch_duty_r(7)
    }
    #[doc = "0x140..0x150 - "]
    #[inline(always)]
    pub const fn hstimer_conf(&self, n: usize) -> &HSTIMER_CONF {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(320)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x150 - "]
    #[inline(always)]
    pub fn hstimer_conf_iter(&self) -> impl Iterator<Item = &HSTIMER_CONF> {
        (0..4).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(320)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x140 - HSTIMER0_CONF"]
    #[inline(always)]
    pub const fn hstimer0_conf(&self) -> &HSTIMER_CONF {
        self.hstimer_conf(0)
    }
    #[doc = "0x148 - HSTIMER1_CONF"]
    #[inline(always)]
    pub const fn hstimer1_conf(&self) -> &HSTIMER_CONF {
        self.hstimer_conf(1)
    }
    #[doc = "0x150 - HSTIMER2_CONF"]
    #[inline(always)]
    pub const fn hstimer2_conf(&self) -> &HSTIMER_CONF {
        self.hstimer_conf(2)
    }
    #[doc = "0x158 - HSTIMER3_CONF"]
    #[inline(always)]
    pub const fn hstimer3_conf(&self) -> &HSTIMER_CONF {
        self.hstimer_conf(3)
    }
    #[doc = "0x144..0x154 - "]
    #[inline(always)]
    pub const fn hstimer_value(&self, n: usize) -> &HSTIMER_VALUE {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(324)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x144..0x154 - "]
    #[inline(always)]
    pub fn hstimer_value_iter(&self) -> impl Iterator<Item = &HSTIMER_VALUE> {
        (0..4).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(324)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x144 - HSTIMER0_VALUE"]
    #[inline(always)]
    pub const fn hstimer0_value(&self) -> &HSTIMER_VALUE {
        self.hstimer_value(0)
    }
    #[doc = "0x14c - HSTIMER1_VALUE"]
    #[inline(always)]
    pub const fn hstimer1_value(&self) -> &HSTIMER_VALUE {
        self.hstimer_value(1)
    }
    #[doc = "0x154 - HSTIMER2_VALUE"]
    #[inline(always)]
    pub const fn hstimer2_value(&self) -> &HSTIMER_VALUE {
        self.hstimer_value(2)
    }
    #[doc = "0x15c - HSTIMER3_VALUE"]
    #[inline(always)]
    pub const fn hstimer3_value(&self) -> &HSTIMER_VALUE {
        self.hstimer_value(3)
    }
    #[doc = "0x160..0x170 - "]
    #[inline(always)]
    pub const fn lstimer_conf(&self, n: usize) -> &LSTIMER_CONF {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(352)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x160..0x170 - "]
    #[inline(always)]
    pub fn lstimer_conf_iter(&self) -> impl Iterator<Item = &LSTIMER_CONF> {
        (0..4).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(352)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x160 - LSTIMER0_CONF"]
    #[inline(always)]
    pub const fn lstimer0_conf(&self) -> &LSTIMER_CONF {
        self.lstimer_conf(0)
    }
    #[doc = "0x168 - LSTIMER1_CONF"]
    #[inline(always)]
    pub const fn lstimer1_conf(&self) -> &LSTIMER_CONF {
        self.lstimer_conf(1)
    }
    #[doc = "0x170 - LSTIMER2_CONF"]
    #[inline(always)]
    pub const fn lstimer2_conf(&self) -> &LSTIMER_CONF {
        self.lstimer_conf(2)
    }
    #[doc = "0x178 - LSTIMER3_CONF"]
    #[inline(always)]
    pub const fn lstimer3_conf(&self) -> &LSTIMER_CONF {
        self.lstimer_conf(3)
    }
    #[doc = "0x164..0x174 - "]
    #[inline(always)]
    pub const fn lstimer_value(&self, n: usize) -> &LSTIMER_VALUE {
        #[allow(clippy::no_effect)]
        [(); 4][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(356)
                .add(8 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x164..0x174 - "]
    #[inline(always)]
    pub fn lstimer_value_iter(&self) -> impl Iterator<Item = &LSTIMER_VALUE> {
        (0..4).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(356)
                .add(8 * n)
                .cast()
        })
    }
    #[doc = "0x164 - LSTIMER0_VALUE"]
    #[inline(always)]
    pub const fn lstimer0_value(&self) -> &LSTIMER_VALUE {
        self.lstimer_value(0)
    }
    #[doc = "0x16c - LSTIMER1_VALUE"]
    #[inline(always)]
    pub const fn lstimer1_value(&self) -> &LSTIMER_VALUE {
        self.lstimer_value(1)
    }
    #[doc = "0x174 - LSTIMER2_VALUE"]
    #[inline(always)]
    pub const fn lstimer2_value(&self) -> &LSTIMER_VALUE {
        self.lstimer_value(2)
    }
    #[doc = "0x17c - LSTIMER3_VALUE"]
    #[inline(always)]
    pub const fn lstimer3_value(&self) -> &LSTIMER_VALUE {
        self.lstimer_value(3)
    }
    #[doc = "0x180 - "]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x184 - "]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x188 - "]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x18c - "]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x190 - "]
    #[inline(always)]
    pub const fn conf(&self) -> &CONF {
        &self.conf
    }
    #[doc = "0x1fc - "]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "HSCH_CONF0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsch_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsch_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsch_conf0`] module"]
pub type HSCH_CONF0 = crate::Reg<hsch_conf0::HSCH_CONF0_SPEC>;
#[doc = ""]
pub mod hsch_conf0;
#[doc = "HSCH_HPOINT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsch_hpoint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsch_hpoint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsch_hpoint`] module"]
pub type HSCH_HPOINT = crate::Reg<hsch_hpoint::HSCH_HPOINT_SPEC>;
#[doc = ""]
pub mod hsch_hpoint;
#[doc = "HSCH_DUTY (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsch_duty::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsch_duty::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsch_duty`] module"]
pub type HSCH_DUTY = crate::Reg<hsch_duty::HSCH_DUTY_SPEC>;
#[doc = ""]
pub mod hsch_duty;
#[doc = "HSCH_CONF1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsch_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsch_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsch_conf1`] module"]
pub type HSCH_CONF1 = crate::Reg<hsch_conf1::HSCH_CONF1_SPEC>;
#[doc = ""]
pub mod hsch_conf1;
#[doc = "HSCH_DUTY_R (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsch_duty_r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsch_duty_r`] module"]
pub type HSCH_DUTY_R = crate::Reg<hsch_duty_r::HSCH_DUTY_R_SPEC>;
#[doc = ""]
pub mod hsch_duty_r;
#[doc = "LSCH_CONF0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsch_conf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsch_conf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsch_conf0`] module"]
pub type LSCH_CONF0 = crate::Reg<lsch_conf0::LSCH_CONF0_SPEC>;
#[doc = ""]
pub mod lsch_conf0;
#[doc = "LSCH_HPOINT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsch_hpoint::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsch_hpoint::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsch_hpoint`] module"]
pub type LSCH_HPOINT = crate::Reg<lsch_hpoint::LSCH_HPOINT_SPEC>;
#[doc = ""]
pub mod lsch_hpoint;
#[doc = "LSCH_DUTY (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsch_duty::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsch_duty::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsch_duty`] module"]
pub type LSCH_DUTY = crate::Reg<lsch_duty::LSCH_DUTY_SPEC>;
#[doc = ""]
pub mod lsch_duty;
#[doc = "LSCH_CONF1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsch_conf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsch_conf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsch_conf1`] module"]
pub type LSCH_CONF1 = crate::Reg<lsch_conf1::LSCH_CONF1_SPEC>;
#[doc = ""]
pub mod lsch_conf1;
#[doc = "LSCH_DUTY_R (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsch_duty_r::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsch_duty_r`] module"]
pub type LSCH_DUTY_R = crate::Reg<lsch_duty_r::LSCH_DUTY_R_SPEC>;
#[doc = ""]
pub mod lsch_duty_r;
#[doc = "HSTIMER_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstimer_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hstimer_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstimer_conf`] module"]
pub type HSTIMER_CONF = crate::Reg<hstimer_conf::HSTIMER_CONF_SPEC>;
#[doc = ""]
pub mod hstimer_conf;
#[doc = "HSTIMER_VALUE (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstimer_value::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hstimer_value`] module"]
pub type HSTIMER_VALUE = crate::Reg<hstimer_value::HSTIMER_VALUE_SPEC>;
#[doc = ""]
pub mod hstimer_value;
#[doc = "LSTIMER_CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lstimer_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lstimer_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lstimer_conf`] module"]
pub type LSTIMER_CONF = crate::Reg<lstimer_conf::LSTIMER_CONF_SPEC>;
#[doc = ""]
pub mod lstimer_conf;
#[doc = "LSTIMER_VALUE (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lstimer_value::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lstimer_value`] module"]
pub type LSTIMER_VALUE = crate::Reg<lstimer_value::LSTIMER_VALUE_SPEC>;
#[doc = ""]
pub mod lstimer_value;
#[doc = "INT_RAW (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = ""]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = ""]
pub mod int_st;
#[doc = "INT_ENA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = ""]
pub mod int_ena;
#[doc = "INT_CLR (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = ""]
pub mod int_clr;
#[doc = "CONF (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`] module"]
pub type CONF = crate::Reg<conf::CONF_SPEC>;
#[doc = ""]
pub mod conf;
#[doc = "DATE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = ""]
pub mod date;
