#[doc = "Register `STATUS_NEXT` reader"]
pub type R = crate::R<STATUS_NEXT_SPEC>;
#[doc = "Field `REG_GPIO_STATUS_INTERRUPT_NEXT` reader - Reserved"]
pub type REG_GPIO_STATUS_INTERRUPT_NEXT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Reserved"]
    #[inline(always)]
    pub fn reg_gpio_status_interrupt_next(&self) -> REG_GPIO_STATUS_INTERRUPT_NEXT_R {
        REG_GPIO_STATUS_INTERRUPT_NEXT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS_NEXT")
            .field(
                "reg_gpio_status_interrupt_next",
                &self.reg_gpio_status_interrupt_next().bits(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS_NEXT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status_next::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_NEXT_SPEC;
impl crate::RegisterSpec for STATUS_NEXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status_next::R`](R) reader structure"]
impl crate::Readable for STATUS_NEXT_SPEC {}
#[doc = "`reset()` method sets STATUS_NEXT to value 0"]
impl crate::Resettable for STATUS_NEXT_SPEC {
    const RESET_VALUE: u32 = 0;
}
