#[doc = "Register `Core_1_NMI_MASK_DISABLE` writer"]
pub struct W(crate::W<CORE_1_NMI_MASK_DISABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_NMI_MASK_DISABLE_SPEC>;
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
impl From<crate::W<CORE_1_NMI_MASK_DISABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_NMI_MASK_DISABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_NMI_MASK_DISABLE` writer - this field is used to disable NMI mask, it will not take effect immediately,only when the CPU executes to the trigger address will it start to cancel NMI mask"]
pub type CORE_1_NMI_MASK_DISABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_NMI_MASK_DISABLE_SPEC, 32, O, u32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_NMI_MASK_DISABLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - this field is used to disable NMI mask, it will not take effect immediately,only when the CPU executes to the trigger address will it start to cancel NMI mask"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_nmi_mask_disable(&mut self) -> CORE_1_NMI_MASK_DISABLE_W<0> {
        CORE_1_NMI_MASK_DISABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core_1 NMI mask disable register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_nmi_mask_disable](index.html) module"]
pub struct CORE_1_NMI_MASK_DISABLE_SPEC;
impl crate::RegisterSpec for CORE_1_NMI_MASK_DISABLE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [core_1_nmi_mask_disable::W](W) writer structure"]
impl crate::Writable for CORE_1_NMI_MASK_DISABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Core_1_NMI_MASK_DISABLE to value 0"]
impl crate::Resettable for CORE_1_NMI_MASK_DISABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
