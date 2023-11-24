#[doc = "Register `GPIO_STATUS` reader"]
pub type R = crate::R<GPIO_STATUS_SPEC>;
#[doc = "Register `GPIO_STATUS` writer"]
pub type W = crate::W<GPIO_STATUS_SPEC>;
#[doc = "Field `GPIO_STATUS_INTERRUPT` reader - Interrupt enable register."]
pub type GPIO_STATUS_INTERRUPT_R = crate::FieldReader<u16>;
#[doc = "Field `GPIO_STATUS_INTERRUPT` writer - Interrupt enable register."]
pub type GPIO_STATUS_INTERRUPT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
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
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Interrupt enable register."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_status_interrupt(&mut self) -> GPIO_STATUS_INTERRUPT_W<GPIO_STATUS_SPEC> {
        GPIO_STATUS_INTERRUPT_W::new(self, 0)
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
#[doc = "GPIO_STATUS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_STATUS_SPEC;
impl crate::RegisterSpec for GPIO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_status::R`](R) reader structure"]
impl crate::Readable for GPIO_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_status::W`](W) writer structure"]
impl crate::Writable for GPIO_STATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_STATUS to value 0"]
impl crate::Resettable for GPIO_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
