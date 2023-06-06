#[doc = "Register `CTRL2` reader"]
pub struct R(crate::R<CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL2` writer"]
pub struct W(crate::W<CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL2_SPEC>;
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
impl From<crate::W<CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEAS_NUM_LIMIT` reader - Enable limit times of SAR ADC sample."]
pub type MEAS_NUM_LIMIT_R = crate::BitReader;
#[doc = "Field `MEAS_NUM_LIMIT` writer - Enable limit times of SAR ADC sample."]
pub type MEAS_NUM_LIMIT_W<'a, const O: u8> = crate::BitWriter<'a, CTRL2_SPEC, O>;
#[doc = "Field `MAX_MEAS_NUM` reader - Set maximum conversion number."]
pub type MAX_MEAS_NUM_R = crate::FieldReader;
#[doc = "Field `MAX_MEAS_NUM` writer - Set maximum conversion number."]
pub type MAX_MEAS_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, CTRL2_SPEC, 8, O>;
#[doc = "Field `SAR1_INV` reader - 1: data to DIG ADC1 CTRL is inverted, otherwise not."]
pub type SAR1_INV_R = crate::BitReader;
#[doc = "Field `SAR1_INV` writer - 1: data to DIG ADC1 CTRL is inverted, otherwise not."]
pub type SAR1_INV_W<'a, const O: u8> = crate::BitWriter<'a, CTRL2_SPEC, O>;
#[doc = "Field `SAR2_INV` reader - 1: data to DIG ADC2 CTRL is inverted, otherwise not."]
pub type SAR2_INV_R = crate::BitReader;
#[doc = "Field `SAR2_INV` writer - 1: data to DIG ADC2 CTRL is inverted, otherwise not."]
pub type SAR2_INV_W<'a, const O: u8> = crate::BitWriter<'a, CTRL2_SPEC, O>;
#[doc = "Field `TIMER_SEL` reader - 1: select saradc timer 0: i2s_ws trigger"]
pub type TIMER_SEL_R = crate::BitReader;
#[doc = "Field `TIMER_SEL` writer - 1: select saradc timer 0: i2s_ws trigger"]
pub type TIMER_SEL_W<'a, const O: u8> = crate::BitWriter<'a, CTRL2_SPEC, O>;
#[doc = "Field `TIMER_TARGET` reader - Set SAR ADC timer target."]
pub type TIMER_TARGET_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER_TARGET` writer - Set SAR ADC timer target."]
pub type TIMER_TARGET_W<'a, const O: u8> = crate::FieldWriter<'a, CTRL2_SPEC, 12, O, u16>;
#[doc = "Field `TIMER_EN` reader - Enable SAR ADC timer trigger."]
pub type TIMER_EN_R = crate::BitReader;
#[doc = "Field `TIMER_EN` writer - Enable SAR ADC timer trigger."]
pub type TIMER_EN_W<'a, const O: u8> = crate::BitWriter<'a, CTRL2_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Enable limit times of SAR ADC sample."]
    #[inline(always)]
    pub fn meas_num_limit(&self) -> MEAS_NUM_LIMIT_R {
        MEAS_NUM_LIMIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - Set maximum conversion number."]
    #[inline(always)]
    pub fn max_meas_num(&self) -> MAX_MEAS_NUM_R {
        MAX_MEAS_NUM_R::new(((self.bits >> 1) & 0xff) as u8)
    }
    #[doc = "Bit 9 - 1: data to DIG ADC1 CTRL is inverted, otherwise not."]
    #[inline(always)]
    pub fn sar1_inv(&self) -> SAR1_INV_R {
        SAR1_INV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: data to DIG ADC2 CTRL is inverted, otherwise not."]
    #[inline(always)]
    pub fn sar2_inv(&self) -> SAR2_INV_R {
        SAR2_INV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: select saradc timer 0: i2s_ws trigger"]
    #[inline(always)]
    pub fn timer_sel(&self) -> TIMER_SEL_R {
        TIMER_SEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:23 - Set SAR ADC timer target."]
    #[inline(always)]
    pub fn timer_target(&self) -> TIMER_TARGET_R {
        TIMER_TARGET_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bit 24 - Enable SAR ADC timer trigger."]
    #[inline(always)]
    pub fn timer_en(&self) -> TIMER_EN_R {
        TIMER_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field(
                "meas_num_limit",
                &format_args!("{}", self.meas_num_limit().bit()),
            )
            .field(
                "max_meas_num",
                &format_args!("{}", self.max_meas_num().bits()),
            )
            .field("sar1_inv", &format_args!("{}", self.sar1_inv().bit()))
            .field("sar2_inv", &format_args!("{}", self.sar2_inv().bit()))
            .field("timer_sel", &format_args!("{}", self.timer_sel().bit()))
            .field(
                "timer_target",
                &format_args!("{}", self.timer_target().bits()),
            )
            .field("timer_en", &format_args!("{}", self.timer_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CTRL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Enable limit times of SAR ADC sample."]
    #[inline(always)]
    #[must_use]
    pub fn meas_num_limit(&mut self) -> MEAS_NUM_LIMIT_W<0> {
        MEAS_NUM_LIMIT_W::new(self)
    }
    #[doc = "Bits 1:8 - Set maximum conversion number."]
    #[inline(always)]
    #[must_use]
    pub fn max_meas_num(&mut self) -> MAX_MEAS_NUM_W<1> {
        MAX_MEAS_NUM_W::new(self)
    }
    #[doc = "Bit 9 - 1: data to DIG ADC1 CTRL is inverted, otherwise not."]
    #[inline(always)]
    #[must_use]
    pub fn sar1_inv(&mut self) -> SAR1_INV_W<9> {
        SAR1_INV_W::new(self)
    }
    #[doc = "Bit 10 - 1: data to DIG ADC2 CTRL is inverted, otherwise not."]
    #[inline(always)]
    #[must_use]
    pub fn sar2_inv(&mut self) -> SAR2_INV_W<10> {
        SAR2_INV_W::new(self)
    }
    #[doc = "Bit 11 - 1: select saradc timer 0: i2s_ws trigger"]
    #[inline(always)]
    #[must_use]
    pub fn timer_sel(&mut self) -> TIMER_SEL_W<11> {
        TIMER_SEL_W::new(self)
    }
    #[doc = "Bits 12:23 - Set SAR ADC timer target."]
    #[inline(always)]
    #[must_use]
    pub fn timer_target(&mut self) -> TIMER_TARGET_W<12> {
        TIMER_TARGET_W::new(self)
    }
    #[doc = "Bit 24 - Enable SAR ADC timer trigger."]
    #[inline(always)]
    #[must_use]
    pub fn timer_en(&mut self) -> TIMER_EN_W<24> {
        TIMER_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DIG ADC common configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](index.html) module"]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl2::R](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0xa1fe"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0xa1fe;
}
