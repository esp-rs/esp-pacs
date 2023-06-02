#[doc = "Register `HOST_SLCHOST_GPIO_IN1` reader"]
pub struct R(crate::R<HOST_SLCHOST_GPIO_IN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLCHOST_GPIO_IN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLCHOST_GPIO_IN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLCHOST_GPIO_IN1_SPEC>) -> Self {
        R(reader)
    }
}
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slchost_gpio_in1](index.html) module"]
pub struct HOST_SLCHOST_GPIO_IN1_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_GPIO_IN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slchost_gpio_in1::R](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_GPIO_IN1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HOST_SLCHOST_GPIO_IN1 to value 0"]
impl crate::Resettable for HOST_SLCHOST_GPIO_IN1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
