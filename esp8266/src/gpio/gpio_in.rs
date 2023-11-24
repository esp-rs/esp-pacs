#[doc = "Register `GPIO_IN` reader"]
pub type R = crate::R<GPIO_IN_SPEC>;
#[doc = "Register `GPIO_IN` writer"]
pub type W = crate::W<GPIO_IN_SPEC>;
#[doc = "Field `GPIO_IN_DATA` reader - The values of the GPIO pins when the GPIO pin is set as input."]
pub type GPIO_IN_DATA_R = crate::FieldReader<u16>;
#[doc = "Field `GPIO_IN_DATA` writer - The values of the GPIO pins when the GPIO pin is set as input."]
pub type GPIO_IN_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `GPIO_STRAPPING` reader - The values of the strapping pins."]
pub type GPIO_STRAPPING_R = crate::FieldReader<u16>;
#[doc = "Field `GPIO_STRAPPING` writer - The values of the strapping pins."]
pub type GPIO_STRAPPING_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
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
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - The values of the GPIO pins when the GPIO pin is set as input."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_in_data(&mut self) -> GPIO_IN_DATA_W<GPIO_IN_SPEC> {
        GPIO_IN_DATA_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - The values of the strapping pins."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_strapping(&mut self) -> GPIO_STRAPPING_W<GPIO_IN_SPEC> {
        GPIO_STRAPPING_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "The values of the strapping pins.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_in::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_in::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_IN_SPEC;
impl crate::RegisterSpec for GPIO_IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_in::R`](R) reader structure"]
impl crate::Readable for GPIO_IN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_in::W`](W) writer structure"]
impl crate::Writable for GPIO_IN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_IN to value 0"]
impl crate::Resettable for GPIO_IN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
