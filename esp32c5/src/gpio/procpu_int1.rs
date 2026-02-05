#[doc = "Register `PROCPU_INT1` reader"]
pub type R = crate::R<PROCPU_INT1_SPEC>;
#[doc = "Field `PROCPU_INT1` reader - Represents the GPIO_PROCPU_INT interrupt status of GPIO32 ~ GPIO32. Each bit represents:(need update in different project)\\\\ 0: Represents GPIO_PROCPU_INT interrupt is not enabled, or the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ 1: Represents the GPIO generates an interrupt configured by GPIO_PIN0_INT_TYPE after the GPIO_PROCPU_INT interrupt is enabled.\\\\ Bit32 ~ bit32 are corresponding to GPIO32 ~ GPIO32. Bitxx ~ bitxx is invalid. This interrupt status is corresponding to the bit in GPIO_STATUS1_REG when assert (high) enable signal (bit13 of GPIO_PIN0_REG). \\\\"]
pub type PROCPU_INT1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Represents the GPIO_PROCPU_INT interrupt status of GPIO32 ~ GPIO32. Each bit represents:(need update in different project)\\\\ 0: Represents GPIO_PROCPU_INT interrupt is not enabled, or the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ 1: Represents the GPIO generates an interrupt configured by GPIO_PIN0_INT_TYPE after the GPIO_PROCPU_INT interrupt is enabled.\\\\ Bit32 ~ bit32 are corresponding to GPIO32 ~ GPIO32. Bitxx ~ bitxx is invalid. This interrupt status is corresponding to the bit in GPIO_STATUS1_REG when assert (high) enable signal (bit13 of GPIO_PIN0_REG). \\\\"]
    #[inline(always)]
    pub fn procpu_int1(&self) -> PROCPU_INT1_R {
        PROCPU_INT1_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PROCPU_INT1")
            .field("procpu_int1", &self.procpu_int1())
            .finish()
    }
}
#[doc = "GPIO_PROCPU_INT interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`procpu_int1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PROCPU_INT1_SPEC;
impl crate::RegisterSpec for PROCPU_INT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`procpu_int1::R`](R) reader structure"]
impl crate::Readable for PROCPU_INT1_SPEC {}
#[doc = "`reset()` method sets PROCPU_INT1 to value 0"]
impl crate::Resettable for PROCPU_INT1_SPEC {}
