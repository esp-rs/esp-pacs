#[doc = "Register `DEC_FAILURE_ADDR` reader"]
pub type R = crate::R<DEC_FAILURE_ADDR_SPEC>;
#[doc = "Field `REG_DEC_FAILURE_ADDR` reader - "]
pub type REG_DEC_FAILURE_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn reg_dec_failure_addr(&self) -> REG_DEC_FAILURE_ADDR_R {
        REG_DEC_FAILURE_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEC_FAILURE_ADDR")
            .field("reg_dec_failure_addr", &self.reg_dec_failure_addr())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dec_failure_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEC_FAILURE_ADDR_SPEC;
impl crate::RegisterSpec for DEC_FAILURE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dec_failure_addr::R`](R) reader structure"]
impl crate::Readable for DEC_FAILURE_ADDR_SPEC {}
#[doc = "`reset()` method sets DEC_FAILURE_ADDR to value 0"]
impl crate::Resettable for DEC_FAILURE_ADDR_SPEC {}
