#[doc = "Register `EXTMEM_REJECT_ADDR` reader"]
pub type R = crate::R<EXTMEM_REJECT_ADDR_SPEC>;
#[doc = "Field `EXTMEM_REJECT_ADDR` reader - This register store the first address rejected by permission control when accessing external RAM."]
pub type EXTMEM_REJECT_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register store the first address rejected by permission control when accessing external RAM."]
    #[inline(always)]
    pub fn extmem_reject_addr(&self) -> EXTMEM_REJECT_ADDR_R {
        EXTMEM_REJECT_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTMEM_REJECT_ADDR")
            .field("extmem_reject_addr", &self.extmem_reject_addr())
            .finish()
    }
}
#[doc = "Reject address accessing external RAM\n\nYou can [`read`](crate::Reg::read) this register and get [`extmem_reject_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTMEM_REJECT_ADDR_SPEC;
impl crate::RegisterSpec for EXTMEM_REJECT_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extmem_reject_addr::R`](R) reader structure"]
impl crate::Readable for EXTMEM_REJECT_ADDR_SPEC {}
#[doc = "`reset()` method sets EXTMEM_REJECT_ADDR to value 0"]
impl crate::Resettable for EXTMEM_REJECT_ADDR_SPEC {}
