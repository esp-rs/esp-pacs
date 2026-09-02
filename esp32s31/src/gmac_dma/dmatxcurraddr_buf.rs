#[doc = "Register `DMATXCURRADDR_BUF` reader"]
pub type R = crate::R<DMATXCURRADDR_BUF_SPEC>;
#[doc = "Field `CURTBUFAPTR` reader - Host Transmit Buffer Address Pointer"]
pub type CURTBUFAPTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Transmit Buffer Address Pointer"]
    #[inline(always)]
    pub fn curtbufaptr(&self) -> CURTBUFAPTR_R {
        CURTBUFAPTR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMATXCURRADDR_BUF")
            .field("curtbufaptr", &self.curtbufaptr())
            .finish()
    }
}
#[doc = "Points to the current Transmit Buffer address read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`dmatxcurraddr_buf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMATXCURRADDR_BUF_SPEC;
impl crate::RegisterSpec for DMATXCURRADDR_BUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatxcurraddr_buf::R`](R) reader structure"]
impl crate::Readable for DMATXCURRADDR_BUF_SPEC {}
#[doc = "`reset()` method sets DMATXCURRADDR_BUF to value 0"]
impl crate::Resettable for DMATXCURRADDR_BUF_SPEC {}
