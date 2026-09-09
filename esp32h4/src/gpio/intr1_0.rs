#[doc = "Register `INTR1_0` reader"]
pub type R = crate::R<INTR1_0_SPEC>;
#[doc = "Field `INTR1_0` reader - Represents the GPIO_PROCPU_INT interrupt status of GPIO32 ~ GPIO43. Each bit represents:(need update in different project)\\\\ 0: Represents GPIO_PROCPU_INT interrupt is not enabled, or the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ 1: Represents the GPIO generates an interrupt configured by GPIO_PIN0_INT_TYPE after the GPIO_PROCPU_INT interrupt is enabled.\\\\ Bit32 ~ bit43 are corresponding to GPIO32 ~ GPIO43. Bitxx ~ bitxx is invalid. This interrupt status is corresponding to the bit in GPIO_STATUS1_REG when assert (high) enable signal (bit13 of GPIO_PIN0_REG). \\\\"]
pub type INTR1_0_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Represents the GPIO_PROCPU_INT interrupt status of GPIO32 ~ GPIO43. Each bit represents:(need update in different project)\\\\ 0: Represents GPIO_PROCPU_INT interrupt is not enabled, or the GPIO does not generate the interrupt configured by GPIO_PIN0_INT_TYPE.\\\\ 1: Represents the GPIO generates an interrupt configured by GPIO_PIN0_INT_TYPE after the GPIO_PROCPU_INT interrupt is enabled.\\\\ Bit32 ~ bit43 are corresponding to GPIO32 ~ GPIO43. Bitxx ~ bitxx is invalid. This interrupt status is corresponding to the bit in GPIO_STATUS1_REG when assert (high) enable signal (bit13 of GPIO_PIN0_REG). \\\\"]
    #[inline(always)]
    pub fn intr1_0(&self) -> INTR1_0_R {
        INTR1_0_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR1_0")
            .field("intr1_0", &self.intr1_0())
            .finish()
    }
}
#[doc = "GPIO_PROCPU_INT interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr1_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR1_0_SPEC;
impl crate::RegisterSpec for INTR1_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr1_0::R`](R) reader structure"]
impl crate::Readable for INTR1_0_SPEC {}
#[doc = "`reset()` method sets INTR1_0 to value 0"]
impl crate::Resettable for INTR1_0_SPEC {}
