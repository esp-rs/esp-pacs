#[doc = "Register `HOST_SLCHOST_GPIO_IN1` reader"]
pub type R = crate::R<HOST_SLCHOST_GPIO_IN1_SPEC>;
#[doc = "Field `HOST_GPIO_SDIO_IN1` reader - "]
pub type HOST_GPIO_SDIO_IN1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_gpio_sdio_in1(&self) -> HOST_GPIO_SDIO_IN1_R {
        HOST_GPIO_SDIO_IN1_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_GPIO_IN1")
            .field(
                "host_gpio_sdio_in1",
                &format_args!("{}", self.host_gpio_sdio_in1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLCHOST_GPIO_IN1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_gpio_in1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOST_GPIO_IN1_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_GPIO_IN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slchost_gpio_in1::R`](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_GPIO_IN1_SPEC {}
#[doc = "`reset()` method sets HOST_SLCHOST_GPIO_IN1 to value 0"]
impl crate::Resettable for HOST_SLCHOST_GPIO_IN1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
