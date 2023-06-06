#[doc = "Register `GPIO_STATUS1` reader"]
pub struct R(crate::R<GPIO_STATUS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_STATUS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_STATUS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_STATUS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GPIO_SDIO_INT1` reader - *******Description***********"]
pub type GPIO_SDIO_INT1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - *******Description***********"]
    #[inline(always)]
    pub fn gpio_sdio_int1(&self) -> GPIO_SDIO_INT1_R {
        GPIO_SDIO_INT1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_STATUS1")
            .field(
                "gpio_sdio_int1",
                &format_args!("{}", self.gpio_sdio_int1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO_STATUS1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "*******Description***********\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_status1](index.html) module"]
pub struct GPIO_STATUS1_SPEC;
impl crate::RegisterSpec for GPIO_STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_status1::R](R) reader structure"]
impl crate::Readable for GPIO_STATUS1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIO_STATUS1 to value 0"]
impl crate::Resettable for GPIO_STATUS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
