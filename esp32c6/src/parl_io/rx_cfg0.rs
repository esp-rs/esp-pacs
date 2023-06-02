#[doc = "Register `RX_CFG0` reader"]
pub struct R(crate::R<RX_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_CFG0` writer"]
pub struct W(crate::W<RX_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RX_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_EOF_GEN_SEL` reader - Write 0 to select eof generated manchnism by configured data byte length. Write 1 to select eof generated manchnism by external enable signal."]
pub type RX_EOF_GEN_SEL_R = crate::BitReader;
#[doc = "Field `RX_EOF_GEN_SEL` writer - Write 0 to select eof generated manchnism by configured data byte length. Write 1 to select eof generated manchnism by external enable signal."]
pub type RX_EOF_GEN_SEL_W<'a, const O: u8> = crate::BitWriter<'a, RX_CFG0_SPEC, O>;
#[doc = "Field `RX_START` reader - Write 1 to start rx global data sampling."]
pub type RX_START_R = crate::BitReader;
#[doc = "Field `RX_START` writer - Write 1 to start rx global data sampling."]
pub type RX_START_W<'a, const O: u8> = crate::BitWriter<'a, RX_CFG0_SPEC, O>;
#[doc = "Field `RX_DATA_BYTELEN` reader - Configures rx receieved data byte length."]
pub type RX_DATA_BYTELEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RX_DATA_BYTELEN` writer - Configures rx receieved data byte length."]
pub type RX_DATA_BYTELEN_W<'a, const O: u8> = crate::FieldWriter<'a, RX_CFG0_SPEC, 16, O, u16, u16>;
#[doc = "Field `RX_SW_EN` reader - Write 1 to enable software data sampling."]
pub type RX_SW_EN_R = crate::BitReader;
#[doc = "Field `RX_SW_EN` writer - Write 1 to enable software data sampling."]
pub type RX_SW_EN_W<'a, const O: u8> = crate::BitWriter<'a, RX_CFG0_SPEC, O>;
#[doc = "Field `RX_PULSE_SUBMODE_SEL` reader - Pulse submode selection. 0000: positive pulse start(data bit included) &amp;&amp; positive pulse end(data bit included) 0001: positive pulse start(data bit included) &amp;&amp; positive pulse end (data bit excluded) 0010: positive pulse start(data bit excluded) &amp;&amp; positive pulse end (data bit included) 0011: positive pulse start(data bit excluded) &amp;&amp; positive pulse end (data bit excluded) 0100: positive pulse start(data bit included) &amp;&amp; length end 0101: positive pulse start(data bit excluded) &amp;&amp; length end 0110: negative pulse start(data bit included) &amp;&amp; negative pulse end(data bit included) 0111: negative pulse start(data bit included) &amp;&amp; negative pulse end (data bit excluded) 1000: negative pulse start(data bit excluded) &amp;&amp; negative pulse end (data bit included) 1001: negative pulse start(data bit excluded) &amp;&amp; negative pulse end (data bit excluded) 1010: negative pulse start(data bit included) &amp;&amp; length end 1011: negative pulse start(data bit excluded) &amp;&amp; length end"]
pub type RX_PULSE_SUBMODE_SEL_R = crate::FieldReader;
#[doc = "Field `RX_PULSE_SUBMODE_SEL` writer - Pulse submode selection. 0000: positive pulse start(data bit included) &amp;&amp; positive pulse end(data bit included) 0001: positive pulse start(data bit included) &amp;&amp; positive pulse end (data bit excluded) 0010: positive pulse start(data bit excluded) &amp;&amp; positive pulse end (data bit included) 0011: positive pulse start(data bit excluded) &amp;&amp; positive pulse end (data bit excluded) 0100: positive pulse start(data bit included) &amp;&amp; length end 0101: positive pulse start(data bit excluded) &amp;&amp; length end 0110: negative pulse start(data bit included) &amp;&amp; negative pulse end(data bit included) 0111: negative pulse start(data bit included) &amp;&amp; negative pulse end (data bit excluded) 1000: negative pulse start(data bit excluded) &amp;&amp; negative pulse end (data bit included) 1001: negative pulse start(data bit excluded) &amp;&amp; negative pulse end (data bit excluded) 1010: negative pulse start(data bit included) &amp;&amp; length end 1011: negative pulse start(data bit excluded) &amp;&amp; length end"]
pub type RX_PULSE_SUBMODE_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, RX_CFG0_SPEC, 4, O>;
#[doc = "Field `RX_LEVEL_SUBMODE_SEL` reader - Write 0 to sample data at high level of external enable signal. Write 1 to sample data at low level of external enable signal."]
pub type RX_LEVEL_SUBMODE_SEL_R = crate::BitReader;
#[doc = "Field `RX_LEVEL_SUBMODE_SEL` writer - Write 0 to sample data at high level of external enable signal. Write 1 to sample data at low level of external enable signal."]
pub type RX_LEVEL_SUBMODE_SEL_W<'a, const O: u8> = crate::BitWriter<'a, RX_CFG0_SPEC, O>;
#[doc = "Field `RX_SMP_MODE_SEL` reader - Rx data sampling mode selection. 000: external level enable mode 001: external pulse enable mode 010: internal software enable mode"]
pub type RX_SMP_MODE_SEL_R = crate::FieldReader;
#[doc = "Field `RX_SMP_MODE_SEL` writer - Rx data sampling mode selection. 000: external level enable mode 001: external pulse enable mode 010: internal software enable mode"]
pub type RX_SMP_MODE_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, RX_CFG0_SPEC, 2, O>;
#[doc = "Field `RX_CLK_EDGE_SEL` reader - Write 0 to enable sampling data on the rising edge of rx clock. Write 0 to enable sampling data on the falling edge of rx clock."]
pub type RX_CLK_EDGE_SEL_R = crate::BitReader;
#[doc = "Field `RX_CLK_EDGE_SEL` writer - Write 0 to enable sampling data on the rising edge of rx clock. Write 0 to enable sampling data on the falling edge of rx clock."]
pub type RX_CLK_EDGE_SEL_W<'a, const O: u8> = crate::BitWriter<'a, RX_CFG0_SPEC, O>;
#[doc = "Field `RX_BIT_PACK_ORDER` reader - Write 0 to pack bits into 1byte from MSB when data bus width is 4/2/1 bits. Write 0 to pack bits into 1byte from LSB when data bus width is 4/2/1 bits."]
pub type RX_BIT_PACK_ORDER_R = crate::BitReader;
#[doc = "Field `RX_BIT_PACK_ORDER` writer - Write 0 to pack bits into 1byte from MSB when data bus width is 4/2/1 bits. Write 0 to pack bits into 1byte from LSB when data bus width is 4/2/1 bits."]
pub type RX_BIT_PACK_ORDER_W<'a, const O: u8> = crate::BitWriter<'a, RX_CFG0_SPEC, O>;
#[doc = "Field `RX_BUS_WID_SEL` reader - Rx data bus width selection. 100: bus width is 1 bit 011: bus width is 2 bits 010: bus width is 4 bits 001: bus width is 8 bits 000: bus width is 16 bits"]
pub type RX_BUS_WID_SEL_R = crate::FieldReader;
#[doc = "Field `RX_BUS_WID_SEL` writer - Rx data bus width selection. 100: bus width is 1 bit 011: bus width is 2 bits 010: bus width is 4 bits 001: bus width is 8 bits 000: bus width is 16 bits"]
pub type RX_BUS_WID_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, RX_CFG0_SPEC, 3, O>;
#[doc = "Field `RX_FIFO_SRST` reader - Write 1 to enable soft reset of async fifo in rx module."]
pub type RX_FIFO_SRST_R = crate::BitReader;
#[doc = "Field `RX_FIFO_SRST` writer - Write 1 to enable soft reset of async fifo in rx module."]
pub type RX_FIFO_SRST_W<'a, const O: u8> = crate::BitWriter<'a, RX_CFG0_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Write 0 to select eof generated manchnism by configured data byte length. Write 1 to select eof generated manchnism by external enable signal."]
    #[inline(always)]
    pub fn rx_eof_gen_sel(&self) -> RX_EOF_GEN_SEL_R {
        RX_EOF_GEN_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to start rx global data sampling."]
    #[inline(always)]
    pub fn rx_start(&self) -> RX_START_R {
        RX_START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:17 - Configures rx receieved data byte length."]
    #[inline(always)]
    pub fn rx_data_bytelen(&self) -> RX_DATA_BYTELEN_R {
        RX_DATA_BYTELEN_R::new(((self.bits >> 2) & 0xffff) as u16)
    }
    #[doc = "Bit 18 - Write 1 to enable software data sampling."]
    #[inline(always)]
    pub fn rx_sw_en(&self) -> RX_SW_EN_R {
        RX_SW_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - Pulse submode selection. 0000: positive pulse start(data bit included) &amp;&amp; positive pulse end(data bit included) 0001: positive pulse start(data bit included) &amp;&amp; positive pulse end (data bit excluded) 0010: positive pulse start(data bit excluded) &amp;&amp; positive pulse end (data bit included) 0011: positive pulse start(data bit excluded) &amp;&amp; positive pulse end (data bit excluded) 0100: positive pulse start(data bit included) &amp;&amp; length end 0101: positive pulse start(data bit excluded) &amp;&amp; length end 0110: negative pulse start(data bit included) &amp;&amp; negative pulse end(data bit included) 0111: negative pulse start(data bit included) &amp;&amp; negative pulse end (data bit excluded) 1000: negative pulse start(data bit excluded) &amp;&amp; negative pulse end (data bit included) 1001: negative pulse start(data bit excluded) &amp;&amp; negative pulse end (data bit excluded) 1010: negative pulse start(data bit included) &amp;&amp; length end 1011: negative pulse start(data bit excluded) &amp;&amp; length end"]
    #[inline(always)]
    pub fn rx_pulse_submode_sel(&self) -> RX_PULSE_SUBMODE_SEL_R {
        RX_PULSE_SUBMODE_SEL_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - Write 0 to sample data at high level of external enable signal. Write 1 to sample data at low level of external enable signal."]
    #[inline(always)]
    pub fn rx_level_submode_sel(&self) -> RX_LEVEL_SUBMODE_SEL_R {
        RX_LEVEL_SUBMODE_SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Rx data sampling mode selection. 000: external level enable mode 001: external pulse enable mode 010: internal software enable mode"]
    #[inline(always)]
    pub fn rx_smp_mode_sel(&self) -> RX_SMP_MODE_SEL_R {
        RX_SMP_MODE_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Write 0 to enable sampling data on the rising edge of rx clock. Write 0 to enable sampling data on the falling edge of rx clock."]
    #[inline(always)]
    pub fn rx_clk_edge_sel(&self) -> RX_CLK_EDGE_SEL_R {
        RX_CLK_EDGE_SEL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Write 0 to pack bits into 1byte from MSB when data bus width is 4/2/1 bits. Write 0 to pack bits into 1byte from LSB when data bus width is 4/2/1 bits."]
    #[inline(always)]
    pub fn rx_bit_pack_order(&self) -> RX_BIT_PACK_ORDER_R {
        RX_BIT_PACK_ORDER_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Rx data bus width selection. 100: bus width is 1 bit 011: bus width is 2 bits 010: bus width is 4 bits 001: bus width is 8 bits 000: bus width is 16 bits"]
    #[inline(always)]
    pub fn rx_bus_wid_sel(&self) -> RX_BUS_WID_SEL_R {
        RX_BUS_WID_SEL_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Write 1 to enable soft reset of async fifo in rx module."]
    #[inline(always)]
    pub fn rx_fifo_srst(&self) -> RX_FIFO_SRST_R {
        RX_FIFO_SRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CFG0")
            .field(
                "rx_eof_gen_sel",
                &format_args!("{}", self.rx_eof_gen_sel().bit()),
            )
            .field("rx_start", &format_args!("{}", self.rx_start().bit()))
            .field(
                "rx_data_bytelen",
                &format_args!("{}", self.rx_data_bytelen().bits()),
            )
            .field("rx_sw_en", &format_args!("{}", self.rx_sw_en().bit()))
            .field(
                "rx_pulse_submode_sel",
                &format_args!("{}", self.rx_pulse_submode_sel().bits()),
            )
            .field(
                "rx_level_submode_sel",
                &format_args!("{}", self.rx_level_submode_sel().bit()),
            )
            .field(
                "rx_smp_mode_sel",
                &format_args!("{}", self.rx_smp_mode_sel().bits()),
            )
            .field(
                "rx_clk_edge_sel",
                &format_args!("{}", self.rx_clk_edge_sel().bit()),
            )
            .field(
                "rx_bit_pack_order",
                &format_args!("{}", self.rx_bit_pack_order().bit()),
            )
            .field(
                "rx_bus_wid_sel",
                &format_args!("{}", self.rx_bus_wid_sel().bits()),
            )
            .field(
                "rx_fifo_srst",
                &format_args!("{}", self.rx_fifo_srst().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_CFG0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Write 0 to select eof generated manchnism by configured data byte length. Write 1 to select eof generated manchnism by external enable signal."]
    #[inline(always)]
    #[must_use]
    pub fn rx_eof_gen_sel(&mut self) -> RX_EOF_GEN_SEL_W<0> {
        RX_EOF_GEN_SEL_W::new(self)
    }
    #[doc = "Bit 1 - Write 1 to start rx global data sampling."]
    #[inline(always)]
    #[must_use]
    pub fn rx_start(&mut self) -> RX_START_W<1> {
        RX_START_W::new(self)
    }
    #[doc = "Bits 2:17 - Configures rx receieved data byte length."]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_bytelen(&mut self) -> RX_DATA_BYTELEN_W<2> {
        RX_DATA_BYTELEN_W::new(self)
    }
    #[doc = "Bit 18 - Write 1 to enable software data sampling."]
    #[inline(always)]
    #[must_use]
    pub fn rx_sw_en(&mut self) -> RX_SW_EN_W<18> {
        RX_SW_EN_W::new(self)
    }
    #[doc = "Bits 19:22 - Pulse submode selection. 0000: positive pulse start(data bit included) &amp;&amp; positive pulse end(data bit included) 0001: positive pulse start(data bit included) &amp;&amp; positive pulse end (data bit excluded) 0010: positive pulse start(data bit excluded) &amp;&amp; positive pulse end (data bit included) 0011: positive pulse start(data bit excluded) &amp;&amp; positive pulse end (data bit excluded) 0100: positive pulse start(data bit included) &amp;&amp; length end 0101: positive pulse start(data bit excluded) &amp;&amp; length end 0110: negative pulse start(data bit included) &amp;&amp; negative pulse end(data bit included) 0111: negative pulse start(data bit included) &amp;&amp; negative pulse end (data bit excluded) 1000: negative pulse start(data bit excluded) &amp;&amp; negative pulse end (data bit included) 1001: negative pulse start(data bit excluded) &amp;&amp; negative pulse end (data bit excluded) 1010: negative pulse start(data bit included) &amp;&amp; length end 1011: negative pulse start(data bit excluded) &amp;&amp; length end"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pulse_submode_sel(&mut self) -> RX_PULSE_SUBMODE_SEL_W<19> {
        RX_PULSE_SUBMODE_SEL_W::new(self)
    }
    #[doc = "Bit 23 - Write 0 to sample data at high level of external enable signal. Write 1 to sample data at low level of external enable signal."]
    #[inline(always)]
    #[must_use]
    pub fn rx_level_submode_sel(&mut self) -> RX_LEVEL_SUBMODE_SEL_W<23> {
        RX_LEVEL_SUBMODE_SEL_W::new(self)
    }
    #[doc = "Bits 24:25 - Rx data sampling mode selection. 000: external level enable mode 001: external pulse enable mode 010: internal software enable mode"]
    #[inline(always)]
    #[must_use]
    pub fn rx_smp_mode_sel(&mut self) -> RX_SMP_MODE_SEL_W<24> {
        RX_SMP_MODE_SEL_W::new(self)
    }
    #[doc = "Bit 26 - Write 0 to enable sampling data on the rising edge of rx clock. Write 0 to enable sampling data on the falling edge of rx clock."]
    #[inline(always)]
    #[must_use]
    pub fn rx_clk_edge_sel(&mut self) -> RX_CLK_EDGE_SEL_W<26> {
        RX_CLK_EDGE_SEL_W::new(self)
    }
    #[doc = "Bit 27 - Write 0 to pack bits into 1byte from MSB when data bus width is 4/2/1 bits. Write 0 to pack bits into 1byte from LSB when data bus width is 4/2/1 bits."]
    #[inline(always)]
    #[must_use]
    pub fn rx_bit_pack_order(&mut self) -> RX_BIT_PACK_ORDER_W<27> {
        RX_BIT_PACK_ORDER_W::new(self)
    }
    #[doc = "Bits 28:30 - Rx data bus width selection. 100: bus width is 1 bit 011: bus width is 2 bits 010: bus width is 4 bits 001: bus width is 8 bits 000: bus width is 16 bits"]
    #[inline(always)]
    #[must_use]
    pub fn rx_bus_wid_sel(&mut self) -> RX_BUS_WID_SEL_W<28> {
        RX_BUS_WID_SEL_W::new(self)
    }
    #[doc = "Bit 31 - Write 1 to enable soft reset of async fifo in rx module."]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_srst(&mut self) -> RX_FIFO_SRST_W<31> {
        RX_FIFO_SRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parallel RX module configuration register0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_cfg0](index.html) module"]
pub struct RX_CFG0_SPEC;
impl crate::RegisterSpec for RX_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_cfg0::R](R) reader structure"]
impl crate::Readable for RX_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_cfg0::W](W) writer structure"]
impl crate::Writable for RX_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_CFG0 to value 0"]
impl crate::Resettable for RX_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
