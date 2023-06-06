#[doc = "Register `GPIO_IN` reader"]
pub struct R(crate::R<GPIO_IN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_IN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_IN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_IN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_IN` writer"]
pub struct W(crate::W<GPIO_IN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_IN_SPEC>;
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
impl From<crate::W<GPIO_IN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_IN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_IN_DATA` reader - The values of the GPIO pins when the GPIO pin is set as input."]
pub type GPIO_IN_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `GPIO_IN_DATA` writer - The values of the GPIO pins when the GPIO pin is set as input."]
pub type GPIO_IN_DATA_W<'a, const O: u8> = crate::FieldWriter<'a, GPIO_IN_SPEC, 16, O, u16>;
#[doc = "Field `GPIO_STRAPPING` reader - The values of the strapping pins."]
pub type GPIO_STRAPPING_R = crate::FieldReader<u16>;
#[doc = "Field `GPIO_STRAPPING` writer - The values of the strapping pins."]
pub type GPIO_STRAPPING_W<'a, const O: u8> = crate::FieldWriter<'a, GPIO_IN_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - The values of the GPIO pins when the GPIO pin is set as input."]
    #[inline(always)]
    pub fn gpio_in_data(&self) -> GPIO_IN_DATA_R {
        GPIO_IN_DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - The values of the strapping pins."]
    #[inline(always)]
    pub fn gpio_strapping(&self) -> GPIO_STRAPPING_R {
        GPIO_STRAPPING_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_IN")
            .field(
                "gpio_strapping",
                &format_args!("{}", self.gpio_strapping().bits()),
            )
            .field(
                "gpio_in_data",
                &format_args!("{}", self.gpio_in_data().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO_IN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - The values of the GPIO pins when the GPIO pin is set as input."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_in_data(&mut self) -> GPIO_IN_DATA_W<0> {
        GPIO_IN_DATA_W::new(self)
    }
    #[doc = "Bits 16:31 - The values of the strapping pins."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_strapping(&mut self) -> GPIO_STRAPPING_W<16> {
        GPIO_STRAPPING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The values of the strapping pins.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_in](index.html) module"]
pub struct GPIO_IN_SPEC;
impl crate::RegisterSpec for GPIO_IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_in::R](R) reader structure"]
impl crate::Readable for GPIO_IN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_in::W](W) writer structure"]
impl crate::Writable for GPIO_IN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_IN to value 0"]
impl crate::Resettable for GPIO_IN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
