#[doc = "Register `INTSTATUS1` reader"]
pub type R = crate::R<INTSTATUS1_SPEC>;
#[doc = "Field `CH1_ECC_PROT_CHMEM_CORRERR_INTSTAT` reader - NA"]
pub type CH1_ECC_PROT_CHMEM_CORRERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ECC_PROT_CHMEM_UNCORRERR_INTSTAT` reader - NA"]
pub type CH1_ECC_PROT_CHMEM_UNCORRERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ECC_PROT_UIDMEM_CORRERR_INTSTAT` reader - NA"]
pub type CH1_ECC_PROT_UIDMEM_CORRERR_INTSTAT_R = crate::BitReader;
#[doc = "Field `CH1_ECC_PROT_UIDMEM_UNCORRERR_INTSTAT` reader - NA"]
pub type CH1_ECC_PROT_UIDMEM_UNCORRERR_INTSTAT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ch1_ecc_prot_chmem_correrr_intstat(&self) -> CH1_ECC_PROT_CHMEM_CORRERR_INTSTAT_R {
        CH1_ECC_PROT_CHMEM_CORRERR_INTSTAT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn ch1_ecc_prot_chmem_uncorrerr_intstat(&self) -> CH1_ECC_PROT_CHMEM_UNCORRERR_INTSTAT_R {
        CH1_ECC_PROT_CHMEM_UNCORRERR_INTSTAT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn ch1_ecc_prot_uidmem_correrr_intstat(&self) -> CH1_ECC_PROT_UIDMEM_CORRERR_INTSTAT_R {
        CH1_ECC_PROT_UIDMEM_CORRERR_INTSTAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn ch1_ecc_prot_uidmem_uncorrerr_intstat(&self) -> CH1_ECC_PROT_UIDMEM_UNCORRERR_INTSTAT_R {
        CH1_ECC_PROT_UIDMEM_UNCORRERR_INTSTAT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTATUS1")
            .field(
                "ch1_ecc_prot_chmem_correrr_intstat",
                &self.ch1_ecc_prot_chmem_correrr_intstat(),
            )
            .field(
                "ch1_ecc_prot_chmem_uncorrerr_intstat",
                &self.ch1_ecc_prot_chmem_uncorrerr_intstat(),
            )
            .field(
                "ch1_ecc_prot_uidmem_correrr_intstat",
                &self.ch1_ecc_prot_uidmem_correrr_intstat(),
            )
            .field(
                "ch1_ecc_prot_uidmem_uncorrerr_intstat",
                &self.ch1_ecc_prot_uidmem_uncorrerr_intstat(),
            )
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatus1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTSTATUS1_SPEC;
impl crate::RegisterSpec for INTSTATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstatus1::R`](R) reader structure"]
impl crate::Readable for INTSTATUS1_SPEC {}
#[doc = "`reset()` method sets INTSTATUS1 to value 0"]
impl crate::Resettable for INTSTATUS1_SPEC {
    const RESET_VALUE: u32 = 0;
}
