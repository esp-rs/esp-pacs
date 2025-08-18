#[doc = "Register `INTCLEAR1` writer"]
pub type W = crate::W<INTCLEAR1_SPEC>;
#[doc = "Field `CH1_CLEAR_ECC_PROT_CHMEM_CORRERR_INTSTAT` writer - NA"]
pub type CH1_CLEAR_ECC_PROT_CHMEM_CORRERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_ECC_PROT_CHMEM_UNCORRERR_INTSTAT` writer - NA"]
pub type CH1_CLEAR_ECC_PROT_CHMEM_UNCORRERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_ECC_PROT_UIDMEM_CORRERR_INTSTAT` writer - NA"]
pub type CH1_CLEAR_ECC_PROT_UIDMEM_CORRERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CLEAR_ECC_PROT_UIDMEM_UNCORRERR_INTSTAT` writer - NA"]
pub type CH1_CLEAR_ECC_PROT_UIDMEM_UNCORRERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTCLEAR1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ch1_clear_ecc_prot_chmem_correrr_intstat(
        &mut self,
    ) -> CH1_CLEAR_ECC_PROT_CHMEM_CORRERR_INTSTAT_W<'_, INTCLEAR1_SPEC> {
        CH1_CLEAR_ECC_PROT_CHMEM_CORRERR_INTSTAT_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn ch1_clear_ecc_prot_chmem_uncorrerr_intstat(
        &mut self,
    ) -> CH1_CLEAR_ECC_PROT_CHMEM_UNCORRERR_INTSTAT_W<'_, INTCLEAR1_SPEC> {
        CH1_CLEAR_ECC_PROT_CHMEM_UNCORRERR_INTSTAT_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn ch1_clear_ecc_prot_uidmem_correrr_intstat(
        &mut self,
    ) -> CH1_CLEAR_ECC_PROT_UIDMEM_CORRERR_INTSTAT_W<'_, INTCLEAR1_SPEC> {
        CH1_CLEAR_ECC_PROT_UIDMEM_CORRERR_INTSTAT_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn ch1_clear_ecc_prot_uidmem_uncorrerr_intstat(
        &mut self,
    ) -> CH1_CLEAR_ECC_PROT_UIDMEM_UNCORRERR_INTSTAT_W<'_, INTCLEAR1_SPEC> {
        CH1_CLEAR_ECC_PROT_UIDMEM_UNCORRERR_INTSTAT_W::new(self, 3)
    }
}
#[doc = "NA\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclear1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTCLEAR1_SPEC;
impl crate::RegisterSpec for INTCLEAR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intclear1::W`](W) writer structure"]
impl crate::Writable for INTCLEAR1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTCLEAR1 to value 0"]
impl crate::Resettable for INTCLEAR1_SPEC {}
