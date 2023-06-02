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
#[doc = "Field `SARADC_MEAS_NUM_LIMIT` reader - enable max meas num"]
pub type SARADC_MEAS_NUM_LIMIT_R = crate::BitReader;
#[doc = "Field `SARADC_MEAS_NUM_LIMIT` writer - enable max meas num"]
pub type SARADC_MEAS_NUM_LIMIT_W<'a, const O: u8> = crate::BitWriter<'a, CTRL2_SPEC, O>;
#[doc = "Field `SARADC_MAX_MEAS_NUM` reader - max conversion number"]
pub type SARADC_MAX_MEAS_NUM_R = crate::FieldReader;
#[doc = "Field `SARADC_MAX_MEAS_NUM` writer - max conversion number"]
pub type SARADC_MAX_MEAS_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, CTRL2_SPEC, 8, O>;
#[doc = "Field `SARADC_SAR1_INV` reader - 1: data to DIG ADC1 CTRL is inverted, otherwise not"]
pub type SARADC_SAR1_INV_R = crate::BitReader;
#[doc = "Field `SARADC_SAR1_INV` writer - 1: data to DIG ADC1 CTRL is inverted, otherwise not"]
pub type SARADC_SAR1_INV_W<'a, const O: u8> = crate::BitWriter<'a, CTRL2_SPEC, O>;
#[doc = "Field `SARADC_SAR2_INV` reader - 1: data to DIG ADC2 CTRL is inverted, otherwise not"]
pub type SARADC_SAR2_INV_R = crate::BitReader;
#[doc = "Field `SARADC_SAR2_INV` writer - 1: data to DIG ADC2 CTRL is inverted, otherwise not"]
pub type SARADC_SAR2_INV_W<'a, const O: u8> = crate::BitWriter<'a, CTRL2_SPEC, O>;
#[doc = "Field `SARADC_TIMER_TARGET` reader - to set saradc timer target"]
pub type SARADC_TIMER_TARGET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SARADC_TIMER_TARGET` writer - to set saradc timer target"]
pub type SARADC_TIMER_TARGET_W<'a, const O: u8> =
    crate::FieldWriter<'a, CTRL2_SPEC, 12, O, u16, u16>;
#[doc = "Field `SARADC_TIMER_EN` reader - to enable saradc timer trigger"]
pub type SARADC_TIMER_EN_R = crate::BitReader;
#[doc = "Field `SARADC_TIMER_EN` writer - to enable saradc timer trigger"]
pub type SARADC_TIMER_EN_W<'a, const O: u8> = crate::BitWriter<'a, CTRL2_SPEC, O>;
impl R {
    #[doc = "Bit 0 - enable max meas num"]
    #[inline(always)]
    pub fn saradc_meas_num_limit(&self) -> SARADC_MEAS_NUM_LIMIT_R {
        SARADC_MEAS_NUM_LIMIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - max conversion number"]
    #[inline(always)]
    pub fn saradc_max_meas_num(&self) -> SARADC_MAX_MEAS_NUM_R {
        SARADC_MAX_MEAS_NUM_R::new(((self.bits >> 1) & 0xff) as u8)
    }
    #[doc = "Bit 9 - 1: data to DIG ADC1 CTRL is inverted, otherwise not"]
    #[inline(always)]
    pub fn saradc_sar1_inv(&self) -> SARADC_SAR1_INV_R {
        SARADC_SAR1_INV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: data to DIG ADC2 CTRL is inverted, otherwise not"]
    #[inline(always)]
    pub fn saradc_sar2_inv(&self) -> SARADC_SAR2_INV_R {
        SARADC_SAR2_INV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:23 - to set saradc timer target"]
    #[inline(always)]
    pub fn saradc_timer_target(&self) -> SARADC_TIMER_TARGET_R {
        SARADC_TIMER_TARGET_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
    #[doc = "Bit 24 - to enable saradc timer trigger"]
    #[inline(always)]
    pub fn saradc_timer_en(&self) -> SARADC_TIMER_EN_R {
        SARADC_TIMER_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field(
                "saradc_meas_num_limit",
                &format_args!("{}", self.saradc_meas_num_limit().bit()),
            )
            .field(
                "saradc_max_meas_num",
                &format_args!("{}", self.saradc_max_meas_num().bits()),
            )
            .field(
                "saradc_sar1_inv",
                &format_args!("{}", self.saradc_sar1_inv().bit()),
            )
            .field(
                "saradc_sar2_inv",
                &format_args!("{}", self.saradc_sar2_inv().bit()),
            )
            .field(
                "saradc_timer_target",
                &format_args!("{}", self.saradc_timer_target().bits()),
            )
            .field(
                "saradc_timer_en",
                &format_args!("{}", self.saradc_timer_en().bit()),
            )
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
    #[doc = "Bit 0 - enable max meas num"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_meas_num_limit(&mut self) -> SARADC_MEAS_NUM_LIMIT_W<0> {
        SARADC_MEAS_NUM_LIMIT_W::new(self)
    }
    #[doc = "Bits 1:8 - max conversion number"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_max_meas_num(&mut self) -> SARADC_MAX_MEAS_NUM_W<1> {
        SARADC_MAX_MEAS_NUM_W::new(self)
    }
    #[doc = "Bit 9 - 1: data to DIG ADC1 CTRL is inverted, otherwise not"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_sar1_inv(&mut self) -> SARADC_SAR1_INV_W<9> {
        SARADC_SAR1_INV_W::new(self)
    }
    #[doc = "Bit 10 - 1: data to DIG ADC2 CTRL is inverted, otherwise not"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_sar2_inv(&mut self) -> SARADC_SAR2_INV_W<10> {
        SARADC_SAR2_INV_W::new(self)
    }
    #[doc = "Bits 12:23 - to set saradc timer target"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_timer_target(&mut self) -> SARADC_TIMER_TARGET_W<12> {
        SARADC_TIMER_TARGET_W::new(self)
    }
    #[doc = "Bit 24 - to enable saradc timer trigger"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_timer_en(&mut self) -> SARADC_TIMER_EN_W<24> {
        SARADC_TIMER_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "digital saradc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](index.html) module"]
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
