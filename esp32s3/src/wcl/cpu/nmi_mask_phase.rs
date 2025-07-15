#[doc = "Register `NMI_MASK_PHASE` reader"]
pub type R = crate::R<NMI_MASK_PHASE_SPEC>;
#[doc = "Field `NMI_MASK_PHASE` reader - this bit is used to indicates whether the NMI interrupt is being masked, 1 means NMI interrupt is being masked"]
pub type NMI_MASK_PHASE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - this bit is used to indicates whether the NMI interrupt is being masked, 1 means NMI interrupt is being masked"]
    #[inline(always)]
    pub fn nmi_mask_phase(&self) -> NMI_MASK_PHASE_R {
        NMI_MASK_PHASE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NMI_MASK_PHASE")
            .field("nmi_mask_phase", &self.nmi_mask_phase())
            .finish()
    }
}
#[doc = "Core_0 NMI mask phase register\n\nYou can [`read`](crate::Reg::read) this register and get [`nmi_mask_phase::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NMI_MASK_PHASE_SPEC;
impl crate::RegisterSpec for NMI_MASK_PHASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nmi_mask_phase::R`](R) reader structure"]
impl crate::Readable for NMI_MASK_PHASE_SPEC {}
#[doc = "`reset()` method sets NMI_MASK_PHASE to value 0"]
impl crate::Resettable for NMI_MASK_PHASE_SPEC {}
