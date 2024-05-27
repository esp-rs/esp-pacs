///Register `Core_1_NMI_MASK_TRIGGER_ADDR` reader
pub type R = crate::R<CORE_1_NMI_MASK_TRIGGER_ADDR_SPEC>;
///Register `Core_1_NMI_MASK_TRIGGER_ADDR` writer
pub type W = crate::W<CORE_1_NMI_MASK_TRIGGER_ADDR_SPEC>;
///Field `CORE_1_NMI_MASK_TRIGGER_ADDR` reader - this field to used to set trigger address
pub type CORE_1_NMI_MASK_TRIGGER_ADDR_R = crate::FieldReader<u32>;
///Field `CORE_1_NMI_MASK_TRIGGER_ADDR` writer - this field to used to set trigger address
pub type CORE_1_NMI_MASK_TRIGGER_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - this field to used to set trigger address
    #[inline(always)]
    pub fn core_1_nmi_mask_trigger_addr(&self) -> CORE_1_NMI_MASK_TRIGGER_ADDR_R {
        CORE_1_NMI_MASK_TRIGGER_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_1_NMI_MASK_TRIGGER_ADDR")
            .field(
                "core_1_nmi_mask_trigger_addr",
                &self.core_1_nmi_mask_trigger_addr(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:31 - this field to used to set trigger address
    #[inline(always)]
    #[must_use]
    pub fn core_1_nmi_mask_trigger_addr(
        &mut self,
    ) -> CORE_1_NMI_MASK_TRIGGER_ADDR_W<CORE_1_NMI_MASK_TRIGGER_ADDR_SPEC> {
        CORE_1_NMI_MASK_TRIGGER_ADDR_W::new(self, 0)
    }
}
/**Core_1 NMI mask trigger addr register

You can [`read`](crate::generic::Reg::read) this register and get [`core_1_nmi_mask_trigger_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_nmi_mask_trigger_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_1_NMI_MASK_TRIGGER_ADDR_SPEC;
impl crate::RegisterSpec for CORE_1_NMI_MASK_TRIGGER_ADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_1_nmi_mask_trigger_addr::R`](R) reader structure
impl crate::Readable for CORE_1_NMI_MASK_TRIGGER_ADDR_SPEC {}
///`write(|w| ..)` method takes [`core_1_nmi_mask_trigger_addr::W`](W) writer structure
impl crate::Writable for CORE_1_NMI_MASK_TRIGGER_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets Core_1_NMI_MASK_TRIGGER_ADDR to value 0
impl crate::Resettable for CORE_1_NMI_MASK_TRIGGER_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
