#[doc = "Register `CH%s_CONF1` reader"]
pub struct R(crate::R<CH_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%s_CONF1` writer"]
pub struct W(crate::W<CH_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_CONF1_SPEC>;
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
impl From<crate::W<CH_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUTY_START` reader - Other configured fields in LEDC_CH%s_CONF1_REG will start to take effect when this bit is set to 1."]
pub type DUTY_START_R = crate::BitReader<bool>;
#[doc = "Field `DUTY_START` writer - Other configured fields in LEDC_CH%s_CONF1_REG will start to take effect when this bit is set to 1."]
pub type DUTY_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH_CONF1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - Other configured fields in LEDC_CH%s_CONF1_REG will start to take effect when this bit is set to 1."]
    #[inline(always)]
    pub fn duty_start(&self) -> DUTY_START_R {
        DUTY_START_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Other configured fields in LEDC_CH%s_CONF1_REG will start to take effect when this bit is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn duty_start(&mut self) -> DUTY_START_W<31> {
        DUTY_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register 1 for channel %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_conf1](index.html) module"]
pub struct CH_CONF1_SPEC;
impl crate::RegisterSpec for CH_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_conf1::R](R) reader structure"]
impl crate::Readable for CH_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_conf1::W](W) writer structure"]
impl crate::Writable for CH_CONF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH%s_CONF1 to value 0"]
impl crate::Resettable for CH_CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
