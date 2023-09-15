#[doc = "Register `DMATXCURRADDR_BUF` reader"]
pub type R = crate::R<DMATXCURRADDR_BUF_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMATXCURRADDR_BUF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "The address of the current receive descriptor list. Cleared on Reset.Pointer updated by the DMA during operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatxcurraddr_buf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMATXCURRADDR_BUF_SPEC;
impl crate::RegisterSpec for DMATXCURRADDR_BUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatxcurraddr_buf::R`](R) reader structure"]
impl crate::Readable for DMATXCURRADDR_BUF_SPEC {}
#[doc = "`reset()` method sets DMATXCURRADDR_BUF to value 0"]
impl crate::Resettable for DMATXCURRADDR_BUF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
