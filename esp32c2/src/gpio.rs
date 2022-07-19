#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO bit select register"]
    pub bt_select: crate::Reg<bt_select::BT_SELECT_SPEC>,
    #[doc = "0x04 - GPIO output register"]
    pub out: crate::Reg<out::OUT_SPEC>,
    #[doc = "0x08 - GPIO output set register"]
    pub out_w1ts: crate::Reg<out_w1ts::OUT_W1TS_SPEC>,
    #[doc = "0x0c - GPIO output clear register"]
    pub out_w1tc: crate::Reg<out_w1tc::OUT_W1TC_SPEC>,
    _reserved4: [u8; 0x0c],
    #[doc = "0x1c - GPIO sdio select register"]
    pub sdio_select: crate::Reg<sdio_select::SDIO_SELECT_SPEC>,
    #[doc = "0x20 - GPIO output enable register"]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    #[doc = "0x24 - GPIO output enable set register"]
    pub enable_w1ts: crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>,
    #[doc = "0x28 - GPIO output enable clear register"]
    pub enable_w1tc: crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>,
    _reserved8: [u8; 0x0c],
    #[doc = "0x38 - pad strapping register"]
    pub strap: crate::Reg<strap::STRAP_SPEC>,
    #[doc = "0x3c - GPIO input register"]
    pub in_: crate::Reg<in_::IN_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x44 - GPIO interrupt status register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x48 - GPIO interrupt status set register"]
    pub status_w1ts: crate::Reg<status_w1ts::STATUS_W1TS_SPEC>,
    #[doc = "0x4c - GPIO interrupt status clear register"]
    pub status_w1tc: crate::Reg<status_w1tc::STATUS_W1TC_SPEC>,
    _reserved13: [u8; 0x0c],
    #[doc = "0x5c - GPIO PRO_CPU interrupt status register"]
    pub pcpu_int: crate::Reg<pcpu_int::PCPU_INT_SPEC>,
    #[doc = "0x60 - GPIO PRO_CPU(not shielded) interrupt status register"]
    pub pcpu_nmi_int: crate::Reg<pcpu_nmi_int::PCPU_NMI_INT_SPEC>,
    #[doc = "0x64 - GPIO CPUSDIO interrupt status register"]
    pub cpusdio_int: crate::Reg<cpusdio_int::CPUSDIO_INT_SPEC>,
    _reserved16: [u8; 0x0c],
    #[doc = "0x74 - GPIO pin configuration register"]
    pub pin0: crate::Reg<pin0::PIN0_SPEC>,
    #[doc = "0x78 - GPIO pin configuration register"]
    pub pin1: crate::Reg<pin1::PIN1_SPEC>,
    #[doc = "0x7c - GPIO pin configuration register"]
    pub pin2: crate::Reg<pin2::PIN2_SPEC>,
    #[doc = "0x80 - GPIO pin configuration register"]
    pub pin3: crate::Reg<pin3::PIN3_SPEC>,
    #[doc = "0x84 - GPIO pin configuration register"]
    pub pin4: crate::Reg<pin4::PIN4_SPEC>,
    #[doc = "0x88 - GPIO pin configuration register"]
    pub pin5: crate::Reg<pin5::PIN5_SPEC>,
    #[doc = "0x8c - GPIO pin configuration register"]
    pub pin6: crate::Reg<pin6::PIN6_SPEC>,
    #[doc = "0x90 - GPIO pin configuration register"]
    pub pin7: crate::Reg<pin7::PIN7_SPEC>,
    #[doc = "0x94 - GPIO pin configuration register"]
    pub pin8: crate::Reg<pin8::PIN8_SPEC>,
    #[doc = "0x98 - GPIO pin configuration register"]
    pub pin9: crate::Reg<pin9::PIN9_SPEC>,
    #[doc = "0x9c - GPIO pin configuration register"]
    pub pin10: crate::Reg<pin10::PIN10_SPEC>,
    #[doc = "0xa0 - GPIO pin configuration register"]
    pub pin11: crate::Reg<pin11::PIN11_SPEC>,
    #[doc = "0xa4 - GPIO pin configuration register"]
    pub pin12: crate::Reg<pin12::PIN12_SPEC>,
    #[doc = "0xa8 - GPIO pin configuration register"]
    pub pin13: crate::Reg<pin13::PIN13_SPEC>,
    #[doc = "0xac - GPIO pin configuration register"]
    pub pin14: crate::Reg<pin14::PIN14_SPEC>,
    #[doc = "0xb0 - GPIO pin configuration register"]
    pub pin15: crate::Reg<pin15::PIN15_SPEC>,
    #[doc = "0xb4 - GPIO pin configuration register"]
    pub pin16: crate::Reg<pin16::PIN16_SPEC>,
    #[doc = "0xb8 - GPIO pin configuration register"]
    pub pin17: crate::Reg<pin17::PIN17_SPEC>,
    #[doc = "0xbc - GPIO pin configuration register"]
    pub pin18: crate::Reg<pin18::PIN18_SPEC>,
    #[doc = "0xc0 - GPIO pin configuration register"]
    pub pin19: crate::Reg<pin19::PIN19_SPEC>,
    #[doc = "0xc4 - GPIO pin configuration register"]
    pub pin20: crate::Reg<pin20::PIN20_SPEC>,
    #[doc = "0xc8 - GPIO pin configuration register"]
    pub pin21: crate::Reg<pin21::PIN21_SPEC>,
    #[doc = "0xcc - GPIO pin configuration register"]
    pub pin22: crate::Reg<pin22::PIN22_SPEC>,
    #[doc = "0xd0 - GPIO pin configuration register"]
    pub pin23: crate::Reg<pin23::PIN23_SPEC>,
    #[doc = "0xd4 - GPIO pin configuration register"]
    pub pin24: crate::Reg<pin24::PIN24_SPEC>,
    _reserved41: [u8; 0x74],
    #[doc = "0x14c - GPIO interrupt source register"]
    pub status_next: crate::Reg<status_next::STATUS_NEXT_SPEC>,
    _reserved42: [u8; 0x04],
    #[doc = "0x154 - GPIO input function configuration register"]
    pub func0_in_sel_cfg: crate::Reg<func0_in_sel_cfg::FUNC0_IN_SEL_CFG_SPEC>,
    #[doc = "0x158 - GPIO input function configuration register"]
    pub func1_in_sel_cfg: crate::Reg<func1_in_sel_cfg::FUNC1_IN_SEL_CFG_SPEC>,
    #[doc = "0x15c - GPIO input function configuration register"]
    pub func2_in_sel_cfg: crate::Reg<func2_in_sel_cfg::FUNC2_IN_SEL_CFG_SPEC>,
    #[doc = "0x160 - GPIO input function configuration register"]
    pub func3_in_sel_cfg: crate::Reg<func3_in_sel_cfg::FUNC3_IN_SEL_CFG_SPEC>,
    #[doc = "0x164 - GPIO input function configuration register"]
    pub func4_in_sel_cfg: crate::Reg<func4_in_sel_cfg::FUNC4_IN_SEL_CFG_SPEC>,
    #[doc = "0x168 - GPIO input function configuration register"]
    pub func5_in_sel_cfg: crate::Reg<func5_in_sel_cfg::FUNC5_IN_SEL_CFG_SPEC>,
    #[doc = "0x16c - GPIO input function configuration register"]
    pub func6_in_sel_cfg: crate::Reg<func6_in_sel_cfg::FUNC6_IN_SEL_CFG_SPEC>,
    #[doc = "0x170 - GPIO input function configuration register"]
    pub func7_in_sel_cfg: crate::Reg<func7_in_sel_cfg::FUNC7_IN_SEL_CFG_SPEC>,
    #[doc = "0x174 - GPIO input function configuration register"]
    pub func8_in_sel_cfg: crate::Reg<func8_in_sel_cfg::FUNC8_IN_SEL_CFG_SPEC>,
    #[doc = "0x178 - GPIO input function configuration register"]
    pub func9_in_sel_cfg: crate::Reg<func9_in_sel_cfg::FUNC9_IN_SEL_CFG_SPEC>,
    #[doc = "0x17c - GPIO input function configuration register"]
    pub func10_in_sel_cfg: crate::Reg<func10_in_sel_cfg::FUNC10_IN_SEL_CFG_SPEC>,
    #[doc = "0x180 - GPIO input function configuration register"]
    pub func11_in_sel_cfg: crate::Reg<func11_in_sel_cfg::FUNC11_IN_SEL_CFG_SPEC>,
    #[doc = "0x184 - GPIO input function configuration register"]
    pub func12_in_sel_cfg: crate::Reg<func12_in_sel_cfg::FUNC12_IN_SEL_CFG_SPEC>,
    #[doc = "0x188 - GPIO input function configuration register"]
    pub func13_in_sel_cfg: crate::Reg<func13_in_sel_cfg::FUNC13_IN_SEL_CFG_SPEC>,
    #[doc = "0x18c - GPIO input function configuration register"]
    pub func14_in_sel_cfg: crate::Reg<func14_in_sel_cfg::FUNC14_IN_SEL_CFG_SPEC>,
    #[doc = "0x190 - GPIO input function configuration register"]
    pub func15_in_sel_cfg: crate::Reg<func15_in_sel_cfg::FUNC15_IN_SEL_CFG_SPEC>,
    #[doc = "0x194 - GPIO input function configuration register"]
    pub func16_in_sel_cfg: crate::Reg<func16_in_sel_cfg::FUNC16_IN_SEL_CFG_SPEC>,
    #[doc = "0x198 - GPIO input function configuration register"]
    pub func17_in_sel_cfg: crate::Reg<func17_in_sel_cfg::FUNC17_IN_SEL_CFG_SPEC>,
    #[doc = "0x19c - GPIO input function configuration register"]
    pub func18_in_sel_cfg: crate::Reg<func18_in_sel_cfg::FUNC18_IN_SEL_CFG_SPEC>,
    #[doc = "0x1a0 - GPIO input function configuration register"]
    pub func19_in_sel_cfg: crate::Reg<func19_in_sel_cfg::FUNC19_IN_SEL_CFG_SPEC>,
    #[doc = "0x1a4 - GPIO input function configuration register"]
    pub func20_in_sel_cfg: crate::Reg<func20_in_sel_cfg::FUNC20_IN_SEL_CFG_SPEC>,
    #[doc = "0x1a8 - GPIO input function configuration register"]
    pub func21_in_sel_cfg: crate::Reg<func21_in_sel_cfg::FUNC21_IN_SEL_CFG_SPEC>,
    #[doc = "0x1ac - GPIO input function configuration register"]
    pub func22_in_sel_cfg: crate::Reg<func22_in_sel_cfg::FUNC22_IN_SEL_CFG_SPEC>,
    #[doc = "0x1b0 - GPIO input function configuration register"]
    pub func23_in_sel_cfg: crate::Reg<func23_in_sel_cfg::FUNC23_IN_SEL_CFG_SPEC>,
    #[doc = "0x1b4 - GPIO input function configuration register"]
    pub func24_in_sel_cfg: crate::Reg<func24_in_sel_cfg::FUNC24_IN_SEL_CFG_SPEC>,
    #[doc = "0x1b8 - GPIO input function configuration register"]
    pub func25_in_sel_cfg: crate::Reg<func25_in_sel_cfg::FUNC25_IN_SEL_CFG_SPEC>,
    #[doc = "0x1bc - GPIO input function configuration register"]
    pub func26_in_sel_cfg: crate::Reg<func26_in_sel_cfg::FUNC26_IN_SEL_CFG_SPEC>,
    #[doc = "0x1c0 - GPIO input function configuration register"]
    pub func27_in_sel_cfg: crate::Reg<func27_in_sel_cfg::FUNC27_IN_SEL_CFG_SPEC>,
    #[doc = "0x1c4 - GPIO input function configuration register"]
    pub func28_in_sel_cfg: crate::Reg<func28_in_sel_cfg::FUNC28_IN_SEL_CFG_SPEC>,
    #[doc = "0x1c8 - GPIO input function configuration register"]
    pub func29_in_sel_cfg: crate::Reg<func29_in_sel_cfg::FUNC29_IN_SEL_CFG_SPEC>,
    #[doc = "0x1cc - GPIO input function configuration register"]
    pub func30_in_sel_cfg: crate::Reg<func30_in_sel_cfg::FUNC30_IN_SEL_CFG_SPEC>,
    #[doc = "0x1d0 - GPIO input function configuration register"]
    pub func31_in_sel_cfg: crate::Reg<func31_in_sel_cfg::FUNC31_IN_SEL_CFG_SPEC>,
    #[doc = "0x1d4 - GPIO input function configuration register"]
    pub func32_in_sel_cfg: crate::Reg<func32_in_sel_cfg::FUNC32_IN_SEL_CFG_SPEC>,
    #[doc = "0x1d8 - GPIO input function configuration register"]
    pub func33_in_sel_cfg: crate::Reg<func33_in_sel_cfg::FUNC33_IN_SEL_CFG_SPEC>,
    #[doc = "0x1dc - GPIO input function configuration register"]
    pub func34_in_sel_cfg: crate::Reg<func34_in_sel_cfg::FUNC34_IN_SEL_CFG_SPEC>,
    #[doc = "0x1e0 - GPIO input function configuration register"]
    pub func35_in_sel_cfg: crate::Reg<func35_in_sel_cfg::FUNC35_IN_SEL_CFG_SPEC>,
    #[doc = "0x1e4 - GPIO input function configuration register"]
    pub func36_in_sel_cfg: crate::Reg<func36_in_sel_cfg::FUNC36_IN_SEL_CFG_SPEC>,
    #[doc = "0x1e8 - GPIO input function configuration register"]
    pub func37_in_sel_cfg: crate::Reg<func37_in_sel_cfg::FUNC37_IN_SEL_CFG_SPEC>,
    #[doc = "0x1ec - GPIO input function configuration register"]
    pub func38_in_sel_cfg: crate::Reg<func38_in_sel_cfg::FUNC38_IN_SEL_CFG_SPEC>,
    #[doc = "0x1f0 - GPIO input function configuration register"]
    pub func39_in_sel_cfg: crate::Reg<func39_in_sel_cfg::FUNC39_IN_SEL_CFG_SPEC>,
    #[doc = "0x1f4 - GPIO input function configuration register"]
    pub func40_in_sel_cfg: crate::Reg<func40_in_sel_cfg::FUNC40_IN_SEL_CFG_SPEC>,
    #[doc = "0x1f8 - GPIO input function configuration register"]
    pub func41_in_sel_cfg: crate::Reg<func41_in_sel_cfg::FUNC41_IN_SEL_CFG_SPEC>,
    #[doc = "0x1fc - GPIO input function configuration register"]
    pub func42_in_sel_cfg: crate::Reg<func42_in_sel_cfg::FUNC42_IN_SEL_CFG_SPEC>,
    #[doc = "0x200 - GPIO input function configuration register"]
    pub func43_in_sel_cfg: crate::Reg<func43_in_sel_cfg::FUNC43_IN_SEL_CFG_SPEC>,
    #[doc = "0x204 - GPIO input function configuration register"]
    pub func44_in_sel_cfg: crate::Reg<func44_in_sel_cfg::FUNC44_IN_SEL_CFG_SPEC>,
    #[doc = "0x208 - GPIO input function configuration register"]
    pub func45_in_sel_cfg: crate::Reg<func45_in_sel_cfg::FUNC45_IN_SEL_CFG_SPEC>,
    #[doc = "0x20c - GPIO input function configuration register"]
    pub func46_in_sel_cfg: crate::Reg<func46_in_sel_cfg::FUNC46_IN_SEL_CFG_SPEC>,
    #[doc = "0x210 - GPIO input function configuration register"]
    pub func47_in_sel_cfg: crate::Reg<func47_in_sel_cfg::FUNC47_IN_SEL_CFG_SPEC>,
    #[doc = "0x214 - GPIO input function configuration register"]
    pub func48_in_sel_cfg: crate::Reg<func48_in_sel_cfg::FUNC48_IN_SEL_CFG_SPEC>,
    #[doc = "0x218 - GPIO input function configuration register"]
    pub func49_in_sel_cfg: crate::Reg<func49_in_sel_cfg::FUNC49_IN_SEL_CFG_SPEC>,
    #[doc = "0x21c - GPIO input function configuration register"]
    pub func50_in_sel_cfg: crate::Reg<func50_in_sel_cfg::FUNC50_IN_SEL_CFG_SPEC>,
    #[doc = "0x220 - GPIO input function configuration register"]
    pub func51_in_sel_cfg: crate::Reg<func51_in_sel_cfg::FUNC51_IN_SEL_CFG_SPEC>,
    #[doc = "0x224 - GPIO input function configuration register"]
    pub func52_in_sel_cfg: crate::Reg<func52_in_sel_cfg::FUNC52_IN_SEL_CFG_SPEC>,
    #[doc = "0x228 - GPIO input function configuration register"]
    pub func53_in_sel_cfg: crate::Reg<func53_in_sel_cfg::FUNC53_IN_SEL_CFG_SPEC>,
    #[doc = "0x22c - GPIO input function configuration register"]
    pub func54_in_sel_cfg: crate::Reg<func54_in_sel_cfg::FUNC54_IN_SEL_CFG_SPEC>,
    #[doc = "0x230 - GPIO input function configuration register"]
    pub func55_in_sel_cfg: crate::Reg<func55_in_sel_cfg::FUNC55_IN_SEL_CFG_SPEC>,
    #[doc = "0x234 - GPIO input function configuration register"]
    pub func56_in_sel_cfg: crate::Reg<func56_in_sel_cfg::FUNC56_IN_SEL_CFG_SPEC>,
    #[doc = "0x238 - GPIO input function configuration register"]
    pub func57_in_sel_cfg: crate::Reg<func57_in_sel_cfg::FUNC57_IN_SEL_CFG_SPEC>,
    #[doc = "0x23c - GPIO input function configuration register"]
    pub func58_in_sel_cfg: crate::Reg<func58_in_sel_cfg::FUNC58_IN_SEL_CFG_SPEC>,
    #[doc = "0x240 - GPIO input function configuration register"]
    pub func59_in_sel_cfg: crate::Reg<func59_in_sel_cfg::FUNC59_IN_SEL_CFG_SPEC>,
    #[doc = "0x244 - GPIO input function configuration register"]
    pub func60_in_sel_cfg: crate::Reg<func60_in_sel_cfg::FUNC60_IN_SEL_CFG_SPEC>,
    #[doc = "0x248 - GPIO input function configuration register"]
    pub func61_in_sel_cfg: crate::Reg<func61_in_sel_cfg::FUNC61_IN_SEL_CFG_SPEC>,
    #[doc = "0x24c - GPIO input function configuration register"]
    pub func62_in_sel_cfg: crate::Reg<func62_in_sel_cfg::FUNC62_IN_SEL_CFG_SPEC>,
    #[doc = "0x250 - GPIO input function configuration register"]
    pub func63_in_sel_cfg: crate::Reg<func63_in_sel_cfg::FUNC63_IN_SEL_CFG_SPEC>,
    #[doc = "0x254 - GPIO input function configuration register"]
    pub func64_in_sel_cfg: crate::Reg<func64_in_sel_cfg::FUNC64_IN_SEL_CFG_SPEC>,
    #[doc = "0x258 - GPIO input function configuration register"]
    pub func65_in_sel_cfg: crate::Reg<func65_in_sel_cfg::FUNC65_IN_SEL_CFG_SPEC>,
    #[doc = "0x25c - GPIO input function configuration register"]
    pub func66_in_sel_cfg: crate::Reg<func66_in_sel_cfg::FUNC66_IN_SEL_CFG_SPEC>,
    #[doc = "0x260 - GPIO input function configuration register"]
    pub func67_in_sel_cfg: crate::Reg<func67_in_sel_cfg::FUNC67_IN_SEL_CFG_SPEC>,
    #[doc = "0x264 - GPIO input function configuration register"]
    pub func68_in_sel_cfg: crate::Reg<func68_in_sel_cfg::FUNC68_IN_SEL_CFG_SPEC>,
    #[doc = "0x268 - GPIO input function configuration register"]
    pub func69_in_sel_cfg: crate::Reg<func69_in_sel_cfg::FUNC69_IN_SEL_CFG_SPEC>,
    #[doc = "0x26c - GPIO input function configuration register"]
    pub func70_in_sel_cfg: crate::Reg<func70_in_sel_cfg::FUNC70_IN_SEL_CFG_SPEC>,
    #[doc = "0x270 - GPIO input function configuration register"]
    pub func71_in_sel_cfg: crate::Reg<func71_in_sel_cfg::FUNC71_IN_SEL_CFG_SPEC>,
    #[doc = "0x274 - GPIO input function configuration register"]
    pub func72_in_sel_cfg: crate::Reg<func72_in_sel_cfg::FUNC72_IN_SEL_CFG_SPEC>,
    #[doc = "0x278 - GPIO input function configuration register"]
    pub func73_in_sel_cfg: crate::Reg<func73_in_sel_cfg::FUNC73_IN_SEL_CFG_SPEC>,
    #[doc = "0x27c - GPIO input function configuration register"]
    pub func74_in_sel_cfg: crate::Reg<func74_in_sel_cfg::FUNC74_IN_SEL_CFG_SPEC>,
    #[doc = "0x280 - GPIO input function configuration register"]
    pub func75_in_sel_cfg: crate::Reg<func75_in_sel_cfg::FUNC75_IN_SEL_CFG_SPEC>,
    #[doc = "0x284 - GPIO input function configuration register"]
    pub func76_in_sel_cfg: crate::Reg<func76_in_sel_cfg::FUNC76_IN_SEL_CFG_SPEC>,
    #[doc = "0x288 - GPIO input function configuration register"]
    pub func77_in_sel_cfg: crate::Reg<func77_in_sel_cfg::FUNC77_IN_SEL_CFG_SPEC>,
    #[doc = "0x28c - GPIO input function configuration register"]
    pub func78_in_sel_cfg: crate::Reg<func78_in_sel_cfg::FUNC78_IN_SEL_CFG_SPEC>,
    #[doc = "0x290 - GPIO input function configuration register"]
    pub func79_in_sel_cfg: crate::Reg<func79_in_sel_cfg::FUNC79_IN_SEL_CFG_SPEC>,
    #[doc = "0x294 - GPIO input function configuration register"]
    pub func80_in_sel_cfg: crate::Reg<func80_in_sel_cfg::FUNC80_IN_SEL_CFG_SPEC>,
    #[doc = "0x298 - GPIO input function configuration register"]
    pub func81_in_sel_cfg: crate::Reg<func81_in_sel_cfg::FUNC81_IN_SEL_CFG_SPEC>,
    #[doc = "0x29c - GPIO input function configuration register"]
    pub func82_in_sel_cfg: crate::Reg<func82_in_sel_cfg::FUNC82_IN_SEL_CFG_SPEC>,
    #[doc = "0x2a0 - GPIO input function configuration register"]
    pub func83_in_sel_cfg: crate::Reg<func83_in_sel_cfg::FUNC83_IN_SEL_CFG_SPEC>,
    #[doc = "0x2a4 - GPIO input function configuration register"]
    pub func84_in_sel_cfg: crate::Reg<func84_in_sel_cfg::FUNC84_IN_SEL_CFG_SPEC>,
    #[doc = "0x2a8 - GPIO input function configuration register"]
    pub func85_in_sel_cfg: crate::Reg<func85_in_sel_cfg::FUNC85_IN_SEL_CFG_SPEC>,
    #[doc = "0x2ac - GPIO input function configuration register"]
    pub func86_in_sel_cfg: crate::Reg<func86_in_sel_cfg::FUNC86_IN_SEL_CFG_SPEC>,
    #[doc = "0x2b0 - GPIO input function configuration register"]
    pub func87_in_sel_cfg: crate::Reg<func87_in_sel_cfg::FUNC87_IN_SEL_CFG_SPEC>,
    #[doc = "0x2b4 - GPIO input function configuration register"]
    pub func88_in_sel_cfg: crate::Reg<func88_in_sel_cfg::FUNC88_IN_SEL_CFG_SPEC>,
    #[doc = "0x2b8 - GPIO input function configuration register"]
    pub func89_in_sel_cfg: crate::Reg<func89_in_sel_cfg::FUNC89_IN_SEL_CFG_SPEC>,
    #[doc = "0x2bc - GPIO input function configuration register"]
    pub func90_in_sel_cfg: crate::Reg<func90_in_sel_cfg::FUNC90_IN_SEL_CFG_SPEC>,
    #[doc = "0x2c0 - GPIO input function configuration register"]
    pub func91_in_sel_cfg: crate::Reg<func91_in_sel_cfg::FUNC91_IN_SEL_CFG_SPEC>,
    #[doc = "0x2c4 - GPIO input function configuration register"]
    pub func92_in_sel_cfg: crate::Reg<func92_in_sel_cfg::FUNC92_IN_SEL_CFG_SPEC>,
    #[doc = "0x2c8 - GPIO input function configuration register"]
    pub func93_in_sel_cfg: crate::Reg<func93_in_sel_cfg::FUNC93_IN_SEL_CFG_SPEC>,
    #[doc = "0x2cc - GPIO input function configuration register"]
    pub func94_in_sel_cfg: crate::Reg<func94_in_sel_cfg::FUNC94_IN_SEL_CFG_SPEC>,
    #[doc = "0x2d0 - GPIO input function configuration register"]
    pub func95_in_sel_cfg: crate::Reg<func95_in_sel_cfg::FUNC95_IN_SEL_CFG_SPEC>,
    #[doc = "0x2d4 - GPIO input function configuration register"]
    pub func96_in_sel_cfg: crate::Reg<func96_in_sel_cfg::FUNC96_IN_SEL_CFG_SPEC>,
    #[doc = "0x2d8 - GPIO input function configuration register"]
    pub func97_in_sel_cfg: crate::Reg<func97_in_sel_cfg::FUNC97_IN_SEL_CFG_SPEC>,
    #[doc = "0x2dc - GPIO input function configuration register"]
    pub func98_in_sel_cfg: crate::Reg<func98_in_sel_cfg::FUNC98_IN_SEL_CFG_SPEC>,
    #[doc = "0x2e0 - GPIO input function configuration register"]
    pub func99_in_sel_cfg: crate::Reg<func99_in_sel_cfg::FUNC99_IN_SEL_CFG_SPEC>,
    #[doc = "0x2e4 - GPIO input function configuration register"]
    pub func100_in_sel_cfg: crate::Reg<func100_in_sel_cfg::FUNC100_IN_SEL_CFG_SPEC>,
    #[doc = "0x2e8 - GPIO input function configuration register"]
    pub func101_in_sel_cfg: crate::Reg<func101_in_sel_cfg::FUNC101_IN_SEL_CFG_SPEC>,
    #[doc = "0x2ec - GPIO input function configuration register"]
    pub func102_in_sel_cfg: crate::Reg<func102_in_sel_cfg::FUNC102_IN_SEL_CFG_SPEC>,
    #[doc = "0x2f0 - GPIO input function configuration register"]
    pub func103_in_sel_cfg: crate::Reg<func103_in_sel_cfg::FUNC103_IN_SEL_CFG_SPEC>,
    #[doc = "0x2f4 - GPIO input function configuration register"]
    pub func104_in_sel_cfg: crate::Reg<func104_in_sel_cfg::FUNC104_IN_SEL_CFG_SPEC>,
    #[doc = "0x2f8 - GPIO input function configuration register"]
    pub func105_in_sel_cfg: crate::Reg<func105_in_sel_cfg::FUNC105_IN_SEL_CFG_SPEC>,
    #[doc = "0x2fc - GPIO input function configuration register"]
    pub func106_in_sel_cfg: crate::Reg<func106_in_sel_cfg::FUNC106_IN_SEL_CFG_SPEC>,
    #[doc = "0x300 - GPIO input function configuration register"]
    pub func107_in_sel_cfg: crate::Reg<func107_in_sel_cfg::FUNC107_IN_SEL_CFG_SPEC>,
    #[doc = "0x304 - GPIO input function configuration register"]
    pub func108_in_sel_cfg: crate::Reg<func108_in_sel_cfg::FUNC108_IN_SEL_CFG_SPEC>,
    #[doc = "0x308 - GPIO input function configuration register"]
    pub func109_in_sel_cfg: crate::Reg<func109_in_sel_cfg::FUNC109_IN_SEL_CFG_SPEC>,
    #[doc = "0x30c - GPIO input function configuration register"]
    pub func110_in_sel_cfg: crate::Reg<func110_in_sel_cfg::FUNC110_IN_SEL_CFG_SPEC>,
    #[doc = "0x310 - GPIO input function configuration register"]
    pub func111_in_sel_cfg: crate::Reg<func111_in_sel_cfg::FUNC111_IN_SEL_CFG_SPEC>,
    #[doc = "0x314 - GPIO input function configuration register"]
    pub func112_in_sel_cfg: crate::Reg<func112_in_sel_cfg::FUNC112_IN_SEL_CFG_SPEC>,
    #[doc = "0x318 - GPIO input function configuration register"]
    pub func113_in_sel_cfg: crate::Reg<func113_in_sel_cfg::FUNC113_IN_SEL_CFG_SPEC>,
    #[doc = "0x31c - GPIO input function configuration register"]
    pub func114_in_sel_cfg: crate::Reg<func114_in_sel_cfg::FUNC114_IN_SEL_CFG_SPEC>,
    #[doc = "0x320 - GPIO input function configuration register"]
    pub func115_in_sel_cfg: crate::Reg<func115_in_sel_cfg::FUNC115_IN_SEL_CFG_SPEC>,
    #[doc = "0x324 - GPIO input function configuration register"]
    pub func116_in_sel_cfg: crate::Reg<func116_in_sel_cfg::FUNC116_IN_SEL_CFG_SPEC>,
    #[doc = "0x328 - GPIO input function configuration register"]
    pub func117_in_sel_cfg: crate::Reg<func117_in_sel_cfg::FUNC117_IN_SEL_CFG_SPEC>,
    #[doc = "0x32c - GPIO input function configuration register"]
    pub func118_in_sel_cfg: crate::Reg<func118_in_sel_cfg::FUNC118_IN_SEL_CFG_SPEC>,
    #[doc = "0x330 - GPIO input function configuration register"]
    pub func119_in_sel_cfg: crate::Reg<func119_in_sel_cfg::FUNC119_IN_SEL_CFG_SPEC>,
    #[doc = "0x334 - GPIO input function configuration register"]
    pub func120_in_sel_cfg: crate::Reg<func120_in_sel_cfg::FUNC120_IN_SEL_CFG_SPEC>,
    #[doc = "0x338 - GPIO input function configuration register"]
    pub func121_in_sel_cfg: crate::Reg<func121_in_sel_cfg::FUNC121_IN_SEL_CFG_SPEC>,
    #[doc = "0x33c - GPIO input function configuration register"]
    pub func122_in_sel_cfg: crate::Reg<func122_in_sel_cfg::FUNC122_IN_SEL_CFG_SPEC>,
    #[doc = "0x340 - GPIO input function configuration register"]
    pub func123_in_sel_cfg: crate::Reg<func123_in_sel_cfg::FUNC123_IN_SEL_CFG_SPEC>,
    #[doc = "0x344 - GPIO input function configuration register"]
    pub func124_in_sel_cfg: crate::Reg<func124_in_sel_cfg::FUNC124_IN_SEL_CFG_SPEC>,
    #[doc = "0x348 - GPIO input function configuration register"]
    pub func125_in_sel_cfg: crate::Reg<func125_in_sel_cfg::FUNC125_IN_SEL_CFG_SPEC>,
    #[doc = "0x34c - GPIO input function configuration register"]
    pub func126_in_sel_cfg: crate::Reg<func126_in_sel_cfg::FUNC126_IN_SEL_CFG_SPEC>,
    #[doc = "0x350 - GPIO input function configuration register"]
    pub func127_in_sel_cfg: crate::Reg<func127_in_sel_cfg::FUNC127_IN_SEL_CFG_SPEC>,
    _reserved170: [u8; 0x0200],
    #[doc = "0x554 - GPIO output function select register"]
    pub func0_out_sel_cfg: crate::Reg<func0_out_sel_cfg::FUNC0_OUT_SEL_CFG_SPEC>,
    #[doc = "0x558 - GPIO output function select register"]
    pub func1_out_sel_cfg: crate::Reg<func1_out_sel_cfg::FUNC1_OUT_SEL_CFG_SPEC>,
    #[doc = "0x55c - GPIO output function select register"]
    pub func2_out_sel_cfg: crate::Reg<func2_out_sel_cfg::FUNC2_OUT_SEL_CFG_SPEC>,
    #[doc = "0x560 - GPIO output function select register"]
    pub func3_out_sel_cfg: crate::Reg<func3_out_sel_cfg::FUNC3_OUT_SEL_CFG_SPEC>,
    #[doc = "0x564 - GPIO output function select register"]
    pub func4_out_sel_cfg: crate::Reg<func4_out_sel_cfg::FUNC4_OUT_SEL_CFG_SPEC>,
    #[doc = "0x568 - GPIO output function select register"]
    pub func5_out_sel_cfg: crate::Reg<func5_out_sel_cfg::FUNC5_OUT_SEL_CFG_SPEC>,
    #[doc = "0x56c - GPIO output function select register"]
    pub func6_out_sel_cfg: crate::Reg<func6_out_sel_cfg::FUNC6_OUT_SEL_CFG_SPEC>,
    #[doc = "0x570 - GPIO output function select register"]
    pub func7_out_sel_cfg: crate::Reg<func7_out_sel_cfg::FUNC7_OUT_SEL_CFG_SPEC>,
    #[doc = "0x574 - GPIO output function select register"]
    pub func8_out_sel_cfg: crate::Reg<func8_out_sel_cfg::FUNC8_OUT_SEL_CFG_SPEC>,
    #[doc = "0x578 - GPIO output function select register"]
    pub func9_out_sel_cfg: crate::Reg<func9_out_sel_cfg::FUNC9_OUT_SEL_CFG_SPEC>,
    #[doc = "0x57c - GPIO output function select register"]
    pub func10_out_sel_cfg: crate::Reg<func10_out_sel_cfg::FUNC10_OUT_SEL_CFG_SPEC>,
    #[doc = "0x580 - GPIO output function select register"]
    pub func11_out_sel_cfg: crate::Reg<func11_out_sel_cfg::FUNC11_OUT_SEL_CFG_SPEC>,
    #[doc = "0x584 - GPIO output function select register"]
    pub func12_out_sel_cfg: crate::Reg<func12_out_sel_cfg::FUNC12_OUT_SEL_CFG_SPEC>,
    #[doc = "0x588 - GPIO output function select register"]
    pub func13_out_sel_cfg: crate::Reg<func13_out_sel_cfg::FUNC13_OUT_SEL_CFG_SPEC>,
    #[doc = "0x58c - GPIO output function select register"]
    pub func14_out_sel_cfg: crate::Reg<func14_out_sel_cfg::FUNC14_OUT_SEL_CFG_SPEC>,
    #[doc = "0x590 - GPIO output function select register"]
    pub func15_out_sel_cfg: crate::Reg<func15_out_sel_cfg::FUNC15_OUT_SEL_CFG_SPEC>,
    #[doc = "0x594 - GPIO output function select register"]
    pub func16_out_sel_cfg: crate::Reg<func16_out_sel_cfg::FUNC16_OUT_SEL_CFG_SPEC>,
    #[doc = "0x598 - GPIO output function select register"]
    pub func17_out_sel_cfg: crate::Reg<func17_out_sel_cfg::FUNC17_OUT_SEL_CFG_SPEC>,
    #[doc = "0x59c - GPIO output function select register"]
    pub func18_out_sel_cfg: crate::Reg<func18_out_sel_cfg::FUNC18_OUT_SEL_CFG_SPEC>,
    #[doc = "0x5a0 - GPIO output function select register"]
    pub func19_out_sel_cfg: crate::Reg<func19_out_sel_cfg::FUNC19_OUT_SEL_CFG_SPEC>,
    #[doc = "0x5a4 - GPIO output function select register"]
    pub func20_out_sel_cfg: crate::Reg<func20_out_sel_cfg::FUNC20_OUT_SEL_CFG_SPEC>,
    #[doc = "0x5a8 - GPIO output function select register"]
    pub func21_out_sel_cfg: crate::Reg<func21_out_sel_cfg::FUNC21_OUT_SEL_CFG_SPEC>,
    #[doc = "0x5ac - GPIO output function select register"]
    pub func22_out_sel_cfg: crate::Reg<func22_out_sel_cfg::FUNC22_OUT_SEL_CFG_SPEC>,
    #[doc = "0x5b0 - GPIO output function select register"]
    pub func23_out_sel_cfg: crate::Reg<func23_out_sel_cfg::FUNC23_OUT_SEL_CFG_SPEC>,
    #[doc = "0x5b4 - GPIO output function select register"]
    pub func24_out_sel_cfg: crate::Reg<func24_out_sel_cfg::FUNC24_OUT_SEL_CFG_SPEC>,
    _reserved195: [u8; 0x74],
    #[doc = "0x62c - GPIO clock gate register"]
    pub clock_gate_reg: crate::Reg<clock_gate_reg::CLOCK_GATE_REG_SPEC>,
    _reserved196: [u8; 0xcc],
    #[doc = "0x6fc - GPIO version register"]
    pub reg_date_reg: crate::Reg<reg_date_reg::REG_DATE_REG_SPEC>,
}
#[doc = "BT_SELECT register accessor: an alias for `Reg<BT_SELECT_SPEC>`"]
pub type BT_SELECT = crate::Reg<bt_select::BT_SELECT_SPEC>;
#[doc = "GPIO bit select register"]
pub mod bt_select;
#[doc = "OUT register accessor: an alias for `Reg<OUT_SPEC>`"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "GPIO output register"]
pub mod out;
#[doc = "OUT_W1TS register accessor: an alias for `Reg<OUT_W1TS_SPEC>`"]
pub type OUT_W1TS = crate::Reg<out_w1ts::OUT_W1TS_SPEC>;
#[doc = "GPIO output set register"]
pub mod out_w1ts;
#[doc = "OUT_W1TC register accessor: an alias for `Reg<OUT_W1TC_SPEC>`"]
pub type OUT_W1TC = crate::Reg<out_w1tc::OUT_W1TC_SPEC>;
#[doc = "GPIO output clear register"]
pub mod out_w1tc;
#[doc = "SDIO_SELECT register accessor: an alias for `Reg<SDIO_SELECT_SPEC>`"]
pub type SDIO_SELECT = crate::Reg<sdio_select::SDIO_SELECT_SPEC>;
#[doc = "GPIO sdio select register"]
pub mod sdio_select;
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "GPIO output enable register"]
pub mod enable;
#[doc = "ENABLE_W1TS register accessor: an alias for `Reg<ENABLE_W1TS_SPEC>`"]
pub type ENABLE_W1TS = crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>;
#[doc = "GPIO output enable set register"]
pub mod enable_w1ts;
#[doc = "ENABLE_W1TC register accessor: an alias for `Reg<ENABLE_W1TC_SPEC>`"]
pub type ENABLE_W1TC = crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>;
#[doc = "GPIO output enable clear register"]
pub mod enable_w1tc;
#[doc = "STRAP register accessor: an alias for `Reg<STRAP_SPEC>`"]
pub type STRAP = crate::Reg<strap::STRAP_SPEC>;
#[doc = "pad strapping register"]
pub mod strap;
#[doc = "IN register accessor: an alias for `Reg<IN_SPEC>`"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "GPIO input register"]
pub mod in_;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "GPIO interrupt status register"]
pub mod status;
#[doc = "STATUS_W1TS register accessor: an alias for `Reg<STATUS_W1TS_SPEC>`"]
pub type STATUS_W1TS = crate::Reg<status_w1ts::STATUS_W1TS_SPEC>;
#[doc = "GPIO interrupt status set register"]
pub mod status_w1ts;
#[doc = "STATUS_W1TC register accessor: an alias for `Reg<STATUS_W1TC_SPEC>`"]
pub type STATUS_W1TC = crate::Reg<status_w1tc::STATUS_W1TC_SPEC>;
#[doc = "GPIO interrupt status clear register"]
pub mod status_w1tc;
#[doc = "PCPU_INT register accessor: an alias for `Reg<PCPU_INT_SPEC>`"]
pub type PCPU_INT = crate::Reg<pcpu_int::PCPU_INT_SPEC>;
#[doc = "GPIO PRO_CPU interrupt status register"]
pub mod pcpu_int;
#[doc = "PCPU_NMI_INT register accessor: an alias for `Reg<PCPU_NMI_INT_SPEC>`"]
pub type PCPU_NMI_INT = crate::Reg<pcpu_nmi_int::PCPU_NMI_INT_SPEC>;
#[doc = "GPIO PRO_CPU(not shielded) interrupt status register"]
pub mod pcpu_nmi_int;
#[doc = "CPUSDIO_INT register accessor: an alias for `Reg<CPUSDIO_INT_SPEC>`"]
pub type CPUSDIO_INT = crate::Reg<cpusdio_int::CPUSDIO_INT_SPEC>;
#[doc = "GPIO CPUSDIO interrupt status register"]
pub mod cpusdio_int;
#[doc = "PIN0 register accessor: an alias for `Reg<PIN0_SPEC>`"]
pub type PIN0 = crate::Reg<pin0::PIN0_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin0;
#[doc = "PIN1 register accessor: an alias for `Reg<PIN1_SPEC>`"]
pub type PIN1 = crate::Reg<pin1::PIN1_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin1;
#[doc = "PIN2 register accessor: an alias for `Reg<PIN2_SPEC>`"]
pub type PIN2 = crate::Reg<pin2::PIN2_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin2;
#[doc = "PIN3 register accessor: an alias for `Reg<PIN3_SPEC>`"]
pub type PIN3 = crate::Reg<pin3::PIN3_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin3;
#[doc = "PIN4 register accessor: an alias for `Reg<PIN4_SPEC>`"]
pub type PIN4 = crate::Reg<pin4::PIN4_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin4;
#[doc = "PIN5 register accessor: an alias for `Reg<PIN5_SPEC>`"]
pub type PIN5 = crate::Reg<pin5::PIN5_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin5;
#[doc = "PIN6 register accessor: an alias for `Reg<PIN6_SPEC>`"]
pub type PIN6 = crate::Reg<pin6::PIN6_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin6;
#[doc = "PIN7 register accessor: an alias for `Reg<PIN7_SPEC>`"]
pub type PIN7 = crate::Reg<pin7::PIN7_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin7;
#[doc = "PIN8 register accessor: an alias for `Reg<PIN8_SPEC>`"]
pub type PIN8 = crate::Reg<pin8::PIN8_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin8;
#[doc = "PIN9 register accessor: an alias for `Reg<PIN9_SPEC>`"]
pub type PIN9 = crate::Reg<pin9::PIN9_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin9;
#[doc = "PIN10 register accessor: an alias for `Reg<PIN10_SPEC>`"]
pub type PIN10 = crate::Reg<pin10::PIN10_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin10;
#[doc = "PIN11 register accessor: an alias for `Reg<PIN11_SPEC>`"]
pub type PIN11 = crate::Reg<pin11::PIN11_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin11;
#[doc = "PIN12 register accessor: an alias for `Reg<PIN12_SPEC>`"]
pub type PIN12 = crate::Reg<pin12::PIN12_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin12;
#[doc = "PIN13 register accessor: an alias for `Reg<PIN13_SPEC>`"]
pub type PIN13 = crate::Reg<pin13::PIN13_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin13;
#[doc = "PIN14 register accessor: an alias for `Reg<PIN14_SPEC>`"]
pub type PIN14 = crate::Reg<pin14::PIN14_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin14;
#[doc = "PIN15 register accessor: an alias for `Reg<PIN15_SPEC>`"]
pub type PIN15 = crate::Reg<pin15::PIN15_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin15;
#[doc = "PIN16 register accessor: an alias for `Reg<PIN16_SPEC>`"]
pub type PIN16 = crate::Reg<pin16::PIN16_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin16;
#[doc = "PIN17 register accessor: an alias for `Reg<PIN17_SPEC>`"]
pub type PIN17 = crate::Reg<pin17::PIN17_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin17;
#[doc = "PIN18 register accessor: an alias for `Reg<PIN18_SPEC>`"]
pub type PIN18 = crate::Reg<pin18::PIN18_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin18;
#[doc = "PIN19 register accessor: an alias for `Reg<PIN19_SPEC>`"]
pub type PIN19 = crate::Reg<pin19::PIN19_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin19;
#[doc = "PIN20 register accessor: an alias for `Reg<PIN20_SPEC>`"]
pub type PIN20 = crate::Reg<pin20::PIN20_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin20;
#[doc = "PIN21 register accessor: an alias for `Reg<PIN21_SPEC>`"]
pub type PIN21 = crate::Reg<pin21::PIN21_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin21;
#[doc = "PIN22 register accessor: an alias for `Reg<PIN22_SPEC>`"]
pub type PIN22 = crate::Reg<pin22::PIN22_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin22;
#[doc = "PIN23 register accessor: an alias for `Reg<PIN23_SPEC>`"]
pub type PIN23 = crate::Reg<pin23::PIN23_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin23;
#[doc = "PIN24 register accessor: an alias for `Reg<PIN24_SPEC>`"]
pub type PIN24 = crate::Reg<pin24::PIN24_SPEC>;
#[doc = "GPIO pin configuration register"]
pub mod pin24;
#[doc = "STATUS_NEXT register accessor: an alias for `Reg<STATUS_NEXT_SPEC>`"]
pub type STATUS_NEXT = crate::Reg<status_next::STATUS_NEXT_SPEC>;
#[doc = "GPIO interrupt source register"]
pub mod status_next;
#[doc = "FUNC0_IN_SEL_CFG register accessor: an alias for `Reg<FUNC0_IN_SEL_CFG_SPEC>`"]
pub type FUNC0_IN_SEL_CFG = crate::Reg<func0_in_sel_cfg::FUNC0_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func0_in_sel_cfg;
#[doc = "FUNC1_IN_SEL_CFG register accessor: an alias for `Reg<FUNC1_IN_SEL_CFG_SPEC>`"]
pub type FUNC1_IN_SEL_CFG = crate::Reg<func1_in_sel_cfg::FUNC1_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func1_in_sel_cfg;
#[doc = "FUNC2_IN_SEL_CFG register accessor: an alias for `Reg<FUNC2_IN_SEL_CFG_SPEC>`"]
pub type FUNC2_IN_SEL_CFG = crate::Reg<func2_in_sel_cfg::FUNC2_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func2_in_sel_cfg;
#[doc = "FUNC3_IN_SEL_CFG register accessor: an alias for `Reg<FUNC3_IN_SEL_CFG_SPEC>`"]
pub type FUNC3_IN_SEL_CFG = crate::Reg<func3_in_sel_cfg::FUNC3_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func3_in_sel_cfg;
#[doc = "FUNC4_IN_SEL_CFG register accessor: an alias for `Reg<FUNC4_IN_SEL_CFG_SPEC>`"]
pub type FUNC4_IN_SEL_CFG = crate::Reg<func4_in_sel_cfg::FUNC4_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func4_in_sel_cfg;
#[doc = "FUNC5_IN_SEL_CFG register accessor: an alias for `Reg<FUNC5_IN_SEL_CFG_SPEC>`"]
pub type FUNC5_IN_SEL_CFG = crate::Reg<func5_in_sel_cfg::FUNC5_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func5_in_sel_cfg;
#[doc = "FUNC6_IN_SEL_CFG register accessor: an alias for `Reg<FUNC6_IN_SEL_CFG_SPEC>`"]
pub type FUNC6_IN_SEL_CFG = crate::Reg<func6_in_sel_cfg::FUNC6_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func6_in_sel_cfg;
#[doc = "FUNC7_IN_SEL_CFG register accessor: an alias for `Reg<FUNC7_IN_SEL_CFG_SPEC>`"]
pub type FUNC7_IN_SEL_CFG = crate::Reg<func7_in_sel_cfg::FUNC7_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func7_in_sel_cfg;
#[doc = "FUNC8_IN_SEL_CFG register accessor: an alias for `Reg<FUNC8_IN_SEL_CFG_SPEC>`"]
pub type FUNC8_IN_SEL_CFG = crate::Reg<func8_in_sel_cfg::FUNC8_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func8_in_sel_cfg;
#[doc = "FUNC9_IN_SEL_CFG register accessor: an alias for `Reg<FUNC9_IN_SEL_CFG_SPEC>`"]
pub type FUNC9_IN_SEL_CFG = crate::Reg<func9_in_sel_cfg::FUNC9_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func9_in_sel_cfg;
#[doc = "FUNC10_IN_SEL_CFG register accessor: an alias for `Reg<FUNC10_IN_SEL_CFG_SPEC>`"]
pub type FUNC10_IN_SEL_CFG = crate::Reg<func10_in_sel_cfg::FUNC10_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func10_in_sel_cfg;
#[doc = "FUNC11_IN_SEL_CFG register accessor: an alias for `Reg<FUNC11_IN_SEL_CFG_SPEC>`"]
pub type FUNC11_IN_SEL_CFG = crate::Reg<func11_in_sel_cfg::FUNC11_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func11_in_sel_cfg;
#[doc = "FUNC12_IN_SEL_CFG register accessor: an alias for `Reg<FUNC12_IN_SEL_CFG_SPEC>`"]
pub type FUNC12_IN_SEL_CFG = crate::Reg<func12_in_sel_cfg::FUNC12_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func12_in_sel_cfg;
#[doc = "FUNC13_IN_SEL_CFG register accessor: an alias for `Reg<FUNC13_IN_SEL_CFG_SPEC>`"]
pub type FUNC13_IN_SEL_CFG = crate::Reg<func13_in_sel_cfg::FUNC13_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func13_in_sel_cfg;
#[doc = "FUNC14_IN_SEL_CFG register accessor: an alias for `Reg<FUNC14_IN_SEL_CFG_SPEC>`"]
pub type FUNC14_IN_SEL_CFG = crate::Reg<func14_in_sel_cfg::FUNC14_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func14_in_sel_cfg;
#[doc = "FUNC15_IN_SEL_CFG register accessor: an alias for `Reg<FUNC15_IN_SEL_CFG_SPEC>`"]
pub type FUNC15_IN_SEL_CFG = crate::Reg<func15_in_sel_cfg::FUNC15_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func15_in_sel_cfg;
#[doc = "FUNC16_IN_SEL_CFG register accessor: an alias for `Reg<FUNC16_IN_SEL_CFG_SPEC>`"]
pub type FUNC16_IN_SEL_CFG = crate::Reg<func16_in_sel_cfg::FUNC16_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func16_in_sel_cfg;
#[doc = "FUNC17_IN_SEL_CFG register accessor: an alias for `Reg<FUNC17_IN_SEL_CFG_SPEC>`"]
pub type FUNC17_IN_SEL_CFG = crate::Reg<func17_in_sel_cfg::FUNC17_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func17_in_sel_cfg;
#[doc = "FUNC18_IN_SEL_CFG register accessor: an alias for `Reg<FUNC18_IN_SEL_CFG_SPEC>`"]
pub type FUNC18_IN_SEL_CFG = crate::Reg<func18_in_sel_cfg::FUNC18_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func18_in_sel_cfg;
#[doc = "FUNC19_IN_SEL_CFG register accessor: an alias for `Reg<FUNC19_IN_SEL_CFG_SPEC>`"]
pub type FUNC19_IN_SEL_CFG = crate::Reg<func19_in_sel_cfg::FUNC19_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func19_in_sel_cfg;
#[doc = "FUNC20_IN_SEL_CFG register accessor: an alias for `Reg<FUNC20_IN_SEL_CFG_SPEC>`"]
pub type FUNC20_IN_SEL_CFG = crate::Reg<func20_in_sel_cfg::FUNC20_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func20_in_sel_cfg;
#[doc = "FUNC21_IN_SEL_CFG register accessor: an alias for `Reg<FUNC21_IN_SEL_CFG_SPEC>`"]
pub type FUNC21_IN_SEL_CFG = crate::Reg<func21_in_sel_cfg::FUNC21_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func21_in_sel_cfg;
#[doc = "FUNC22_IN_SEL_CFG register accessor: an alias for `Reg<FUNC22_IN_SEL_CFG_SPEC>`"]
pub type FUNC22_IN_SEL_CFG = crate::Reg<func22_in_sel_cfg::FUNC22_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func22_in_sel_cfg;
#[doc = "FUNC23_IN_SEL_CFG register accessor: an alias for `Reg<FUNC23_IN_SEL_CFG_SPEC>`"]
pub type FUNC23_IN_SEL_CFG = crate::Reg<func23_in_sel_cfg::FUNC23_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func23_in_sel_cfg;
#[doc = "FUNC24_IN_SEL_CFG register accessor: an alias for `Reg<FUNC24_IN_SEL_CFG_SPEC>`"]
pub type FUNC24_IN_SEL_CFG = crate::Reg<func24_in_sel_cfg::FUNC24_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func24_in_sel_cfg;
#[doc = "FUNC25_IN_SEL_CFG register accessor: an alias for `Reg<FUNC25_IN_SEL_CFG_SPEC>`"]
pub type FUNC25_IN_SEL_CFG = crate::Reg<func25_in_sel_cfg::FUNC25_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func25_in_sel_cfg;
#[doc = "FUNC26_IN_SEL_CFG register accessor: an alias for `Reg<FUNC26_IN_SEL_CFG_SPEC>`"]
pub type FUNC26_IN_SEL_CFG = crate::Reg<func26_in_sel_cfg::FUNC26_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func26_in_sel_cfg;
#[doc = "FUNC27_IN_SEL_CFG register accessor: an alias for `Reg<FUNC27_IN_SEL_CFG_SPEC>`"]
pub type FUNC27_IN_SEL_CFG = crate::Reg<func27_in_sel_cfg::FUNC27_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func27_in_sel_cfg;
#[doc = "FUNC28_IN_SEL_CFG register accessor: an alias for `Reg<FUNC28_IN_SEL_CFG_SPEC>`"]
pub type FUNC28_IN_SEL_CFG = crate::Reg<func28_in_sel_cfg::FUNC28_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func28_in_sel_cfg;
#[doc = "FUNC29_IN_SEL_CFG register accessor: an alias for `Reg<FUNC29_IN_SEL_CFG_SPEC>`"]
pub type FUNC29_IN_SEL_CFG = crate::Reg<func29_in_sel_cfg::FUNC29_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func29_in_sel_cfg;
#[doc = "FUNC30_IN_SEL_CFG register accessor: an alias for `Reg<FUNC30_IN_SEL_CFG_SPEC>`"]
pub type FUNC30_IN_SEL_CFG = crate::Reg<func30_in_sel_cfg::FUNC30_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func30_in_sel_cfg;
#[doc = "FUNC31_IN_SEL_CFG register accessor: an alias for `Reg<FUNC31_IN_SEL_CFG_SPEC>`"]
pub type FUNC31_IN_SEL_CFG = crate::Reg<func31_in_sel_cfg::FUNC31_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func31_in_sel_cfg;
#[doc = "FUNC32_IN_SEL_CFG register accessor: an alias for `Reg<FUNC32_IN_SEL_CFG_SPEC>`"]
pub type FUNC32_IN_SEL_CFG = crate::Reg<func32_in_sel_cfg::FUNC32_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func32_in_sel_cfg;
#[doc = "FUNC33_IN_SEL_CFG register accessor: an alias for `Reg<FUNC33_IN_SEL_CFG_SPEC>`"]
pub type FUNC33_IN_SEL_CFG = crate::Reg<func33_in_sel_cfg::FUNC33_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func33_in_sel_cfg;
#[doc = "FUNC34_IN_SEL_CFG register accessor: an alias for `Reg<FUNC34_IN_SEL_CFG_SPEC>`"]
pub type FUNC34_IN_SEL_CFG = crate::Reg<func34_in_sel_cfg::FUNC34_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func34_in_sel_cfg;
#[doc = "FUNC35_IN_SEL_CFG register accessor: an alias for `Reg<FUNC35_IN_SEL_CFG_SPEC>`"]
pub type FUNC35_IN_SEL_CFG = crate::Reg<func35_in_sel_cfg::FUNC35_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func35_in_sel_cfg;
#[doc = "FUNC36_IN_SEL_CFG register accessor: an alias for `Reg<FUNC36_IN_SEL_CFG_SPEC>`"]
pub type FUNC36_IN_SEL_CFG = crate::Reg<func36_in_sel_cfg::FUNC36_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func36_in_sel_cfg;
#[doc = "FUNC37_IN_SEL_CFG register accessor: an alias for `Reg<FUNC37_IN_SEL_CFG_SPEC>`"]
pub type FUNC37_IN_SEL_CFG = crate::Reg<func37_in_sel_cfg::FUNC37_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func37_in_sel_cfg;
#[doc = "FUNC38_IN_SEL_CFG register accessor: an alias for `Reg<FUNC38_IN_SEL_CFG_SPEC>`"]
pub type FUNC38_IN_SEL_CFG = crate::Reg<func38_in_sel_cfg::FUNC38_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func38_in_sel_cfg;
#[doc = "FUNC39_IN_SEL_CFG register accessor: an alias for `Reg<FUNC39_IN_SEL_CFG_SPEC>`"]
pub type FUNC39_IN_SEL_CFG = crate::Reg<func39_in_sel_cfg::FUNC39_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func39_in_sel_cfg;
#[doc = "FUNC40_IN_SEL_CFG register accessor: an alias for `Reg<FUNC40_IN_SEL_CFG_SPEC>`"]
pub type FUNC40_IN_SEL_CFG = crate::Reg<func40_in_sel_cfg::FUNC40_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func40_in_sel_cfg;
#[doc = "FUNC41_IN_SEL_CFG register accessor: an alias for `Reg<FUNC41_IN_SEL_CFG_SPEC>`"]
pub type FUNC41_IN_SEL_CFG = crate::Reg<func41_in_sel_cfg::FUNC41_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func41_in_sel_cfg;
#[doc = "FUNC42_IN_SEL_CFG register accessor: an alias for `Reg<FUNC42_IN_SEL_CFG_SPEC>`"]
pub type FUNC42_IN_SEL_CFG = crate::Reg<func42_in_sel_cfg::FUNC42_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func42_in_sel_cfg;
#[doc = "FUNC43_IN_SEL_CFG register accessor: an alias for `Reg<FUNC43_IN_SEL_CFG_SPEC>`"]
pub type FUNC43_IN_SEL_CFG = crate::Reg<func43_in_sel_cfg::FUNC43_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func43_in_sel_cfg;
#[doc = "FUNC44_IN_SEL_CFG register accessor: an alias for `Reg<FUNC44_IN_SEL_CFG_SPEC>`"]
pub type FUNC44_IN_SEL_CFG = crate::Reg<func44_in_sel_cfg::FUNC44_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func44_in_sel_cfg;
#[doc = "FUNC45_IN_SEL_CFG register accessor: an alias for `Reg<FUNC45_IN_SEL_CFG_SPEC>`"]
pub type FUNC45_IN_SEL_CFG = crate::Reg<func45_in_sel_cfg::FUNC45_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func45_in_sel_cfg;
#[doc = "FUNC46_IN_SEL_CFG register accessor: an alias for `Reg<FUNC46_IN_SEL_CFG_SPEC>`"]
pub type FUNC46_IN_SEL_CFG = crate::Reg<func46_in_sel_cfg::FUNC46_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func46_in_sel_cfg;
#[doc = "FUNC47_IN_SEL_CFG register accessor: an alias for `Reg<FUNC47_IN_SEL_CFG_SPEC>`"]
pub type FUNC47_IN_SEL_CFG = crate::Reg<func47_in_sel_cfg::FUNC47_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func47_in_sel_cfg;
#[doc = "FUNC48_IN_SEL_CFG register accessor: an alias for `Reg<FUNC48_IN_SEL_CFG_SPEC>`"]
pub type FUNC48_IN_SEL_CFG = crate::Reg<func48_in_sel_cfg::FUNC48_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func48_in_sel_cfg;
#[doc = "FUNC49_IN_SEL_CFG register accessor: an alias for `Reg<FUNC49_IN_SEL_CFG_SPEC>`"]
pub type FUNC49_IN_SEL_CFG = crate::Reg<func49_in_sel_cfg::FUNC49_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func49_in_sel_cfg;
#[doc = "FUNC50_IN_SEL_CFG register accessor: an alias for `Reg<FUNC50_IN_SEL_CFG_SPEC>`"]
pub type FUNC50_IN_SEL_CFG = crate::Reg<func50_in_sel_cfg::FUNC50_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func50_in_sel_cfg;
#[doc = "FUNC51_IN_SEL_CFG register accessor: an alias for `Reg<FUNC51_IN_SEL_CFG_SPEC>`"]
pub type FUNC51_IN_SEL_CFG = crate::Reg<func51_in_sel_cfg::FUNC51_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func51_in_sel_cfg;
#[doc = "FUNC52_IN_SEL_CFG register accessor: an alias for `Reg<FUNC52_IN_SEL_CFG_SPEC>`"]
pub type FUNC52_IN_SEL_CFG = crate::Reg<func52_in_sel_cfg::FUNC52_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func52_in_sel_cfg;
#[doc = "FUNC53_IN_SEL_CFG register accessor: an alias for `Reg<FUNC53_IN_SEL_CFG_SPEC>`"]
pub type FUNC53_IN_SEL_CFG = crate::Reg<func53_in_sel_cfg::FUNC53_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func53_in_sel_cfg;
#[doc = "FUNC54_IN_SEL_CFG register accessor: an alias for `Reg<FUNC54_IN_SEL_CFG_SPEC>`"]
pub type FUNC54_IN_SEL_CFG = crate::Reg<func54_in_sel_cfg::FUNC54_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func54_in_sel_cfg;
#[doc = "FUNC55_IN_SEL_CFG register accessor: an alias for `Reg<FUNC55_IN_SEL_CFG_SPEC>`"]
pub type FUNC55_IN_SEL_CFG = crate::Reg<func55_in_sel_cfg::FUNC55_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func55_in_sel_cfg;
#[doc = "FUNC56_IN_SEL_CFG register accessor: an alias for `Reg<FUNC56_IN_SEL_CFG_SPEC>`"]
pub type FUNC56_IN_SEL_CFG = crate::Reg<func56_in_sel_cfg::FUNC56_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func56_in_sel_cfg;
#[doc = "FUNC57_IN_SEL_CFG register accessor: an alias for `Reg<FUNC57_IN_SEL_CFG_SPEC>`"]
pub type FUNC57_IN_SEL_CFG = crate::Reg<func57_in_sel_cfg::FUNC57_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func57_in_sel_cfg;
#[doc = "FUNC58_IN_SEL_CFG register accessor: an alias for `Reg<FUNC58_IN_SEL_CFG_SPEC>`"]
pub type FUNC58_IN_SEL_CFG = crate::Reg<func58_in_sel_cfg::FUNC58_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func58_in_sel_cfg;
#[doc = "FUNC59_IN_SEL_CFG register accessor: an alias for `Reg<FUNC59_IN_SEL_CFG_SPEC>`"]
pub type FUNC59_IN_SEL_CFG = crate::Reg<func59_in_sel_cfg::FUNC59_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func59_in_sel_cfg;
#[doc = "FUNC60_IN_SEL_CFG register accessor: an alias for `Reg<FUNC60_IN_SEL_CFG_SPEC>`"]
pub type FUNC60_IN_SEL_CFG = crate::Reg<func60_in_sel_cfg::FUNC60_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func60_in_sel_cfg;
#[doc = "FUNC61_IN_SEL_CFG register accessor: an alias for `Reg<FUNC61_IN_SEL_CFG_SPEC>`"]
pub type FUNC61_IN_SEL_CFG = crate::Reg<func61_in_sel_cfg::FUNC61_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func61_in_sel_cfg;
#[doc = "FUNC62_IN_SEL_CFG register accessor: an alias for `Reg<FUNC62_IN_SEL_CFG_SPEC>`"]
pub type FUNC62_IN_SEL_CFG = crate::Reg<func62_in_sel_cfg::FUNC62_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func62_in_sel_cfg;
#[doc = "FUNC63_IN_SEL_CFG register accessor: an alias for `Reg<FUNC63_IN_SEL_CFG_SPEC>`"]
pub type FUNC63_IN_SEL_CFG = crate::Reg<func63_in_sel_cfg::FUNC63_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func63_in_sel_cfg;
#[doc = "FUNC64_IN_SEL_CFG register accessor: an alias for `Reg<FUNC64_IN_SEL_CFG_SPEC>`"]
pub type FUNC64_IN_SEL_CFG = crate::Reg<func64_in_sel_cfg::FUNC64_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func64_in_sel_cfg;
#[doc = "FUNC65_IN_SEL_CFG register accessor: an alias for `Reg<FUNC65_IN_SEL_CFG_SPEC>`"]
pub type FUNC65_IN_SEL_CFG = crate::Reg<func65_in_sel_cfg::FUNC65_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func65_in_sel_cfg;
#[doc = "FUNC66_IN_SEL_CFG register accessor: an alias for `Reg<FUNC66_IN_SEL_CFG_SPEC>`"]
pub type FUNC66_IN_SEL_CFG = crate::Reg<func66_in_sel_cfg::FUNC66_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func66_in_sel_cfg;
#[doc = "FUNC67_IN_SEL_CFG register accessor: an alias for `Reg<FUNC67_IN_SEL_CFG_SPEC>`"]
pub type FUNC67_IN_SEL_CFG = crate::Reg<func67_in_sel_cfg::FUNC67_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func67_in_sel_cfg;
#[doc = "FUNC68_IN_SEL_CFG register accessor: an alias for `Reg<FUNC68_IN_SEL_CFG_SPEC>`"]
pub type FUNC68_IN_SEL_CFG = crate::Reg<func68_in_sel_cfg::FUNC68_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func68_in_sel_cfg;
#[doc = "FUNC69_IN_SEL_CFG register accessor: an alias for `Reg<FUNC69_IN_SEL_CFG_SPEC>`"]
pub type FUNC69_IN_SEL_CFG = crate::Reg<func69_in_sel_cfg::FUNC69_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func69_in_sel_cfg;
#[doc = "FUNC70_IN_SEL_CFG register accessor: an alias for `Reg<FUNC70_IN_SEL_CFG_SPEC>`"]
pub type FUNC70_IN_SEL_CFG = crate::Reg<func70_in_sel_cfg::FUNC70_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func70_in_sel_cfg;
#[doc = "FUNC71_IN_SEL_CFG register accessor: an alias for `Reg<FUNC71_IN_SEL_CFG_SPEC>`"]
pub type FUNC71_IN_SEL_CFG = crate::Reg<func71_in_sel_cfg::FUNC71_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func71_in_sel_cfg;
#[doc = "FUNC72_IN_SEL_CFG register accessor: an alias for `Reg<FUNC72_IN_SEL_CFG_SPEC>`"]
pub type FUNC72_IN_SEL_CFG = crate::Reg<func72_in_sel_cfg::FUNC72_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func72_in_sel_cfg;
#[doc = "FUNC73_IN_SEL_CFG register accessor: an alias for `Reg<FUNC73_IN_SEL_CFG_SPEC>`"]
pub type FUNC73_IN_SEL_CFG = crate::Reg<func73_in_sel_cfg::FUNC73_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func73_in_sel_cfg;
#[doc = "FUNC74_IN_SEL_CFG register accessor: an alias for `Reg<FUNC74_IN_SEL_CFG_SPEC>`"]
pub type FUNC74_IN_SEL_CFG = crate::Reg<func74_in_sel_cfg::FUNC74_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func74_in_sel_cfg;
#[doc = "FUNC75_IN_SEL_CFG register accessor: an alias for `Reg<FUNC75_IN_SEL_CFG_SPEC>`"]
pub type FUNC75_IN_SEL_CFG = crate::Reg<func75_in_sel_cfg::FUNC75_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func75_in_sel_cfg;
#[doc = "FUNC76_IN_SEL_CFG register accessor: an alias for `Reg<FUNC76_IN_SEL_CFG_SPEC>`"]
pub type FUNC76_IN_SEL_CFG = crate::Reg<func76_in_sel_cfg::FUNC76_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func76_in_sel_cfg;
#[doc = "FUNC77_IN_SEL_CFG register accessor: an alias for `Reg<FUNC77_IN_SEL_CFG_SPEC>`"]
pub type FUNC77_IN_SEL_CFG = crate::Reg<func77_in_sel_cfg::FUNC77_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func77_in_sel_cfg;
#[doc = "FUNC78_IN_SEL_CFG register accessor: an alias for `Reg<FUNC78_IN_SEL_CFG_SPEC>`"]
pub type FUNC78_IN_SEL_CFG = crate::Reg<func78_in_sel_cfg::FUNC78_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func78_in_sel_cfg;
#[doc = "FUNC79_IN_SEL_CFG register accessor: an alias for `Reg<FUNC79_IN_SEL_CFG_SPEC>`"]
pub type FUNC79_IN_SEL_CFG = crate::Reg<func79_in_sel_cfg::FUNC79_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func79_in_sel_cfg;
#[doc = "FUNC80_IN_SEL_CFG register accessor: an alias for `Reg<FUNC80_IN_SEL_CFG_SPEC>`"]
pub type FUNC80_IN_SEL_CFG = crate::Reg<func80_in_sel_cfg::FUNC80_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func80_in_sel_cfg;
#[doc = "FUNC81_IN_SEL_CFG register accessor: an alias for `Reg<FUNC81_IN_SEL_CFG_SPEC>`"]
pub type FUNC81_IN_SEL_CFG = crate::Reg<func81_in_sel_cfg::FUNC81_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func81_in_sel_cfg;
#[doc = "FUNC82_IN_SEL_CFG register accessor: an alias for `Reg<FUNC82_IN_SEL_CFG_SPEC>`"]
pub type FUNC82_IN_SEL_CFG = crate::Reg<func82_in_sel_cfg::FUNC82_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func82_in_sel_cfg;
#[doc = "FUNC83_IN_SEL_CFG register accessor: an alias for `Reg<FUNC83_IN_SEL_CFG_SPEC>`"]
pub type FUNC83_IN_SEL_CFG = crate::Reg<func83_in_sel_cfg::FUNC83_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func83_in_sel_cfg;
#[doc = "FUNC84_IN_SEL_CFG register accessor: an alias for `Reg<FUNC84_IN_SEL_CFG_SPEC>`"]
pub type FUNC84_IN_SEL_CFG = crate::Reg<func84_in_sel_cfg::FUNC84_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func84_in_sel_cfg;
#[doc = "FUNC85_IN_SEL_CFG register accessor: an alias for `Reg<FUNC85_IN_SEL_CFG_SPEC>`"]
pub type FUNC85_IN_SEL_CFG = crate::Reg<func85_in_sel_cfg::FUNC85_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func85_in_sel_cfg;
#[doc = "FUNC86_IN_SEL_CFG register accessor: an alias for `Reg<FUNC86_IN_SEL_CFG_SPEC>`"]
pub type FUNC86_IN_SEL_CFG = crate::Reg<func86_in_sel_cfg::FUNC86_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func86_in_sel_cfg;
#[doc = "FUNC87_IN_SEL_CFG register accessor: an alias for `Reg<FUNC87_IN_SEL_CFG_SPEC>`"]
pub type FUNC87_IN_SEL_CFG = crate::Reg<func87_in_sel_cfg::FUNC87_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func87_in_sel_cfg;
#[doc = "FUNC88_IN_SEL_CFG register accessor: an alias for `Reg<FUNC88_IN_SEL_CFG_SPEC>`"]
pub type FUNC88_IN_SEL_CFG = crate::Reg<func88_in_sel_cfg::FUNC88_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func88_in_sel_cfg;
#[doc = "FUNC89_IN_SEL_CFG register accessor: an alias for `Reg<FUNC89_IN_SEL_CFG_SPEC>`"]
pub type FUNC89_IN_SEL_CFG = crate::Reg<func89_in_sel_cfg::FUNC89_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func89_in_sel_cfg;
#[doc = "FUNC90_IN_SEL_CFG register accessor: an alias for `Reg<FUNC90_IN_SEL_CFG_SPEC>`"]
pub type FUNC90_IN_SEL_CFG = crate::Reg<func90_in_sel_cfg::FUNC90_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func90_in_sel_cfg;
#[doc = "FUNC91_IN_SEL_CFG register accessor: an alias for `Reg<FUNC91_IN_SEL_CFG_SPEC>`"]
pub type FUNC91_IN_SEL_CFG = crate::Reg<func91_in_sel_cfg::FUNC91_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func91_in_sel_cfg;
#[doc = "FUNC92_IN_SEL_CFG register accessor: an alias for `Reg<FUNC92_IN_SEL_CFG_SPEC>`"]
pub type FUNC92_IN_SEL_CFG = crate::Reg<func92_in_sel_cfg::FUNC92_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func92_in_sel_cfg;
#[doc = "FUNC93_IN_SEL_CFG register accessor: an alias for `Reg<FUNC93_IN_SEL_CFG_SPEC>`"]
pub type FUNC93_IN_SEL_CFG = crate::Reg<func93_in_sel_cfg::FUNC93_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func93_in_sel_cfg;
#[doc = "FUNC94_IN_SEL_CFG register accessor: an alias for `Reg<FUNC94_IN_SEL_CFG_SPEC>`"]
pub type FUNC94_IN_SEL_CFG = crate::Reg<func94_in_sel_cfg::FUNC94_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func94_in_sel_cfg;
#[doc = "FUNC95_IN_SEL_CFG register accessor: an alias for `Reg<FUNC95_IN_SEL_CFG_SPEC>`"]
pub type FUNC95_IN_SEL_CFG = crate::Reg<func95_in_sel_cfg::FUNC95_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func95_in_sel_cfg;
#[doc = "FUNC96_IN_SEL_CFG register accessor: an alias for `Reg<FUNC96_IN_SEL_CFG_SPEC>`"]
pub type FUNC96_IN_SEL_CFG = crate::Reg<func96_in_sel_cfg::FUNC96_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func96_in_sel_cfg;
#[doc = "FUNC97_IN_SEL_CFG register accessor: an alias for `Reg<FUNC97_IN_SEL_CFG_SPEC>`"]
pub type FUNC97_IN_SEL_CFG = crate::Reg<func97_in_sel_cfg::FUNC97_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func97_in_sel_cfg;
#[doc = "FUNC98_IN_SEL_CFG register accessor: an alias for `Reg<FUNC98_IN_SEL_CFG_SPEC>`"]
pub type FUNC98_IN_SEL_CFG = crate::Reg<func98_in_sel_cfg::FUNC98_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func98_in_sel_cfg;
#[doc = "FUNC99_IN_SEL_CFG register accessor: an alias for `Reg<FUNC99_IN_SEL_CFG_SPEC>`"]
pub type FUNC99_IN_SEL_CFG = crate::Reg<func99_in_sel_cfg::FUNC99_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func99_in_sel_cfg;
#[doc = "FUNC100_IN_SEL_CFG register accessor: an alias for `Reg<FUNC100_IN_SEL_CFG_SPEC>`"]
pub type FUNC100_IN_SEL_CFG = crate::Reg<func100_in_sel_cfg::FUNC100_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func100_in_sel_cfg;
#[doc = "FUNC101_IN_SEL_CFG register accessor: an alias for `Reg<FUNC101_IN_SEL_CFG_SPEC>`"]
pub type FUNC101_IN_SEL_CFG = crate::Reg<func101_in_sel_cfg::FUNC101_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func101_in_sel_cfg;
#[doc = "FUNC102_IN_SEL_CFG register accessor: an alias for `Reg<FUNC102_IN_SEL_CFG_SPEC>`"]
pub type FUNC102_IN_SEL_CFG = crate::Reg<func102_in_sel_cfg::FUNC102_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func102_in_sel_cfg;
#[doc = "FUNC103_IN_SEL_CFG register accessor: an alias for `Reg<FUNC103_IN_SEL_CFG_SPEC>`"]
pub type FUNC103_IN_SEL_CFG = crate::Reg<func103_in_sel_cfg::FUNC103_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func103_in_sel_cfg;
#[doc = "FUNC104_IN_SEL_CFG register accessor: an alias for `Reg<FUNC104_IN_SEL_CFG_SPEC>`"]
pub type FUNC104_IN_SEL_CFG = crate::Reg<func104_in_sel_cfg::FUNC104_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func104_in_sel_cfg;
#[doc = "FUNC105_IN_SEL_CFG register accessor: an alias for `Reg<FUNC105_IN_SEL_CFG_SPEC>`"]
pub type FUNC105_IN_SEL_CFG = crate::Reg<func105_in_sel_cfg::FUNC105_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func105_in_sel_cfg;
#[doc = "FUNC106_IN_SEL_CFG register accessor: an alias for `Reg<FUNC106_IN_SEL_CFG_SPEC>`"]
pub type FUNC106_IN_SEL_CFG = crate::Reg<func106_in_sel_cfg::FUNC106_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func106_in_sel_cfg;
#[doc = "FUNC107_IN_SEL_CFG register accessor: an alias for `Reg<FUNC107_IN_SEL_CFG_SPEC>`"]
pub type FUNC107_IN_SEL_CFG = crate::Reg<func107_in_sel_cfg::FUNC107_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func107_in_sel_cfg;
#[doc = "FUNC108_IN_SEL_CFG register accessor: an alias for `Reg<FUNC108_IN_SEL_CFG_SPEC>`"]
pub type FUNC108_IN_SEL_CFG = crate::Reg<func108_in_sel_cfg::FUNC108_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func108_in_sel_cfg;
#[doc = "FUNC109_IN_SEL_CFG register accessor: an alias for `Reg<FUNC109_IN_SEL_CFG_SPEC>`"]
pub type FUNC109_IN_SEL_CFG = crate::Reg<func109_in_sel_cfg::FUNC109_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func109_in_sel_cfg;
#[doc = "FUNC110_IN_SEL_CFG register accessor: an alias for `Reg<FUNC110_IN_SEL_CFG_SPEC>`"]
pub type FUNC110_IN_SEL_CFG = crate::Reg<func110_in_sel_cfg::FUNC110_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func110_in_sel_cfg;
#[doc = "FUNC111_IN_SEL_CFG register accessor: an alias for `Reg<FUNC111_IN_SEL_CFG_SPEC>`"]
pub type FUNC111_IN_SEL_CFG = crate::Reg<func111_in_sel_cfg::FUNC111_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func111_in_sel_cfg;
#[doc = "FUNC112_IN_SEL_CFG register accessor: an alias for `Reg<FUNC112_IN_SEL_CFG_SPEC>`"]
pub type FUNC112_IN_SEL_CFG = crate::Reg<func112_in_sel_cfg::FUNC112_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func112_in_sel_cfg;
#[doc = "FUNC113_IN_SEL_CFG register accessor: an alias for `Reg<FUNC113_IN_SEL_CFG_SPEC>`"]
pub type FUNC113_IN_SEL_CFG = crate::Reg<func113_in_sel_cfg::FUNC113_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func113_in_sel_cfg;
#[doc = "FUNC114_IN_SEL_CFG register accessor: an alias for `Reg<FUNC114_IN_SEL_CFG_SPEC>`"]
pub type FUNC114_IN_SEL_CFG = crate::Reg<func114_in_sel_cfg::FUNC114_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func114_in_sel_cfg;
#[doc = "FUNC115_IN_SEL_CFG register accessor: an alias for `Reg<FUNC115_IN_SEL_CFG_SPEC>`"]
pub type FUNC115_IN_SEL_CFG = crate::Reg<func115_in_sel_cfg::FUNC115_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func115_in_sel_cfg;
#[doc = "FUNC116_IN_SEL_CFG register accessor: an alias for `Reg<FUNC116_IN_SEL_CFG_SPEC>`"]
pub type FUNC116_IN_SEL_CFG = crate::Reg<func116_in_sel_cfg::FUNC116_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func116_in_sel_cfg;
#[doc = "FUNC117_IN_SEL_CFG register accessor: an alias for `Reg<FUNC117_IN_SEL_CFG_SPEC>`"]
pub type FUNC117_IN_SEL_CFG = crate::Reg<func117_in_sel_cfg::FUNC117_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func117_in_sel_cfg;
#[doc = "FUNC118_IN_SEL_CFG register accessor: an alias for `Reg<FUNC118_IN_SEL_CFG_SPEC>`"]
pub type FUNC118_IN_SEL_CFG = crate::Reg<func118_in_sel_cfg::FUNC118_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func118_in_sel_cfg;
#[doc = "FUNC119_IN_SEL_CFG register accessor: an alias for `Reg<FUNC119_IN_SEL_CFG_SPEC>`"]
pub type FUNC119_IN_SEL_CFG = crate::Reg<func119_in_sel_cfg::FUNC119_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func119_in_sel_cfg;
#[doc = "FUNC120_IN_SEL_CFG register accessor: an alias for `Reg<FUNC120_IN_SEL_CFG_SPEC>`"]
pub type FUNC120_IN_SEL_CFG = crate::Reg<func120_in_sel_cfg::FUNC120_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func120_in_sel_cfg;
#[doc = "FUNC121_IN_SEL_CFG register accessor: an alias for `Reg<FUNC121_IN_SEL_CFG_SPEC>`"]
pub type FUNC121_IN_SEL_CFG = crate::Reg<func121_in_sel_cfg::FUNC121_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func121_in_sel_cfg;
#[doc = "FUNC122_IN_SEL_CFG register accessor: an alias for `Reg<FUNC122_IN_SEL_CFG_SPEC>`"]
pub type FUNC122_IN_SEL_CFG = crate::Reg<func122_in_sel_cfg::FUNC122_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func122_in_sel_cfg;
#[doc = "FUNC123_IN_SEL_CFG register accessor: an alias for `Reg<FUNC123_IN_SEL_CFG_SPEC>`"]
pub type FUNC123_IN_SEL_CFG = crate::Reg<func123_in_sel_cfg::FUNC123_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func123_in_sel_cfg;
#[doc = "FUNC124_IN_SEL_CFG register accessor: an alias for `Reg<FUNC124_IN_SEL_CFG_SPEC>`"]
pub type FUNC124_IN_SEL_CFG = crate::Reg<func124_in_sel_cfg::FUNC124_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func124_in_sel_cfg;
#[doc = "FUNC125_IN_SEL_CFG register accessor: an alias for `Reg<FUNC125_IN_SEL_CFG_SPEC>`"]
pub type FUNC125_IN_SEL_CFG = crate::Reg<func125_in_sel_cfg::FUNC125_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func125_in_sel_cfg;
#[doc = "FUNC126_IN_SEL_CFG register accessor: an alias for `Reg<FUNC126_IN_SEL_CFG_SPEC>`"]
pub type FUNC126_IN_SEL_CFG = crate::Reg<func126_in_sel_cfg::FUNC126_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func126_in_sel_cfg;
#[doc = "FUNC127_IN_SEL_CFG register accessor: an alias for `Reg<FUNC127_IN_SEL_CFG_SPEC>`"]
pub type FUNC127_IN_SEL_CFG = crate::Reg<func127_in_sel_cfg::FUNC127_IN_SEL_CFG_SPEC>;
#[doc = "GPIO input function configuration register"]
pub mod func127_in_sel_cfg;
#[doc = "FUNC0_OUT_SEL_CFG register accessor: an alias for `Reg<FUNC0_OUT_SEL_CFG_SPEC>`"]
pub type FUNC0_OUT_SEL_CFG = crate::Reg<func0_out_sel_cfg::FUNC0_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func0_out_sel_cfg;
#[doc = "FUNC1_OUT_SEL_CFG register accessor: an alias for `Reg<FUNC1_OUT_SEL_CFG_SPEC>`"]
pub type FUNC1_OUT_SEL_CFG = crate::Reg<func1_out_sel_cfg::FUNC1_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func1_out_sel_cfg;
#[doc = "FUNC2_OUT_SEL_CFG register accessor: an alias for `Reg<FUNC2_OUT_SEL_CFG_SPEC>`"]
pub type FUNC2_OUT_SEL_CFG = crate::Reg<func2_out_sel_cfg::FUNC2_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func2_out_sel_cfg;
#[doc = "FUNC3_OUT_SEL_CFG register accessor: an alias for `Reg<FUNC3_OUT_SEL_CFG_SPEC>`"]
pub type FUNC3_OUT_SEL_CFG = crate::Reg<func3_out_sel_cfg::FUNC3_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func3_out_sel_cfg;
#[doc = "FUNC4_OUT_SEL_CFG register accessor: an alias for `Reg<FUNC4_OUT_SEL_CFG_SPEC>`"]
pub type FUNC4_OUT_SEL_CFG = crate::Reg<func4_out_sel_cfg::FUNC4_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func4_out_sel_cfg;
#[doc = "FUNC5_OUT_SEL_CFG register accessor: an alias for `Reg<FUNC5_OUT_SEL_CFG_SPEC>`"]
pub type FUNC5_OUT_SEL_CFG = crate::Reg<func5_out_sel_cfg::FUNC5_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func5_out_sel_cfg;
#[doc = "FUNC6_OUT_SEL_CFG register accessor: an alias for `Reg<FUNC6_OUT_SEL_CFG_SPEC>`"]
pub type FUNC6_OUT_SEL_CFG = crate::Reg<func6_out_sel_cfg::FUNC6_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func6_out_sel_cfg;
#[doc = "FUNC7_OUT_SEL_CFG register accessor: an alias for `Reg<FUNC7_OUT_SEL_CFG_SPEC>`"]
pub type FUNC7_OUT_SEL_CFG = crate::Reg<func7_out_sel_cfg::FUNC7_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func7_out_sel_cfg;
#[doc = "FUNC8_OUT_SEL_CFG register accessor: an alias for `Reg<FUNC8_OUT_SEL_CFG_SPEC>`"]
pub type FUNC8_OUT_SEL_CFG = crate::Reg<func8_out_sel_cfg::FUNC8_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func8_out_sel_cfg;
#[doc = "FUNC9_OUT_SEL_CFG register accessor: an alias for `Reg<FUNC9_OUT_SEL_CFG_SPEC>`"]
pub type FUNC9_OUT_SEL_CFG = crate::Reg<func9_out_sel_cfg::FUNC9_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func9_out_sel_cfg;
#[doc = "FUNC10_OUT_SEL_CFG register accessor: an alias for `Reg<FUNC10_OUT_SEL_CFG_SPEC>`"]
pub type FUNC10_OUT_SEL_CFG = crate::Reg<func10_out_sel_cfg::FUNC10_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func10_out_sel_cfg;
#[doc = "FUNC11_OUT_SEL_CFG register accessor: an alias for `Reg<FUNC11_OUT_SEL_CFG_SPEC>`"]
pub type FUNC11_OUT_SEL_CFG = crate::Reg<func11_out_sel_cfg::FUNC11_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func11_out_sel_cfg;
#[doc = "FUNC12_OUT_SEL_CFG register accessor: an alias for `Reg<FUNC12_OUT_SEL_CFG_SPEC>`"]
pub type FUNC12_OUT_SEL_CFG = crate::Reg<func12_out_sel_cfg::FUNC12_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func12_out_sel_cfg;
#[doc = "FUNC13_OUT_SEL_CFG register accessor: an alias for `Reg<FUNC13_OUT_SEL_CFG_SPEC>`"]
pub type FUNC13_OUT_SEL_CFG = crate::Reg<func13_out_sel_cfg::FUNC13_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func13_out_sel_cfg;
#[doc = "FUNC14_OUT_SEL_CFG register accessor: an alias for `Reg<FUNC14_OUT_SEL_CFG_SPEC>`"]
pub type FUNC14_OUT_SEL_CFG = crate::Reg<func14_out_sel_cfg::FUNC14_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func14_out_sel_cfg;
#[doc = "FUNC15_OUT_SEL_CFG register accessor: an alias for `Reg<FUNC15_OUT_SEL_CFG_SPEC>`"]
pub type FUNC15_OUT_SEL_CFG = crate::Reg<func15_out_sel_cfg::FUNC15_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func15_out_sel_cfg;
#[doc = "FUNC16_OUT_SEL_CFG register accessor: an alias for `Reg<FUNC16_OUT_SEL_CFG_SPEC>`"]
pub type FUNC16_OUT_SEL_CFG = crate::Reg<func16_out_sel_cfg::FUNC16_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func16_out_sel_cfg;
#[doc = "FUNC17_OUT_SEL_CFG register accessor: an alias for `Reg<FUNC17_OUT_SEL_CFG_SPEC>`"]
pub type FUNC17_OUT_SEL_CFG = crate::Reg<func17_out_sel_cfg::FUNC17_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func17_out_sel_cfg;
#[doc = "FUNC18_OUT_SEL_CFG register accessor: an alias for `Reg<FUNC18_OUT_SEL_CFG_SPEC>`"]
pub type FUNC18_OUT_SEL_CFG = crate::Reg<func18_out_sel_cfg::FUNC18_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func18_out_sel_cfg;
#[doc = "FUNC19_OUT_SEL_CFG register accessor: an alias for `Reg<FUNC19_OUT_SEL_CFG_SPEC>`"]
pub type FUNC19_OUT_SEL_CFG = crate::Reg<func19_out_sel_cfg::FUNC19_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func19_out_sel_cfg;
#[doc = "FUNC20_OUT_SEL_CFG register accessor: an alias for `Reg<FUNC20_OUT_SEL_CFG_SPEC>`"]
pub type FUNC20_OUT_SEL_CFG = crate::Reg<func20_out_sel_cfg::FUNC20_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func20_out_sel_cfg;
#[doc = "FUNC21_OUT_SEL_CFG register accessor: an alias for `Reg<FUNC21_OUT_SEL_CFG_SPEC>`"]
pub type FUNC21_OUT_SEL_CFG = crate::Reg<func21_out_sel_cfg::FUNC21_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func21_out_sel_cfg;
#[doc = "FUNC22_OUT_SEL_CFG register accessor: an alias for `Reg<FUNC22_OUT_SEL_CFG_SPEC>`"]
pub type FUNC22_OUT_SEL_CFG = crate::Reg<func22_out_sel_cfg::FUNC22_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func22_out_sel_cfg;
#[doc = "FUNC23_OUT_SEL_CFG register accessor: an alias for `Reg<FUNC23_OUT_SEL_CFG_SPEC>`"]
pub type FUNC23_OUT_SEL_CFG = crate::Reg<func23_out_sel_cfg::FUNC23_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func23_out_sel_cfg;
#[doc = "FUNC24_OUT_SEL_CFG register accessor: an alias for `Reg<FUNC24_OUT_SEL_CFG_SPEC>`"]
pub type FUNC24_OUT_SEL_CFG = crate::Reg<func24_out_sel_cfg::FUNC24_OUT_SEL_CFG_SPEC>;
#[doc = "GPIO output function select register"]
pub mod func24_out_sel_cfg;
#[doc = "CLOCK_GATE_REG register accessor: an alias for `Reg<CLOCK_GATE_REG_SPEC>`"]
pub type CLOCK_GATE_REG = crate::Reg<clock_gate_reg::CLOCK_GATE_REG_SPEC>;
#[doc = "GPIO clock gate register"]
pub mod clock_gate_reg;
#[doc = "REG_DATE_REG register accessor: an alias for `Reg<REG_DATE_REG_SPEC>`"]
pub type REG_DATE_REG = crate::Reg<reg_date_reg::REG_DATE_REG_SPEC>;
#[doc = "GPIO version register"]
pub mod reg_date_reg;
