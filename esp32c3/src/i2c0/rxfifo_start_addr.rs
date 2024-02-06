#[doc = "Register `RXFIFO_START_ADDR` reader"]
pub type R = crate::R<RXFIFO_START_ADDR_SPEC>;
#[doc = "Field `RXFIFO_START_ADDR` reader - reg_rxfifo_start_addr."]
pub type RXFIFO_START_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - reg_rxfifo_start_addr."]
    #[inline(always)]
    pub fn rxfifo_start_addr(&self) -> RXFIFO_START_ADDR_R {
        RXFIFO_START_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXFIFO_START_ADDR")
            .field(
                "rxfifo_start_addr",
                &format_args!("{}", self.rxfifo_start_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RXFIFO_START_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "I2C_RXFIFO_START_ADDR_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxfifo_start_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXFIFO_START_ADDR_SPEC;
impl crate::RegisterSpec for RXFIFO_START_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxfifo_start_addr::R`](R) reader structure"]
impl crate::Readable for RXFIFO_START_ADDR_SPEC {}
#[doc = "`reset()` method sets RXFIFO_START_ADDR to value 0"]
impl crate::Resettable for RXFIFO_START_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
