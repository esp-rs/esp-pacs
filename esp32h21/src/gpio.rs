#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    strap: STRAP,
    out: OUT,
    out_w1ts: OUT_W1TS,
    out_w1tc: OUT_W1TC,
    _reserved4: [u8; 0x24],
    enable: ENABLE,
    enable_w1ts: ENABLE_W1TS,
    enable_w1tc: ENABLE_W1TC,
    _reserved7: [u8; 0x24],
    in_: IN,
    _reserved8: [u8; 0x0c],
    status: STATUS,
    status_w1ts: STATUS_W1TS,
    status_w1tc: STATUS_W1TC,
    _reserved11: [u8; 0x24],
    procpu_int: PROCPU_INT,
    procpu_nmi_int: PROCPU_NMI_INT,
    _reserved13: [u8; 0x18],
    status_next: STATUS_NEXT,
    _reserved14: [u8; 0x0c],
    pin: [PIN; 30],
    _reserved15: [u8; 0x0188],
    func0_in_sel_cfg: FUNC0_IN_SEL_CFG,
    _reserved16: [u8; 0x14],
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
    _reserved28: [u8; 0x04],
    func19_in_sel_cfg: FUNC19_IN_SEL_CFG,
    _reserved29: [u8; 0x20],
    func28_in_sel_cfg: FUNC28_IN_SEL_CFG,
    func29_in_sel_cfg: FUNC29_IN_SEL_CFG,
    func30_in_sel_cfg: FUNC30_IN_SEL_CFG,
    func31_in_sel_cfg: FUNC31_IN_SEL_CFG,
    func32_in_sel_cfg: FUNC32_IN_SEL_CFG,
    func33_in_sel_cfg: FUNC33_IN_SEL_CFG,
    func34_in_sel_cfg: FUNC34_IN_SEL_CFG,
    func35_in_sel_cfg: FUNC35_IN_SEL_CFG,
    _reserved37: [u8; 0x10],
    func40_in_sel_cfg: FUNC40_IN_SEL_CFG,
    func41_in_sel_cfg: FUNC41_IN_SEL_CFG,
    func42_in_sel_cfg: FUNC42_IN_SEL_CFG,
    _reserved40: [u8; 0x08],
    func45_in_sel_cfg: FUNC45_IN_SEL_CFG,
    func46_in_sel_cfg: FUNC46_IN_SEL_CFG,
    func47_in_sel_cfg: FUNC47_IN_SEL_CFG,
    func48_in_sel_cfg: FUNC48_IN_SEL_CFG,
    func49_in_sel_cfg: FUNC49_IN_SEL_CFG,
    func50_in_sel_cfg: FUNC50_IN_SEL_CFG,
    func51_in_sel_cfg: FUNC51_IN_SEL_CFG,
    func52_in_sel_cfg: FUNC52_IN_SEL_CFG,
    func53_in_sel_cfg: FUNC53_IN_SEL_CFG,
    func54_in_sel_cfg: FUNC54_IN_SEL_CFG,
    func55_in_sel_cfg: FUNC55_IN_SEL_CFG,
    func56_in_sel_cfg: FUNC56_IN_SEL_CFG,
    _reserved52: [u8; 0x18],
    func63_in_sel_cfg: FUNC63_IN_SEL_CFG,
    func64_in_sel_cfg: FUNC64_IN_SEL_CFG,
    func65_in_sel_cfg: FUNC65_IN_SEL_CFG,
    func66_in_sel_cfg: FUNC66_IN_SEL_CFG,
    func67_in_sel_cfg: FUNC67_IN_SEL_CFG,
    func68_in_sel_cfg: FUNC68_IN_SEL_CFG,
    func69_in_sel_cfg: FUNC69_IN_SEL_CFG,
    func70_in_sel_cfg: FUNC70_IN_SEL_CFG,
    func71_in_sel_cfg: FUNC71_IN_SEL_CFG,
    func72_in_sel_cfg: FUNC72_IN_SEL_CFG,
    func73_in_sel_cfg: FUNC73_IN_SEL_CFG,
    _reserved63: [u8; 0x0c],
    func77_in_sel_cfg: FUNC77_IN_SEL_CFG,
    func78_in_sel_cfg: FUNC78_IN_SEL_CFG,
    func79_in_sel_cfg: FUNC79_IN_SEL_CFG,
    func80_in_sel_cfg: FUNC80_IN_SEL_CFG,
    func81_in_sel_cfg: FUNC81_IN_SEL_CFG,
    func82_in_sel_cfg: FUNC82_IN_SEL_CFG,
    _reserved69: [u8; 0x10],
    func87_in_sel_cfg: FUNC87_IN_SEL_CFG,
    func88_in_sel_cfg: FUNC88_IN_SEL_CFG,
    func89_in_sel_cfg: FUNC89_IN_SEL_CFG,
    func90_in_sel_cfg: FUNC90_IN_SEL_CFG,
    func91_in_sel_cfg: FUNC91_IN_SEL_CFG,
    func92_in_sel_cfg: FUNC92_IN_SEL_CFG,
    func93_in_sel_cfg: FUNC93_IN_SEL_CFG,
    func94_in_sel_cfg: FUNC94_IN_SEL_CFG,
    func95_in_sel_cfg: FUNC95_IN_SEL_CFG,
    _reserved78: [u8; 0x04],
    func97_in_sel_cfg: FUNC97_IN_SEL_CFG,
    func98_in_sel_cfg: FUNC98_IN_SEL_CFG,
    func99_in_sel_cfg: FUNC99_IN_SEL_CFG,
    func100_in_sel_cfg: FUNC100_IN_SEL_CFG,
    func101_in_sel_cfg: FUNC101_IN_SEL_CFG,
    func102_in_sel_cfg: FUNC102_IN_SEL_CFG,
    func103_in_sel_cfg: FUNC103_IN_SEL_CFG,
    func104_in_sel_cfg: FUNC104_IN_SEL_CFG,
    func105_in_sel_cfg: FUNC105_IN_SEL_CFG,
    func106_in_sel_cfg: FUNC106_IN_SEL_CFG,
    func107_in_sel_cfg: FUNC107_IN_SEL_CFG,
    func108_in_sel_cfg: FUNC108_IN_SEL_CFG,
    func109_in_sel_cfg: FUNC109_IN_SEL_CFG,
    func110_in_sel_cfg: FUNC110_IN_SEL_CFG,
    func111_in_sel_cfg: FUNC111_IN_SEL_CFG,
    func112_in_sel_cfg: FUNC112_IN_SEL_CFG,
    func113_in_sel_cfg: FUNC113_IN_SEL_CFG,
    func114_in_sel_cfg: FUNC114_IN_SEL_CFG,
    func115_in_sel_cfg: FUNC115_IN_SEL_CFG,
    func116_in_sel_cfg: FUNC116_IN_SEL_CFG,
    func117_in_sel_cfg: FUNC117_IN_SEL_CFG,
    func118_in_sel_cfg: FUNC118_IN_SEL_CFG,
    func119_in_sel_cfg: FUNC119_IN_SEL_CFG,
    func120_in_sel_cfg: FUNC120_IN_SEL_CFG,
    func121_in_sel_cfg: FUNC121_IN_SEL_CFG,
    func122_in_sel_cfg: FUNC122_IN_SEL_CFG,
    func123_in_sel_cfg: FUNC123_IN_SEL_CFG,
    func124_in_sel_cfg: FUNC124_IN_SEL_CFG,
    _reserved106: [u8; 0x060c],
    func_out_sel_cfg: [FUNC_OUT_SEL_CFG; 30],
    _reserved107: [u8; 0x02ac],
    clock_gate: CLOCK_GATE,
    date: DATE,
}
impl RegisterBlock {
    #[doc = "0x00 - pad strapping register"]
    #[inline(always)]
    pub const fn strap(&self) -> &STRAP {
        &self.strap
    }
    #[doc = "0x04 - GPIO output register for GPIO0-29"]
    #[inline(always)]
    pub const fn out(&self) -> &OUT {
        &self.out
    }
    #[doc = "0x08 - GPIO output set register for GPIO0-29"]
    #[inline(always)]
    pub const fn out_w1ts(&self) -> &OUT_W1TS {
        &self.out_w1ts
    }
    #[doc = "0x0c - GPIO output clear register for GPIO0-29"]
    #[inline(always)]
    pub const fn out_w1tc(&self) -> &OUT_W1TC {
        &self.out_w1tc
    }
    #[doc = "0x34 - GPIO output enable register for GPIO0-29"]
    #[inline(always)]
    pub const fn enable(&self) -> &ENABLE {
        &self.enable
    }
    #[doc = "0x38 - GPIO output enable set register for GPIO0-29"]
    #[inline(always)]
    pub const fn enable_w1ts(&self) -> &ENABLE_W1TS {
        &self.enable_w1ts
    }
    #[doc = "0x3c - GPIO output enable clear register for GPIO0-29"]
    #[inline(always)]
    pub const fn enable_w1tc(&self) -> &ENABLE_W1TC {
        &self.enable_w1tc
    }
    #[doc = "0x64 - GPIO input register for GPIO0-29"]
    #[inline(always)]
    pub const fn in_(&self) -> &IN {
        &self.in_
    }
    #[doc = "0x74 - GPIO interrupt status register for GPIO0-29"]
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    #[doc = "0x78 - GPIO interrupt status set register for GPIO0-29"]
    #[inline(always)]
    pub const fn status_w1ts(&self) -> &STATUS_W1TS {
        &self.status_w1ts
    }
    #[doc = "0x7c - GPIO interrupt status clear register for GPIO0-29"]
    #[inline(always)]
    pub const fn status_w1tc(&self) -> &STATUS_W1TC {
        &self.status_w1tc
    }
    #[doc = "0xa4 - GPIO_PROCPU_INT interrupt status register for GPIO0-29"]
    #[inline(always)]
    pub const fn procpu_int(&self) -> &PROCPU_INT {
        &self.procpu_int
    }
    #[doc = "0xa8 - GPIO_PROCPU_NMI_INT interrupt status register for GPIO0-29"]
    #[inline(always)]
    pub const fn procpu_nmi_int(&self) -> &PROCPU_NMI_INT {
        &self.procpu_nmi_int
    }
    #[doc = "0xc4 - GPIO interrupt source register for GPIO0-29"]
    #[inline(always)]
    pub const fn status_next(&self) -> &STATUS_NEXT {
        &self.status_next
    }
    #[doc = "0xd4..0x14c - GPIO pin configuration register"]
    #[inline(always)]
    pub const fn pin(&self, n: usize) -> &PIN {
        &self.pin[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xd4..0x14c - GPIO pin configuration register"]
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
    #[doc = "0x320 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func19_in_sel_cfg(&self) -> &FUNC19_IN_SEL_CFG {
        &self.func19_in_sel_cfg
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
    #[doc = "0x374 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func40_in_sel_cfg(&self) -> &FUNC40_IN_SEL_CFG {
        &self.func40_in_sel_cfg
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
    #[doc = "0x388 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func45_in_sel_cfg(&self) -> &FUNC45_IN_SEL_CFG {
        &self.func45_in_sel_cfg
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
    #[doc = "0x394 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func48_in_sel_cfg(&self) -> &FUNC48_IN_SEL_CFG {
        &self.func48_in_sel_cfg
    }
    #[doc = "0x398 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func49_in_sel_cfg(&self) -> &FUNC49_IN_SEL_CFG {
        &self.func49_in_sel_cfg
    }
    #[doc = "0x39c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func50_in_sel_cfg(&self) -> &FUNC50_IN_SEL_CFG {
        &self.func50_in_sel_cfg
    }
    #[doc = "0x3a0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func51_in_sel_cfg(&self) -> &FUNC51_IN_SEL_CFG {
        &self.func51_in_sel_cfg
    }
    #[doc = "0x3a4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func52_in_sel_cfg(&self) -> &FUNC52_IN_SEL_CFG {
        &self.func52_in_sel_cfg
    }
    #[doc = "0x3a8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func53_in_sel_cfg(&self) -> &FUNC53_IN_SEL_CFG {
        &self.func53_in_sel_cfg
    }
    #[doc = "0x3ac - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func54_in_sel_cfg(&self) -> &FUNC54_IN_SEL_CFG {
        &self.func54_in_sel_cfg
    }
    #[doc = "0x3b0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func55_in_sel_cfg(&self) -> &FUNC55_IN_SEL_CFG {
        &self.func55_in_sel_cfg
    }
    #[doc = "0x3b4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func56_in_sel_cfg(&self) -> &FUNC56_IN_SEL_CFG {
        &self.func56_in_sel_cfg
    }
    #[doc = "0x3d0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func63_in_sel_cfg(&self) -> &FUNC63_IN_SEL_CFG {
        &self.func63_in_sel_cfg
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
    #[doc = "0x3ec - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func70_in_sel_cfg(&self) -> &FUNC70_IN_SEL_CFG {
        &self.func70_in_sel_cfg
    }
    #[doc = "0x3f0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func71_in_sel_cfg(&self) -> &FUNC71_IN_SEL_CFG {
        &self.func71_in_sel_cfg
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
    #[doc = "0x408 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func77_in_sel_cfg(&self) -> &FUNC77_IN_SEL_CFG {
        &self.func77_in_sel_cfg
    }
    #[doc = "0x40c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func78_in_sel_cfg(&self) -> &FUNC78_IN_SEL_CFG {
        &self.func78_in_sel_cfg
    }
    #[doc = "0x410 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func79_in_sel_cfg(&self) -> &FUNC79_IN_SEL_CFG {
        &self.func79_in_sel_cfg
    }
    #[doc = "0x414 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func80_in_sel_cfg(&self) -> &FUNC80_IN_SEL_CFG {
        &self.func80_in_sel_cfg
    }
    #[doc = "0x418 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func81_in_sel_cfg(&self) -> &FUNC81_IN_SEL_CFG {
        &self.func81_in_sel_cfg
    }
    #[doc = "0x41c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func82_in_sel_cfg(&self) -> &FUNC82_IN_SEL_CFG {
        &self.func82_in_sel_cfg
    }
    #[doc = "0x430 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func87_in_sel_cfg(&self) -> &FUNC87_IN_SEL_CFG {
        &self.func87_in_sel_cfg
    }
    #[doc = "0x434 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func88_in_sel_cfg(&self) -> &FUNC88_IN_SEL_CFG {
        &self.func88_in_sel_cfg
    }
    #[doc = "0x438 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func89_in_sel_cfg(&self) -> &FUNC89_IN_SEL_CFG {
        &self.func89_in_sel_cfg
    }
    #[doc = "0x43c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func90_in_sel_cfg(&self) -> &FUNC90_IN_SEL_CFG {
        &self.func90_in_sel_cfg
    }
    #[doc = "0x440 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func91_in_sel_cfg(&self) -> &FUNC91_IN_SEL_CFG {
        &self.func91_in_sel_cfg
    }
    #[doc = "0x444 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func92_in_sel_cfg(&self) -> &FUNC92_IN_SEL_CFG {
        &self.func92_in_sel_cfg
    }
    #[doc = "0x448 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func93_in_sel_cfg(&self) -> &FUNC93_IN_SEL_CFG {
        &self.func93_in_sel_cfg
    }
    #[doc = "0x44c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func94_in_sel_cfg(&self) -> &FUNC94_IN_SEL_CFG {
        &self.func94_in_sel_cfg
    }
    #[doc = "0x450 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func95_in_sel_cfg(&self) -> &FUNC95_IN_SEL_CFG {
        &self.func95_in_sel_cfg
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
    #[doc = "0x468 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func101_in_sel_cfg(&self) -> &FUNC101_IN_SEL_CFG {
        &self.func101_in_sel_cfg
    }
    #[doc = "0x46c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func102_in_sel_cfg(&self) -> &FUNC102_IN_SEL_CFG {
        &self.func102_in_sel_cfg
    }
    #[doc = "0x470 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func103_in_sel_cfg(&self) -> &FUNC103_IN_SEL_CFG {
        &self.func103_in_sel_cfg
    }
    #[doc = "0x474 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func104_in_sel_cfg(&self) -> &FUNC104_IN_SEL_CFG {
        &self.func104_in_sel_cfg
    }
    #[doc = "0x478 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func105_in_sel_cfg(&self) -> &FUNC105_IN_SEL_CFG {
        &self.func105_in_sel_cfg
    }
    #[doc = "0x47c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func106_in_sel_cfg(&self) -> &FUNC106_IN_SEL_CFG {
        &self.func106_in_sel_cfg
    }
    #[doc = "0x480 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func107_in_sel_cfg(&self) -> &FUNC107_IN_SEL_CFG {
        &self.func107_in_sel_cfg
    }
    #[doc = "0x484 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func108_in_sel_cfg(&self) -> &FUNC108_IN_SEL_CFG {
        &self.func108_in_sel_cfg
    }
    #[doc = "0x488 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func109_in_sel_cfg(&self) -> &FUNC109_IN_SEL_CFG {
        &self.func109_in_sel_cfg
    }
    #[doc = "0x48c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func110_in_sel_cfg(&self) -> &FUNC110_IN_SEL_CFG {
        &self.func110_in_sel_cfg
    }
    #[doc = "0x490 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func111_in_sel_cfg(&self) -> &FUNC111_IN_SEL_CFG {
        &self.func111_in_sel_cfg
    }
    #[doc = "0x494 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func112_in_sel_cfg(&self) -> &FUNC112_IN_SEL_CFG {
        &self.func112_in_sel_cfg
    }
    #[doc = "0x498 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func113_in_sel_cfg(&self) -> &FUNC113_IN_SEL_CFG {
        &self.func113_in_sel_cfg
    }
    #[doc = "0x49c - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func114_in_sel_cfg(&self) -> &FUNC114_IN_SEL_CFG {
        &self.func114_in_sel_cfg
    }
    #[doc = "0x4a0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func115_in_sel_cfg(&self) -> &FUNC115_IN_SEL_CFG {
        &self.func115_in_sel_cfg
    }
    #[doc = "0x4a4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func116_in_sel_cfg(&self) -> &FUNC116_IN_SEL_CFG {
        &self.func116_in_sel_cfg
    }
    #[doc = "0x4a8 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func117_in_sel_cfg(&self) -> &FUNC117_IN_SEL_CFG {
        &self.func117_in_sel_cfg
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
    #[doc = "0x4bc - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func122_in_sel_cfg(&self) -> &FUNC122_IN_SEL_CFG {
        &self.func122_in_sel_cfg
    }
    #[doc = "0x4c0 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func123_in_sel_cfg(&self) -> &FUNC123_IN_SEL_CFG {
        &self.func123_in_sel_cfg
    }
    #[doc = "0x4c4 - GPIO input function configuration register"]
    #[inline(always)]
    pub const fn func124_in_sel_cfg(&self) -> &FUNC124_IN_SEL_CFG {
        &self.func124_in_sel_cfg
    }
    #[doc = "0xad4..0xb4c - GPIO output function select register"]
    #[inline(always)]
    pub const fn func_out_sel_cfg(&self, n: usize) -> &FUNC_OUT_SEL_CFG {
        &self.func_out_sel_cfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xad4..0xb4c - GPIO output function select register"]
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
#[doc = "OUT (rw) register accessor: GPIO output register for GPIO0-29\n\nYou can [`read`](crate::Reg::read) this register and get [`out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out`] module"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "GPIO output register for GPIO0-29"]
pub mod out;
#[doc = "OUT_W1TS (w) register accessor: GPIO output set register for GPIO0-29\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_w1ts`] module"]
pub type OUT_W1TS = crate::Reg<out_w1ts::OUT_W1TS_SPEC>;
#[doc = "GPIO output set register for GPIO0-29"]
pub mod out_w1ts;
#[doc = "OUT_W1TC (w) register accessor: GPIO output clear register for GPIO0-29\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out_w1tc`] module"]
pub type OUT_W1TC = crate::Reg<out_w1tc::OUT_W1TC_SPEC>;
#[doc = "GPIO output clear register for GPIO0-29"]
pub mod out_w1tc;
#[doc = "ENABLE (rw) register accessor: GPIO output enable register for GPIO0-29\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "GPIO output enable register for GPIO0-29"]
pub mod enable;
#[doc = "ENABLE_W1TS (w) register accessor: GPIO output enable set register for GPIO0-29\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_w1ts`] module"]
pub type ENABLE_W1TS = crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>;
#[doc = "GPIO output enable set register for GPIO0-29"]
pub mod enable_w1ts;
#[doc = "ENABLE_W1TC (w) register accessor: GPIO output enable clear register for GPIO0-29\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_w1tc`] module"]
pub type ENABLE_W1TC = crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>;
#[doc = "GPIO output enable clear register for GPIO0-29"]
pub mod enable_w1tc;
#[doc = "IN (r) register accessor: GPIO input register for GPIO0-29\n\nYou can [`read`](crate::Reg::read) this register and get [`in_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_`] module"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "GPIO input register for GPIO0-29"]
pub mod in_;
#[doc = "STATUS (rw) register accessor: GPIO interrupt status register for GPIO0-29\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "GPIO interrupt status register for GPIO0-29"]
pub mod status;
#[doc = "STATUS_W1TS (w) register accessor: GPIO interrupt status set register for GPIO0-29\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_w1ts`] module"]
pub type STATUS_W1TS = crate::Reg<status_w1ts::STATUS_W1TS_SPEC>;
#[doc = "GPIO interrupt status set register for GPIO0-29"]
pub mod status_w1ts;
#[doc = "STATUS_W1TC (w) register accessor: GPIO interrupt status clear register for GPIO0-29\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_w1tc`] module"]
pub type STATUS_W1TC = crate::Reg<status_w1tc::STATUS_W1TC_SPEC>;
#[doc = "GPIO interrupt status clear register for GPIO0-29"]
pub mod status_w1tc;
#[doc = "PROCPU_INT (r) register accessor: GPIO_PROCPU_INT interrupt status register for GPIO0-29\n\nYou can [`read`](crate::Reg::read) this register and get [`procpu_int::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@procpu_int`] module"]
pub type PROCPU_INT = crate::Reg<procpu_int::PROCPU_INT_SPEC>;
#[doc = "GPIO_PROCPU_INT interrupt status register for GPIO0-29"]
pub mod procpu_int;
#[doc = "PROCPU_NMI_INT (r) register accessor: GPIO_PROCPU_NMI_INT interrupt status register for GPIO0-29\n\nYou can [`read`](crate::Reg::read) this register and get [`procpu_nmi_int::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@procpu_nmi_int`] module"]
pub type PROCPU_NMI_INT = crate::Reg<procpu_nmi_int::PROCPU_NMI_INT_SPEC>;
#[doc = "GPIO_PROCPU_NMI_INT interrupt status register for GPIO0-29"]
pub mod procpu_nmi_int;
#[doc = "STATUS_NEXT (r) register accessor: GPIO interrupt source register for GPIO0-29\n\nYou can [`read`](crate::Reg::read) this register and get [`status_next::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_next`] module"]
pub type STATUS_NEXT = crate::Reg<status_next::STATUS_NEXT_SPEC>;
#[doc = "GPIO interrupt source register for GPIO0-29"]
pub mod status_next;
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
#[doc = "FUNC19_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func19_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func19_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func19_in_sel_cfg`] module"]
pub type FUNC19_IN_SEL_CFG = crate::Reg<func19_in_sel_cfg::FUNC19_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func19_in_sel_cfg;
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
#[doc = "FUNC40_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func40_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func40_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func40_in_sel_cfg`] module"]
pub type FUNC40_IN_SEL_CFG = crate::Reg<func40_in_sel_cfg::FUNC40_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func40_in_sel_cfg;
#[doc = "FUNC41_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func41_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func41_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func41_in_sel_cfg`] module"]
pub type FUNC41_IN_SEL_CFG = crate::Reg<func41_in_sel_cfg::FUNC41_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func41_in_sel_cfg;
#[doc = "FUNC42_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func42_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func42_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func42_in_sel_cfg`] module"]
pub type FUNC42_IN_SEL_CFG = crate::Reg<func42_in_sel_cfg::FUNC42_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func42_in_sel_cfg;
#[doc = "FUNC45_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func45_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func45_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func45_in_sel_cfg`] module"]
pub type FUNC45_IN_SEL_CFG = crate::Reg<func45_in_sel_cfg::FUNC45_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func45_in_sel_cfg;
#[doc = "FUNC46_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func46_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func46_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func46_in_sel_cfg`] module"]
pub type FUNC46_IN_SEL_CFG = crate::Reg<func46_in_sel_cfg::FUNC46_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func46_in_sel_cfg;
#[doc = "FUNC47_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func47_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func47_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func47_in_sel_cfg`] module"]
pub type FUNC47_IN_SEL_CFG = crate::Reg<func47_in_sel_cfg::FUNC47_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func47_in_sel_cfg;
#[doc = "FUNC48_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func48_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func48_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func48_in_sel_cfg`] module"]
pub type FUNC48_IN_SEL_CFG = crate::Reg<func48_in_sel_cfg::FUNC48_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func48_in_sel_cfg;
#[doc = "FUNC49_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func49_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func49_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func49_in_sel_cfg`] module"]
pub type FUNC49_IN_SEL_CFG = crate::Reg<func49_in_sel_cfg::FUNC49_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func49_in_sel_cfg;
#[doc = "FUNC50_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func50_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func50_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func50_in_sel_cfg`] module"]
pub type FUNC50_IN_SEL_CFG = crate::Reg<func50_in_sel_cfg::FUNC50_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func50_in_sel_cfg;
#[doc = "FUNC51_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func51_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func51_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func51_in_sel_cfg`] module"]
pub type FUNC51_IN_SEL_CFG = crate::Reg<func51_in_sel_cfg::FUNC51_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func51_in_sel_cfg;
#[doc = "FUNC52_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func52_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func52_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func52_in_sel_cfg`] module"]
pub type FUNC52_IN_SEL_CFG = crate::Reg<func52_in_sel_cfg::FUNC52_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func52_in_sel_cfg;
#[doc = "FUNC53_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func53_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func53_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func53_in_sel_cfg`] module"]
pub type FUNC53_IN_SEL_CFG = crate::Reg<func53_in_sel_cfg::FUNC53_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func53_in_sel_cfg;
#[doc = "FUNC54_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func54_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func54_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func54_in_sel_cfg`] module"]
pub type FUNC54_IN_SEL_CFG = crate::Reg<func54_in_sel_cfg::FUNC54_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func54_in_sel_cfg;
#[doc = "FUNC55_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func55_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func55_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func55_in_sel_cfg`] module"]
pub type FUNC55_IN_SEL_CFG = crate::Reg<func55_in_sel_cfg::FUNC55_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func55_in_sel_cfg;
#[doc = "FUNC56_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func56_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func56_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func56_in_sel_cfg`] module"]
pub type FUNC56_IN_SEL_CFG = crate::Reg<func56_in_sel_cfg::FUNC56_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func56_in_sel_cfg;
#[doc = "FUNC63_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func63_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func63_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func63_in_sel_cfg`] module"]
pub type FUNC63_IN_SEL_CFG = crate::Reg<func63_in_sel_cfg::FUNC63_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func63_in_sel_cfg;
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
#[doc = "FUNC70_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func70_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func70_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func70_in_sel_cfg`] module"]
pub type FUNC70_IN_SEL_CFG = crate::Reg<func70_in_sel_cfg::FUNC70_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func70_in_sel_cfg;
#[doc = "FUNC71_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func71_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func71_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func71_in_sel_cfg`] module"]
pub type FUNC71_IN_SEL_CFG = crate::Reg<func71_in_sel_cfg::FUNC71_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func71_in_sel_cfg;
#[doc = "FUNC72_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func72_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func72_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func72_in_sel_cfg`] module"]
pub type FUNC72_IN_SEL_CFG = crate::Reg<func72_in_sel_cfg::FUNC72_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func72_in_sel_cfg;
#[doc = "FUNC73_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func73_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func73_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func73_in_sel_cfg`] module"]
pub type FUNC73_IN_SEL_CFG = crate::Reg<func73_in_sel_cfg::FUNC73_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func73_in_sel_cfg;
#[doc = "FUNC77_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func77_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func77_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func77_in_sel_cfg`] module"]
pub type FUNC77_IN_SEL_CFG = crate::Reg<func77_in_sel_cfg::FUNC77_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func77_in_sel_cfg;
#[doc = "FUNC78_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func78_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func78_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func78_in_sel_cfg`] module"]
pub type FUNC78_IN_SEL_CFG = crate::Reg<func78_in_sel_cfg::FUNC78_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func78_in_sel_cfg;
#[doc = "FUNC79_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func79_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func79_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func79_in_sel_cfg`] module"]
pub type FUNC79_IN_SEL_CFG = crate::Reg<func79_in_sel_cfg::FUNC79_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func79_in_sel_cfg;
#[doc = "FUNC80_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func80_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func80_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func80_in_sel_cfg`] module"]
pub type FUNC80_IN_SEL_CFG = crate::Reg<func80_in_sel_cfg::FUNC80_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func80_in_sel_cfg;
#[doc = "FUNC81_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func81_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func81_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func81_in_sel_cfg`] module"]
pub type FUNC81_IN_SEL_CFG = crate::Reg<func81_in_sel_cfg::FUNC81_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func81_in_sel_cfg;
#[doc = "FUNC82_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func82_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func82_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func82_in_sel_cfg`] module"]
pub type FUNC82_IN_SEL_CFG = crate::Reg<func82_in_sel_cfg::FUNC82_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func82_in_sel_cfg;
#[doc = "FUNC87_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func87_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func87_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func87_in_sel_cfg`] module"]
pub type FUNC87_IN_SEL_CFG = crate::Reg<func87_in_sel_cfg::FUNC87_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func87_in_sel_cfg;
#[doc = "FUNC88_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func88_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func88_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func88_in_sel_cfg`] module"]
pub type FUNC88_IN_SEL_CFG = crate::Reg<func88_in_sel_cfg::FUNC88_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func88_in_sel_cfg;
#[doc = "FUNC89_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func89_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func89_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func89_in_sel_cfg`] module"]
pub type FUNC89_IN_SEL_CFG = crate::Reg<func89_in_sel_cfg::FUNC89_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func89_in_sel_cfg;
#[doc = "FUNC90_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func90_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func90_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func90_in_sel_cfg`] module"]
pub type FUNC90_IN_SEL_CFG = crate::Reg<func90_in_sel_cfg::FUNC90_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func90_in_sel_cfg;
#[doc = "FUNC91_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func91_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func91_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func91_in_sel_cfg`] module"]
pub type FUNC91_IN_SEL_CFG = crate::Reg<func91_in_sel_cfg::FUNC91_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func91_in_sel_cfg;
#[doc = "FUNC92_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func92_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func92_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func92_in_sel_cfg`] module"]
pub type FUNC92_IN_SEL_CFG = crate::Reg<func92_in_sel_cfg::FUNC92_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func92_in_sel_cfg;
#[doc = "FUNC93_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func93_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func93_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func93_in_sel_cfg`] module"]
pub type FUNC93_IN_SEL_CFG = crate::Reg<func93_in_sel_cfg::FUNC93_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func93_in_sel_cfg;
#[doc = "FUNC94_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func94_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func94_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func94_in_sel_cfg`] module"]
pub type FUNC94_IN_SEL_CFG = crate::Reg<func94_in_sel_cfg::FUNC94_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func94_in_sel_cfg;
#[doc = "FUNC95_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func95_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func95_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func95_in_sel_cfg`] module"]
pub type FUNC95_IN_SEL_CFG = crate::Reg<func95_in_sel_cfg::FUNC95_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func95_in_sel_cfg;
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
#[doc = "FUNC101_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func101_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func101_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func101_in_sel_cfg`] module"]
pub type FUNC101_IN_SEL_CFG = crate::Reg<func101_in_sel_cfg::FUNC101_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func101_in_sel_cfg;
#[doc = "FUNC102_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func102_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func102_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func102_in_sel_cfg`] module"]
pub type FUNC102_IN_SEL_CFG = crate::Reg<func102_in_sel_cfg::FUNC102_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func102_in_sel_cfg;
#[doc = "FUNC103_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func103_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func103_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func103_in_sel_cfg`] module"]
pub type FUNC103_IN_SEL_CFG = crate::Reg<func103_in_sel_cfg::FUNC103_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func103_in_sel_cfg;
#[doc = "FUNC104_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func104_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func104_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func104_in_sel_cfg`] module"]
pub type FUNC104_IN_SEL_CFG = crate::Reg<func104_in_sel_cfg::FUNC104_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func104_in_sel_cfg;
#[doc = "FUNC105_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func105_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func105_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func105_in_sel_cfg`] module"]
pub type FUNC105_IN_SEL_CFG = crate::Reg<func105_in_sel_cfg::FUNC105_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func105_in_sel_cfg;
#[doc = "FUNC106_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func106_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func106_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func106_in_sel_cfg`] module"]
pub type FUNC106_IN_SEL_CFG = crate::Reg<func106_in_sel_cfg::FUNC106_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func106_in_sel_cfg;
#[doc = "FUNC107_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func107_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func107_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func107_in_sel_cfg`] module"]
pub type FUNC107_IN_SEL_CFG = crate::Reg<func107_in_sel_cfg::FUNC107_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func107_in_sel_cfg;
#[doc = "FUNC108_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func108_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func108_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func108_in_sel_cfg`] module"]
pub type FUNC108_IN_SEL_CFG = crate::Reg<func108_in_sel_cfg::FUNC108_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func108_in_sel_cfg;
#[doc = "FUNC109_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func109_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func109_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func109_in_sel_cfg`] module"]
pub type FUNC109_IN_SEL_CFG = crate::Reg<func109_in_sel_cfg::FUNC109_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func109_in_sel_cfg;
#[doc = "FUNC110_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func110_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func110_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func110_in_sel_cfg`] module"]
pub type FUNC110_IN_SEL_CFG = crate::Reg<func110_in_sel_cfg::FUNC110_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func110_in_sel_cfg;
#[doc = "FUNC111_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func111_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func111_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func111_in_sel_cfg`] module"]
pub type FUNC111_IN_SEL_CFG = crate::Reg<func111_in_sel_cfg::FUNC111_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func111_in_sel_cfg;
#[doc = "FUNC112_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func112_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func112_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func112_in_sel_cfg`] module"]
pub type FUNC112_IN_SEL_CFG = crate::Reg<func112_in_sel_cfg::FUNC112_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func112_in_sel_cfg;
#[doc = "FUNC113_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func113_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func113_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func113_in_sel_cfg`] module"]
pub type FUNC113_IN_SEL_CFG = crate::Reg<func113_in_sel_cfg::FUNC113_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func113_in_sel_cfg;
#[doc = "FUNC114_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func114_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func114_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func114_in_sel_cfg`] module"]
pub type FUNC114_IN_SEL_CFG = crate::Reg<func114_in_sel_cfg::FUNC114_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func114_in_sel_cfg;
#[doc = "FUNC115_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func115_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func115_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func115_in_sel_cfg`] module"]
pub type FUNC115_IN_SEL_CFG = crate::Reg<func115_in_sel_cfg::FUNC115_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func115_in_sel_cfg;
#[doc = "FUNC116_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func116_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func116_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func116_in_sel_cfg`] module"]
pub type FUNC116_IN_SEL_CFG = crate::Reg<func116_in_sel_cfg::FUNC116_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func116_in_sel_cfg;
#[doc = "FUNC117_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func117_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func117_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func117_in_sel_cfg`] module"]
pub type FUNC117_IN_SEL_CFG = crate::Reg<func117_in_sel_cfg::FUNC117_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func117_in_sel_cfg;
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
#[doc = "FUNC122_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func122_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func122_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func122_in_sel_cfg`] module"]
pub type FUNC122_IN_SEL_CFG = crate::Reg<func122_in_sel_cfg::FUNC122_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func122_in_sel_cfg;
#[doc = "FUNC123_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func123_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func123_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func123_in_sel_cfg`] module"]
pub type FUNC123_IN_SEL_CFG = crate::Reg<func123_in_sel_cfg::FUNC123_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func123_in_sel_cfg;
#[doc = "FUNC124_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`func124_in_sel_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func124_in_sel_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func124_in_sel_cfg`] module"]
pub type FUNC124_IN_SEL_CFG = crate::Reg<func124_in_sel_cfg::FUNC124_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func124_in_sel_cfg;
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
