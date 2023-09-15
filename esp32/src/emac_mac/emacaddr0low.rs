#[doc = "Register `EMACADDR0LOW` reader"]
pub type R = crate::R<EMACADDR0LOW_SPEC>;
#[doc = "Register `EMACADDR0LOW` writer"]
pub type W = crate::W<EMACADDR0LOW_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EMACADDR0LOW_SPEC> {
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
#[doc = "This field contains the lower 32 bits of the first 6-byte MAC address. This is used by the MAC for filtering the received frames and inserting the MAC address in the Transmit Flow Control (Pause) Frames.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emacaddr0low::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr0low::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMACADDR0LOW_SPEC;
impl crate::RegisterSpec for EMACADDR0LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emacaddr0low::R`](R) reader structure"]
impl crate::Readable for EMACADDR0LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emacaddr0low::W`](W) writer structure"]
impl crate::Writable for EMACADDR0LOW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMACADDR0LOW to value 0"]
impl crate::Resettable for EMACADDR0LOW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
