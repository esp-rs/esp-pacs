#[doc = "Register `INTSTATUS_ENABLE1` reader"]
pub type R = crate::R<INTSTATUS_ENABLE1_SPEC>;
#[doc = "Field `CH1_ENABLE_ECC_PROT_CHMEM_CORRERR_INTSTAT` reader - NA"]
pub type CH1_ENABLE_ECC_PROT_CHMEM_CORRERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_ECC_PROT_CHMEM_UNCORRERR_INTSTAT` reader - NA"]
pub type CH1_ENABLE_ECC_PROT_CHMEM_UNCORRERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_ECC_PROT_UIDMEM_CORRERR_INTSTAT` reader - NA"]
pub type CH1_ENABLE_ECC_PROT_UIDMEM_CORRERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ENABLE_ECC_PROT_UIDMEM_UNCORRERR_INTSTAT` reader - NA"]
pub type CH1_ENABLE_ECC_PROT_UIDMEM_UNCORRERR_INTSTAT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ch1_enable_ecc_prot_chmem_correrr_intstat(
        &self,
    ) -> CH1_ENABLE_ECC_PROT_CHMEM_CORRERR_INTSTAT_R {
        CH1_ENABLE_ECC_PROT_CHMEM_CORRERR_INTSTAT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn ch1_enable_ecc_prot_chmem_uncorrerr_intstat(
        &self,
    ) -> CH1_ENABLE_ECC_PROT_CHMEM_UNCORRERR_INTSTAT_R {
        CH1_ENABLE_ECC_PROT_CHMEM_UNCORRERR_INTSTAT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn ch1_enable_ecc_prot_uidmem_correrr_intstat(
        &self,
    ) -> CH1_ENABLE_ECC_PROT_UIDMEM_CORRERR_INTSTAT_R {
        CH1_ENABLE_ECC_PROT_UIDMEM_CORRERR_INTSTAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn ch1_enable_ecc_prot_uidmem_uncorrerr_intstat(
        &self,
    ) -> CH1_ENABLE_ECC_PROT_UIDMEM_UNCORRERR_INTSTAT_R {
        CH1_ENABLE_ECC_PROT_UIDMEM_UNCORRERR_INTSTAT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTATUS_ENABLE1")
            .field(
                "ch1_enable_ecc_prot_chmem_correrr_intstat",
                &self.ch1_enable_ecc_prot_chmem_correrr_intstat().bit(),
            )
            .field(
                "ch1_enable_ecc_prot_chmem_uncorrerr_intstat",
                &self.ch1_enable_ecc_prot_chmem_uncorrerr_intstat().bit(),
            )
            .field(
                "ch1_enable_ecc_prot_uidmem_correrr_intstat",
                &self.ch1_enable_ecc_prot_uidmem_correrr_intstat().bit(),
            )
            .field(
                "ch1_enable_ecc_prot_uidmem_uncorrerr_intstat",
                &self.ch1_enable_ecc_prot_uidmem_uncorrerr_intstat().bit(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTSTATUS_ENABLE1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intstatus_enable1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTSTATUS_ENABLE1_SPEC;
impl crate::RegisterSpec for INTSTATUS_ENABLE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstatus_enable1::R`](R) reader structure"]
impl crate::Readable for INTSTATUS_ENABLE1_SPEC {}
#[doc = "`reset()` method sets INTSTATUS_ENABLE1 to value 0x0f"]
impl crate::Resettable for INTSTATUS_ENABLE1_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
