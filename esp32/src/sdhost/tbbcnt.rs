#[doc = "Register `TBBCNT` reader"]
pub type R = crate::R<TBBCNT_SPEC>;
#[doc = "Field `TBBCNT` reader - Number of bytes transferred between Host/DMA memory and BIU FIFO."]
pub type TBBCNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes transferred between Host/DMA memory and BIU FIFO."]
    #[inline(always)]
    pub fn tbbcnt(&self) -> TBBCNT_R {
        TBBCNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TBBCNT")
            .field("tbbcnt", &self.tbbcnt().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TBBCNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Transferred byte count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbbcnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBBCNT_SPEC;
impl crate::RegisterSpec for TBBCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbbcnt::R`](R) reader structure"]
impl crate::Readable for TBBCNT_SPEC {}
#[doc = "`reset()` method sets TBBCNT to value 0"]
impl crate::Resettable for TBBCNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
