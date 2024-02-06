#[doc = "Register `GPIO_IN1` reader"]
pub type R = crate::R<GPIO_IN1_SPEC>;
#[doc = "Field `GPIO_SDIO_IN1` reader - *******Description***********"]
pub type GPIO_SDIO_IN1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - *******Description***********"]
    #[inline(always)]
    pub fn gpio_sdio_in1(&self) -> GPIO_SDIO_IN1_R {
        GPIO_SDIO_IN1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_IN1")
            .field(
                "gpio_sdio_in1",
                &format_args!("{}", self.gpio_sdio_in1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO_IN1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_in1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_IN1_SPEC;
impl crate::RegisterSpec for GPIO_IN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_in1::R`](R) reader structure"]
impl crate::Readable for GPIO_IN1_SPEC {}
#[doc = "`reset()` method sets GPIO_IN1 to value 0"]
impl crate::Resettable for GPIO_IN1_SPEC {
    const RESET_VALUE: u32 = 0;
}
