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
    _reserved7: [u8; 0x18],
    enable: ENABLE,
    enable_w1ts: ENABLE_W1TS,
    enable_w1tc: ENABLE_W1TC,
    enable1: ENABLE1,
    enable1_w1ts: ENABLE1_W1TS,
    enable1_w1tc: ENABLE1_W1TC,
    _reserved13: [u8; 0x18],
    in_: IN,
    in1: IN1,
    _reserved15: [u8; 0x08],
    status: STATUS,
    status_w1ts: STATUS_W1TS,
    status_w1tc: STATUS_W1TC,
    status1: STATUS1,
    status1_w1ts: STATUS1_W1TS,
    status1_w1tc: STATUS1_W1TC,
    _reserved21: [u8; 0x18],
    procpu_int: PROCPU_INT,
    sdio_int: SDIO_INT,
    procpu_int1: PROCPU_INT1,
    sdio_int1: SDIO_INT1,
    _reserved25: [u8; 0x10],
    status_next: STATUS_NEXT,
    status_next1: STATUS_NEXT1,
    _reserved27: [u8; 0x08],
    pin: [PIN; 34],
    _reserved28: [u8; 0x0178],
    func0_in_sel_cfg: FUNC0_IN_SEL_CFG,
    _reserved29: [u8; 0x14],
    func6_in_sel_cfg: FUNC6_IN_SEL_CFG,
    func7_in_sel_cfg: FUNC7_IN_SEL_CFG,
    func8_in_sel_cfg: FUNC8_IN_SEL_CFG,
    func9_in_sel_cfg: FUNC9_IN_SEL_CFG,
    func10_in_sel_cfg: FUNC10_IN_SEL_CFG,
    func11_in_sel_cfg: FUNC11_IN_SEL_CFG,
    func12_in_sel_cfg: FUNC12_IN_SEL_CFG,
    func13_in_sel_cfg: FUNC13_IN_SEL_CFG,
    func14_in_sel_cfg: FUNC14_IN_SEL_CFG,
    func15_in_sel_cfg: FUNC15_IN_SEL_CFG,
    func16_in_sel_cfg: FUNC16_IN_SEL_CFG,
    func17_in_sel_cfg: FUNC17_IN_SEL_CFG,
    _reserved41: [u8; 0x24],
    func27_in_sel_cfg: FUNC27_IN_SEL_CFG,
    func28_in_sel_cfg: FUNC28_IN_SEL_CFG,
    func29_in_sel_cfg: FUNC29_IN_SEL_CFG,
    func30_in_sel_cfg: FUNC30_IN_SEL_CFG,
    func31_in_sel_cfg: FUNC31_IN_SEL_CFG,
    func32_in_sel_cfg: FUNC32_IN_SEL_CFG,
    func33_in_sel_cfg: FUNC33_IN_SEL_CFG,
    func34_in_sel_cfg: FUNC34_IN_SEL_CFG,
    func35_in_sel_cfg: FUNC35_IN_SEL_CFG,
    _reserved50: [u8; 0x14],
    func41_in_sel_cfg: FUNC41_IN_SEL_CFG,
    func42_in_sel_cfg: FUNC42_IN_SEL_CFG,
    func43_in_sel_cfg: FUNC43_IN_SEL_CFG,
    _reserved53: [u8; 0x08],
    func46_in_sel_cfg: FUNC46_IN_SEL_CFG,
    func47_in_sel_cfg: FUNC47_IN_SEL_CFG,
    _reserved55: [u8; 0x40],
    func64_in_sel_cfg: FUNC64_IN_SEL_CFG,
    func65_in_sel_cfg: FUNC65_IN_SEL_CFG,
    func66_in_sel_cfg: FUNC66_IN_SEL_CFG,
    func67_in_sel_cfg: FUNC67_IN_SEL_CFG,
    func68_in_sel_cfg: FUNC68_IN_SEL_CFG,
    func69_in_sel_cfg: FUNC69_IN_SEL_CFG,
    _reserved61: [u8; 0x08],
    func72_in_sel_cfg: FUNC72_IN_SEL_CFG,
    func73_in_sel_cfg: FUNC73_IN_SEL_CFG,
    func74_in_sel_cfg: FUNC74_IN_SEL_CFG,
    _reserved64: [u8; 0x1c],
    func82_in_sel_cfg: FUNC82_IN_SEL_CFG,
    func83_in_sel_cfg: FUNC83_IN_SEL_CFG,
    _reserved66: [u8; 0x34],
    func97_in_sel_cfg: FUNC97_IN_SEL_CFG,
    func98_in_sel_cfg: FUNC98_IN_SEL_CFG,
    func99_in_sel_cfg: FUNC99_IN_SEL_CFG,
    func100_in_sel_cfg: FUNC100_IN_SEL_CFG,
    _reserved70: [u8; 0x44],
    func118_in_sel_cfg: FUNC118_IN_SEL_CFG,
    func119_in_sel_cfg: FUNC119_IN_SEL_CFG,
    func120_in_sel_cfg: FUNC120_IN_SEL_CFG,
    func121_in_sel_cfg: FUNC121_IN_SEL_CFG,
    _reserved74: [u8; 0x0618],
    func_out_sel_cfg: [FUNC_OUT_SEL_CFG; 34],
    _reserved75: [u8; 0x029c],
    clock_gate: CLOCK_GATE,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - pad strapping register"]
    #[inline(always)]
    pub const fn strap(&self) -> &STRAP {
        &self.strap
    }
    #[doc = "0x04 - GPIO output register for GPIO0-31"]
    #[inline(always)]
    pub const fn out(&self) -> &OUT {
        &self.out
    }
    #[doc = "0x08 - GPIO output set register for GPIO0-31"]
    #[inline(always)]
    pub const fn out_w1ts(&self) -> &OUT_W1TS {
        &self.out_w1ts
    }
    #[doc = "0x0c - GPIO output clear register for GPIO0-31"]
    #[inline(always)]
    pub const fn out_w1tc(&self) -> &OUT_W1TC {
        &self.out_w1tc
    }
    #[doc = "0x10 - GPIO output register for GPIO32-33"]
    #[inline(always)]
    pub const fn out1(&self) -> &OUT1 {
        &self.out1
    }
    #[doc = "0x14 - GPIO output set register for GPIO32-33"]
    #[inline(always)]
    pub const fn out1_w1ts(&self) -> &OUT1_W1TS {
        &self.out1_w1ts
    }
    #[doc = "0x18 - GPIO output clear register for GPIO32-33"]
    #[inline(always)]
    pub const fn out1_w1tc(&self) -> &OUT1_W1TC {
        &self.out1_w1tc
    }
    #[doc = "0x34 - GPIO output enable register for GPIO0-31"]
    #[inline(always)]
    pub const fn enable(&self) -> &ENABLE {
        &self.enable
    }
    #[doc = "0x38 - GPIO output enable set register for GPIO0-31"]
    #[inline(always)]
    pub const fn enable_w1ts(&self) -> &ENABLE_W1TS {
        &self.enable_w1ts
    }
    #[doc = "0x3c - GPIO output enable clear register for GPIO0-31"]
    #[inline(always)]
    pub const fn enable_w1tc(&self) -> &ENABLE_W1TC {
        &self.enable_w1tc
    }
    #[doc = "0x40 - GPIO output enable register for GPIO32-33"]
    #[inline(always)]
    pub const fn enable1(&self) -> &ENABLE1 {
        &self.enable1
    }
    #[doc = "0x44 - GPIO output enable set register for GPIO32-33"]
    #[inline(always)]
    pub const fn enable1_w1ts(&self) -> &ENABLE1_W1TS {
        &self.enable1_w1ts
    }
    #[doc = "0x48 - GPIO output enable clear register for GPIO32-33"]
    #[inline(always)]
    pub const fn enable1_w1tc(&self) -> &ENABLE1_W1TC {
        &self.enable1_w1tc
    }
    #[doc = "0x64 - GPIO input register for GPIO0-31"]
    #[inline(always)]
    pub const fn in_(&self) -> &IN {
        &self.in_
    }
    #[doc = "0x68 - GPIO input register for GPIO32-33"]
    #[inline(always)]
    pub const fn in1(&self) -> &IN1 {
        &self.in1
    }
    #[doc = "0x74 - GPIO interrupt status register for GPIO0-31"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x78 - GPIO interrupt status set register for GPIO0-31"]
    #[inline(always)]
    pub const fn status_w1ts(&self) -> &STATUS_W1TS {
        &self.status_w1ts
    }
    #[doc = "0x7c - GPIO interrupt status clear register for GPIO0-31"]
    #[inline(always)]
    pub const fn status_w1tc(&self) -> &STATUS_W1TC {
        &self.status_w1tc
    }
    #[doc = "0x80 - GPIO interrupt status register for GPIO32-33"]
    #[inline(always)]
    pub const fn status1(&self) -> &STATUS1 {
        &self.status1
    }
    #[doc = "0x84 - GPIO interrupt status set register for GPIO32-33"]
    #[inline(always)]
    pub const fn status1_w1ts(&self) -> &STATUS1_W1TS {
        &self.status1_w1ts
    }
    #[doc = "0x88 - GPIO interrupt status clear register for GPIO32-33"]
    #[inline(always)]
    pub const fn status1_w1tc(&self) -> &STATUS1_W1TC {
        &self.status1_w1tc
    }
    #[doc = "0xa4 - GPIO_PROCPU_INT interrupt status register for GPIO0-31"]
    #[inline(always)]
    pub const fn procpu_int(&self) -> &PROCPU_INT {
        &self.procpu_int
    }
    #[doc = "0xa8 - GPIO_SDIO_INT interrupt status register for GPIO0-31"]
    #[inline(always)]
    pub const fn sdio_int(&self) -> &SDIO_INT {
        &self.sdio_int
    }
    #[doc = "0xac - GPIO_PROCPU_INT interrupt status register for GPIO32-33"]
    #[inline(always)]
    pub const fn procpu_int1(&self) -> &PROCPU_INT1 {
        &self.procpu_int1
    }
    #[doc = "0xb0 - GPIO_SDIO_INT interrupt status register for GPIO32-33"]
    #[inline(always)]
    pub const fn sdio_int1(&self) -> &SDIO_INT1 {
        &self.sdio_int1
    }
    #[doc = "0xc4 - GPIO interrupt source register for GPIO0-31"]
    #[inline(always)]
    pub const fn status_next(&self) -> &STATUS_NEXT {
        &self.status_next
    }
    #[doc = "0xc8 - GPIO interrupt source register for GPIO32-33"]
    #[inline(always)]
    pub const fn status_next1(&self) -> &STATUS_NEXT1 {
        &self.status_next1
    }
    #[doc = "0xd4..0x15c - GPIO pin configuration register"]
    #[inline(always)]
    pub const fn pin(&self, n: usize) -> &PIN {
        &self.pin[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xd4..0x15c - GPIO pin configuration register"]
    #[inline(always)]
    pub fn pin_iter(&self) -> impl Iterator<Item = &PIN> {
        self.pin.iter()
    }
    #[doc = "0x2d4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func0_in_sel_cfg(&self) -> &FUNC0_IN_SEL_CFG {
        &self.func0_in_sel_cfg
    }
    #[doc = "0x2ec - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func6_in_sel_cfg(&self) -> &FUNC6_IN_SEL_CFG {
        &self.func6_in_sel_cfg
    }
    #[doc = "0x2f0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func7_in_sel_cfg(&self) -> &FUNC7_IN_SEL_CFG {
        &self.func7_in_sel_cfg
    }
    #[doc = "0x2f4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func8_in_sel_cfg(&self) -> &FUNC8_IN_SEL_CFG {
        &self.func8_in_sel_cfg
    }
    #[doc = "0x2f8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func9_in_sel_cfg(&self) -> &FUNC9_IN_SEL_CFG {
        &self.func9_in_sel_cfg
    }
    #[doc = "0x2fc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func10_in_sel_cfg(&self) -> &FUNC10_IN_SEL_CFG {
        &self.func10_in_sel_cfg
    }
    #[doc = "0x300 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func11_in_sel_cfg(&self) -> &FUNC11_IN_SEL_CFG {
        &self.func11_in_sel_cfg
    }
    #[doc = "0x304 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func12_in_sel_cfg(&self) -> &FUNC12_IN_SEL_CFG {
        &self.func12_in_sel_cfg
    }
    #[doc = "0x308 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func13_in_sel_cfg(&self) -> &FUNC13_IN_SEL_CFG {
        &self.func13_in_sel_cfg
    }
    #[doc = "0x30c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func14_in_sel_cfg(&self) -> &FUNC14_IN_SEL_CFG {
        &self.func14_in_sel_cfg
    }
    #[doc = "0x310 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func15_in_sel_cfg(&self) -> &FUNC15_IN_SEL_CFG {
        &self.func15_in_sel_cfg
    }
    #[doc = "0x314 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func16_in_sel_cfg(&self) -> &FUNC16_IN_SEL_CFG {
        &self.func16_in_sel_cfg
    }
    #[doc = "0x318 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func17_in_sel_cfg(&self) -> &FUNC17_IN_SEL_CFG {
        &self.func17_in_sel_cfg
    }
    #[doc = "0x340 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func27_in_sel_cfg(&self) -> &FUNC27_IN_SEL_CFG {
        &self.func27_in_sel_cfg
    }
    #[doc = "0x344 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func28_in_sel_cfg(&self) -> &FUNC28_IN_SEL_CFG {
        &self.func28_in_sel_cfg
    }
    #[doc = "0x348 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func29_in_sel_cfg(&self) -> &FUNC29_IN_SEL_CFG {
        &self.func29_in_sel_cfg
    }
    #[doc = "0x34c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func30_in_sel_cfg(&self) -> &FUNC30_IN_SEL_CFG {
        &self.func30_in_sel_cfg
    }
    #[doc = "0x350 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func31_in_sel_cfg(&self) -> &FUNC31_IN_SEL_CFG {
        &self.func31_in_sel_cfg
    }
    #[doc = "0x354 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func32_in_sel_cfg(&self) -> &FUNC32_IN_SEL_CFG {
        &self.func32_in_sel_cfg
    }
    #[doc = "0x358 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func33_in_sel_cfg(&self) -> &FUNC33_IN_SEL_CFG {
        &self.func33_in_sel_cfg
    }
    #[doc = "0x35c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func34_in_sel_cfg(&self) -> &FUNC34_IN_SEL_CFG {
        &self.func34_in_sel_cfg
    }
    #[doc = "0x360 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func35_in_sel_cfg(&self) -> &FUNC35_IN_SEL_CFG {
        &self.func35_in_sel_cfg
    }
    #[doc = "0x378 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func41_in_sel_cfg(&self) -> &FUNC41_IN_SEL_CFG {
        &self.func41_in_sel_cfg
    }
    #[doc = "0x37c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func42_in_sel_cfg(&self) -> &FUNC42_IN_SEL_CFG {
        &self.func42_in_sel_cfg
    }
    #[doc = "0x380 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func43_in_sel_cfg(&self) -> &FUNC43_IN_SEL_CFG {
        &self.func43_in_sel_cfg
    }
    #[doc = "0x38c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func46_in_sel_cfg(&self) -> &FUNC46_IN_SEL_CFG {
        &self.func46_in_sel_cfg
    }
    #[doc = "0x390 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func47_in_sel_cfg(&self) -> &FUNC47_IN_SEL_CFG {
        &self.func47_in_sel_cfg
    }
    #[doc = "0x3d4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func64_in_sel_cfg(&self) -> &FUNC64_IN_SEL_CFG {
        &self.func64_in_sel_cfg
    }
    #[doc = "0x3d8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func65_in_sel_cfg(&self) -> &FUNC65_IN_SEL_CFG {
        &self.func65_in_sel_cfg
    }
    #[doc = "0x3dc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func66_in_sel_cfg(&self) -> &FUNC66_IN_SEL_CFG {
        &self.func66_in_sel_cfg
    }
    #[doc = "0x3e0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func67_in_sel_cfg(&self) -> &FUNC67_IN_SEL_CFG {
        &self.func67_in_sel_cfg
    }
    #[doc = "0x3e4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func68_in_sel_cfg(&self) -> &FUNC68_IN_SEL_CFG {
        &self.func68_in_sel_cfg
    }
    #[doc = "0x3e8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func69_in_sel_cfg(&self) -> &FUNC69_IN_SEL_CFG {
        &self.func69_in_sel_cfg
    }
    #[doc = "0x3f4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func72_in_sel_cfg(&self) -> &FUNC72_IN_SEL_CFG {
        &self.func72_in_sel_cfg
    }
    #[doc = "0x3f8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func73_in_sel_cfg(&self) -> &FUNC73_IN_SEL_CFG {
        &self.func73_in_sel_cfg
    }
    #[doc = "0x3fc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func74_in_sel_cfg(&self) -> &FUNC74_IN_SEL_CFG {
        &self.func74_in_sel_cfg
    }
    #[doc = "0x41c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func82_in_sel_cfg(&self) -> &FUNC82_IN_SEL_CFG {
        &self.func82_in_sel_cfg
    }
    #[doc = "0x420 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func83_in_sel_cfg(&self) -> &FUNC83_IN_SEL_CFG {
        &self.func83_in_sel_cfg
    }
    #[doc = "0x458 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func97_in_sel_cfg(&self) -> &FUNC97_IN_SEL_CFG {
        &self.func97_in_sel_cfg
    }
    #[doc = "0x45c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func98_in_sel_cfg(&self) -> &FUNC98_IN_SEL_CFG {
        &self.func98_in_sel_cfg
    }
    #[doc = "0x460 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func99_in_sel_cfg(&self) -> &FUNC99_IN_SEL_CFG {
        &self.func99_in_sel_cfg
    }
    #[doc = "0x464 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func100_in_sel_cfg(&self) -> &FUNC100_IN_SEL_CFG {
        &self.func100_in_sel_cfg
    }
    #[doc = "0x4ac - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func118_in_sel_cfg(&self) -> &FUNC118_IN_SEL_CFG {
        &self.func118_in_sel_cfg
    }
    #[doc = "0x4b0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func119_in_sel_cfg(&self) -> &FUNC119_IN_SEL_CFG {
        &self.func119_in_sel_cfg
    }
    #[doc = "0x4b4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func120_in_sel_cfg(&self) -> &FUNC120_IN_SEL_CFG {
        &self.func120_in_sel_cfg
    }
    #[doc = "0x4b8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func121_in_sel_cfg(&self) -> &FUNC121_IN_SEL_CFG {
        &self.func121_in_sel_cfg
    }
    #[doc = "0xad4..0xb5c - GPIO output function select register"]
    #[inline(always)]
    pub const fn func_out_sel_cfg(&self, n: usize) -> &FUNC_OUT_SEL_CFG {
        &self.func_out_sel_cfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xad4..0xb5c - GPIO output function select register"]
    #[inline(always)]
    pub fn func_out_sel_cfg_iter(&self) -> impl Iterator<Item = &FUNC_OUT_SEL_CFG> {
        self.func_out_sel_cfg.iter()
    }
    #[doc = "0xad4 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func0_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(0)
    }
    #[doc = "0xad8 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func1_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(1)
    }
    #[doc = "0xadc - GPIO output function select register"]
    #[inline(always)]
    pub const fn func2_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(2)
    }
    #[doc = "0xae0 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func3_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(3)
    }
    #[doc = "0xae4 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func4_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(4)
    }
    #[doc = "0xae8 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func5_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(5)
    }
    #[doc = "0xaec - GPIO output function select register"]
    #[inline(always)]
    pub const fn func6_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(6)
    }
    #[doc = "0xaf0 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func7_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(7)
    }
    #[doc = "0xaf4 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func8_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(8)
    }
    #[doc = "0xaf8 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func9_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(9)
    }
    #[doc = "0xafc - GPIO output function select register"]
    #[inline(always)]
    pub const fn func10_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(10)
    }
    #[doc = "0xb00 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func11_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(11)
    }
    #[doc = "0xb04 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func12_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(12)
    }
    #[doc = "0xb08 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func13_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(13)
    }
    #[doc = "0xb0c - GPIO output function select register"]
    #[inline(always)]
    pub const fn func14_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(14)
    }
    #[doc = "0xb10 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func15_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(15)
    }
    #[doc = "0xb14 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func16_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(16)
    }
    #[doc = "0xb18 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func17_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(17)
    }
    #[doc = "0xb1c - GPIO output function select register"]
    #[inline(always)]
    pub const fn func18_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(18)
    }
    #[doc = "0xb20 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func19_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(19)
    }
    #[doc = "0xb24 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func20_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(20)
    }
    #[doc = "0xb28 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func21_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(21)
    }
    #[doc = "0xb2c - GPIO output function select register"]
    #[inline(always)]
    pub const fn func22_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(22)
    }
    #[doc = "0xb30 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func23_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(23)
    }
    #[doc = "0xb34 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func24_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(24)
    }
    #[doc = "0xb38 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func25_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(25)
    }
    #[doc = "0xb3c - GPIO output function select register"]
    #[inline(always)]
    pub const fn func26_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(26)
    }
    #[doc = "0xb40 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func27_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(27)
    }
    #[doc = "0xb44 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func28_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(28)
    }
    #[doc = "0xb48 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func29_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(29)
    }
    #[doc = "0xb4c - GPIO output function select register"]
    #[inline(always)]
    pub const fn func30_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(30)
    }
    #[doc = "0xb50 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func31_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(31)
    }
    #[doc = "0xb54 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func32_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(32)
    }
    #[doc = "0xb58 - GPIO output function select register"]
    #[inline(always)]
    pub const fn func33_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(33)
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
#[doc = "STRAP (r) register accessor: pad strapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`strap::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@strap`] module"]
pub type STRAP = crate::Reg<strap::STRAP_SPEC>;
#[doc = "pad strapping register"]
pub mod strap;
#[doc = "OUT (rw) register accessor: GPIO output register for GPIO0-31\n\nYou can [`read`](crate::Reg::read) this register and get [`out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out`] module"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "GPIO output register for GPIO0-31"]
pub mod out;
#[doc = "OUT_W1TS (w) register accessor: GPIO output set register for GPIO0-31\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_w1ts`] module"]
pub type OUT_W1TS = crate::Reg<out_w1ts::OUT_W1TS_SPEC>;
#[doc = "GPIO output set register for GPIO0-31"]
pub mod out_w1ts;
#[doc = "OUT_W1TC (w) register accessor: GPIO output clear register for GPIO0-31\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_w1tc`] module"]
pub type OUT_W1TC = crate::Reg<out_w1tc::OUT_W1TC_SPEC>;
#[doc = "GPIO output clear register for GPIO0-31"]
pub mod out_w1tc;
#[doc = "OUT1 (rw) register accessor: GPIO output register for GPIO32-33\n\nYou can [`read`](crate::Reg::read) this register and get [`out1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out1`] module"]
pub type OUT1 = crate::Reg<out1::OUT1_SPEC>;
#[doc = "GPIO output register for GPIO32-33"]
pub mod out1;
#[doc = "OUT1_W1TS (w) register accessor: GPIO output set register for GPIO32-33\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out1_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out1_w1ts`] module"]
pub type OUT1_W1TS = crate::Reg<out1_w1ts::OUT1_W1TS_SPEC>;
#[doc = "GPIO output set register for GPIO32-33"]
pub mod out1_w1ts;
#[doc = "OUT1_W1TC (w) register accessor: GPIO output clear register for GPIO32-33\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out1_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out1_w1tc`] module"]
pub type OUT1_W1TC = crate::Reg<out1_w1tc::OUT1_W1TC_SPEC>;
#[doc = "GPIO output clear register for GPIO32-33"]
pub mod out1_w1tc;
#[doc = "ENABLE (rw) register accessor: GPIO output enable register for GPIO0-31\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "GPIO output enable register for GPIO0-31"]
pub mod enable;
#[doc = "ENABLE_W1TS (w) register accessor: GPIO output enable set register for GPIO0-31\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_w1ts`] module"]
pub type ENABLE_W1TS = crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>;
#[doc = "GPIO output enable set register for GPIO0-31"]
pub mod enable_w1ts;
#[doc = "ENABLE_W1TC (w) register accessor: GPIO output enable clear register for GPIO0-31\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_w1tc`] module"]
pub type ENABLE_W1TC = crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>;
#[doc = "GPIO output enable clear register for GPIO0-31"]
pub mod enable_w1tc;
#[doc = "ENABLE1 (rw) register accessor: GPIO output enable register for GPIO32-33\n\nYou can [`read`](crate::Reg::read) this register and get [`enable1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable1`] module"]
pub type ENABLE1 = crate::Reg<enable1::ENABLE1_SPEC>;
#[doc = "GPIO output enable register for GPIO32-33"]
pub mod enable1;
#[doc = "ENABLE1_W1TS (w) register accessor: GPIO output enable set register for GPIO32-33\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable1_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable1_w1ts`] module"]
pub type ENABLE1_W1TS = crate::Reg<enable1_w1ts::ENABLE1_W1TS_SPEC>;
#[doc = "GPIO output enable set register for GPIO32-33"]
pub mod enable1_w1ts;
#[doc = "ENABLE1_W1TC (w) register accessor: GPIO output enable clear register for GPIO32-33\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable1_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable1_w1tc`] module"]
pub type ENABLE1_W1TC = crate::Reg<enable1_w1tc::ENABLE1_W1TC_SPEC>;
#[doc = "GPIO output enable clear register for GPIO32-33"]
pub mod enable1_w1tc;
#[doc = "IN (r) register accessor: GPIO input register for GPIO0-31\n\nYou can [`read`](crate::Reg::read) this register and get [`in_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_`] module"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "GPIO input register for GPIO0-31"]
pub mod in_;
#[doc = "IN1 (r) register accessor: GPIO input register for GPIO32-33\n\nYou can [`read`](crate::Reg::read) this register and get [`in1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in1`] module"]
pub type IN1 = crate::Reg<in1::IN1_SPEC>;
#[doc = "GPIO input register for GPIO32-33"]
pub mod in1;
#[doc = "STATUS (rw) register accessor: GPIO interrupt status register for GPIO0-31\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "GPIO interrupt status register for GPIO0-31"]
pub mod status;
#[doc = "STATUS_W1TS (w) register accessor: GPIO interrupt status set register for GPIO0-31\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_w1ts`] module"]
pub type STATUS_W1TS = crate::Reg<status_w1ts::STATUS_W1TS_SPEC>;
#[doc = "GPIO interrupt status set register for GPIO0-31"]
pub mod status_w1ts;
#[doc = "STATUS_W1TC (w) register accessor: GPIO interrupt status clear register for GPIO0-31\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_w1tc`] module"]
pub type STATUS_W1TC = crate::Reg<status_w1tc::STATUS_W1TC_SPEC>;
#[doc = "GPIO interrupt status clear register for GPIO0-31"]
pub mod status_w1tc;
#[doc = "STATUS1 (rw) register accessor: GPIO interrupt status register for GPIO32-33\n\nYou can [`read`](crate::Reg::read) this register and get [`status1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status1`] module"]
pub type STATUS1 = crate::Reg<status1::STATUS1_SPEC>;
#[doc = "GPIO interrupt status register for GPIO32-33"]
pub mod status1;
#[doc = "STATUS1_W1TS (w) register accessor: GPIO interrupt status set register for GPIO32-33\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status1_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status1_w1ts`] module"]
pub type STATUS1_W1TS = crate::Reg<status1_w1ts::STATUS1_W1TS_SPEC>;
#[doc = "GPIO interrupt status set register for GPIO32-33"]
pub mod status1_w1ts;
#[doc = "STATUS1_W1TC (w) register accessor: GPIO interrupt status clear register for GPIO32-33\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status1_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status1_w1tc`] module"]
pub type STATUS1_W1TC = crate::Reg<status1_w1tc::STATUS1_W1TC_SPEC>;
#[doc = "GPIO interrupt status clear register for GPIO32-33"]
pub mod status1_w1tc;
#[doc = "PROCPU_INT (r) register accessor: GPIO_PROCPU_INT interrupt status register for GPIO0-31\n\nYou can [`read`](crate::Reg::read) this register and get [`procpu_int::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@procpu_int`] module"]
pub type PROCPU_INT = crate::Reg<procpu_int::PROCPU_INT_SPEC>;
#[doc = "GPIO_PROCPU_INT interrupt status register for GPIO0-31"]
pub mod procpu_int;
#[doc = "SDIO_INT (r) register accessor: GPIO_SDIO_INT interrupt status register for GPIO0-31\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_int::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_int`] module"]
pub type SDIO_INT = crate::Reg<sdio_int::SDIO_INT_SPEC>;
#[doc = "GPIO_SDIO_INT interrupt status register for GPIO0-31"]
pub mod sdio_int;
#[doc = "PROCPU_INT1 (r) register accessor: GPIO_PROCPU_INT interrupt status register for GPIO32-33\n\nYou can [`read`](crate::Reg::read) this register and get [`procpu_int1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@procpu_int1`] module"]
pub type PROCPU_INT1 = crate::Reg<procpu_int1::PROCPU_INT1_SPEC>;
#[doc = "GPIO_PROCPU_INT interrupt status register for GPIO32-33"]
pub mod procpu_int1;
#[doc = "SDIO_INT1 (r) register accessor: GPIO_SDIO_INT interrupt status register for GPIO32-33\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_int1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdio_int1`] module"]
pub type SDIO_INT1 = crate::Reg<sdio_int1::SDIO_INT1_SPEC>;
#[doc = "GPIO_SDIO_INT interrupt status register for GPIO32-33"]
pub mod sdio_int1;
#[doc = "STATUS_NEXT (r) register accessor: GPIO interrupt source register for GPIO0-31\n\nYou can [`read`](crate::Reg::read) this register and get [`status_next::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_next`] module"]
pub type STATUS_NEXT = crate::Reg<status_next::STATUS_NEXT_SPEC>;
#[doc = "GPIO interrupt source register for GPIO0-31"]
pub mod status_next;
#[doc = "STATUS_NEXT1 (r) register accessor: GPIO interrupt source register for GPIO32-33\n\nYou can [`read`](crate::Reg::read) this register and get [`status_next1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_next1`] module"]
pub type STATUS_NEXT1 = crate::Reg<status_next1::STATUS_NEXT1_SPEC>;
#[doc = "GPIO interrupt source register for GPIO32-33"]
pub mod status_next1;
#[doc = "PIN (rw) register accessor: GPIO pin configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pin::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pin::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pin`] module"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin;
#[doc = "FUNC0_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func0_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func0_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func0_in_sel_cfg`] module"]
pub type FUNC0_IN_SEL_CFG = crate::Reg<func0_in_sel_cfg::FUNC0_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func0_in_sel_cfg;
#[doc = "FUNC6_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func6_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func6_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func6_in_sel_cfg`] module"]
pub type FUNC6_IN_SEL_CFG = crate::Reg<func6_in_sel_cfg::FUNC6_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func6_in_sel_cfg;
#[doc = "FUNC7_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func7_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func7_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func7_in_sel_cfg`] module"]
pub type FUNC7_IN_SEL_CFG = crate::Reg<func7_in_sel_cfg::FUNC7_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func7_in_sel_cfg;
#[doc = "FUNC8_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func8_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func8_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func8_in_sel_cfg`] module"]
pub type FUNC8_IN_SEL_CFG = crate::Reg<func8_in_sel_cfg::FUNC8_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func8_in_sel_cfg;
#[doc = "FUNC9_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func9_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func9_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func9_in_sel_cfg`] module"]
pub type FUNC9_IN_SEL_CFG = crate::Reg<func9_in_sel_cfg::FUNC9_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func9_in_sel_cfg;
#[doc = "FUNC10_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func10_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func10_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func10_in_sel_cfg`] module"]
pub type FUNC10_IN_SEL_CFG = crate::Reg<func10_in_sel_cfg::FUNC10_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func10_in_sel_cfg;
#[doc = "FUNC11_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func11_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func11_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func11_in_sel_cfg`] module"]
pub type FUNC11_IN_SEL_CFG = crate::Reg<func11_in_sel_cfg::FUNC11_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func11_in_sel_cfg;
#[doc = "FUNC12_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func12_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func12_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func12_in_sel_cfg`] module"]
pub type FUNC12_IN_SEL_CFG = crate::Reg<func12_in_sel_cfg::FUNC12_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func12_in_sel_cfg;
#[doc = "FUNC13_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func13_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func13_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func13_in_sel_cfg`] module"]
pub type FUNC13_IN_SEL_CFG = crate::Reg<func13_in_sel_cfg::FUNC13_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func13_in_sel_cfg;
#[doc = "FUNC14_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func14_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func14_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func14_in_sel_cfg`] module"]
pub type FUNC14_IN_SEL_CFG = crate::Reg<func14_in_sel_cfg::FUNC14_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func14_in_sel_cfg;
#[doc = "FUNC15_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func15_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func15_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func15_in_sel_cfg`] module"]
pub type FUNC15_IN_SEL_CFG = crate::Reg<func15_in_sel_cfg::FUNC15_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func15_in_sel_cfg;
#[doc = "FUNC16_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func16_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func16_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func16_in_sel_cfg`] module"]
pub type FUNC16_IN_SEL_CFG = crate::Reg<func16_in_sel_cfg::FUNC16_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func16_in_sel_cfg;
#[doc = "FUNC17_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func17_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func17_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func17_in_sel_cfg`] module"]
pub type FUNC17_IN_SEL_CFG = crate::Reg<func17_in_sel_cfg::FUNC17_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func17_in_sel_cfg;
#[doc = "FUNC27_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func27_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func27_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func27_in_sel_cfg`] module"]
pub type FUNC27_IN_SEL_CFG = crate::Reg<func27_in_sel_cfg::FUNC27_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func27_in_sel_cfg;
#[doc = "FUNC28_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func28_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func28_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func28_in_sel_cfg`] module"]
pub type FUNC28_IN_SEL_CFG = crate::Reg<func28_in_sel_cfg::FUNC28_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func28_in_sel_cfg;
#[doc = "FUNC29_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func29_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func29_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func29_in_sel_cfg`] module"]
pub type FUNC29_IN_SEL_CFG = crate::Reg<func29_in_sel_cfg::FUNC29_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func29_in_sel_cfg;
#[doc = "FUNC30_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func30_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func30_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func30_in_sel_cfg`] module"]
pub type FUNC30_IN_SEL_CFG = crate::Reg<func30_in_sel_cfg::FUNC30_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func30_in_sel_cfg;
#[doc = "FUNC31_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func31_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func31_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func31_in_sel_cfg`] module"]
pub type FUNC31_IN_SEL_CFG = crate::Reg<func31_in_sel_cfg::FUNC31_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func31_in_sel_cfg;
#[doc = "FUNC32_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func32_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func32_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func32_in_sel_cfg`] module"]
pub type FUNC32_IN_SEL_CFG = crate::Reg<func32_in_sel_cfg::FUNC32_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func32_in_sel_cfg;
#[doc = "FUNC33_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func33_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func33_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func33_in_sel_cfg`] module"]
pub type FUNC33_IN_SEL_CFG = crate::Reg<func33_in_sel_cfg::FUNC33_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func33_in_sel_cfg;
#[doc = "FUNC34_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func34_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func34_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func34_in_sel_cfg`] module"]
pub type FUNC34_IN_SEL_CFG = crate::Reg<func34_in_sel_cfg::FUNC34_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func34_in_sel_cfg;
#[doc = "FUNC35_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func35_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func35_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func35_in_sel_cfg`] module"]
pub type FUNC35_IN_SEL_CFG = crate::Reg<func35_in_sel_cfg::FUNC35_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func35_in_sel_cfg;
#[doc = "FUNC41_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func41_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func41_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func41_in_sel_cfg`] module"]
pub type FUNC41_IN_SEL_CFG = crate::Reg<func41_in_sel_cfg::FUNC41_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func41_in_sel_cfg;
#[doc = "FUNC42_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func42_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func42_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func42_in_sel_cfg`] module"]
pub type FUNC42_IN_SEL_CFG = crate::Reg<func42_in_sel_cfg::FUNC42_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func42_in_sel_cfg;
#[doc = "FUNC43_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func43_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func43_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func43_in_sel_cfg`] module"]
pub type FUNC43_IN_SEL_CFG = crate::Reg<func43_in_sel_cfg::FUNC43_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func43_in_sel_cfg;
#[doc = "FUNC46_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func46_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func46_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func46_in_sel_cfg`] module"]
pub type FUNC46_IN_SEL_CFG = crate::Reg<func46_in_sel_cfg::FUNC46_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func46_in_sel_cfg;
#[doc = "FUNC47_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func47_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func47_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func47_in_sel_cfg`] module"]
pub type FUNC47_IN_SEL_CFG = crate::Reg<func47_in_sel_cfg::FUNC47_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func47_in_sel_cfg;
#[doc = "FUNC64_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func64_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func64_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func64_in_sel_cfg`] module"]
pub type FUNC64_IN_SEL_CFG = crate::Reg<func64_in_sel_cfg::FUNC64_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func64_in_sel_cfg;
#[doc = "FUNC65_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func65_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func65_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func65_in_sel_cfg`] module"]
pub type FUNC65_IN_SEL_CFG = crate::Reg<func65_in_sel_cfg::FUNC65_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func65_in_sel_cfg;
#[doc = "FUNC66_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func66_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func66_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func66_in_sel_cfg`] module"]
pub type FUNC66_IN_SEL_CFG = crate::Reg<func66_in_sel_cfg::FUNC66_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func66_in_sel_cfg;
#[doc = "FUNC67_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func67_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func67_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func67_in_sel_cfg`] module"]
pub type FUNC67_IN_SEL_CFG = crate::Reg<func67_in_sel_cfg::FUNC67_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func67_in_sel_cfg;
#[doc = "FUNC68_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func68_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func68_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func68_in_sel_cfg`] module"]
pub type FUNC68_IN_SEL_CFG = crate::Reg<func68_in_sel_cfg::FUNC68_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func68_in_sel_cfg;
#[doc = "FUNC69_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func69_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func69_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func69_in_sel_cfg`] module"]
pub type FUNC69_IN_SEL_CFG = crate::Reg<func69_in_sel_cfg::FUNC69_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func69_in_sel_cfg;
#[doc = "FUNC72_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func72_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func72_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func72_in_sel_cfg`] module"]
pub type FUNC72_IN_SEL_CFG = crate::Reg<func72_in_sel_cfg::FUNC72_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func72_in_sel_cfg;
#[doc = "FUNC73_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func73_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func73_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func73_in_sel_cfg`] module"]
pub type FUNC73_IN_SEL_CFG = crate::Reg<func73_in_sel_cfg::FUNC73_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func73_in_sel_cfg;
#[doc = "FUNC74_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func74_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func74_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func74_in_sel_cfg`] module"]
pub type FUNC74_IN_SEL_CFG = crate::Reg<func74_in_sel_cfg::FUNC74_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func74_in_sel_cfg;
#[doc = "FUNC82_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func82_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func82_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func82_in_sel_cfg`] module"]
pub type FUNC82_IN_SEL_CFG = crate::Reg<func82_in_sel_cfg::FUNC82_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func82_in_sel_cfg;
#[doc = "FUNC83_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func83_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func83_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func83_in_sel_cfg`] module"]
pub type FUNC83_IN_SEL_CFG = crate::Reg<func83_in_sel_cfg::FUNC83_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func83_in_sel_cfg;
#[doc = "FUNC97_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func97_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func97_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func97_in_sel_cfg`] module"]
pub type FUNC97_IN_SEL_CFG = crate::Reg<func97_in_sel_cfg::FUNC97_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func97_in_sel_cfg;
#[doc = "FUNC98_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func98_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func98_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func98_in_sel_cfg`] module"]
pub type FUNC98_IN_SEL_CFG = crate::Reg<func98_in_sel_cfg::FUNC98_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func98_in_sel_cfg;
#[doc = "FUNC99_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func99_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func99_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func99_in_sel_cfg`] module"]
pub type FUNC99_IN_SEL_CFG = crate::Reg<func99_in_sel_cfg::FUNC99_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func99_in_sel_cfg;
#[doc = "FUNC100_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func100_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func100_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func100_in_sel_cfg`] module"]
pub type FUNC100_IN_SEL_CFG = crate::Reg<func100_in_sel_cfg::FUNC100_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func100_in_sel_cfg;
#[doc = "FUNC118_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func118_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func118_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func118_in_sel_cfg`] module"]
pub type FUNC118_IN_SEL_CFG = crate::Reg<func118_in_sel_cfg::FUNC118_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func118_in_sel_cfg;
#[doc = "FUNC119_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func119_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func119_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func119_in_sel_cfg`] module"]
pub type FUNC119_IN_SEL_CFG = crate::Reg<func119_in_sel_cfg::FUNC119_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func119_in_sel_cfg;
#[doc = "FUNC120_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func120_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func120_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func120_in_sel_cfg`] module"]
pub type FUNC120_IN_SEL_CFG = crate::Reg<func120_in_sel_cfg::FUNC120_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func120_in_sel_cfg;
#[doc = "FUNC121_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func121_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func121_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func121_in_sel_cfg`] module"]
pub type FUNC121_IN_SEL_CFG = crate::Reg<func121_in_sel_cfg::FUNC121_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func121_in_sel_cfg;
#[doc = "FUNC_OUT_SEL_CFG (rw) register accessor: GPIO output function select register\n\nYou can [`read`](crate::Reg::read) this register and get [`func_out_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func_out_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_out_sel_cfg`] module"]
pub type FUNC_OUT_SEL_CFG = crate::Reg<func_out_sel_cfg::FUNC_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func_out_sel_cfg;
#[doc = "CLOCK_GATE (rw) register accessor: GPIO clock gate register\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_gate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock_gate`] module"]
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
#[doc = "GPIO clock gate register"]
pub mod clock_gate;
#[doc = "DATE (rw) register accessor: GPIO version register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@date`] module"]
pub type DATE = crate::Reg<date::DATE_SPEC>;
#[doc = "GPIO version register"]
pub mod date;
