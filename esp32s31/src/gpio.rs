#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    strap: STRAP,
    out: OUT,
    out_w1ts: OUT_W1TS,
    out_w1tc: OUT_W1TC,
    out1: OUT1,
    out1_w1ts: OUT1_W1TS,
    out1_w1tc: OUT1_W1TC,
    out2: OUT2,
    out2_w1ts: OUT2_W1TS,
    out2_w1tc: OUT2_W1TC,
    _reserved10: [u8; 0x0c],
    enable: ENABLE,
    enable_w1ts: ENABLE_W1TS,
    enable_w1tc: ENABLE_W1TC,
    enable1: ENABLE1,
    enable1_w1ts: ENABLE1_W1TS,
    enable1_w1tc: ENABLE1_W1TC,
    enable2: ENABLE2,
    enable2_w1ts: ENABLE2_W1TS,
    enable2_w1tc: ENABLE2_W1TC,
    _reserved19: [u8; 0x0c],
    in_: IN,
    in1: IN1,
    in2: IN2,
    _reserved22: [u8; 0x04],
    status: STATUS,
    status_w1ts: STATUS_W1TS,
    status_w1tc: STATUS_W1TC,
    status1: STATUS1,
    status1_w1ts: STATUS1_W1TS,
    status1_w1tc: STATUS1_W1TC,
    status2: STATUS2,
    status2_w1ts: STATUS2_W1TS,
    status2_w1tc: STATUS2_W1TC,
    _reserved31: [u8; 0x0c],
    int_0: INT_0,
    int_1: INT_1,
    int_2: INT_2,
    int_3: INT_3,
    int_01: INT_01,
    int_11: INT_11,
    int_21: INT_21,
    int_31: INT_31,
    int_02: INT_02,
    int_12: INT_12,
    int_22: INT_22,
    int_32: INT_32,
    _reserved43: [u8; 0x10],
    status_next: STATUS_NEXT,
    status_next1: STATUS_NEXT1,
    status_next2: STATUS_NEXT2,
    _reserved46: [u8; 0x04],
    pin: [PIN; 63],
    pin63: PIN63,
    pin64: PIN64,
    pin65: PIN65,
    pin66: PIN66,
    _reserved51: [u8; 0xf4],
    func_in_sel_cfg: [FUNC_IN_SEL_CFG; 8],
    _reserved52: [u8; 0x08],
    func0_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func1_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func2_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func3_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func4_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func5_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func6_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func7_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func8_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func9_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func10_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func11_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func12_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func13_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func14_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func15_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func16_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func17_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func18_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func19_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func20_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func21_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func22_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func23_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func24_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func25_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func26_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func27_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func28_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func29_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func30_in_sel_cfg: FUNC0_IN_SEL_CFG,
    _reserved83: [u8; 0x08],
    func0_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func1_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func2_in_sel_cfg: FUNC0_IN_SEL_CFG,
    _reserved86: [u8; 0x04],
    func0_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func1_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func2_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func3_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func4_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func5_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func6_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func7_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func8_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func9_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func10_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func11_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func12_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func13_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func14_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func15_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func16_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func17_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func18_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func19_in_sel_cfg: FUNC0_IN_SEL_CFG,
    _reserved106: [u8; 0x04],
    func_in_sel_cfg: [FUNC_IN_SEL_CFG; 48],
    _reserved107: [u8; 0x08],
    func0_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func1_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func2_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func3_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func4_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func5_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func6_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func7_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func8_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func9_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func10_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func11_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func12_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func13_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func14_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func15_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func16_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func17_in_sel_cfg: FUNC0_IN_SEL_CFG,
    _reserved125: [u8; 0x04],
    func0_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func1_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func2_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func3_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func4_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func5_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func6_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func7_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func8_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func9_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func10_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func11_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func12_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func13_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func14_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func15_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func16_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func17_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func18_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func19_in_sel_cfg: FUNC0_IN_SEL_CFG,
    _reserved145: [u8; 0x04],
    func0_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func1_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func2_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func3_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func4_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func5_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func6_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func7_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func8_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func9_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func10_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func11_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func12_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func13_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func14_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func15_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func16_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func17_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func18_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func19_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func20_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func21_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func22_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func23_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func24_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func25_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func26_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func27_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func28_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func29_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func30_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func31_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func32_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func33_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func34_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func35_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func36_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func37_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func38_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func39_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func40_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func41_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func42_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func43_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func44_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func45_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func46_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func47_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func48_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func49_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func50_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func51_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func52_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func53_in_sel_cfg: FUNC0_IN_SEL_CFG,
    _reserved199: [u8; 0x08],
    func0_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func1_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func2_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func3_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func4_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func5_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func6_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func7_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func8_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func9_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func10_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func11_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func12_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func13_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func14_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func15_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func16_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func17_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func18_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func19_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func20_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func21_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func22_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func23_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func24_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func25_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func26_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func27_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func28_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func29_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func30_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func31_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func32_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func33_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func34_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func35_in_sel_cfg: FUNC0_IN_SEL_CFG,
    _reserved235: [u8; 0x04],
    func0_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func1_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func2_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func3_in_sel_cfg: FUNC0_IN_SEL_CFG,
    func4_in_sel_cfg: FUNC0_IN_SEL_CFG,
    _reserved240: [u8; 0x0400],
    func_out_sel_cfg: [FUNC_OUT_SEL_CFG; 63],
    func63_out_sel_cfg: FUNC63_OUT_SEL_CFG,
    func64_out_sel_cfg: FUNC64_OUT_SEL_CFG,
    func65_out_sel_cfg: FUNC65_OUT_SEL_CFG,
    func66_out_sel_cfg: FUNC66_OUT_SEL_CFG,
    _reserved245: [u8; 0x01f8],
    clock_gate: CLOCK_GATE,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - Strapping pin register"]
    #[inline(always)]
    pub const fn strap(&self) -> &STRAP {
        &self.strap
    }
    #[doc = "0x04 - GPIO output register"]
    #[inline(always)]
    pub const fn out(&self) -> &OUT {
        &self.out
    }
    #[doc = "0x08 - GPIO output set register"]
    #[inline(always)]
    pub const fn out_w1ts(&self) -> &OUT_W1TS {
        &self.out_w1ts
    }
    #[doc = "0x0c - GPIO output clear register"]
    #[inline(always)]
    pub const fn out_w1tc(&self) -> &OUT_W1TC {
        &self.out_w1tc
    }
    #[doc = "0x10 - GPIO output register"]
    #[inline(always)]
    pub const fn out1(&self) -> &OUT1 {
        &self.out1
    }
    #[doc = "0x14 - GPIO output set register"]
    #[inline(always)]
    pub const fn out1_w1ts(&self) -> &OUT1_W1TS {
        &self.out1_w1ts
    }
    #[doc = "0x18 - GPIO output clear register"]
    #[inline(always)]
    pub const fn out1_w1tc(&self) -> &OUT1_W1TC {
        &self.out1_w1tc
    }
    #[doc = "0x1c - GPIO output register"]
    #[inline(always)]
    pub const fn out2(&self) -> &OUT2 {
        &self.out2
    }
    #[doc = "0x20 - GPIO output set register"]
    #[inline(always)]
    pub const fn out2_w1ts(&self) -> &OUT2_W1TS {
        &self.out2_w1ts
    }
    #[doc = "0x24 - GPIO output clear register"]
    #[inline(always)]
    pub const fn out2_w1tc(&self) -> &OUT2_W1TC {
        &self.out2_w1tc
    }
    #[doc = "0x34 - GPIO output enable register"]
    #[inline(always)]
    pub const fn enable(&self) -> &ENABLE {
        &self.enable
    }
    #[doc = "0x38 - GPIO output enable set register"]
    #[inline(always)]
    pub const fn enable_w1ts(&self) -> &ENABLE_W1TS {
        &self.enable_w1ts
    }
    #[doc = "0x3c - GPIO output enable clear register"]
    #[inline(always)]
    pub const fn enable_w1tc(&self) -> &ENABLE_W1TC {
        &self.enable_w1tc
    }
    #[doc = "0x40 - GPIO output enable register"]
    #[inline(always)]
    pub const fn enable1(&self) -> &ENABLE1 {
        &self.enable1
    }
    #[doc = "0x44 - GPIO output enable set register"]
    #[inline(always)]
    pub const fn enable1_w1ts(&self) -> &ENABLE1_W1TS {
        &self.enable1_w1ts
    }
    #[doc = "0x48 - GPIO output enable clear register"]
    #[inline(always)]
    pub const fn enable1_w1tc(&self) -> &ENABLE1_W1TC {
        &self.enable1_w1tc
    }
    #[doc = "0x4c - GPIO output enable register"]
    #[inline(always)]
    pub const fn enable2(&self) -> &ENABLE2 {
        &self.enable2
    }
    #[doc = "0x50 - GPIO output enable set register"]
    #[inline(always)]
    pub const fn enable2_w1ts(&self) -> &ENABLE2_W1TS {
        &self.enable2_w1ts
    }
    #[doc = "0x54 - GPIO output enable clear register"]
    #[inline(always)]
    pub const fn enable2_w1tc(&self) -> &ENABLE2_W1TC {
        &self.enable2_w1tc
    }
    #[doc = "0x64 - GPIO input register"]
    #[inline(always)]
    pub const fn in_(&self) -> &IN {
        &self.in_
    }
    #[doc = "0x68 - GPIO input register"]
    #[inline(always)]
    pub const fn in1(&self) -> &IN1 {
        &self.in1
    }
    #[doc = "0x6c - GPIO input register"]
    #[inline(always)]
    pub const fn in2(&self) -> &IN2 {
        &self.in2
    }
    #[doc = "0x74 - GPIO interrupt status register"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x78 - GPIO interrupt status set register"]
    #[inline(always)]
    pub const fn status_w1ts(&self) -> &STATUS_W1TS {
        &self.status_w1ts
    }
    #[doc = "0x7c - GPIO interrupt status clear register"]
    #[inline(always)]
    pub const fn status_w1tc(&self) -> &STATUS_W1TC {
        &self.status_w1tc
    }
    #[doc = "0x80 - GPIO interrupt status register"]
    #[inline(always)]
    pub const fn status1(&self) -> &STATUS1 {
        &self.status1
    }
    #[doc = "0x84 - GPIO interrupt status set register"]
    #[inline(always)]
    pub const fn status1_w1ts(&self) -> &STATUS1_W1TS {
        &self.status1_w1ts
    }
    #[doc = "0x88 - GPIO interrupt status clear register"]
    #[inline(always)]
    pub const fn status1_w1tc(&self) -> &STATUS1_W1TC {
        &self.status1_w1tc
    }
    #[doc = "0x8c - GPIO interrupt status register"]
    #[inline(always)]
    pub const fn status2(&self) -> &STATUS2 {
        &self.status2
    }
    #[doc = "0x90 - GPIO interrupt status set register"]
    #[inline(always)]
    pub const fn status2_w1ts(&self) -> &STATUS2_W1TS {
        &self.status2_w1ts
    }
    #[doc = "0x94 - GPIO interrupt status clear register"]
    #[inline(always)]
    pub const fn status2_w1tc(&self) -> &STATUS2_W1TC {
        &self.status2_w1tc
    }
    #[doc = "0xa4 - GPIO_INT_0 interrupt status register"]
    #[inline(always)]
    pub const fn int_0(&self) -> &INT_0 {
        &self.int_0
    }
    #[doc = "0xa8 - GPIO_INT_1 interrupt status register"]
    #[inline(always)]
    pub const fn int_1(&self) -> &INT_1 {
        &self.int_1
    }
    #[doc = "0xac - GPIO_INT_2 interrupt status register"]
    #[inline(always)]
    pub const fn int_2(&self) -> &INT_2 {
        &self.int_2
    }
    #[doc = "0xb0 - GPIO_INT_3 interrupt status register"]
    #[inline(always)]
    pub const fn int_3(&self) -> &INT_3 {
        &self.int_3
    }
    #[doc = "0xb4 - GPIO_INT_0 interrupt status register"]
    #[inline(always)]
    pub const fn int_01(&self) -> &INT_01 {
        &self.int_01
    }
    #[doc = "0xb8 - GPIO_INT_1 interrupt status register"]
    #[inline(always)]
    pub const fn int_11(&self) -> &INT_11 {
        &self.int_11
    }
    #[doc = "0xbc - GPIO_INT_2 interrupt status register"]
    #[inline(always)]
    pub const fn int_21(&self) -> &INT_21 {
        &self.int_21
    }
    #[doc = "0xc0 - GPIO_INT_3 interrupt status register"]
    #[inline(always)]
    pub const fn int_31(&self) -> &INT_31 {
        &self.int_31
    }
    #[doc = "0xc4 - GPIO_INT_0 interrupt status register"]
    #[inline(always)]
    pub const fn int_02(&self) -> &INT_02 {
        &self.int_02
    }
    #[doc = "0xc8 - GPIO_INT_1 interrupt status register"]
    #[inline(always)]
    pub const fn int_12(&self) -> &INT_12 {
        &self.int_12
    }
    #[doc = "0xcc - GPIO_INT_2 interrupt status register"]
    #[inline(always)]
    pub const fn int_22(&self) -> &INT_22 {
        &self.int_22
    }
    #[doc = "0xd0 - GPIO_INT_3 interrupt status register"]
    #[inline(always)]
    pub const fn int_32(&self) -> &INT_32 {
        &self.int_32
    }
    #[doc = "0xe4 - GPIO interrupt source register"]
    #[inline(always)]
    pub const fn status_next(&self) -> &STATUS_NEXT {
        &self.status_next
    }
    #[doc = "0xe8 - GPIO interrupt source register"]
    #[inline(always)]
    pub const fn status_next1(&self) -> &STATUS_NEXT1 {
        &self.status_next1
    }
    #[doc = "0xec - GPIO interrupt source register"]
    #[inline(always)]
    pub const fn status_next2(&self) -> &STATUS_NEXT2 {
        &self.status_next2
    }
    #[doc = "0xf4..0x1f0 - GPIO%s configuration register"]
    #[inline(always)]
    pub const fn pin(&self, n: usize) -> &PIN {
        &self.pin[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xf4..0x1f0 - GPIO%s configuration register"]
    #[inline(always)]
    pub fn pin_iter(&self) -> impl Iterator<Item = &PIN> {
        self.pin.iter()
    }
    #[doc = "0x1f0 - GPIO$n configuration register"]
    #[inline(always)]
    pub const fn pin63(&self) -> &PIN63 {
        &self.pin63
    }
    #[doc = "0x1f4 - GPIO$n configuration register"]
    #[inline(always)]
    pub const fn pin64(&self) -> &PIN64 {
        &self.pin64
    }
    #[doc = "0x1f8 - GPIO$n configuration register"]
    #[inline(always)]
    pub const fn pin65(&self) -> &PIN65 {
        &self.pin65
    }
    #[doc = "0x1fc - GPIO$n configuration register"]
    #[inline(always)]
    pub const fn pin66(&self) -> &PIN66 {
        &self.pin66
    }
    #[doc = "0x2f4..0x314 - Configuration register for input signal %s"]
    #[inline(always)]
    pub const fn func_in_sel_cfg(&self, n: usize) -> &FUNC_IN_SEL_CFG {
        &self.func_in_sel_cfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2f4..0x314 - Configuration register for input signal %s"]
    #[inline(always)]
    pub fn func_in_sel_cfg_iter(&self) -> impl Iterator<Item = &FUNC_IN_SEL_CFG> {
        self.func_in_sel_cfg.iter()
    }
    #[doc = "0x2f4 - Configuration register for input signal 0"]
    #[inline(always)]
    pub const fn func0_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(0)
    }
    #[doc = "0x2f8 - Configuration register for input signal 1"]
    #[inline(always)]
    pub const fn func1_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(1)
    }
    #[doc = "0x2fc - Configuration register for input signal 2"]
    #[inline(always)]
    pub const fn func2_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(2)
    }
    #[doc = "0x300 - Configuration register for input signal 3"]
    #[inline(always)]
    pub const fn func3_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(3)
    }
    #[doc = "0x304 - Configuration register for input signal 4"]
    #[inline(always)]
    pub const fn func4_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(4)
    }
    #[doc = "0x308 - Configuration register for input signal 5"]
    #[inline(always)]
    pub const fn func5_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(5)
    }
    #[doc = "0x30c - Configuration register for input signal 6"]
    #[inline(always)]
    pub const fn func6_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(6)
    }
    #[doc = "0x310 - Configuration register for input signal 7"]
    #[inline(always)]
    pub const fn func7_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(7)
    }
    #[doc = "0x31c - Configuration register for input signal 0"]
    #[inline(always)]
    pub const fn func0_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func0_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 1"]
    #[inline(always)]
    pub const fn func1_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func1_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 2"]
    #[inline(always)]
    pub const fn func2_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func2_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 3"]
    #[inline(always)]
    pub const fn func3_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func3_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 4"]
    #[inline(always)]
    pub const fn func4_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func4_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 5"]
    #[inline(always)]
    pub const fn func5_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func5_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 6"]
    #[inline(always)]
    pub const fn func6_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func6_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 7"]
    #[inline(always)]
    pub const fn func7_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func7_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 8"]
    #[inline(always)]
    pub const fn func8_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func8_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 9"]
    #[inline(always)]
    pub const fn func9_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func9_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 10"]
    #[inline(always)]
    pub const fn func10_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func10_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 11"]
    #[inline(always)]
    pub const fn func11_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func11_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 12"]
    #[inline(always)]
    pub const fn func12_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func12_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 13"]
    #[inline(always)]
    pub const fn func13_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func13_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 14"]
    #[inline(always)]
    pub const fn func14_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func14_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 15"]
    #[inline(always)]
    pub const fn func15_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func15_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 16"]
    #[inline(always)]
    pub const fn func16_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func16_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 17"]
    #[inline(always)]
    pub const fn func17_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func17_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 18"]
    #[inline(always)]
    pub const fn func18_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func18_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 19"]
    #[inline(always)]
    pub const fn func19_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func19_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 20"]
    #[inline(always)]
    pub const fn func20_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func20_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 21"]
    #[inline(always)]
    pub const fn func21_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func21_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 22"]
    #[inline(always)]
    pub const fn func22_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func22_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 23"]
    #[inline(always)]
    pub const fn func23_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func23_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 24"]
    #[inline(always)]
    pub const fn func24_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func24_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 25"]
    #[inline(always)]
    pub const fn func25_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func25_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 26"]
    #[inline(always)]
    pub const fn func26_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func26_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 27"]
    #[inline(always)]
    pub const fn func27_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func27_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 28"]
    #[inline(always)]
    pub const fn func28_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func28_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 29"]
    #[inline(always)]
    pub const fn func29_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func29_in_sel_cfg
    }
    #[doc = "0x31c - Configuration register for input signal 30"]
    #[inline(always)]
    pub const fn func30_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func30_in_sel_cfg
    }
    #[doc = "0x3a0 - Configuration register for input signal 0"]
    #[inline(always)]
    pub const fn func0_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func0_in_sel_cfg
    }
    #[doc = "0x3a0 - Configuration register for input signal 1"]
    #[inline(always)]
    pub const fn func1_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func1_in_sel_cfg
    }
    #[doc = "0x3a0 - Configuration register for input signal 2"]
    #[inline(always)]
    pub const fn func2_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func2_in_sel_cfg
    }
    #[doc = "0x3b0 - Configuration register for input signal 0"]
    #[inline(always)]
    pub const fn func0_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func0_in_sel_cfg
    }
    #[doc = "0x3b0 - Configuration register for input signal 1"]
    #[inline(always)]
    pub const fn func1_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func1_in_sel_cfg
    }
    #[doc = "0x3b0 - Configuration register for input signal 2"]
    #[inline(always)]
    pub const fn func2_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func2_in_sel_cfg
    }
    #[doc = "0x3b0 - Configuration register for input signal 3"]
    #[inline(always)]
    pub const fn func3_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func3_in_sel_cfg
    }
    #[doc = "0x3b0 - Configuration register for input signal 4"]
    #[inline(always)]
    pub const fn func4_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func4_in_sel_cfg
    }
    #[doc = "0x3b0 - Configuration register for input signal 5"]
    #[inline(always)]
    pub const fn func5_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func5_in_sel_cfg
    }
    #[doc = "0x3b0 - Configuration register for input signal 6"]
    #[inline(always)]
    pub const fn func6_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func6_in_sel_cfg
    }
    #[doc = "0x3b0 - Configuration register for input signal 7"]
    #[inline(always)]
    pub const fn func7_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func7_in_sel_cfg
    }
    #[doc = "0x3b0 - Configuration register for input signal 8"]
    #[inline(always)]
    pub const fn func8_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func8_in_sel_cfg
    }
    #[doc = "0x3b0 - Configuration register for input signal 9"]
    #[inline(always)]
    pub const fn func9_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func9_in_sel_cfg
    }
    #[doc = "0x3b0 - Configuration register for input signal 10"]
    #[inline(always)]
    pub const fn func10_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func10_in_sel_cfg
    }
    #[doc = "0x3b0 - Configuration register for input signal 11"]
    #[inline(always)]
    pub const fn func11_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func11_in_sel_cfg
    }
    #[doc = "0x3b0 - Configuration register for input signal 12"]
    #[inline(always)]
    pub const fn func12_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func12_in_sel_cfg
    }
    #[doc = "0x3b0 - Configuration register for input signal 13"]
    #[inline(always)]
    pub const fn func13_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func13_in_sel_cfg
    }
    #[doc = "0x3b0 - Configuration register for input signal 14"]
    #[inline(always)]
    pub const fn func14_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func14_in_sel_cfg
    }
    #[doc = "0x3b0 - Configuration register for input signal 15"]
    #[inline(always)]
    pub const fn func15_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func15_in_sel_cfg
    }
    #[doc = "0x3b0 - Configuration register for input signal 16"]
    #[inline(always)]
    pub const fn func16_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func16_in_sel_cfg
    }
    #[doc = "0x3b0 - Configuration register for input signal 17"]
    #[inline(always)]
    pub const fn func17_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func17_in_sel_cfg
    }
    #[doc = "0x3b0 - Configuration register for input signal 18"]
    #[inline(always)]
    pub const fn func18_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func18_in_sel_cfg
    }
    #[doc = "0x3b0 - Configuration register for input signal 19"]
    #[inline(always)]
    pub const fn func19_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func19_in_sel_cfg
    }
    #[doc = "0x404..0x4c4 - Configuration register for input signal %s"]
    #[inline(always)]
    pub const fn func_in_sel_cfg(&self, n: usize) -> &FUNC_IN_SEL_CFG {
        &self.func_in_sel_cfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x404..0x4c4 - Configuration register for input signal %s"]
    #[inline(always)]
    pub fn func_in_sel_cfg_iter(&self) -> impl Iterator<Item = &FUNC_IN_SEL_CFG> {
        self.func_in_sel_cfg.iter()
    }
    #[doc = "0x404 - Configuration register for input signal 0"]
    #[inline(always)]
    pub const fn func0_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(0)
    }
    #[doc = "0x408 - Configuration register for input signal 1"]
    #[inline(always)]
    pub const fn func1_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(1)
    }
    #[doc = "0x40c - Configuration register for input signal 2"]
    #[inline(always)]
    pub const fn func2_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(2)
    }
    #[doc = "0x410 - Configuration register for input signal 3"]
    #[inline(always)]
    pub const fn func3_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(3)
    }
    #[doc = "0x414 - Configuration register for input signal 4"]
    #[inline(always)]
    pub const fn func4_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(4)
    }
    #[doc = "0x418 - Configuration register for input signal 5"]
    #[inline(always)]
    pub const fn func5_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(5)
    }
    #[doc = "0x41c - Configuration register for input signal 6"]
    #[inline(always)]
    pub const fn func6_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(6)
    }
    #[doc = "0x420 - Configuration register for input signal 7"]
    #[inline(always)]
    pub const fn func7_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(7)
    }
    #[doc = "0x424 - Configuration register for input signal 8"]
    #[inline(always)]
    pub const fn func8_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(8)
    }
    #[doc = "0x428 - Configuration register for input signal 9"]
    #[inline(always)]
    pub const fn func9_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(9)
    }
    #[doc = "0x42c - Configuration register for input signal 10"]
    #[inline(always)]
    pub const fn func10_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(10)
    }
    #[doc = "0x430 - Configuration register for input signal 11"]
    #[inline(always)]
    pub const fn func11_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(11)
    }
    #[doc = "0x434 - Configuration register for input signal 12"]
    #[inline(always)]
    pub const fn func12_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(12)
    }
    #[doc = "0x438 - Configuration register for input signal 13"]
    #[inline(always)]
    pub const fn func13_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(13)
    }
    #[doc = "0x43c - Configuration register for input signal 14"]
    #[inline(always)]
    pub const fn func14_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(14)
    }
    #[doc = "0x440 - Configuration register for input signal 15"]
    #[inline(always)]
    pub const fn func15_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(15)
    }
    #[doc = "0x444 - Configuration register for input signal 16"]
    #[inline(always)]
    pub const fn func16_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(16)
    }
    #[doc = "0x448 - Configuration register for input signal 17"]
    #[inline(always)]
    pub const fn func17_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(17)
    }
    #[doc = "0x44c - Configuration register for input signal 18"]
    #[inline(always)]
    pub const fn func18_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(18)
    }
    #[doc = "0x450 - Configuration register for input signal 19"]
    #[inline(always)]
    pub const fn func19_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(19)
    }
    #[doc = "0x454 - Configuration register for input signal 20"]
    #[inline(always)]
    pub const fn func20_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(20)
    }
    #[doc = "0x458 - Configuration register for input signal 21"]
    #[inline(always)]
    pub const fn func21_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(21)
    }
    #[doc = "0x45c - Configuration register for input signal 22"]
    #[inline(always)]
    pub const fn func22_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(22)
    }
    #[doc = "0x460 - Configuration register for input signal 23"]
    #[inline(always)]
    pub const fn func23_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(23)
    }
    #[doc = "0x464 - Configuration register for input signal 24"]
    #[inline(always)]
    pub const fn func24_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(24)
    }
    #[doc = "0x468 - Configuration register for input signal 25"]
    #[inline(always)]
    pub const fn func25_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(25)
    }
    #[doc = "0x46c - Configuration register for input signal 26"]
    #[inline(always)]
    pub const fn func26_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(26)
    }
    #[doc = "0x470 - Configuration register for input signal 27"]
    #[inline(always)]
    pub const fn func27_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(27)
    }
    #[doc = "0x474 - Configuration register for input signal 28"]
    #[inline(always)]
    pub const fn func28_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(28)
    }
    #[doc = "0x478 - Configuration register for input signal 29"]
    #[inline(always)]
    pub const fn func29_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(29)
    }
    #[doc = "0x47c - Configuration register for input signal 30"]
    #[inline(always)]
    pub const fn func30_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(30)
    }
    #[doc = "0x480 - Configuration register for input signal 31"]
    #[inline(always)]
    pub const fn func31_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(31)
    }
    #[doc = "0x484 - Configuration register for input signal 32"]
    #[inline(always)]
    pub const fn func32_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(32)
    }
    #[doc = "0x488 - Configuration register for input signal 33"]
    #[inline(always)]
    pub const fn func33_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(33)
    }
    #[doc = "0x48c - Configuration register for input signal 34"]
    #[inline(always)]
    pub const fn func34_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(34)
    }
    #[doc = "0x490 - Configuration register for input signal 35"]
    #[inline(always)]
    pub const fn func35_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(35)
    }
    #[doc = "0x494 - Configuration register for input signal 36"]
    #[inline(always)]
    pub const fn func36_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(36)
    }
    #[doc = "0x498 - Configuration register for input signal 37"]
    #[inline(always)]
    pub const fn func37_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(37)
    }
    #[doc = "0x49c - Configuration register for input signal 38"]
    #[inline(always)]
    pub const fn func38_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(38)
    }
    #[doc = "0x4a0 - Configuration register for input signal 39"]
    #[inline(always)]
    pub const fn func39_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(39)
    }
    #[doc = "0x4a4 - Configuration register for input signal 40"]
    #[inline(always)]
    pub const fn func40_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(40)
    }
    #[doc = "0x4a8 - Configuration register for input signal 41"]
    #[inline(always)]
    pub const fn func41_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(41)
    }
    #[doc = "0x4ac - Configuration register for input signal 42"]
    #[inline(always)]
    pub const fn func42_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(42)
    }
    #[doc = "0x4b0 - Configuration register for input signal 43"]
    #[inline(always)]
    pub const fn func43_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(43)
    }
    #[doc = "0x4b4 - Configuration register for input signal 44"]
    #[inline(always)]
    pub const fn func44_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(44)
    }
    #[doc = "0x4b8 - Configuration register for input signal 45"]
    #[inline(always)]
    pub const fn func45_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(45)
    }
    #[doc = "0x4bc - Configuration register for input signal 46"]
    #[inline(always)]
    pub const fn func46_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(46)
    }
    #[doc = "0x4c0 - Configuration register for input signal 47"]
    #[inline(always)]
    pub const fn func47_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(47)
    }
    #[doc = "0x4cc - Configuration register for input signal 0"]
    #[inline(always)]
    pub const fn func0_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func0_in_sel_cfg
    }
    #[doc = "0x4cc - Configuration register for input signal 1"]
    #[inline(always)]
    pub const fn func1_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func1_in_sel_cfg
    }
    #[doc = "0x4cc - Configuration register for input signal 2"]
    #[inline(always)]
    pub const fn func2_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func2_in_sel_cfg
    }
    #[doc = "0x4cc - Configuration register for input signal 3"]
    #[inline(always)]
    pub const fn func3_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func3_in_sel_cfg
    }
    #[doc = "0x4cc - Configuration register for input signal 4"]
    #[inline(always)]
    pub const fn func4_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func4_in_sel_cfg
    }
    #[doc = "0x4cc - Configuration register for input signal 5"]
    #[inline(always)]
    pub const fn func5_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func5_in_sel_cfg
    }
    #[doc = "0x4cc - Configuration register for input signal 6"]
    #[inline(always)]
    pub const fn func6_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func6_in_sel_cfg
    }
    #[doc = "0x4cc - Configuration register for input signal 7"]
    #[inline(always)]
    pub const fn func7_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func7_in_sel_cfg
    }
    #[doc = "0x4cc - Configuration register for input signal 8"]
    #[inline(always)]
    pub const fn func8_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func8_in_sel_cfg
    }
    #[doc = "0x4cc - Configuration register for input signal 9"]
    #[inline(always)]
    pub const fn func9_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func9_in_sel_cfg
    }
    #[doc = "0x4cc - Configuration register for input signal 10"]
    #[inline(always)]
    pub const fn func10_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func10_in_sel_cfg
    }
    #[doc = "0x4cc - Configuration register for input signal 11"]
    #[inline(always)]
    pub const fn func11_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func11_in_sel_cfg
    }
    #[doc = "0x4cc - Configuration register for input signal 12"]
    #[inline(always)]
    pub const fn func12_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func12_in_sel_cfg
    }
    #[doc = "0x4cc - Configuration register for input signal 13"]
    #[inline(always)]
    pub const fn func13_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func13_in_sel_cfg
    }
    #[doc = "0x4cc - Configuration register for input signal 14"]
    #[inline(always)]
    pub const fn func14_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func14_in_sel_cfg
    }
    #[doc = "0x4cc - Configuration register for input signal 15"]
    #[inline(always)]
    pub const fn func15_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func15_in_sel_cfg
    }
    #[doc = "0x4cc - Configuration register for input signal 16"]
    #[inline(always)]
    pub const fn func16_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func16_in_sel_cfg
    }
    #[doc = "0x4cc - Configuration register for input signal 17"]
    #[inline(always)]
    pub const fn func17_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func17_in_sel_cfg
    }
    #[doc = "0x518 - Configuration register for input signal 0"]
    #[inline(always)]
    pub const fn func0_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func0_in_sel_cfg
    }
    #[doc = "0x518 - Configuration register for input signal 1"]
    #[inline(always)]
    pub const fn func1_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func1_in_sel_cfg
    }
    #[doc = "0x518 - Configuration register for input signal 2"]
    #[inline(always)]
    pub const fn func2_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func2_in_sel_cfg
    }
    #[doc = "0x518 - Configuration register for input signal 3"]
    #[inline(always)]
    pub const fn func3_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func3_in_sel_cfg
    }
    #[doc = "0x518 - Configuration register for input signal 4"]
    #[inline(always)]
    pub const fn func4_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func4_in_sel_cfg
    }
    #[doc = "0x518 - Configuration register for input signal 5"]
    #[inline(always)]
    pub const fn func5_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func5_in_sel_cfg
    }
    #[doc = "0x518 - Configuration register for input signal 6"]
    #[inline(always)]
    pub const fn func6_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func6_in_sel_cfg
    }
    #[doc = "0x518 - Configuration register for input signal 7"]
    #[inline(always)]
    pub const fn func7_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func7_in_sel_cfg
    }
    #[doc = "0x518 - Configuration register for input signal 8"]
    #[inline(always)]
    pub const fn func8_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func8_in_sel_cfg
    }
    #[doc = "0x518 - Configuration register for input signal 9"]
    #[inline(always)]
    pub const fn func9_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func9_in_sel_cfg
    }
    #[doc = "0x518 - Configuration register for input signal 10"]
    #[inline(always)]
    pub const fn func10_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func10_in_sel_cfg
    }
    #[doc = "0x518 - Configuration register for input signal 11"]
    #[inline(always)]
    pub const fn func11_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func11_in_sel_cfg
    }
    #[doc = "0x518 - Configuration register for input signal 12"]
    #[inline(always)]
    pub const fn func12_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func12_in_sel_cfg
    }
    #[doc = "0x518 - Configuration register for input signal 13"]
    #[inline(always)]
    pub const fn func13_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func13_in_sel_cfg
    }
    #[doc = "0x518 - Configuration register for input signal 14"]
    #[inline(always)]
    pub const fn func14_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func14_in_sel_cfg
    }
    #[doc = "0x518 - Configuration register for input signal 15"]
    #[inline(always)]
    pub const fn func15_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func15_in_sel_cfg
    }
    #[doc = "0x518 - Configuration register for input signal 16"]
    #[inline(always)]
    pub const fn func16_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func16_in_sel_cfg
    }
    #[doc = "0x518 - Configuration register for input signal 17"]
    #[inline(always)]
    pub const fn func17_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func17_in_sel_cfg
    }
    #[doc = "0x518 - Configuration register for input signal 18"]
    #[inline(always)]
    pub const fn func18_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func18_in_sel_cfg
    }
    #[doc = "0x518 - Configuration register for input signal 19"]
    #[inline(always)]
    pub const fn func19_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func19_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 0"]
    #[inline(always)]
    pub const fn func0_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func0_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 1"]
    #[inline(always)]
    pub const fn func1_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func1_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 2"]
    #[inline(always)]
    pub const fn func2_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func2_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 3"]
    #[inline(always)]
    pub const fn func3_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func3_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 4"]
    #[inline(always)]
    pub const fn func4_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func4_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 5"]
    #[inline(always)]
    pub const fn func5_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func5_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 6"]
    #[inline(always)]
    pub const fn func6_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func6_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 7"]
    #[inline(always)]
    pub const fn func7_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func7_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 8"]
    #[inline(always)]
    pub const fn func8_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func8_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 9"]
    #[inline(always)]
    pub const fn func9_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func9_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 10"]
    #[inline(always)]
    pub const fn func10_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func10_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 11"]
    #[inline(always)]
    pub const fn func11_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func11_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 12"]
    #[inline(always)]
    pub const fn func12_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func12_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 13"]
    #[inline(always)]
    pub const fn func13_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func13_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 14"]
    #[inline(always)]
    pub const fn func14_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func14_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 15"]
    #[inline(always)]
    pub const fn func15_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func15_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 16"]
    #[inline(always)]
    pub const fn func16_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func16_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 17"]
    #[inline(always)]
    pub const fn func17_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func17_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 18"]
    #[inline(always)]
    pub const fn func18_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func18_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 19"]
    #[inline(always)]
    pub const fn func19_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func19_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 20"]
    #[inline(always)]
    pub const fn func20_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func20_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 21"]
    #[inline(always)]
    pub const fn func21_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func21_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 22"]
    #[inline(always)]
    pub const fn func22_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func22_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 23"]
    #[inline(always)]
    pub const fn func23_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func23_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 24"]
    #[inline(always)]
    pub const fn func24_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func24_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 25"]
    #[inline(always)]
    pub const fn func25_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func25_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 26"]
    #[inline(always)]
    pub const fn func26_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func26_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 27"]
    #[inline(always)]
    pub const fn func27_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func27_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 28"]
    #[inline(always)]
    pub const fn func28_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func28_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 29"]
    #[inline(always)]
    pub const fn func29_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func29_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 30"]
    #[inline(always)]
    pub const fn func30_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func30_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 31"]
    #[inline(always)]
    pub const fn func31_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func31_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 32"]
    #[inline(always)]
    pub const fn func32_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func32_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 33"]
    #[inline(always)]
    pub const fn func33_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func33_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 34"]
    #[inline(always)]
    pub const fn func34_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func34_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 35"]
    #[inline(always)]
    pub const fn func35_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func35_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 36"]
    #[inline(always)]
    pub const fn func36_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func36_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 37"]
    #[inline(always)]
    pub const fn func37_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func37_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 38"]
    #[inline(always)]
    pub const fn func38_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func38_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 39"]
    #[inline(always)]
    pub const fn func39_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func39_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 40"]
    #[inline(always)]
    pub const fn func40_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func40_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 41"]
    #[inline(always)]
    pub const fn func41_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func41_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 42"]
    #[inline(always)]
    pub const fn func42_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func42_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 43"]
    #[inline(always)]
    pub const fn func43_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func43_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 44"]
    #[inline(always)]
    pub const fn func44_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func44_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 45"]
    #[inline(always)]
    pub const fn func45_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func45_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 46"]
    #[inline(always)]
    pub const fn func46_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func46_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 47"]
    #[inline(always)]
    pub const fn func47_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func47_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 48"]
    #[inline(always)]
    pub const fn func48_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func48_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 49"]
    #[inline(always)]
    pub const fn func49_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func49_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 50"]
    #[inline(always)]
    pub const fn func50_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func50_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 51"]
    #[inline(always)]
    pub const fn func51_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func51_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 52"]
    #[inline(always)]
    pub const fn func52_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func52_in_sel_cfg
    }
    #[doc = "0x56c - Configuration register for input signal 53"]
    #[inline(always)]
    pub const fn func53_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func53_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 0"]
    #[inline(always)]
    pub const fn func0_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func0_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 1"]
    #[inline(always)]
    pub const fn func1_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func1_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 2"]
    #[inline(always)]
    pub const fn func2_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func2_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 3"]
    #[inline(always)]
    pub const fn func3_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func3_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 4"]
    #[inline(always)]
    pub const fn func4_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func4_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 5"]
    #[inline(always)]
    pub const fn func5_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func5_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 6"]
    #[inline(always)]
    pub const fn func6_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func6_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 7"]
    #[inline(always)]
    pub const fn func7_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func7_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 8"]
    #[inline(always)]
    pub const fn func8_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func8_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 9"]
    #[inline(always)]
    pub const fn func9_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func9_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 10"]
    #[inline(always)]
    pub const fn func10_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func10_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 11"]
    #[inline(always)]
    pub const fn func11_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func11_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 12"]
    #[inline(always)]
    pub const fn func12_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func12_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 13"]
    #[inline(always)]
    pub const fn func13_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func13_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 14"]
    #[inline(always)]
    pub const fn func14_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func14_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 15"]
    #[inline(always)]
    pub const fn func15_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func15_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 16"]
    #[inline(always)]
    pub const fn func16_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func16_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 17"]
    #[inline(always)]
    pub const fn func17_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func17_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 18"]
    #[inline(always)]
    pub const fn func18_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func18_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 19"]
    #[inline(always)]
    pub const fn func19_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func19_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 20"]
    #[inline(always)]
    pub const fn func20_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func20_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 21"]
    #[inline(always)]
    pub const fn func21_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func21_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 22"]
    #[inline(always)]
    pub const fn func22_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func22_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 23"]
    #[inline(always)]
    pub const fn func23_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func23_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 24"]
    #[inline(always)]
    pub const fn func24_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func24_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 25"]
    #[inline(always)]
    pub const fn func25_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func25_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 26"]
    #[inline(always)]
    pub const fn func26_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func26_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 27"]
    #[inline(always)]
    pub const fn func27_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func27_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 28"]
    #[inline(always)]
    pub const fn func28_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func28_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 29"]
    #[inline(always)]
    pub const fn func29_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func29_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 30"]
    #[inline(always)]
    pub const fn func30_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func30_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 31"]
    #[inline(always)]
    pub const fn func31_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func31_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 32"]
    #[inline(always)]
    pub const fn func32_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func32_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 33"]
    #[inline(always)]
    pub const fn func33_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func33_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 34"]
    #[inline(always)]
    pub const fn func34_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func34_in_sel_cfg
    }
    #[doc = "0x64c - Configuration register for input signal 35"]
    #[inline(always)]
    pub const fn func35_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func35_in_sel_cfg
    }
    #[doc = "0x6e0 - Configuration register for input signal 0"]
    #[inline(always)]
    pub const fn func0_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func0_in_sel_cfg
    }
    #[doc = "0x6e0 - Configuration register for input signal 1"]
    #[inline(always)]
    pub const fn func1_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func1_in_sel_cfg
    }
    #[doc = "0x6e0 - Configuration register for input signal 2"]
    #[inline(always)]
    pub const fn func2_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func2_in_sel_cfg
    }
    #[doc = "0x6e0 - Configuration register for input signal 3"]
    #[inline(always)]
    pub const fn func3_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func3_in_sel_cfg
    }
    #[doc = "0x6e0 - Configuration register for input signal 4"]
    #[inline(always)]
    pub const fn func4_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func4_in_sel_cfg
    }
    #[doc = "0xaf4..0xbf0 - Configuration register for GPIO%s output"]
    #[inline(always)]
    pub const fn func_out_sel_cfg(&self, n: usize) -> &FUNC_OUT_SEL_CFG {
        &self.func_out_sel_cfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xaf4..0xbf0 - Configuration register for GPIO%s output"]
    #[inline(always)]
    pub fn func_out_sel_cfg_iter(&self) -> impl Iterator<Item = &FUNC_OUT_SEL_CFG> {
        self.func_out_sel_cfg.iter()
    }
    #[doc = "0xaf4 - Configuration register for GPIO0 output"]
    #[inline(always)]
    pub const fn func0_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(0)
    }
    #[doc = "0xaf8 - Configuration register for GPIO1 output"]
    #[inline(always)]
    pub const fn func1_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(1)
    }
    #[doc = "0xafc - Configuration register for GPIO2 output"]
    #[inline(always)]
    pub const fn func2_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(2)
    }
    #[doc = "0xb00 - Configuration register for GPIO3 output"]
    #[inline(always)]
    pub const fn func3_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(3)
    }
    #[doc = "0xb04 - Configuration register for GPIO4 output"]
    #[inline(always)]
    pub const fn func4_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(4)
    }
    #[doc = "0xb08 - Configuration register for GPIO5 output"]
    #[inline(always)]
    pub const fn func5_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(5)
    }
    #[doc = "0xb0c - Configuration register for GPIO6 output"]
    #[inline(always)]
    pub const fn func6_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(6)
    }
    #[doc = "0xb10 - Configuration register for GPIO7 output"]
    #[inline(always)]
    pub const fn func7_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(7)
    }
    #[doc = "0xb14 - Configuration register for GPIO8 output"]
    #[inline(always)]
    pub const fn func8_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(8)
    }
    #[doc = "0xb18 - Configuration register for GPIO9 output"]
    #[inline(always)]
    pub const fn func9_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(9)
    }
    #[doc = "0xb1c - Configuration register for GPIO10 output"]
    #[inline(always)]
    pub const fn func10_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(10)
    }
    #[doc = "0xb20 - Configuration register for GPIO11 output"]
    #[inline(always)]
    pub const fn func11_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(11)
    }
    #[doc = "0xb24 - Configuration register for GPIO12 output"]
    #[inline(always)]
    pub const fn func12_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(12)
    }
    #[doc = "0xb28 - Configuration register for GPIO13 output"]
    #[inline(always)]
    pub const fn func13_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(13)
    }
    #[doc = "0xb2c - Configuration register for GPIO14 output"]
    #[inline(always)]
    pub const fn func14_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(14)
    }
    #[doc = "0xb30 - Configuration register for GPIO15 output"]
    #[inline(always)]
    pub const fn func15_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(15)
    }
    #[doc = "0xb34 - Configuration register for GPIO16 output"]
    #[inline(always)]
    pub const fn func16_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(16)
    }
    #[doc = "0xb38 - Configuration register for GPIO17 output"]
    #[inline(always)]
    pub const fn func17_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(17)
    }
    #[doc = "0xb3c - Configuration register for GPIO18 output"]
    #[inline(always)]
    pub const fn func18_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(18)
    }
    #[doc = "0xb40 - Configuration register for GPIO19 output"]
    #[inline(always)]
    pub const fn func19_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(19)
    }
    #[doc = "0xb44 - Configuration register for GPIO20 output"]
    #[inline(always)]
    pub const fn func20_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(20)
    }
    #[doc = "0xb48 - Configuration register for GPIO21 output"]
    #[inline(always)]
    pub const fn func21_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(21)
    }
    #[doc = "0xb4c - Configuration register for GPIO22 output"]
    #[inline(always)]
    pub const fn func22_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(22)
    }
    #[doc = "0xb50 - Configuration register for GPIO23 output"]
    #[inline(always)]
    pub const fn func23_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(23)
    }
    #[doc = "0xb54 - Configuration register for GPIO24 output"]
    #[inline(always)]
    pub const fn func24_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(24)
    }
    #[doc = "0xb58 - Configuration register for GPIO25 output"]
    #[inline(always)]
    pub const fn func25_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(25)
    }
    #[doc = "0xb5c - Configuration register for GPIO26 output"]
    #[inline(always)]
    pub const fn func26_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(26)
    }
    #[doc = "0xb60 - Configuration register for GPIO27 output"]
    #[inline(always)]
    pub const fn func27_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(27)
    }
    #[doc = "0xb64 - Configuration register for GPIO28 output"]
    #[inline(always)]
    pub const fn func28_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(28)
    }
    #[doc = "0xb68 - Configuration register for GPIO29 output"]
    #[inline(always)]
    pub const fn func29_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(29)
    }
    #[doc = "0xb6c - Configuration register for GPIO30 output"]
    #[inline(always)]
    pub const fn func30_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(30)
    }
    #[doc = "0xb70 - Configuration register for GPIO31 output"]
    #[inline(always)]
    pub const fn func31_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(31)
    }
    #[doc = "0xb74 - Configuration register for GPIO32 output"]
    #[inline(always)]
    pub const fn func32_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(32)
    }
    #[doc = "0xb78 - Configuration register for GPIO33 output"]
    #[inline(always)]
    pub const fn func33_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(33)
    }
    #[doc = "0xb7c - Configuration register for GPIO34 output"]
    #[inline(always)]
    pub const fn func34_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(34)
    }
    #[doc = "0xb80 - Configuration register for GPIO35 output"]
    #[inline(always)]
    pub const fn func35_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(35)
    }
    #[doc = "0xb84 - Configuration register for GPIO36 output"]
    #[inline(always)]
    pub const fn func36_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(36)
    }
    #[doc = "0xb88 - Configuration register for GPIO37 output"]
    #[inline(always)]
    pub const fn func37_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(37)
    }
    #[doc = "0xb8c - Configuration register for GPIO38 output"]
    #[inline(always)]
    pub const fn func38_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(38)
    }
    #[doc = "0xb90 - Configuration register for GPIO39 output"]
    #[inline(always)]
    pub const fn func39_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(39)
    }
    #[doc = "0xb94 - Configuration register for GPIO40 output"]
    #[inline(always)]
    pub const fn func40_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(40)
    }
    #[doc = "0xb98 - Configuration register for GPIO41 output"]
    #[inline(always)]
    pub const fn func41_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(41)
    }
    #[doc = "0xb9c - Configuration register for GPIO42 output"]
    #[inline(always)]
    pub const fn func42_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(42)
    }
    #[doc = "0xba0 - Configuration register for GPIO43 output"]
    #[inline(always)]
    pub const fn func43_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(43)
    }
    #[doc = "0xba4 - Configuration register for GPIO44 output"]
    #[inline(always)]
    pub const fn func44_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(44)
    }
    #[doc = "0xba8 - Configuration register for GPIO45 output"]
    #[inline(always)]
    pub const fn func45_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(45)
    }
    #[doc = "0xbac - Configuration register for GPIO46 output"]
    #[inline(always)]
    pub const fn func46_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(46)
    }
    #[doc = "0xbb0 - Configuration register for GPIO47 output"]
    #[inline(always)]
    pub const fn func47_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(47)
    }
    #[doc = "0xbb4 - Configuration register for GPIO48 output"]
    #[inline(always)]
    pub const fn func48_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(48)
    }
    #[doc = "0xbb8 - Configuration register for GPIO49 output"]
    #[inline(always)]
    pub const fn func49_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(49)
    }
    #[doc = "0xbbc - Configuration register for GPIO50 output"]
    #[inline(always)]
    pub const fn func50_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(50)
    }
    #[doc = "0xbc0 - Configuration register for GPIO51 output"]
    #[inline(always)]
    pub const fn func51_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(51)
    }
    #[doc = "0xbc4 - Configuration register for GPIO52 output"]
    #[inline(always)]
    pub const fn func52_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(52)
    }
    #[doc = "0xbc8 - Configuration register for GPIO53 output"]
    #[inline(always)]
    pub const fn func53_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(53)
    }
    #[doc = "0xbcc - Configuration register for GPIO54 output"]
    #[inline(always)]
    pub const fn func54_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(54)
    }
    #[doc = "0xbd0 - Configuration register for GPIO55 output"]
    #[inline(always)]
    pub const fn func55_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(55)
    }
    #[doc = "0xbd4 - Configuration register for GPIO56 output"]
    #[inline(always)]
    pub const fn func56_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(56)
    }
    #[doc = "0xbd8 - Configuration register for GPIO57 output"]
    #[inline(always)]
    pub const fn func57_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(57)
    }
    #[doc = "0xbdc - Configuration register for GPIO58 output"]
    #[inline(always)]
    pub const fn func58_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(58)
    }
    #[doc = "0xbe0 - Configuration register for GPIO59 output"]
    #[inline(always)]
    pub const fn func59_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(59)
    }
    #[doc = "0xbe4 - Configuration register for GPIO60 output"]
    #[inline(always)]
    pub const fn func60_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(60)
    }
    #[doc = "0xbe8 - Configuration register for GPIO61 output"]
    #[inline(always)]
    pub const fn func61_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(61)
    }
    #[doc = "0xbec - Configuration register for GPIO62 output"]
    #[inline(always)]
    pub const fn func62_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(62)
    }
    #[doc = "0xbf0 - Configuration register for GPIO$n output"]
    #[inline(always)]
    pub const fn func63_out_sel_cfg(&self) -> &FUNC63_OUT_SEL_CFG {
        &self.func63_out_sel_cfg
    }
    #[doc = "0xbf4 - Configuration register for GPIO$n output"]
    #[inline(always)]
    pub const fn func64_out_sel_cfg(&self) -> &FUNC64_OUT_SEL_CFG {
        &self.func64_out_sel_cfg
    }
    #[doc = "0xbf8 - Configuration register for GPIO$n output"]
    #[inline(always)]
    pub const fn func65_out_sel_cfg(&self) -> &FUNC65_OUT_SEL_CFG {
        &self.func65_out_sel_cfg
    }
    #[doc = "0xbfc - Configuration register for GPIO$n output"]
    #[inline(always)]
    pub const fn func66_out_sel_cfg(&self) -> &FUNC66_OUT_SEL_CFG {
        &self.func66_out_sel_cfg
    }
    #[doc = "0xdf8 - GPIO clock gate register"]
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    #[doc = "0xdfc - GPIO version register"]
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
#[doc = "STRAP (r) register accessor: Strapping pin register\n\nYou can [`read`](crate::Reg::read) this register and get [`strap::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@strap`] module"]
pub type STRAP = crate::Reg<strap::STRAP_SPEC>;
#[doc = "Strapping pin register"]
pub mod strap;
#[doc = "OUT (rw) register accessor: GPIO output register\n\nYou can [`read`](crate::Reg::read) this register and get [`out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out`] module"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "GPIO output register"]
pub mod out;
#[doc = "OUT_W1TS (w) register accessor: GPIO output set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_w1ts`] module"]
pub type OUT_W1TS = crate::Reg<out_w1ts::OUT_W1TS_SPEC>;
#[doc = "GPIO output set register"]
pub mod out_w1ts;
#[doc = "OUT_W1TC (w) register accessor: GPIO output clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_w1tc`] module"]
pub type OUT_W1TC = crate::Reg<out_w1tc::OUT_W1TC_SPEC>;
#[doc = "GPIO output clear register"]
pub mod out_w1tc;
#[doc = "OUT1 (rw) register accessor: GPIO output register\n\nYou can [`read`](crate::Reg::read) this register and get [`out1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out1`] module"]
pub type OUT1 = crate::Reg<out1::OUT1_SPEC>;
#[doc = "GPIO output register"]
pub mod out1;
#[doc = "OUT1_W1TS (w) register accessor: GPIO output set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out1_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out1_w1ts`] module"]
pub type OUT1_W1TS = crate::Reg<out1_w1ts::OUT1_W1TS_SPEC>;
#[doc = "GPIO output set register"]
pub mod out1_w1ts;
#[doc = "OUT1_W1TC (w) register accessor: GPIO output clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out1_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out1_w1tc`] module"]
pub type OUT1_W1TC = crate::Reg<out1_w1tc::OUT1_W1TC_SPEC>;
#[doc = "GPIO output clear register"]
pub mod out1_w1tc;
#[doc = "OUT2 (rw) register accessor: GPIO output register\n\nYou can [`read`](crate::Reg::read) this register and get [`out2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out2`] module"]
pub type OUT2 = crate::Reg<out2::OUT2_SPEC>;
#[doc = "GPIO output register"]
pub mod out2;
#[doc = "OUT2_W1TS (w) register accessor: GPIO output set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out2_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out2_w1ts`] module"]
pub type OUT2_W1TS = crate::Reg<out2_w1ts::OUT2_W1TS_SPEC>;
#[doc = "GPIO output set register"]
pub mod out2_w1ts;
#[doc = "OUT2_W1TC (w) register accessor: GPIO output clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out2_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out2_w1tc`] module"]
pub type OUT2_W1TC = crate::Reg<out2_w1tc::OUT2_W1TC_SPEC>;
#[doc = "GPIO output clear register"]
pub mod out2_w1tc;
#[doc = "ENABLE (rw) register accessor: GPIO output enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "GPIO output enable register"]
pub mod enable;
#[doc = "ENABLE_W1TS (w) register accessor: GPIO output enable set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_w1ts`] module"]
pub type ENABLE_W1TS = crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>;
#[doc = "GPIO output enable set register"]
pub mod enable_w1ts;
#[doc = "ENABLE_W1TC (w) register accessor: GPIO output enable clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_w1tc`] module"]
pub type ENABLE_W1TC = crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>;
#[doc = "GPIO output enable clear register"]
pub mod enable_w1tc;
#[doc = "ENABLE1 (rw) register accessor: GPIO output enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`enable1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable1`] module"]
pub type ENABLE1 = crate::Reg<enable1::ENABLE1_SPEC>;
#[doc = "GPIO output enable register"]
pub mod enable1;
#[doc = "ENABLE1_W1TS (w) register accessor: GPIO output enable set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable1_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable1_w1ts`] module"]
pub type ENABLE1_W1TS = crate::Reg<enable1_w1ts::ENABLE1_W1TS_SPEC>;
#[doc = "GPIO output enable set register"]
pub mod enable1_w1ts;
#[doc = "ENABLE1_W1TC (w) register accessor: GPIO output enable clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable1_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable1_w1tc`] module"]
pub type ENABLE1_W1TC = crate::Reg<enable1_w1tc::ENABLE1_W1TC_SPEC>;
#[doc = "GPIO output enable clear register"]
pub mod enable1_w1tc;
#[doc = "ENABLE2 (rw) register accessor: GPIO output enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`enable2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable2`] module"]
pub type ENABLE2 = crate::Reg<enable2::ENABLE2_SPEC>;
#[doc = "GPIO output enable register"]
pub mod enable2;
#[doc = "ENABLE2_W1TS (w) register accessor: GPIO output enable set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable2_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable2_w1ts`] module"]
pub type ENABLE2_W1TS = crate::Reg<enable2_w1ts::ENABLE2_W1TS_SPEC>;
#[doc = "GPIO output enable set register"]
pub mod enable2_w1ts;
#[doc = "ENABLE2_W1TC (w) register accessor: GPIO output enable clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable2_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable2_w1tc`] module"]
pub type ENABLE2_W1TC = crate::Reg<enable2_w1tc::ENABLE2_W1TC_SPEC>;
#[doc = "GPIO output enable clear register"]
pub mod enable2_w1tc;
#[doc = "IN (r) register accessor: GPIO input register\n\nYou can [`read`](crate::Reg::read) this register and get [`in_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_`] module"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "GPIO input register"]
pub mod in_;
#[doc = "IN1 (r) register accessor: GPIO input register\n\nYou can [`read`](crate::Reg::read) this register and get [`in1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in1`] module"]
pub type IN1 = crate::Reg<in1::IN1_SPEC>;
#[doc = "GPIO input register"]
pub mod in1;
#[doc = "IN2 (r) register accessor: GPIO input register\n\nYou can [`read`](crate::Reg::read) this register and get [`in2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in2`] module"]
pub type IN2 = crate::Reg<in2::IN2_SPEC>;
#[doc = "GPIO input register"]
pub mod in2;
#[doc = "STATUS (rw) register accessor: GPIO interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "GPIO interrupt status register"]
pub mod status;
#[doc = "STATUS_W1TS (w) register accessor: GPIO interrupt status set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_w1ts`] module"]
pub type STATUS_W1TS = crate::Reg<status_w1ts::STATUS_W1TS_SPEC>;
#[doc = "GPIO interrupt status set register"]
pub mod status_w1ts;
#[doc = "STATUS_W1TC (w) register accessor: GPIO interrupt status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_w1tc`] module"]
pub type STATUS_W1TC = crate::Reg<status_w1tc::STATUS_W1TC_SPEC>;
#[doc = "GPIO interrupt status clear register"]
pub mod status_w1tc;
#[doc = "STATUS1 (rw) register accessor: GPIO interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status1`] module"]
pub type STATUS1 = crate::Reg<status1::STATUS1_SPEC>;
#[doc = "GPIO interrupt status register"]
pub mod status1;
#[doc = "STATUS1_W1TS (w) register accessor: GPIO interrupt status set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status1_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status1_w1ts`] module"]
pub type STATUS1_W1TS = crate::Reg<status1_w1ts::STATUS1_W1TS_SPEC>;
#[doc = "GPIO interrupt status set register"]
pub mod status1_w1ts;
#[doc = "STATUS1_W1TC (w) register accessor: GPIO interrupt status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status1_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status1_w1tc`] module"]
pub type STATUS1_W1TC = crate::Reg<status1_w1tc::STATUS1_W1TC_SPEC>;
#[doc = "GPIO interrupt status clear register"]
pub mod status1_w1tc;
#[doc = "STATUS2 (rw) register accessor: GPIO interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status2`] module"]
pub type STATUS2 = crate::Reg<status2::STATUS2_SPEC>;
#[doc = "GPIO interrupt status register"]
pub mod status2;
#[doc = "STATUS2_W1TS (w) register accessor: GPIO interrupt status set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status2_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status2_w1ts`] module"]
pub type STATUS2_W1TS = crate::Reg<status2_w1ts::STATUS2_W1TS_SPEC>;
#[doc = "GPIO interrupt status set register"]
pub mod status2_w1ts;
#[doc = "STATUS2_W1TC (w) register accessor: GPIO interrupt status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status2_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status2_w1tc`] module"]
pub type STATUS2_W1TC = crate::Reg<status2_w1tc::STATUS2_W1TC_SPEC>;
#[doc = "GPIO interrupt status clear register"]
pub mod status2_w1tc;
#[doc = "INT_0 (r) register accessor: GPIO_INT_0 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_0`] module"]
pub type INT_0 = crate::Reg<int_0::INT_0_SPEC>;
#[doc = "GPIO_INT_0 interrupt status register"]
pub mod int_0;
#[doc = "INT_1 (r) register accessor: GPIO_INT_1 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_1`] module"]
pub type INT_1 = crate::Reg<int_1::INT_1_SPEC>;
#[doc = "GPIO_INT_1 interrupt status register"]
pub mod int_1;
#[doc = "INT_2 (r) register accessor: GPIO_INT_2 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_2`] module"]
pub type INT_2 = crate::Reg<int_2::INT_2_SPEC>;
#[doc = "GPIO_INT_2 interrupt status register"]
pub mod int_2;
#[doc = "INT_3 (r) register accessor: GPIO_INT_3 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_3`] module"]
pub type INT_3 = crate::Reg<int_3::INT_3_SPEC>;
#[doc = "GPIO_INT_3 interrupt status register"]
pub mod int_3;
#[doc = "INT_01 (r) register accessor: GPIO_INT_0 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_01::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_01`] module"]
pub type INT_01 = crate::Reg<int_01::INT_01_SPEC>;
#[doc = "GPIO_INT_0 interrupt status register"]
pub mod int_01;
#[doc = "INT_11 (r) register accessor: GPIO_INT_1 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_11`] module"]
pub type INT_11 = crate::Reg<int_11::INT_11_SPEC>;
#[doc = "GPIO_INT_1 interrupt status register"]
pub mod int_11;
#[doc = "INT_21 (r) register accessor: GPIO_INT_2 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_21::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_21`] module"]
pub type INT_21 = crate::Reg<int_21::INT_21_SPEC>;
#[doc = "GPIO_INT_2 interrupt status register"]
pub mod int_21;
#[doc = "INT_31 (r) register accessor: GPIO_INT_3 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_31::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_31`] module"]
pub type INT_31 = crate::Reg<int_31::INT_31_SPEC>;
#[doc = "GPIO_INT_3 interrupt status register"]
pub mod int_31;
#[doc = "INT_02 (r) register accessor: GPIO_INT_0 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_02::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_02`] module"]
pub type INT_02 = crate::Reg<int_02::INT_02_SPEC>;
#[doc = "GPIO_INT_0 interrupt status register"]
pub mod int_02;
#[doc = "INT_12 (r) register accessor: GPIO_INT_1 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_12`] module"]
pub type INT_12 = crate::Reg<int_12::INT_12_SPEC>;
#[doc = "GPIO_INT_1 interrupt status register"]
pub mod int_12;
#[doc = "INT_22 (r) register accessor: GPIO_INT_2 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_22::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_22`] module"]
pub type INT_22 = crate::Reg<int_22::INT_22_SPEC>;
#[doc = "GPIO_INT_2 interrupt status register"]
pub mod int_22;
#[doc = "INT_32 (r) register accessor: GPIO_INT_3 interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_32::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_32`] module"]
pub type INT_32 = crate::Reg<int_32::INT_32_SPEC>;
#[doc = "GPIO_INT_3 interrupt status register"]
pub mod int_32;
#[doc = "STATUS_NEXT (r) register accessor: GPIO interrupt source register\n\nYou can [`read`](crate::Reg::read) this register and get [`status_next::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_next`] module"]
pub type STATUS_NEXT = crate::Reg<status_next::STATUS_NEXT_SPEC>;
#[doc = "GPIO interrupt source register"]
pub mod status_next;
#[doc = "STATUS_NEXT1 (r) register accessor: GPIO interrupt source register\n\nYou can [`read`](crate::Reg::read) this register and get [`status_next1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_next1`] module"]
pub type STATUS_NEXT1 = crate::Reg<status_next1::STATUS_NEXT1_SPEC>;
#[doc = "GPIO interrupt source register"]
pub mod status_next1;
#[doc = "STATUS_NEXT2 (r) register accessor: GPIO interrupt source register\n\nYou can [`read`](crate::Reg::read) this register and get [`status_next2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_next2`] module"]
pub type STATUS_NEXT2 = crate::Reg<status_next2::STATUS_NEXT2_SPEC>;
#[doc = "GPIO interrupt source register"]
pub mod status_next2;
#[doc = "PIN (rw) register accessor: GPIO%s configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin`] module"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = "GPIO%s configuration register"]
pub mod pin;
pub use pin as pin63;
pub use pin as pin64;
pub use pin as pin65;
pub use pin as pin66;
pub use PIN as PIN63;
pub use PIN as PIN64;
pub use PIN as PIN65;
pub use PIN as PIN66;
#[doc = "FUNC_IN_SEL_CFG (rw) register accessor: Configuration register for input signal %s\n\nYou can [`read`](crate::Reg::read) this register and get [`func_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_in_sel_cfg`] module"]
pub type FUNC_IN_SEL_CFG = crate::Reg<func_in_sel_cfg::FUNC_IN_SEL_CFG_SPEC>;
#[doc = "Configuration register for input signal %s"]
pub mod func_in_sel_cfg;
pub use func_in_sel_cfg;
pub use FUNC_IN_SEL_CFG;
#[doc = "FUNC_IN_SEL_CFG (rw) register accessor: Configuration register for input signal %s\n\nYou can [`read`](crate::Reg::read) this register and get [`func_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_in_sel_cfg`] module"]
pub type FUNC_IN_SEL_CFG = crate::Reg<func_in_sel_cfg::FUNC_IN_SEL_CFG_SPEC>;
#[doc = "Configuration register for input signal %s"]
pub mod func_in_sel_cfg;
pub use func_in_sel_cfg;
pub use FUNC_IN_SEL_CFG;
#[doc = "FUNC_OUT_SEL_CFG (rw) register accessor: Configuration register for GPIO%s output\n\nYou can [`read`](crate::Reg::read) this register and get [`func_out_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func_out_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_out_sel_cfg`] module"]
pub type FUNC_OUT_SEL_CFG = crate::Reg<func_out_sel_cfg::FUNC_OUT_SEL_CFG_SPEC>;
#[doc = "Configuration register for GPIO%s output"]
pub mod func_out_sel_cfg;
#[doc = "FUNC63_OUT_SEL_CFG (rw) register accessor: Configuration register for GPIO$n output\n\nYou can [`read`](crate::Reg::read) this register and get [`func63_out_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func63_out_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func63_out_sel_cfg`] module"]
pub type FUNC63_OUT_SEL_CFG = crate::Reg<func63_out_sel_cfg::FUNC63_OUT_SEL_CFG_SPEC>;
#[doc = "Configuration register for GPIO$n output"]
pub mod func63_out_sel_cfg;
#[doc = "FUNC64_OUT_SEL_CFG (rw) register accessor: Configuration register for GPIO$n output\n\nYou can [`read`](crate::Reg::read) this register and get [`func64_out_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func64_out_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func64_out_sel_cfg`] module"]
pub type FUNC64_OUT_SEL_CFG = crate::Reg<func64_out_sel_cfg::FUNC64_OUT_SEL_CFG_SPEC>;
#[doc = "Configuration register for GPIO$n output"]
pub mod func64_out_sel_cfg;
#[doc = "FUNC65_OUT_SEL_CFG (rw) register accessor: Configuration register for GPIO$n output\n\nYou can [`read`](crate::Reg::read) this register and get [`func65_out_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func65_out_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func65_out_sel_cfg`] module"]
pub type FUNC65_OUT_SEL_CFG = crate::Reg<func65_out_sel_cfg::FUNC65_OUT_SEL_CFG_SPEC>;
#[doc = "Configuration register for GPIO$n output"]
pub mod func65_out_sel_cfg;
#[doc = "FUNC66_OUT_SEL_CFG (rw) register accessor: Configuration register for GPIO$n output\n\nYou can [`read`](crate::Reg::read) this register and get [`func66_out_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func66_out_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func66_out_sel_cfg`] module"]
pub type FUNC66_OUT_SEL_CFG = crate::Reg<func66_out_sel_cfg::FUNC66_OUT_SEL_CFG_SPEC>;
#[doc = "Configuration register for GPIO$n output"]
pub mod func66_out_sel_cfg;
#[doc = "CLOCK_GATE (rw) register accessor: GPIO clock gate register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "GPIO clock gate register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: GPIO version register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "GPIO version register"]
pub mod date;
