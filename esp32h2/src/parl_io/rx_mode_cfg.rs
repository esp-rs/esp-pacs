#[doc = "Register `RX_MODE_CFG` reader"]
pub struct R(crate::R<RX_MODE_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_MODE_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_MODE_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_MODE_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_MODE_CFG` writer"]
pub struct W(crate::W<RX_MODE_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_MODE_CFG_SPEC>;
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
impl From<crate::W<RX_MODE_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_MODE_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_EXT_EN_SEL` reader - Configures rx external enable signal selection from IO PAD."]
pub type RX_EXT_EN_SEL_R = crate::FieldReader;
#[doc = "Field `RX_EXT_EN_SEL` writer - Configures rx external enable signal selection from IO PAD."]
pub type RX_EXT_EN_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, RX_MODE_CFG_SPEC, 4, O>;
#[doc = "Field `RX_SW_EN` reader - Set this bit to enable data sampling by software."]
pub type RX_SW_EN_R = crate::BitReader;
#[doc = "Field `RX_SW_EN` writer - Set this bit to enable data sampling by software."]
pub type RX_SW_EN_W<'a, const O: u8> = crate::BitWriter<'a, RX_MODE_CFG_SPEC, O>;
#[doc = "Field `RX_EXT_EN_INV` reader - Set this bit to invert the external enable signal."]
pub type RX_EXT_EN_INV_R = crate::BitReader;
#[doc = "Field `RX_EXT_EN_INV` writer - Set this bit to invert the external enable signal."]
pub type RX_EXT_EN_INV_W<'a, const O: u8> = crate::BitWriter<'a, RX_MODE_CFG_SPEC, O>;
#[doc = "Field `RX_PULSE_SUBMODE_SEL` reader - Configures the rxd pulse sampling submode. 4'd0: positive pulse start(data bit included) &amp;&amp; positive pulse end(data bit included) 4'd1: positive pulse start(data bit included) &amp;&amp; positive pulse end (data bit excluded) 4'd2: positive pulse start(data bit excluded) &amp;&amp; positive pulse end (data bit included) 4'd3: positive pulse start(data bit excluded) &amp;&amp; positive pulse end (data bit excluded) 4'd4: positive pulse start(data bit included) &amp;&amp; length end 4'd5: positive pulse start(data bit excluded) &amp;&amp; length end"]
pub type RX_PULSE_SUBMODE_SEL_R = crate::FieldReader;
#[doc = "Field `RX_PULSE_SUBMODE_SEL` writer - Configures the rxd pulse sampling submode. 4'd0: positive pulse start(data bit included) &amp;&amp; positive pulse end(data bit included) 4'd1: positive pulse start(data bit included) &amp;&amp; positive pulse end (data bit excluded) 4'd2: positive pulse start(data bit excluded) &amp;&amp; positive pulse end (data bit included) 4'd3: positive pulse start(data bit excluded) &amp;&amp; positive pulse end (data bit excluded) 4'd4: positive pulse start(data bit included) &amp;&amp; length end 4'd5: positive pulse start(data bit excluded) &amp;&amp; length end"]
pub type RX_PULSE_SUBMODE_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, RX_MODE_CFG_SPEC, 3, O>;
#[doc = "Field `RX_SMP_MODE_SEL` reader - Configures the rxd sampling mode. 2'b00: external level enable mode 2'b01: external pulse enable mode 2'b10: internal software enable mode"]
pub type RX_SMP_MODE_SEL_R = crate::FieldReader;
#[doc = "Field `RX_SMP_MODE_SEL` writer - Configures the rxd sampling mode. 2'b00: external level enable mode 2'b01: external pulse enable mode 2'b10: internal software enable mode"]
pub type RX_SMP_MODE_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, RX_MODE_CFG_SPEC, 2, O>;
impl R {
    #[doc = "Bits 21:24 - Configures rx external enable signal selection from IO PAD."]
    #[inline(always)]
    pub fn rx_ext_en_sel(&self) -> RX_EXT_EN_SEL_R {
        RX_EXT_EN_SEL_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bit 25 - Set this bit to enable data sampling by software."]
    #[inline(always)]
    pub fn rx_sw_en(&self) -> RX_SW_EN_R {
        RX_SW_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Set this bit to invert the external enable signal."]
    #[inline(always)]
    pub fn rx_ext_en_inv(&self) -> RX_EXT_EN_INV_R {
        RX_EXT_EN_INV_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:29 - Configures the rxd pulse sampling submode. 4'd0: positive pulse start(data bit included) &amp;&amp; positive pulse end(data bit included) 4'd1: positive pulse start(data bit included) &amp;&amp; positive pulse end (data bit excluded) 4'd2: positive pulse start(data bit excluded) &amp;&amp; positive pulse end (data bit included) 4'd3: positive pulse start(data bit excluded) &amp;&amp; positive pulse end (data bit excluded) 4'd4: positive pulse start(data bit included) &amp;&amp; length end 4'd5: positive pulse start(data bit excluded) &amp;&amp; length end"]
    #[inline(always)]
    pub fn rx_pulse_submode_sel(&self) -> RX_PULSE_SUBMODE_SEL_R {
        RX_PULSE_SUBMODE_SEL_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bits 30:31 - Configures the rxd sampling mode. 2'b00: external level enable mode 2'b01: external pulse enable mode 2'b10: internal software enable mode"]
    #[inline(always)]
    pub fn rx_smp_mode_sel(&self) -> RX_SMP_MODE_SEL_R {
        RX_SMP_MODE_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_MODE_CFG")
            .field(
                "rx_ext_en_sel",
                &format_args!("{}", self.rx_ext_en_sel().bits()),
            )
            .field("rx_sw_en", &format_args!("{}", self.rx_sw_en().bit()))
            .field(
                "rx_ext_en_inv",
                &format_args!("{}", self.rx_ext_en_inv().bit()),
            )
            .field(
                "rx_pulse_submode_sel",
                &format_args!("{}", self.rx_pulse_submode_sel().bits()),
            )
            .field(
                "rx_smp_mode_sel",
                &format_args!("{}", self.rx_smp_mode_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_MODE_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 21:24 - Configures rx external enable signal selection from IO PAD."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ext_en_sel(&mut self) -> RX_EXT_EN_SEL_W<21> {
        RX_EXT_EN_SEL_W::new(self)
    }
    #[doc = "Bit 25 - Set this bit to enable data sampling by software."]
    #[inline(always)]
    #[must_use]
    pub fn rx_sw_en(&mut self) -> RX_SW_EN_W<25> {
        RX_SW_EN_W::new(self)
    }
    #[doc = "Bit 26 - Set this bit to invert the external enable signal."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ext_en_inv(&mut self) -> RX_EXT_EN_INV_W<26> {
        RX_EXT_EN_INV_W::new(self)
    }
    #[doc = "Bits 27:29 - Configures the rxd pulse sampling submode. 4'd0: positive pulse start(data bit included) &amp;&amp; positive pulse end(data bit included) 4'd1: positive pulse start(data bit included) &amp;&amp; positive pulse end (data bit excluded) 4'd2: positive pulse start(data bit excluded) &amp;&amp; positive pulse end (data bit included) 4'd3: positive pulse start(data bit excluded) &amp;&amp; positive pulse end (data bit excluded) 4'd4: positive pulse start(data bit included) &amp;&amp; length end 4'd5: positive pulse start(data bit excluded) &amp;&amp; length end"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pulse_submode_sel(&mut self) -> RX_PULSE_SUBMODE_SEL_W<27> {
        RX_PULSE_SUBMODE_SEL_W::new(self)
    }
    #[doc = "Bits 30:31 - Configures the rxd sampling mode. 2'b00: external level enable mode 2'b01: external pulse enable mode 2'b10: internal software enable mode"]
    #[inline(always)]
    #[must_use]
    pub fn rx_smp_mode_sel(&mut self) -> RX_SMP_MODE_SEL_W<30> {
        RX_SMP_MODE_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parallel RX Sampling mode configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_mode_cfg](index.html) module"]
pub struct RX_MODE_CFG_SPEC;
impl crate::RegisterSpec for RX_MODE_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_mode_cfg::R](R) reader structure"]
impl crate::Readable for RX_MODE_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_mode_cfg::W](W) writer structure"]
impl crate::Writable for RX_MODE_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_MODE_CFG to value 0x00e0_0000"]
impl crate::Resettable for RX_MODE_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x00e0_0000;
}
