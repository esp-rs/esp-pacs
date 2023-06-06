#[doc = "Register `SAR_DAC_CTRL1` reader"]
pub struct R(crate::R<SAR_DAC_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_DAC_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_DAC_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_DAC_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_DAC_CTRL1` writer"]
pub struct W(crate::W<SAR_DAC_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_DAC_CTRL1_SPEC>;
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
impl From<crate::W<SAR_DAC_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_DAC_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW_FSTEP` reader - frequency step for CW generator can be used to adjust the frequency"]
pub type SW_FSTEP_R = crate::FieldReader<u16>;
#[doc = "Field `SW_FSTEP` writer - frequency step for CW generator can be used to adjust the frequency"]
pub type SW_FSTEP_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_DAC_CTRL1_SPEC, 16, O, u16>;
#[doc = "Field `SW_TONE_EN` reader - 1: enable CW generator 0: disable CW generator"]
pub type SW_TONE_EN_R = crate::BitReader;
#[doc = "Field `SW_TONE_EN` writer - 1: enable CW generator 0: disable CW generator"]
pub type SW_TONE_EN_W<'a, const O: u8> = crate::BitWriter<'a, SAR_DAC_CTRL1_SPEC, O>;
#[doc = "Field `DEBUG_BIT_SEL` reader - "]
pub type DEBUG_BIT_SEL_R = crate::FieldReader;
#[doc = "Field `DEBUG_BIT_SEL` writer - "]
pub type DEBUG_BIT_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_DAC_CTRL1_SPEC, 5, O>;
#[doc = "Field `DAC_DIG_FORCE` reader - 1: DAC1 &amp; DAC2 use DMA 0: DAC1 &amp; DAC2 do not use DMA"]
pub type DAC_DIG_FORCE_R = crate::BitReader;
#[doc = "Field `DAC_DIG_FORCE` writer - 1: DAC1 &amp; DAC2 use DMA 0: DAC1 &amp; DAC2 do not use DMA"]
pub type DAC_DIG_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, SAR_DAC_CTRL1_SPEC, O>;
#[doc = "Field `DAC_CLK_FORCE_LOW` reader - 1: force PDAC_CLK to low"]
pub type DAC_CLK_FORCE_LOW_R = crate::BitReader;
#[doc = "Field `DAC_CLK_FORCE_LOW` writer - 1: force PDAC_CLK to low"]
pub type DAC_CLK_FORCE_LOW_W<'a, const O: u8> = crate::BitWriter<'a, SAR_DAC_CTRL1_SPEC, O>;
#[doc = "Field `DAC_CLK_FORCE_HIGH` reader - 1: force PDAC_CLK to high"]
pub type DAC_CLK_FORCE_HIGH_R = crate::BitReader;
#[doc = "Field `DAC_CLK_FORCE_HIGH` writer - 1: force PDAC_CLK to high"]
pub type DAC_CLK_FORCE_HIGH_W<'a, const O: u8> = crate::BitWriter<'a, SAR_DAC_CTRL1_SPEC, O>;
#[doc = "Field `DAC_CLK_INV` reader - 1: invert PDAC_CLK"]
pub type DAC_CLK_INV_R = crate::BitReader;
#[doc = "Field `DAC_CLK_INV` writer - 1: invert PDAC_CLK"]
pub type DAC_CLK_INV_W<'a, const O: u8> = crate::BitWriter<'a, SAR_DAC_CTRL1_SPEC, O>;
impl R {
    #[doc = "Bits 0:15 - frequency step for CW generator can be used to adjust the frequency"]
    #[inline(always)]
    pub fn sw_fstep(&self) -> SW_FSTEP_R {
        SW_FSTEP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 1: enable CW generator 0: disable CW generator"]
    #[inline(always)]
    pub fn sw_tone_en(&self) -> SW_TONE_EN_R {
        SW_TONE_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn debug_bit_sel(&self) -> DEBUG_BIT_SEL_R {
        DEBUG_BIT_SEL_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - 1: DAC1 &amp; DAC2 use DMA 0: DAC1 &amp; DAC2 do not use DMA"]
    #[inline(always)]
    pub fn dac_dig_force(&self) -> DAC_DIG_FORCE_R {
        DAC_DIG_FORCE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: force PDAC_CLK to low"]
    #[inline(always)]
    pub fn dac_clk_force_low(&self) -> DAC_CLK_FORCE_LOW_R {
        DAC_CLK_FORCE_LOW_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: force PDAC_CLK to high"]
    #[inline(always)]
    pub fn dac_clk_force_high(&self) -> DAC_CLK_FORCE_HIGH_R {
        DAC_CLK_FORCE_HIGH_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: invert PDAC_CLK"]
    #[inline(always)]
    pub fn dac_clk_inv(&self) -> DAC_CLK_INV_R {
        DAC_CLK_INV_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_DAC_CTRL1")
            .field("sw_fstep", &format_args!("{}", self.sw_fstep().bits()))
            .field("sw_tone_en", &format_args!("{}", self.sw_tone_en().bit()))
            .field(
                "debug_bit_sel",
                &format_args!("{}", self.debug_bit_sel().bits()),
            )
            .field(
                "dac_dig_force",
                &format_args!("{}", self.dac_dig_force().bit()),
            )
            .field(
                "dac_clk_force_low",
                &format_args!("{}", self.dac_clk_force_low().bit()),
            )
            .field(
                "dac_clk_force_high",
                &format_args!("{}", self.dac_clk_force_high().bit()),
            )
            .field("dac_clk_inv", &format_args!("{}", self.dac_clk_inv().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_DAC_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - frequency step for CW generator can be used to adjust the frequency"]
    #[inline(always)]
    #[must_use]
    pub fn sw_fstep(&mut self) -> SW_FSTEP_W<0> {
        SW_FSTEP_W::new(self)
    }
    #[doc = "Bit 16 - 1: enable CW generator 0: disable CW generator"]
    #[inline(always)]
    #[must_use]
    pub fn sw_tone_en(&mut self) -> SW_TONE_EN_W<16> {
        SW_TONE_EN_W::new(self)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn debug_bit_sel(&mut self) -> DEBUG_BIT_SEL_W<17> {
        DEBUG_BIT_SEL_W::new(self)
    }
    #[doc = "Bit 22 - 1: DAC1 &amp; DAC2 use DMA 0: DAC1 &amp; DAC2 do not use DMA"]
    #[inline(always)]
    #[must_use]
    pub fn dac_dig_force(&mut self) -> DAC_DIG_FORCE_W<22> {
        DAC_DIG_FORCE_W::new(self)
    }
    #[doc = "Bit 23 - 1: force PDAC_CLK to low"]
    #[inline(always)]
    #[must_use]
    pub fn dac_clk_force_low(&mut self) -> DAC_CLK_FORCE_LOW_W<23> {
        DAC_CLK_FORCE_LOW_W::new(self)
    }
    #[doc = "Bit 24 - 1: force PDAC_CLK to high"]
    #[inline(always)]
    #[must_use]
    pub fn dac_clk_force_high(&mut self) -> DAC_CLK_FORCE_HIGH_W<24> {
        DAC_CLK_FORCE_HIGH_W::new(self)
    }
    #[doc = "Bit 25 - 1: invert PDAC_CLK"]
    #[inline(always)]
    #[must_use]
    pub fn dac_clk_inv(&mut self) -> DAC_CLK_INV_W<25> {
        DAC_CLK_INV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_dac_ctrl1](index.html) module"]
pub struct SAR_DAC_CTRL1_SPEC;
impl crate::RegisterSpec for SAR_DAC_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_dac_ctrl1::R](R) reader structure"]
impl crate::Readable for SAR_DAC_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_dac_ctrl1::W](W) writer structure"]
impl crate::Writable for SAR_DAC_CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_DAC_CTRL1 to value 0"]
impl crate::Resettable for SAR_DAC_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
