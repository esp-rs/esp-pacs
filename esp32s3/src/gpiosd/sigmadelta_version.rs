#[doc = "Register `SIGMADELTA_VERSION` reader"]
pub struct R(crate::R<SIGMADELTA_VERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGMADELTA_VERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGMADELTA_VERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGMADELTA_VERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGMADELTA_VERSION` writer"]
pub struct W(crate::W<SIGMADELTA_VERSION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGMADELTA_VERSION_SPEC>;
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
impl From<crate::W<SIGMADELTA_VERSION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGMADELTA_VERSION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_SD_DATE` reader - Version control register."]
pub type GPIO_SD_DATE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GPIO_SD_DATE` writer - Version control register."]
pub type GPIO_SD_DATE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SIGMADELTA_VERSION_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:27 - Version control register."]
    #[inline(always)]
    pub fn gpio_sd_date(&self) -> GPIO_SD_DATE_R {
        GPIO_SD_DATE_R::new((self.bits & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:27 - Version control register."]
    #[inline(always)]
    pub fn gpio_sd_date(&mut self) -> GPIO_SD_DATE_W<0> {
        GPIO_SD_DATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Version Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigmadelta_version](index.html) module"]
pub struct SIGMADELTA_VERSION_SPEC;
impl crate::RegisterSpec for SIGMADELTA_VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigmadelta_version::R](R) reader structure"]
impl crate::Readable for SIGMADELTA_VERSION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sigmadelta_version::W](W) writer structure"]
impl crate::Writable for SIGMADELTA_VERSION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIGMADELTA_VERSION to value 0x0180_2260"]
impl crate::Resettable for SIGMADELTA_VERSION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0180_2260
    }
}
