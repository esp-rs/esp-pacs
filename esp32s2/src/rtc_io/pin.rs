#[doc = "Register `PIN%s` reader"]
pub struct R(crate::R<PIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIN%s` writer"]
pub struct W(crate::W<PIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIN_SPEC>;
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
impl From<crate::W<PIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAD_DRIVER` reader - Pad driver selection. 0: normal output. 1: open drain."]
pub type PAD_DRIVER_R = crate::BitReader;
#[doc = "Field `PAD_DRIVER` writer - Pad driver selection. 0: normal output. 1: open drain."]
pub type PAD_DRIVER_W<'a, const O: u8> = crate::BitWriter<'a, PIN_SPEC, O>;
#[doc = "Field `GPIO_PIN0_INT_TYPE` reader - GPIO interrupt type selection. 0: GPIO interrupt disabled. 1: rising edge trigger. 2: falling edge trigger. 3: any edge trigger. 4: low level trigger. 5: high level trigger."]
pub type GPIO_PIN0_INT_TYPE_R = crate::FieldReader;
#[doc = "Field `GPIO_PIN0_INT_TYPE` writer - GPIO interrupt type selection. 0: GPIO interrupt disabled. 1: rising edge trigger. 2: falling edge trigger. 3: any edge trigger. 4: low level trigger. 5: high level trigger."]
pub type GPIO_PIN0_INT_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, PIN_SPEC, 3, O>;
#[doc = "Field `GPIO_PIN0_WAKEUP_ENABLE` reader - GPIO wake-up enable. This will only wake up ESP32-S2 from Light-sleep."]
pub type GPIO_PIN0_WAKEUP_ENABLE_R = crate::BitReader;
#[doc = "Field `GPIO_PIN0_WAKEUP_ENABLE` writer - GPIO wake-up enable. This will only wake up ESP32-S2 from Light-sleep."]
pub type GPIO_PIN0_WAKEUP_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, PIN_SPEC, O>;
impl R {
    #[doc = "Bit 2 - Pad driver selection. 0: normal output. 1: open drain."]
    #[inline(always)]
    pub fn pad_driver(&self) -> PAD_DRIVER_R {
        PAD_DRIVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 7:9 - GPIO interrupt type selection. 0: GPIO interrupt disabled. 1: rising edge trigger. 2: falling edge trigger. 3: any edge trigger. 4: low level trigger. 5: high level trigger."]
    #[inline(always)]
    pub fn gpio_pin0_int_type(&self) -> GPIO_PIN0_INT_TYPE_R {
        GPIO_PIN0_INT_TYPE_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - GPIO wake-up enable. This will only wake up ESP32-S2 from Light-sleep."]
    #[inline(always)]
    pub fn gpio_pin0_wakeup_enable(&self) -> GPIO_PIN0_WAKEUP_ENABLE_R {
        GPIO_PIN0_WAKEUP_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIN")
            .field("pad_driver", &format_args!("{}", self.pad_driver().bit()))
            .field(
                "gpio_pin0_int_type",
                &format_args!("{}", self.gpio_pin0_int_type().bits()),
            )
            .field(
                "gpio_pin0_wakeup_enable",
                &format_args!("{}", self.gpio_pin0_wakeup_enable().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PIN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 2 - Pad driver selection. 0: normal output. 1: open drain."]
    #[inline(always)]
    #[must_use]
    pub fn pad_driver(&mut self) -> PAD_DRIVER_W<2> {
        PAD_DRIVER_W::new(self)
    }
    #[doc = "Bits 7:9 - GPIO interrupt type selection. 0: GPIO interrupt disabled. 1: rising edge trigger. 2: falling edge trigger. 3: any edge trigger. 4: low level trigger. 5: high level trigger."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin0_int_type(&mut self) -> GPIO_PIN0_INT_TYPE_W<7> {
        GPIO_PIN0_INT_TYPE_W::new(self)
    }
    #[doc = "Bit 10 - GPIO wake-up enable. This will only wake up ESP32-S2 from Light-sleep."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin0_wakeup_enable(&mut self) -> GPIO_PIN0_WAKEUP_ENABLE_W<10> {
        GPIO_PIN0_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC configuration for pin %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin](index.html) module"]
pub struct PIN_SPEC;
impl crate::RegisterSpec for PIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pin::R](R) reader structure"]
impl crate::Readable for PIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pin::W](W) writer structure"]
impl crate::Writable for PIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIN%s to value 0"]
impl crate::Resettable for PIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
