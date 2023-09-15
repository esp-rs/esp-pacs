#[doc = "Register `DMARXCURRADDR_BUF` reader"]
pub type R = crate::R<DMARXCURRADDR_BUF_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMARXCURRADDR_BUF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmarxcurraddr_buf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMARXCURRADDR_BUF_SPEC;
impl crate::RegisterSpec for DMARXCURRADDR_BUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmarxcurraddr_buf::R`](R) reader structure"]
impl crate::Readable for DMARXCURRADDR_BUF_SPEC {}
#[doc = "`reset()` method sets DMARXCURRADDR_BUF to value 0"]
impl crate::Resettable for DMARXCURRADDR_BUF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
