#[doc = "Register `DMARXCURRDESC` reader"]
pub type R = crate::R<DMARXCURRDESC_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMARXCURRDESC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmarxcurrdesc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMARXCURRDESC_SPEC;
impl crate::RegisterSpec for DMARXCURRDESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmarxcurrdesc::R`](R) reader structure"]
impl crate::Readable for DMARXCURRDESC_SPEC {}
#[doc = "`reset()` method sets DMARXCURRDESC to value 0"]
impl crate::Resettable for DMARXCURRDESC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
