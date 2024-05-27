///Register `GPIO_STATUS1` reader
pub type R = crate::R<GPIO_STATUS1_SPEC>;
///Field `GPIO_SDIO_INT1` reader - *******Description***********
pub type GPIO_SDIO_INT1_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - *******Description***********
    #[inline(always)]
    pub fn gpio_sdio_int1(&self) -> GPIO_SDIO_INT1_R {
        GPIO_SDIO_INT1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_STATUS1")
            .field("gpio_sdio_int1", &self.gpio_sdio_int1())
            .finish()
    }
}
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_status1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_STATUS1_SPEC;
impl crate::RegisterSpec for GPIO_STATUS1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gpio_status1::R`](R) reader structure
impl crate::Readable for GPIO_STATUS1_SPEC {}
///`reset()` method sets GPIO_STATUS1 to value 0
impl crate::Resettable for GPIO_STATUS1_SPEC {
    const RESET_VALUE: u32 = 0;
}
