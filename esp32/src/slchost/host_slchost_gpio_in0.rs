#[doc = "Register `HOST_SLCHOST_GPIO_IN0` reader"]
pub type R = crate::R<HOST_SLCHOST_GPIO_IN0_SPEC>;
#[doc = "Field `HOST_GPIO_SDIO_IN0` reader - "]
pub type HOST_GPIO_SDIO_IN0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn host_gpio_sdio_in0(&self) -> HOST_GPIO_SDIO_IN0_R {
        HOST_GPIO_SDIO_IN0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_GPIO_IN0")
            .field(
                "host_gpio_sdio_in0",
                &format_args!("{}", self.host_gpio_sdio_in0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLCHOST_GPIO_IN0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_gpio_in0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOST_GPIO_IN0_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_GPIO_IN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slchost_gpio_in0::R`](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_GPIO_IN0_SPEC {}
#[doc = "`reset()` method sets HOST_SLCHOST_GPIO_IN0 to value 0"]
impl crate::Resettable for HOST_SLCHOST_GPIO_IN0_SPEC {
    const RESET_VALUE: u32 = 0;
}
