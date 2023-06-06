#[doc = "Register `GPIO_ENABLE` reader"]
pub struct R(crate::R<GPIO_ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_ENABLE` writer"]
pub struct W(crate::W<GPIO_ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GPIO_ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_ENABLE_DATA` reader - The output enable register."]
pub type GPIO_ENABLE_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `GPIO_ENABLE_DATA` writer - The output enable register."]
pub type GPIO_ENABLE_DATA_W<'a, const O: u8> = crate::FieldWriter<'a, GPIO_ENABLE_SPEC, 16, O, u16>;
#[doc = "Field `GPIO_SDIO_SEL` reader - SDIO-dis selection register"]
pub type GPIO_SDIO_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO_SDIO_SEL` writer - SDIO-dis selection register"]
pub type GPIO_SDIO_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, GPIO_ENABLE_SPEC, 6, O>;
impl R {
    #[doc = "Bits 0:15 - The output enable register."]
    #[inline(always)]
    pub fn gpio_enable_data(&self) -> GPIO_ENABLE_DATA_R {
        GPIO_ENABLE_DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - SDIO-dis selection register"]
    #[inline(always)]
    pub fn gpio_sdio_sel(&self) -> GPIO_SDIO_SEL_R {
        GPIO_SDIO_SEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_ENABLE")
            .field(
                "gpio_sdio_sel",
                &format_args!("{}", self.gpio_sdio_sel().bits()),
            )
            .field(
                "gpio_enable_data",
                &format_args!("{}", self.gpio_enable_data().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO_ENABLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - The output enable register."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_enable_data(&mut self) -> GPIO_ENABLE_DATA_W<0> {
        GPIO_ENABLE_DATA_W::new(self)
    }
    #[doc = "Bits 16:21 - SDIO-dis selection register"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_sdio_sel(&mut self) -> GPIO_SDIO_SEL_W<16> {
        GPIO_SDIO_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO_ENABLE\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_enable](index.html) module"]
pub struct GPIO_ENABLE_SPEC;
impl crate::RegisterSpec for GPIO_ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_enable::R](R) reader structure"]
impl crate::Readable for GPIO_ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_enable::W](W) writer structure"]
impl crate::Writable for GPIO_ENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_ENABLE to value 0"]
impl crate::Resettable for GPIO_ENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
