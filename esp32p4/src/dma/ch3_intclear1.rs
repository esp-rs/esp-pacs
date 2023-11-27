#[doc = "Register `CH3_INTCLEAR1` writer"]
pub type W = crate::W<CH3_INTCLEAR1_SPEC>;
#[doc = "Field `CH3_CLEAR_ECC_PROT_CHMEM_CORRERR_INTSTAT` writer - NA"]
pub type CH3_CLEAR_ECC_PROT_CHMEM_CORRERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_CLEAR_ECC_PROT_CHMEM_UNCORRERR_INTSTAT` writer - NA"]
pub type CH3_CLEAR_ECC_PROT_CHMEM_UNCORRERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_CLEAR_ECC_PROT_UIDMEM_CORRERR_INTSTAT` writer - NA"]
pub type CH3_CLEAR_ECC_PROT_UIDMEM_CORRERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_CLEAR_ECC_PROT_UIDMEM_UNCORRERR_INTSTAT` writer - NA"]
pub type CH3_CLEAR_ECC_PROT_UIDMEM_UNCORRERR_INTSTAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH3_INTCLEAR1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_clear_ecc_prot_chmem_correrr_intstat(
        &mut self,
    ) -> CH3_CLEAR_ECC_PROT_CHMEM_CORRERR_INTSTAT_W<CH3_INTCLEAR1_SPEC> {
        CH3_CLEAR_ECC_PROT_CHMEM_CORRERR_INTSTAT_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_clear_ecc_prot_chmem_uncorrerr_intstat(
        &mut self,
    ) -> CH3_CLEAR_ECC_PROT_CHMEM_UNCORRERR_INTSTAT_W<CH3_INTCLEAR1_SPEC> {
        CH3_CLEAR_ECC_PROT_CHMEM_UNCORRERR_INTSTAT_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_clear_ecc_prot_uidmem_correrr_intstat(
        &mut self,
    ) -> CH3_CLEAR_ECC_PROT_UIDMEM_CORRERR_INTSTAT_W<CH3_INTCLEAR1_SPEC> {
        CH3_CLEAR_ECC_PROT_UIDMEM_CORRERR_INTSTAT_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch3_clear_ecc_prot_uidmem_uncorrerr_intstat(
        &mut self,
    ) -> CH3_CLEAR_ECC_PROT_UIDMEM_UNCORRERR_INTSTAT_W<CH3_INTCLEAR1_SPEC> {
        CH3_CLEAR_ECC_PROT_UIDMEM_UNCORRERR_INTSTAT_W::new(self, 3)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "NA\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3_intclear1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH3_INTCLEAR1_SPEC;
impl crate::RegisterSpec for CH3_INTCLEAR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ch3_intclear1::W`](W) writer structure"]
impl crate::Writable for CH3_INTCLEAR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH3_INTCLEAR1 to value 0"]
impl crate::Resettable for CH3_INTCLEAR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
