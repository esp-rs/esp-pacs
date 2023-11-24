#[doc = "Register `GPIO_INTERRUPT_PRO_MAP` reader"]
pub type R = crate::R<GPIO_INTERRUPT_PRO_MAP_SPEC>;
#[doc = "Register `GPIO_INTERRUPT_PRO_MAP` writer"]
pub type W = crate::W<GPIO_INTERRUPT_PRO_MAP_SPEC>;
#[doc = "Field `GPIO_INTERRUPT_PRO_MAP` reader - CORE0_GPIO_INTERRUPT_PRO mapping register"]
pub type GPIO_INTERRUPT_PRO_MAP_R = crate::FieldReader;
#[doc = "Field `GPIO_INTERRUPT_PRO_MAP` writer - CORE0_GPIO_INTERRUPT_PRO mapping register"]
pub type GPIO_INTERRUPT_PRO_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - CORE0_GPIO_INTERRUPT_PRO mapping register"]
    #[inline(always)]
    pub fn gpio_interrupt_pro_map(&self) -> GPIO_INTERRUPT_PRO_MAP_R {
        GPIO_INTERRUPT_PRO_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_INTERRUPT_PRO_MAP")
            .field(
                "gpio_interrupt_pro_map",
                &format_args!("{}", self.gpio_interrupt_pro_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO_INTERRUPT_PRO_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - CORE0_GPIO_INTERRUPT_PRO mapping register"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_interrupt_pro_map(
        &mut self,
    ) -> GPIO_INTERRUPT_PRO_MAP_W<GPIO_INTERRUPT_PRO_MAP_SPEC> {
        GPIO_INTERRUPT_PRO_MAP_W::new(self, 0)
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
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_interrupt_pro_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_interrupt_pro_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_INTERRUPT_PRO_MAP_SPEC;
impl crate::RegisterSpec for GPIO_INTERRUPT_PRO_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_interrupt_pro_map::R`](R) reader structure"]
impl crate::Readable for GPIO_INTERRUPT_PRO_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_interrupt_pro_map::W`](W) writer structure"]
impl crate::Writable for GPIO_INTERRUPT_PRO_MAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_INTERRUPT_PRO_MAP to value 0"]
impl crate::Resettable for GPIO_INTERRUPT_PRO_MAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
