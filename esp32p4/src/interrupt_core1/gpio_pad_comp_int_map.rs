///Register `GPIO_PAD_COMP_INT_MAP` reader
pub type R = crate::R<GPIO_PAD_COMP_INT_MAP_SPEC>;
///Register `GPIO_PAD_COMP_INT_MAP` writer
pub type W = crate::W<GPIO_PAD_COMP_INT_MAP_SPEC>;
///Field `CORE1_GPIO_PAD_COMP_INT_MAP` reader - NA
pub type CORE1_GPIO_PAD_COMP_INT_MAP_R = crate::FieldReader;
///Field `CORE1_GPIO_PAD_COMP_INT_MAP` writer - NA
pub type CORE1_GPIO_PAD_COMP_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    ///Bits 0:5 - NA
    #[inline(always)]
    pub fn core1_gpio_pad_comp_int_map(&self) -> CORE1_GPIO_PAD_COMP_INT_MAP_R {
        CORE1_GPIO_PAD_COMP_INT_MAP_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_PAD_COMP_INT_MAP")
            .field(
                "core1_gpio_pad_comp_int_map",
                &self.core1_gpio_pad_comp_int_map(),
            )
            .finish()
    }
}
impl W {
    ///Bits 0:5 - NA
    #[inline(always)]
    #[must_use]
    pub fn core1_gpio_pad_comp_int_map(
        &mut self,
    ) -> CORE1_GPIO_PAD_COMP_INT_MAP_W<GPIO_PAD_COMP_INT_MAP_SPEC> {
        CORE1_GPIO_PAD_COMP_INT_MAP_W::new(self, 0)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_pad_comp_int_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_pad_comp_int_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GPIO_PAD_COMP_INT_MAP_SPEC;
impl crate::RegisterSpec for GPIO_PAD_COMP_INT_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gpio_pad_comp_int_map::R`](R) reader structure
impl crate::Readable for GPIO_PAD_COMP_INT_MAP_SPEC {}
///`write(|w| ..)` method takes [`gpio_pad_comp_int_map::W`](W) writer structure
impl crate::Writable for GPIO_PAD_COMP_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GPIO_PAD_COMP_INT_MAP to value 0
impl crate::Resettable for GPIO_PAD_COMP_INT_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
