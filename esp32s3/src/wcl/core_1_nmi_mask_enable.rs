#[doc = "Register `Core_1_NMI_MASK_ENABLE` writer"]
pub type W = crate::W<CORE_1_NMI_MASK_ENABLE_SPEC>;
#[doc = "Field `CORE_1_NMI_MASK_ENABLE` writer - this field is used to set NMI mask, it can write any value, when write this register,the hardware start masking NMI interrupt"]
pub type CORE_1_NMI_MASK_ENABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_NMI_MASK_ENABLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - this field is used to set NMI mask, it can write any value, when write this register,the hardware start masking NMI interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_nmi_mask_enable(
        &mut self,
    ) -> CORE_1_NMI_MASK_ENABLE_W<CORE_1_NMI_MASK_ENABLE_SPEC> {
        CORE_1_NMI_MASK_ENABLE_W::new(self, 0)
    }
}
#[doc = "Core_1 NMI mask enable register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_nmi_mask_enable::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_NMI_MASK_ENABLE_SPEC;
impl crate::RegisterSpec for CORE_1_NMI_MASK_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`core_1_nmi_mask_enable::W`](W) writer structure"]
impl crate::Writable for CORE_1_NMI_MASK_ENABLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Core_1_NMI_MASK_ENABLE to value 0"]
impl crate::Resettable for CORE_1_NMI_MASK_ENABLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
