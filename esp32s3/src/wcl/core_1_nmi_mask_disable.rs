///Register `Core_1_NMI_MASK_DISABLE` writer
pub type W = crate::W<CORE_1_NMI_MASK_DISABLE_SPEC>;
///Field `CORE_1_NMI_MASK_DISABLE` writer - this field is used to disable NMI mask, it will not take effect immediately,only when the CPU executes to the trigger address will it start to cancel NMI mask
pub type CORE_1_NMI_MASK_DISABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_NMI_MASK_DISABLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:31 - this field is used to disable NMI mask, it will not take effect immediately,only when the CPU executes to the trigger address will it start to cancel NMI mask
    #[inline(always)]
    #[must_use]
    pub fn core_1_nmi_mask_disable(
        &mut self,
    ) -> CORE_1_NMI_MASK_DISABLE_W<CORE_1_NMI_MASK_DISABLE_SPEC> {
        CORE_1_NMI_MASK_DISABLE_W::new(self, 0)
    }
}
/**Core_1 NMI mask disable register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_nmi_mask_disable::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_1_NMI_MASK_DISABLE_SPEC;
impl crate::RegisterSpec for CORE_1_NMI_MASK_DISABLE_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`core_1_nmi_mask_disable::W`](W) writer structure
impl crate::Writable for CORE_1_NMI_MASK_DISABLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets Core_1_NMI_MASK_DISABLE to value 0
impl crate::Resettable for CORE_1_NMI_MASK_DISABLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
