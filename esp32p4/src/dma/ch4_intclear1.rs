#[doc = "Register `CH4_INTCLEAR1` writer"]
pub type W = crate::W<CH4_INTCLEAR1_SPEC>;
#[doc = "Field `CH4_CLEAR_ECC_PROT_CHMEM_CORRERR_INTSTAT` writer - NA"]
pub type CH4_CLEAR_ECC_PROT_CHMEM_CORRERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_ECC_PROT_CHMEM_UNCORRERR_INTSTAT` writer - NA"]
pub type CH4_CLEAR_ECC_PROT_CHMEM_UNCORRERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_ECC_PROT_UIDMEM_CORRERR_INTSTAT` writer - NA"]
pub type CH4_CLEAR_ECC_PROT_UIDMEM_CORRERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_CLEAR_ECC_PROT_UIDMEM_UNCORRERR_INTSTAT` writer - NA"]
pub type CH4_CLEAR_ECC_PROT_UIDMEM_UNCORRERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH4_INTCLEAR1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_ecc_prot_chmem_correrr_intstat(
        &mut self,
    ) -> CH4_CLEAR_ECC_PROT_CHMEM_CORRERR_INTSTAT_W<CH4_INTCLEAR1_SPEC> {
        CH4_CLEAR_ECC_PROT_CHMEM_CORRERR_INTSTAT_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_ecc_prot_chmem_uncorrerr_intstat(
        &mut self,
    ) -> CH4_CLEAR_ECC_PROT_CHMEM_UNCORRERR_INTSTAT_W<CH4_INTCLEAR1_SPEC> {
        CH4_CLEAR_ECC_PROT_CHMEM_UNCORRERR_INTSTAT_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_ecc_prot_uidmem_correrr_intstat(
        &mut self,
    ) -> CH4_CLEAR_ECC_PROT_UIDMEM_CORRERR_INTSTAT_W<CH4_INTCLEAR1_SPEC> {
        CH4_CLEAR_ECC_PROT_UIDMEM_CORRERR_INTSTAT_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch4_clear_ecc_prot_uidmem_uncorrerr_intstat(
        &mut self,
    ) -> CH4_CLEAR_ECC_PROT_UIDMEM_UNCORRERR_INTSTAT_W<CH4_INTCLEAR1_SPEC> {
        CH4_CLEAR_ECC_PROT_UIDMEM_UNCORRERR_INTSTAT_W::new(self, 3)
    }
}
#[doc = "NA\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch4_intclear1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH4_INTCLEAR1_SPEC;
impl crate::RegisterSpec for CH4_INTCLEAR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ch4_intclear1::W`](W) writer structure"]
impl crate::Writable for CH4_INTCLEAR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH4_INTCLEAR1 to value 0"]
impl crate::Resettable for CH4_INTCLEAR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
