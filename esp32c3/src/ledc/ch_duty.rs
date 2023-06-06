#[doc = "Register `CH%s_DUTY` reader"]
pub struct R(crate::R<CH_DUTY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_DUTY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_DUTY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_DUTY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%s_DUTY` writer"]
pub struct W(crate::W<CH_DUTY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_DUTY_SPEC>;
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
impl From<crate::W<CH_DUTY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_DUTY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUTY` reader - reg_duty_lsch0."]
pub type DUTY_R = crate::FieldReader<u32>;
#[doc = "Field `DUTY` writer - reg_duty_lsch0."]
pub type DUTY_W<'a, const O: u8> = crate::FieldWriter<'a, CH_DUTY_SPEC, 19, O, u32>;
impl R {
    #[doc = "Bits 0:18 - reg_duty_lsch0."]
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(self.bits & 0x0007_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_DUTY")
            .field("duty", &format_args!("{}", self.duty().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_DUTY_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:18 - reg_duty_lsch0."]
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
#[doc = "LEDC_LSCH%s_DUTY.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_duty](index.html) module"]
pub struct CH_DUTY_SPEC;
impl crate::RegisterSpec for CH_DUTY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_duty::R](R) reader structure"]
impl crate::Readable for CH_DUTY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_duty::W](W) writer structure"]
impl crate::Writable for CH_DUTY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH%s_DUTY to value 0"]
impl crate::Resettable for CH_DUTY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
