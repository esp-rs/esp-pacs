#[doc = "Register `GPIO_STATUS` reader"]
pub struct R(crate::R<GPIO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_STATUS` writer"]
pub struct W(crate::W<GPIO_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_STATUS_SPEC>;
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
impl From<crate::W<GPIO_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_STATUS_INTERRUPT` reader - Interrupt enable register."]
pub type GPIO_STATUS_INTERRUPT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `GPIO_STATUS_INTERRUPT` writer - Interrupt enable register."]
pub type GPIO_STATUS_INTERRUPT_W<'a, const O: u8> =
    crate::FieldWriter<'a, GPIO_STATUS_SPEC, 16, O, u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Interrupt enable register."]
    #[inline(always)]
    pub fn gpio_status_interrupt(&self) -> GPIO_STATUS_INTERRUPT_R {
        GPIO_STATUS_INTERRUPT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_STATUS")
            .field(
                "gpio_status_interrupt",
                &format_args!("{}", self.gpio_status_interrupt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Interrupt enable register."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_status_interrupt(&mut self) -> GPIO_STATUS_INTERRUPT_W<0> {
        GPIO_STATUS_INTERRUPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_status](index.html) module"]
pub struct GPIO_STATUS_SPEC;
impl crate::RegisterSpec for GPIO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_status::R](R) reader structure"]
impl crate::Readable for GPIO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_status::W](W) writer structure"]
impl crate::Writable for GPIO_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_STATUS to value 0"]
impl crate::Resettable for GPIO_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
