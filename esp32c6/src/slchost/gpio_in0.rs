#[doc = "Register `GPIO_IN0` reader"]
pub struct R(crate::R<GPIO_IN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_IN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_IN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_IN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GPIO_SDIO_IN0` reader - *******Description***********"]
pub type GPIO_SDIO_IN0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - *******Description***********"]
    #[inline(always)]
    pub fn gpio_sdio_in0(&self) -> GPIO_SDIO_IN0_R {
        GPIO_SDIO_IN0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_IN0")
            .field(
                "gpio_sdio_in0",
                &format_args!("{}", self.gpio_sdio_in0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO_IN0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_in0](index.html) module"]
pub struct GPIO_IN0_SPEC;
impl crate::RegisterSpec for GPIO_IN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_in0::R](R) reader structure"]
impl crate::Readable for GPIO_IN0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIO_IN0 to value 0"]
impl crate::Resettable for GPIO_IN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
