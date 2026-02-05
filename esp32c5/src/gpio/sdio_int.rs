#[doc = "Register `SDIO_INT` reader"]
pub type R = crate::R<SDIO_INT_SPEC>;
#[doc = "Field `SDIO_INT` reader - Represents the GPIO_SDIO_INT interrupt status of GPIO0 ~ GPIO31. Each bit represents:(need update in different project)\\\\ 0: Represents GPIO_SDIO_INT interrupt is not enabled, or the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ 1: Represents the GPIO generates an interrupt configured by GPIO_PIN0_INT_TYPE after the GPIO_SDIO_INT interrupt is enabled.\\\\ Bit0 ~ bit31 are corresponding to GPIO0 ~ GPIO31. Bitxx ~ bitxx is invalid. This interrupt status is corresponding to the bit in GPIO_STATUS_REG when assert (high) enable signal (bit13 of GPIO_PIN0_REG). \\\\"]
pub type SDIO_INT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Represents the GPIO_SDIO_INT interrupt status of GPIO0 ~ GPIO31. Each bit represents:(need update in different project)\\\\ 0: Represents GPIO_SDIO_INT interrupt is not enabled, or the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ 1: Represents the GPIO generates an interrupt configured by GPIO_PIN0_INT_TYPE after the GPIO_SDIO_INT interrupt is enabled.\\\\ Bit0 ~ bit31 are corresponding to GPIO0 ~ GPIO31. Bitxx ~ bitxx is invalid. This interrupt status is corresponding to the bit in GPIO_STATUS_REG when assert (high) enable signal (bit13 of GPIO_PIN0_REG). \\\\"]
    #[inline(always)]
    pub fn sdio_int(&self) -> SDIO_INT_R {
        SDIO_INT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_INT")
            .field("sdio_int", &self.sdio_int())
            .finish()
    }
}
#[doc = "GPIO_SDIO_INT interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_int::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIO_INT_SPEC;
impl crate::RegisterSpec for SDIO_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_int::R`](R) reader structure"]
impl crate::Readable for SDIO_INT_SPEC {}
#[doc = "`reset()` method sets SDIO_INT to value 0"]
impl crate::Resettable for SDIO_INT_SPEC {}
