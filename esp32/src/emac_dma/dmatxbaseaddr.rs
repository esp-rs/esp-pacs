#[doc = "Register `DMATXBASEADDR` reader"]
pub type R = crate::R<DMATXBASEADDR_SPEC>;
#[doc = "Register `DMATXBASEADDR` writer"]
pub type W = crate::W<DMATXBASEADDR_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMATXBASEADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This field contains the base address of the first descriptor in the Transmit Descriptor list. The LSB Bits\\[1:0\\] are ignored and are internally taken as all-zero by the DMA.Therefore these LSB bits are read-only.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatxbaseaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatxbaseaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMATXBASEADDR_SPEC;
impl crate::RegisterSpec for DMATXBASEADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatxbaseaddr::R`](R) reader structure"]
impl crate::Readable for DMATXBASEADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmatxbaseaddr::W`](W) writer structure"]
impl crate::Writable for DMATXBASEADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMATXBASEADDR to value 0"]
impl crate::Resettable for DMATXBASEADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
