#[doc = "Register `OUT_ENABLE_W1TC` writer"]
pub struct W(crate::W<OUT_ENABLE_W1TC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_ENABLE_W1TC_SPEC>;
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
impl From<crate::W<OUT_ENABLE_W1TC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_ENABLE_W1TC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_GPIO_ENABLE_W1TC` writer - clear one time output data"]
pub type LP_GPIO_ENABLE_W1TC_W<'a, const O: u8> =
    crate::FieldWriter<'a, OUT_ENABLE_W1TC_SPEC, 8, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_ENABLE_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - clear one time output data"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio_enable_w1tc(&mut self) -> LP_GPIO_ENABLE_W1TC_W<0> {
        LP_GPIO_ENABLE_W1TC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need des\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_enable_w1tc](index.html) module"]
pub struct OUT_ENABLE_W1TC_SPEC;
impl crate::RegisterSpec for OUT_ENABLE_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [out_enable_w1tc::W](W) writer structure"]
impl crate::Writable for OUT_ENABLE_W1TC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_ENABLE_W1TC to value 0"]
impl crate::Resettable for OUT_ENABLE_W1TC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
