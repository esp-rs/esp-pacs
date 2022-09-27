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
#[doc = "Field `SW_FSTEP` reader - Frequency step for CW generator can be used to adjust the frequency."]
pub type SW_FSTEP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SW_FSTEP` writer - Frequency step for CW generator can be used to adjust the frequency."]
pub type SW_FSTEP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_DAC_CTRL1_SPEC, u16, u16, 16, O>;
#[doc = "Field `SW_TONE_EN` reader - 0: disable CW generator. 1: enable CW generator."]
pub type SW_TONE_EN_R = crate::BitReader<bool>;
#[doc = "Field `SW_TONE_EN` writer - 0: disable CW generator. 1: enable CW generator."]
pub type SW_TONE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAR_DAC_CTRL1_SPEC, bool, O>;
#[doc = "Field `DEBUG_BIT_SEL` reader - "]
pub type DEBUG_BIT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEBUG_BIT_SEL` writer - "]
pub type DEBUG_BIT_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SAR_DAC_CTRL1_SPEC, u8, u8, 5, O>;
#[doc = "Field `DAC_DIG_FORCE` reader - 0: DAC1 and DAC2 do not use DMA. 1: DAC1 and DAC2 use DMA."]
pub type DAC_DIG_FORCE_R = crate::BitReader<bool>;
#[doc = "Field `DAC_DIG_FORCE` writer - 0: DAC1 and DAC2 do not use DMA. 1: DAC1 and DAC2 use DMA."]
pub type DAC_DIG_FORCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAR_DAC_CTRL1_SPEC, bool, O>;
#[doc = "Field `DAC_CLK_FORCE_LOW` reader - 1: force PDAC_CLK to low"]
pub type DAC_CLK_FORCE_LOW_R = crate::BitReader<bool>;
#[doc = "Field `DAC_CLK_FORCE_LOW` writer - 1: force PDAC_CLK to low"]
pub type DAC_CLK_FORCE_LOW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_DAC_CTRL1_SPEC, bool, O>;
#[doc = "Field `DAC_CLK_FORCE_HIGH` reader - 1: force PDAC_CLK to high"]
pub type DAC_CLK_FORCE_HIGH_R = crate::BitReader<bool>;
#[doc = "Field `DAC_CLK_FORCE_HIGH` writer - 1: force PDAC_CLK to high"]
pub type DAC_CLK_FORCE_HIGH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SAR_DAC_CTRL1_SPEC, bool, O>;
#[doc = "Field `DAC_CLK_INV` reader - 1: invert PDAC_CLK."]
pub type DAC_CLK_INV_R = crate::BitReader<bool>;
#[doc = "Field `DAC_CLK_INV` writer - 1: invert PDAC_CLK."]
pub type DAC_CLK_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAR_DAC_CTRL1_SPEC, bool, O>;
#[doc = "Field `DAC_RESET` reader - Reset DAC by software."]
pub type DAC_RESET_R = crate::BitReader<bool>;
#[doc = "Field `DAC_RESET` writer - Reset DAC by software."]
pub type DAC_RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAR_DAC_CTRL1_SPEC, bool, O>;
#[doc = "Field `DAC_CLKGATE_EN` reader - DAC clock gate enable bit."]
pub type DAC_CLKGATE_EN_R = crate::BitReader<bool>;
#[doc = "Field `DAC_CLKGATE_EN` writer - DAC clock gate enable bit."]
pub type DAC_CLKGATE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAR_DAC_CTRL1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - Frequency step for CW generator can be used to adjust the frequency."]
    #[inline(always)]
    pub fn sw_fstep(&self) -> SW_FSTEP_R {
        SW_FSTEP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 0: disable CW generator. 1: enable CW generator."]
    #[inline(always)]
    pub fn sw_tone_en(&self) -> SW_TONE_EN_R {
        SW_TONE_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn debug_bit_sel(&self) -> DEBUG_BIT_SEL_R {
        DEBUG_BIT_SEL_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - 0: DAC1 and DAC2 do not use DMA. 1: DAC1 and DAC2 use DMA."]
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
    #[doc = "Bit 25 - 1: invert PDAC_CLK."]
    #[inline(always)]
    pub fn dac_clk_inv(&self) -> DAC_CLK_INV_R {
        DAC_CLK_INV_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reset DAC by software."]
    #[inline(always)]
    pub fn dac_reset(&self) -> DAC_RESET_R {
        DAC_RESET_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DAC clock gate enable bit."]
    #[inline(always)]
    pub fn dac_clkgate_en(&self) -> DAC_CLKGATE_EN_R {
        DAC_CLKGATE_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Frequency step for CW generator can be used to adjust the frequency."]
    #[inline(always)]
    pub fn sw_fstep(&mut self) -> SW_FSTEP_W<0> {
        SW_FSTEP_W::new(self)
    }
    #[doc = "Bit 16 - 0: disable CW generator. 1: enable CW generator."]
    #[inline(always)]
    pub fn sw_tone_en(&mut self) -> SW_TONE_EN_W<16> {
        SW_TONE_EN_W::new(self)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn debug_bit_sel(&mut self) -> DEBUG_BIT_SEL_W<17> {
        DEBUG_BIT_SEL_W::new(self)
    }
    #[doc = "Bit 22 - 0: DAC1 and DAC2 do not use DMA. 1: DAC1 and DAC2 use DMA."]
    #[inline(always)]
    pub fn dac_dig_force(&mut self) -> DAC_DIG_FORCE_W<22> {
        DAC_DIG_FORCE_W::new(self)
    }
    #[doc = "Bit 23 - 1: force PDAC_CLK to low"]
    #[inline(always)]
    pub fn dac_clk_force_low(&mut self) -> DAC_CLK_FORCE_LOW_W<23> {
        DAC_CLK_FORCE_LOW_W::new(self)
    }
    #[doc = "Bit 24 - 1: force PDAC_CLK to high"]
    #[inline(always)]
    pub fn dac_clk_force_high(&mut self) -> DAC_CLK_FORCE_HIGH_W<24> {
        DAC_CLK_FORCE_HIGH_W::new(self)
    }
    #[doc = "Bit 25 - 1: invert PDAC_CLK."]
    #[inline(always)]
    pub fn dac_clk_inv(&mut self) -> DAC_CLK_INV_W<25> {
        DAC_CLK_INV_W::new(self)
    }
    #[doc = "Bit 26 - Reset DAC by software."]
    #[inline(always)]
    pub fn dac_reset(&mut self) -> DAC_RESET_W<26> {
        DAC_RESET_W::new(self)
    }
    #[doc = "Bit 27 - DAC clock gate enable bit."]
    #[inline(always)]
    pub fn dac_clkgate_en(&mut self) -> DAC_CLKGATE_EN_W<27> {
        DAC_CLKGATE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_dac_ctrl1](index.html) module"]
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
}
#[doc = "`reset()` method sets SAR_DAC_CTRL1 to value 0"]
impl crate::Resettable for SAR_DAC_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
