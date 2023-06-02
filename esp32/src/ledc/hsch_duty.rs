#[doc = "Register `HSCH%s_DUTY` reader"]
pub struct R(crate::R<HSCH_DUTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSCH_DUTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSCH_DUTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSCH_DUTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSCH%s_DUTY` writer"]
pub struct W(crate::W<HSCH_DUTY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSCH_DUTY_SPEC>;
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
impl From<crate::W<HSCH_DUTY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSCH_DUTY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUTY` reader - This register represents the current duty of the output signal for high speed channel0."]
pub type DUTY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DUTY` writer - This register represents the current duty of the output signal for high speed channel0."]
pub type DUTY_W<'a, const O: u8> = crate::FieldWriter<'a, HSCH_DUTY_SPEC, 25, O, u32, u32>;
impl R {
    #[doc = "Bits 0:24 - This register represents the current duty of the output signal for high speed channel0."]
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(self.bits & 0x01ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSCH_DUTY")
            .field("duty", &format_args!("{}", self.duty().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HSCH_DUTY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:24 - This register represents the current duty of the output signal for high speed channel0."]
    #[inline(always)]
    #[must_use]
    pub fn duty(&mut self) -> DUTY_W<0> {
        DUTY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsch_duty](index.html) module"]
pub struct HSCH_DUTY_SPEC;
impl crate::RegisterSpec for HSCH_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsch_duty::R](R) reader structure"]
impl crate::Readable for HSCH_DUTY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsch_duty::W](W) writer structure"]
impl crate::Writable for HSCH_DUTY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSCH%s_DUTY to value 0"]
impl crate::Resettable for HSCH_DUTY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
