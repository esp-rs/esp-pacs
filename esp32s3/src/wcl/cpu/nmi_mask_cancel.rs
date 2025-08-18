#[doc = "Register `NMI_MASK_CANCEL` writer"]
pub type W = crate::W<NMI_MASK_CANCEL_SPEC>;
#[doc = "Field `NMI_MASK_CANCEL` writer - this field is used to cancel NMI mask disable function."]
pub type NMI_MASK_CANCEL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<NMI_MASK_CANCEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - this field is used to cancel NMI mask disable function."]
    #[inline(always)]
    pub fn nmi_mask_cancel(&mut self) -> NMI_MASK_CANCEL_W<'_, NMI_MASK_CANCEL_SPEC> {
        NMI_MASK_CANCEL_W::new(self, 0)
    }
}
#[doc = "Core_0 NMI mask disable register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmi_mask_cancel::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NMI_MASK_CANCEL_SPEC;
impl crate::RegisterSpec for NMI_MASK_CANCEL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`nmi_mask_cancel::W`](W) writer structure"]
impl crate::Writable for NMI_MASK_CANCEL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NMI_MASK_CANCEL to value 0"]
impl crate::Resettable for NMI_MASK_CANCEL_SPEC {}
