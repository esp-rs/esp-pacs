#[doc = "Register `DMARXCURRDESC` reader"]
pub type R = crate::R<DMARXCURRDESC_SPEC>;
#[doc = "Field `CURRDESAPTR` reader - Host Receive Descriptor Address Pointer"]
pub type CURRDESAPTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Descriptor Address Pointer"]
    #[inline(always)]
    pub fn currdesaptr(&self) -> CURRDESAPTR_R {
        CURRDESAPTR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMARXCURRDESC")
            .field("currdesaptr", &self.currdesaptr())
            .finish()
    }
}
#[doc = "Points to the start of current Receive Descriptor read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`dmarxcurrdesc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMARXCURRDESC_SPEC;
impl crate::RegisterSpec for DMARXCURRDESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmarxcurrdesc::R`](R) reader structure"]
impl crate::Readable for DMARXCURRDESC_SPEC {}
#[doc = "`reset()` method sets DMARXCURRDESC to value 0"]
impl crate::Resettable for DMARXCURRDESC_SPEC {}
