///Register `Core_0_NMI_MASK_PHASE` reader
pub type R = crate::R<CORE_0_NMI_MASK_PHASE_SPEC>;
///Field `CORE_0_NMI_MASK_PHASE` reader - this bit is used to indicates whether the NMI interrupt is being masked, 1 means NMI interrupt is being masked
pub type CORE_0_NMI_MASK_PHASE_R = crate::BitReader;
impl R {
    ///Bit 0 - this bit is used to indicates whether the NMI interrupt is being masked, 1 means NMI interrupt is being masked
    #[inline(always)]
    pub fn core_0_nmi_mask_phase(&self) -> CORE_0_NMI_MASK_PHASE_R {
        CORE_0_NMI_MASK_PHASE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_0_NMI_MASK_PHASE")
            .field("core_0_nmi_mask_phase", &self.core_0_nmi_mask_phase())
            .finish()
    }
}
/**Core_0 NMI mask phase register

You can [`read`](crate::generic::Reg::read) this register and get [`core_0_nmi_mask_phase::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CORE_0_NMI_MASK_PHASE_SPEC;
impl crate::RegisterSpec for CORE_0_NMI_MASK_PHASE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`core_0_nmi_mask_phase::R`](R) reader structure
impl crate::Readable for CORE_0_NMI_MASK_PHASE_SPEC {}
///`reset()` method sets Core_0_NMI_MASK_PHASE to value 0
impl crate::Resettable for CORE_0_NMI_MASK_PHASE_SPEC {
    const RESET_VALUE: u32 = 0;
}
