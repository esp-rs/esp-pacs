#[doc = "Register `GPIO_STATUS_W1TC` writer"]
pub type W = crate::W<GPIO_STATUS_W1TC_SPEC>;
#[doc = "Field `GPIO_STATUS_INTERRUPT_W1TC` writer - Writing 1 into a bit in this register will clear the related bit in GPIO_STATUS_INTERRUPT"]
pub type GPIO_STATUS_INTERRUPT_W1TC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO_STATUS_W1TC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - Writing 1 into a bit in this register will clear the related bit in GPIO_STATUS_INTERRUPT"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_status_interrupt_w1tc(
        &mut self,
    ) -> GPIO_STATUS_INTERRUPT_W1TC_W<GPIO_STATUS_W1TC_SPEC> {
        GPIO_STATUS_INTERRUPT_W1TC_W::new(self, 0)
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
#[doc = "GPIO_STATUS_W1TC\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_status_w1tc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_STATUS_W1TC_SPEC;
impl crate::RegisterSpec for GPIO_STATUS_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpio_status_w1tc::W`](W) writer structure"]
impl crate::Writable for GPIO_STATUS_W1TC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_STATUS_W1TC to value 0"]
impl crate::Resettable for GPIO_STATUS_W1TC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
