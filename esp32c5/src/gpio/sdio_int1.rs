#[doc = "Register `SDIO_INT1` reader"]
pub type R = crate::R<SDIO_INT1_SPEC>;
#[doc = "Field `SDIO_INT1` reader - Represents the GPIO_SDIO_INT interrupt status of GPIO32 ~ GPIO32. Each bit represents:(need update in different project)\\\\ 0: Represents GPIO_SDIO_INT interrupt is not enabled, or the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ 1: Represents the GPIO generates an interrupt configured by GPIO_PIN0_INT_TYPE after the GPIO_SDIO_INT interrupt is enabled.\\\\ Bit32 ~ bit32 are corresponding to GPIO32 ~ GPIO32. Bitxx ~ bitxx is invalid. This interrupt status is corresponding to the bit in GPIO_STATUS1_REG when assert (high) enable signal (bit13 of GPIO_PIN0_REG). \\\\"]
pub type SDIO_INT1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Represents the GPIO_SDIO_INT interrupt status of GPIO32 ~ GPIO32. Each bit represents:(need update in different project)\\\\ 0: Represents GPIO_SDIO_INT interrupt is not enabled, or the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ 1: Represents the GPIO generates an interrupt configured by GPIO_PIN0_INT_TYPE after the GPIO_SDIO_INT interrupt is enabled.\\\\ Bit32 ~ bit32 are corresponding to GPIO32 ~ GPIO32. Bitxx ~ bitxx is invalid. This interrupt status is corresponding to the bit in GPIO_STATUS1_REG when assert (high) enable signal (bit13 of GPIO_PIN0_REG). \\\\"]
    #[inline(always)]
    pub fn sdio_int1(&self) -> SDIO_INT1_R {
        SDIO_INT1_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_INT1")
            .field("sdio_int1", &self.sdio_int1())
            .finish()
    }
}
#[doc = "GPIO_SDIO_INT interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_int1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIO_INT1_SPEC;
impl crate::RegisterSpec for SDIO_INT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_int1::R`](R) reader structure"]
impl crate::Readable for SDIO_INT1_SPEC {}
#[doc = "`reset()` method sets SDIO_INT1 to value 0"]
impl crate::Resettable for SDIO_INT1_SPEC {}
