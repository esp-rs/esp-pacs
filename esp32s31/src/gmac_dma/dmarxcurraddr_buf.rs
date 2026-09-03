#[doc = "Register `DMARXCURRADDR_BUF` reader"]
pub type R = crate::R<DMARXCURRADDR_BUF_SPEC>;
#[doc = "Field `CURRBUFAPTR` reader - Host Receive Buffer Address Pointer"]
pub type CURRBUFAPTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Buffer Address Pointer"]
    #[inline(always)]
    pub fn currbufaptr(&self) -> CURRBUFAPTR_R {
        CURRBUFAPTR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMARXCURRADDR_BUF")
            .field("currbufaptr", &self.currbufaptr())
            .finish()
    }
}
#[doc = "Points to the current Receive Buffer address read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`dmarxcurraddr_buf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMARXCURRADDR_BUF_SPEC;
impl crate::RegisterSpec for DMARXCURRADDR_BUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmarxcurraddr_buf::R`](R) reader structure"]
impl crate::Readable for DMARXCURRADDR_BUF_SPEC {}
#[doc = "`reset()` method sets DMARXCURRADDR_BUF to value 0"]
impl crate::Resettable for DMARXCURRADDR_BUF_SPEC {}
