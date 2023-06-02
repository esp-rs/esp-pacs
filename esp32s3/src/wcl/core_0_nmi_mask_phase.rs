#[doc = "Register `Core_0_NMI_MASK_PHASE` reader"]
pub struct R(crate::R<CORE_0_NMI_MASK_PHASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_NMI_MASK_PHASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_NMI_MASK_PHASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_NMI_MASK_PHASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_NMI_MASK_PHASE` reader - this bit is used to indicates whether the NMI interrupt is being masked, 1 means NMI interrupt is being masked"]
pub type CORE_0_NMI_MASK_PHASE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - this bit is used to indicates whether the NMI interrupt is being masked, 1 means NMI interrupt is being masked"]
    #[inline(always)]
    pub fn core_0_nmi_mask_phase(&self) -> CORE_0_NMI_MASK_PHASE_R {
        CORE_0_NMI_MASK_PHASE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_0_NMI_MASK_PHASE")
            .field(
                "core_0_nmi_mask_phase",
                &format_args!("{}", self.core_0_nmi_mask_phase().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_NMI_MASK_PHASE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Core_0 NMI mask phase register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_nmi_mask_phase](index.html) module"]
pub struct CORE_0_NMI_MASK_PHASE_SPEC;
impl crate::RegisterSpec for CORE_0_NMI_MASK_PHASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_nmi_mask_phase::R](R) reader structure"]
impl crate::Readable for CORE_0_NMI_MASK_PHASE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets Core_0_NMI_MASK_PHASE to value 0"]
impl crate::Resettable for CORE_0_NMI_MASK_PHASE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
