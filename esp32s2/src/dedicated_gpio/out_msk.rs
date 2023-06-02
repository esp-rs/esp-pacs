#[doc = "Register `OUT_MSK` writer"]
pub struct W(crate::W<OUT_MSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_MSK_SPEC>;
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
impl From<crate::W<OUT_MSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_MSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT_VALUE` writer - This register is used to configure updated output value of 8-channel dedicated GPIO."]
pub type OUT_VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, OUT_MSK_SPEC, 8, O>;
#[doc = "Field `OUT_MSK` writer - This register is used to configure channels which would be updated. 1: corresponding channel's output would be updated."]
pub type OUT_MSK_W<'a, const O: u8> = crate::FieldWriter<'a, OUT_MSK_SPEC, 8, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_MSK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is used to configure updated output value of 8-channel dedicated GPIO."]
    #[inline(always)]
    #[must_use]
    pub fn out_value(&mut self) -> OUT_VALUE_W<0> {
        OUT_VALUE_W::new(self)
    }
    #[doc = "Bits 8:15 - This register is used to configure channels which would be updated. 1: corresponding channel's output would be updated."]
    #[inline(always)]
    #[must_use]
    pub fn out_msk(&mut self) -> OUT_MSK_W<8> {
        OUT_MSK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dedicated GPIO mask output register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_msk](index.html) module"]
pub struct OUT_MSK_SPEC;
impl crate::RegisterSpec for OUT_MSK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [out_msk::W](W) writer structure"]
impl crate::Writable for OUT_MSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_MSK to value 0"]
impl crate::Resettable for OUT_MSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
