#[doc = "Register `DMARXBASEADDR` reader"]
pub type R = crate::R<DMARXBASEADDR_SPEC>;
#[doc = "Register `DMARXBASEADDR` writer"]
pub type W = crate::W<DMARXBASEADDR_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMARXBASEADDR_SPEC> {
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
#[doc = "This field contains the base address of the first descriptor in the Receive Descriptor list. The LSB Bits\\[1:0\\] are ignored and internally taken as all-zero by the DMA. Therefore these LSB bits are read-only.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmarxbaseaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmarxbaseaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMARXBASEADDR_SPEC;
impl crate::RegisterSpec for DMARXBASEADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmarxbaseaddr::R`](R) reader structure"]
impl crate::Readable for DMARXBASEADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmarxbaseaddr::W`](W) writer structure"]
impl crate::Writable for DMARXBASEADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMARXBASEADDR to value 0"]
impl crate::Resettable for DMARXBASEADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
