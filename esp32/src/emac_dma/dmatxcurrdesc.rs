#[doc = "Register `DMATXCURRDESC` reader"]
pub type R = crate::R<DMATXCURRDESC_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMATXCURRDESC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatxcurrdesc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMATXCURRDESC_SPEC;
impl crate::RegisterSpec for DMATXCURRDESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatxcurrdesc::R`](R) reader structure"]
impl crate::Readable for DMATXCURRDESC_SPEC {}
#[doc = "`reset()` method sets DMATXCURRDESC to value 0"]
impl crate::Resettable for DMATXCURRDESC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
