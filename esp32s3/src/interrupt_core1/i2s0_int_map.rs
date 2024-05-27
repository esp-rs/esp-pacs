///Register `I2S0_INT_MAP` reader
pub type R = crate::R<I2S0_INT_MAP_SPEC>;
///Register `I2S0_INT_MAP` writer
pub type W = crate::W<I2S0_INT_MAP_SPEC>;
///Field `I2S0_INT_MAP` reader - this register used to map i2s0 interrupt to one of core1's external interrupt
pub type I2S0_INT_MAP_R = crate::FieldReader;
///Field `I2S0_INT_MAP` writer - this register used to map i2s0 interrupt to one of core1's external interrupt
pub type I2S0_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - this register used to map i2s0 interrupt to one of core1's external interrupt
    #[inline(always)]
    pub fn i2s0_int_map(&self) -> I2S0_INT_MAP_R {
        I2S0_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2S0_INT_MAP")
            .field("i2s0_int_map", &self.i2s0_int_map())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - this register used to map i2s0 interrupt to one of core1's external interrupt
    #[inline(always)]
    #[must_use]
    pub fn i2s0_int_map(&mut self) -> I2S0_INT_MAP_W<I2S0_INT_MAP_SPEC> {
        I2S0_INT_MAP_W::new(self, 0)
    }
}
/**i2s0 interrupt configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`i2s0_int_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s0_int_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct I2S0_INT_MAP_SPEC;
impl crate::RegisterSpec for I2S0_INT_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`i2s0_int_map::R`](R) reader structure
impl crate::Readable for I2S0_INT_MAP_SPEC {}
///`write(|w| ..)` method takes [`i2s0_int_map::W`](W) writer structure
impl crate::Writable for I2S0_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets I2S0_INT_MAP to value 0x10
impl crate::Resettable for I2S0_INT_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
