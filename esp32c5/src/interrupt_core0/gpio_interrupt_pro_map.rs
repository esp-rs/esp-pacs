#[doc = "Register `GPIO_INTERRUPT_PRO_MAP` reader"]
pub type R = crate::R<GPIO_INTERRUPT_PRO_MAP_SPEC>;
#[doc = "Register `GPIO_INTERRUPT_PRO_MAP` writer"]
pub type W = crate::W<GPIO_INTERRUPT_PRO_MAP_SPEC>;
#[doc = "Field `GPIO_INTERRUPT_PRO_MAP` reader - Configures the interrupt source into one CPU interrupt."]
pub type GPIO_INTERRUPT_PRO_MAP_R = crate::FieldReader;
#[doc = "Field `GPIO_INTERRUPT_PRO_MAP` writer - Configures the interrupt source into one CPU interrupt."]
pub type GPIO_INTERRUPT_PRO_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn gpio_interrupt_pro_map(&self) -> GPIO_INTERRUPT_PRO_MAP_R {
        GPIO_INTERRUPT_PRO_MAP_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_INTERRUPT_PRO_MAP")
            .field("gpio_interrupt_pro_map", &self.gpio_interrupt_pro_map())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn gpio_interrupt_pro_map(
        &mut self,
    ) -> GPIO_INTERRUPT_PRO_MAP_W<GPIO_INTERRUPT_PRO_MAP_SPEC> {
        GPIO_INTERRUPT_PRO_MAP_W::new(self, 0)
    }
}
#[doc = "GPIO_INTERRUPT_PRO mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_interrupt_pro_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_interrupt_pro_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_INTERRUPT_PRO_MAP_SPEC;
impl crate::RegisterSpec for GPIO_INTERRUPT_PRO_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_interrupt_pro_map::R`](R) reader structure"]
impl crate::Readable for GPIO_INTERRUPT_PRO_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_interrupt_pro_map::W`](W) writer structure"]
impl crate::Writable for GPIO_INTERRUPT_PRO_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_INTERRUPT_PRO_MAP to value 0"]
impl crate::Resettable for GPIO_INTERRUPT_PRO_MAP_SPEC {}
