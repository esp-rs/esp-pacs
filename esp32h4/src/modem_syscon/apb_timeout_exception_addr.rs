#[doc = "Register `APB_TIMEOUT_EXCEPTION_ADDR` reader"]
pub type R = crate::R<APB_TIMEOUT_EXCEPTION_ADDR_SPEC>;
#[doc = "Field `MODEM_APB_TIMEOUT_EXCEPTION_PADDR` reader - "]
pub type MODEM_APB_TIMEOUT_EXCEPTION_PADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn modem_apb_timeout_exception_paddr(&self) -> MODEM_APB_TIMEOUT_EXCEPTION_PADDR_R {
        MODEM_APB_TIMEOUT_EXCEPTION_PADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_TIMEOUT_EXCEPTION_ADDR")
            .field(
                "modem_apb_timeout_exception_paddr",
                &self.modem_apb_timeout_exception_paddr(),
            )
            .finish()
    }
}
#[doc = "APB_TIMEOUT_EXCEPTION_ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`apb_timeout_exception_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB_TIMEOUT_EXCEPTION_ADDR_SPEC;
impl crate::RegisterSpec for APB_TIMEOUT_EXCEPTION_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_timeout_exception_addr::R`](R) reader structure"]
impl crate::Readable for APB_TIMEOUT_EXCEPTION_ADDR_SPEC {}
#[doc = "`reset()` method sets APB_TIMEOUT_EXCEPTION_ADDR to value 0"]
impl crate::Resettable for APB_TIMEOUT_EXCEPTION_ADDR_SPEC {}
