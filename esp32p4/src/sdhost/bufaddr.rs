#[doc = "Register `BUFADDR` reader"]
pub type R = crate::R<BUFADDR_SPEC>;
#[doc = "Field `BUFADDR` reader - Host Buffer Address Pointer, updated by IDMAC during operation and cleared on reset. This register points to the current Data Buffer Address being accessed by the IDMAC."]
pub type BUFADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Buffer Address Pointer, updated by IDMAC during operation and cleared on reset. This register points to the current Data Buffer Address being accessed by the IDMAC."]
    #[inline(always)]
    pub fn bufaddr(&self) -> BUFADDR_R {
        BUFADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUFADDR")
            .field("bufaddr", &self.bufaddr())
            .finish()
    }
}
#[doc = "Host buffer address pointer register\n\nYou can [`read`](crate::Reg::read) this register and get [`bufaddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUFADDR_SPEC;
impl crate::RegisterSpec for BUFADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bufaddr::R`](R) reader structure"]
impl crate::Readable for BUFADDR_SPEC {}
#[doc = "`reset()` method sets BUFADDR to value 0"]
impl crate::Resettable for BUFADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
