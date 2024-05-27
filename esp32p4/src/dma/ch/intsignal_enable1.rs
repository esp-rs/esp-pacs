///Register `INTSIGNAL_ENABLE1` reader
pub type R = crate::R<INTSIGNAL_ENABLE1_SPEC>;
///Field `CH1_ENABLE_ECC_PROT_CHMEM_CORRERR_INTSIGNAL` reader - NA
pub type CH1_ENABLE_ECC_PROT_CHMEM_CORRERR_INTSIGNAL_R = crate::BitReader;
///Field `CH1_ENABLE_ECC_PROT_CHMEM_UNCORRERR_INTSIGNAL` reader - NA
pub type CH1_ENABLE_ECC_PROT_CHMEM_UNCORRERR_INTSIGNAL_R = crate::BitReader;
///Field `CH1_ENABLE_ECC_PROT_UIDMEM_CORRERR_INTSIGNAL` reader - NA
pub type CH1_ENABLE_ECC_PROT_UIDMEM_CORRERR_INTSIGNAL_R = crate::BitReader;
///Field `CH1_ENABLE_ECC_PROT_UIDMEM_UNCORRERR_INTSIGNAL` reader - NA
pub type CH1_ENABLE_ECC_PROT_UIDMEM_UNCORRERR_INTSIGNAL_R = crate::BitReader;
impl R {
    ///Bit 0 - NA
    #[inline(always)]
    pub fn ch1_enable_ecc_prot_chmem_correrr_intsignal(
        &self,
    ) -> CH1_ENABLE_ECC_PROT_CHMEM_CORRERR_INTSIGNAL_R {
        CH1_ENABLE_ECC_PROT_CHMEM_CORRERR_INTSIGNAL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - NA
    #[inline(always)]
    pub fn ch1_enable_ecc_prot_chmem_uncorrerr_intsignal(
        &self,
    ) -> CH1_ENABLE_ECC_PROT_CHMEM_UNCORRERR_INTSIGNAL_R {
        CH1_ENABLE_ECC_PROT_CHMEM_UNCORRERR_INTSIGNAL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - NA
    #[inline(always)]
    pub fn ch1_enable_ecc_prot_uidmem_correrr_intsignal(
        &self,
    ) -> CH1_ENABLE_ECC_PROT_UIDMEM_CORRERR_INTSIGNAL_R {
        CH1_ENABLE_ECC_PROT_UIDMEM_CORRERR_INTSIGNAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - NA
    #[inline(always)]
    pub fn ch1_enable_ecc_prot_uidmem_uncorrerr_intsignal(
        &self,
    ) -> CH1_ENABLE_ECC_PROT_UIDMEM_UNCORRERR_INTSIGNAL_R {
        CH1_ENABLE_ECC_PROT_UIDMEM_UNCORRERR_INTSIGNAL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSIGNAL_ENABLE1")
            .field(
                "ch1_enable_ecc_prot_chmem_correrr_intsignal",
                &self.ch1_enable_ecc_prot_chmem_correrr_intsignal(),
            )
            .field(
                "ch1_enable_ecc_prot_chmem_uncorrerr_intsignal",
                &self.ch1_enable_ecc_prot_chmem_uncorrerr_intsignal(),
            )
            .field(
                "ch1_enable_ecc_prot_uidmem_correrr_intsignal",
                &self.ch1_enable_ecc_prot_uidmem_correrr_intsignal(),
            )
            .field(
                "ch1_enable_ecc_prot_uidmem_uncorrerr_intsignal",
                &self.ch1_enable_ecc_prot_uidmem_uncorrerr_intsignal(),
            )
            .finish()
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`intsignal_enable1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INTSIGNAL_ENABLE1_SPEC;
impl crate::RegisterSpec for INTSIGNAL_ENABLE1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`intsignal_enable1::R`](R) reader structure
impl crate::Readable for INTSIGNAL_ENABLE1_SPEC {}
///`reset()` method sets INTSIGNAL_ENABLE1 to value 0x0f
impl crate::Resettable for INTSIGNAL_ENABLE1_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
