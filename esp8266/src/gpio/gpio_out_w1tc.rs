#[doc = "Register `GPIO_OUT_W1TC` writer"]
pub struct W(crate::W<GPIO_OUT_W1TC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_OUT_W1TC_SPEC>;
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
impl From<crate::W<GPIO_OUT_W1TC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_OUT_W1TC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_OUT_DATA_W1TC` writer - Writing 1 into a bit in this register will clear the related bit in GPIO_OUT_DATA"]
pub type GPIO_OUT_DATA_W1TC_W<'a, const O: u8> =
    crate::FieldWriter<'a, GPIO_OUT_W1TC_SPEC, 16, O, u16>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO_OUT_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - Writing 1 into a bit in this register will clear the related bit in GPIO_OUT_DATA"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_out_data_w1tc(&mut self) -> GPIO_OUT_DATA_W1TC_W<0> {
        GPIO_OUT_DATA_W1TC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO_OUT_W1TC\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_out_w1tc](index.html) module"]
pub struct GPIO_OUT_W1TC_SPEC;
impl crate::RegisterSpec for GPIO_OUT_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gpio_out_w1tc::W](W) writer structure"]
impl crate::Writable for GPIO_OUT_W1TC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_OUT_W1TC to value 0"]
impl crate::Resettable for GPIO_OUT_W1TC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
