///Register `GPIO_INTERRUPT_APP_MAP` reader
pub type R = crate::R<GPIO_INTERRUPT_APP_MAP_SPEC>;
///Register `GPIO_INTERRUPT_APP_MAP` writer
pub type W = crate::W<GPIO_INTERRUPT_APP_MAP_SPEC>;
///Field `GPIO_INTERRUPT_APP_MAP` reader - this register used to map gpio_interrupt_app interrupt to one of core1's external interrupt
pub type GPIO_INTERRUPT_APP_MAP_R = crate::FieldReader;
///Field `GPIO_INTERRUPT_APP_MAP` writer - this register used to map gpio_interrupt_app interrupt to one of core1's external interrupt
pub type GPIO_INTERRUPT_APP_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - this register used to map gpio_interrupt_app interrupt to one of core1's external interrupt
    #[inline(always)]
    pub fn gpio_interrupt_app_map(&self) -> GPIO_INTERRUPT_APP_MAP_R {
        GPIO_INTERRUPT_APP_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_INTERRUPT_APP_MAP")
            .field("gpio_interrupt_app_map", &self.gpio_interrupt_app_map())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - this register used to map gpio_interrupt_app interrupt to one of core1's external interrupt
    #[inline(always)]
    #[must_use]
    pub fn gpio_interrupt_app_map(
        &mut self,
    ) -> GPIO_INTERRUPT_APP_MAP_W<GPIO_INTERRUPT_APP_MAP_SPEC> {
        GPIO_INTERRUPT_APP_MAP_W::new(self, 0)
    }
}
/**gpio_interrupt_app interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_interrupt_app_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_interrupt_app_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GPIO_INTERRUPT_APP_MAP_SPEC;
impl crate::RegisterSpec for GPIO_INTERRUPT_APP_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gpio_interrupt_app_map::R`](R) reader structure
impl crate::Readable for GPIO_INTERRUPT_APP_MAP_SPEC {}
///`write(|w| ..)` method takes [`gpio_interrupt_app_map::W`](W) writer structure
impl crate::Writable for GPIO_INTERRUPT_APP_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GPIO_INTERRUPT_APP_MAP to value 0x10
impl crate::Resettable for GPIO_INTERRUPT_APP_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
