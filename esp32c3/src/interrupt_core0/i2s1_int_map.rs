#[doc = "Register `I2S1_INT_MAP` reader"]
pub type R = crate::R<I2S1_INT_MAP_SPEC>;
#[doc = "Register `I2S1_INT_MAP` writer"]
pub type W = crate::W<I2S1_INT_MAP_SPEC>;
#[doc = "Field `I2S1_INT_MAP` reader - reg_core0_i2s1_int_map"]
pub type I2S1_INT_MAP_R = crate::FieldReader;
#[doc = "Field `I2S1_INT_MAP` writer - reg_core0_i2s1_int_map"]
pub type I2S1_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - reg_core0_i2s1_int_map"]
    #[inline(always)]
    pub fn i2s1_int_map(&self) -> I2S1_INT_MAP_R {
        I2S1_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2S1_INT_MAP")
            .field(
                "i2s1_int_map",
                &format_args!("{}", self.i2s1_int_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2S1_INT_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - reg_core0_i2s1_int_map"]
    #[inline(always)]
    #[must_use]
    pub fn i2s1_int_map(&mut self) -> I2S1_INT_MAP_W<I2S1_INT_MAP_SPEC> {
        I2S1_INT_MAP_W::new(self, 0)
    }
}
#[doc = "spi2 intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s1_int_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s1_int_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2S1_INT_MAP_SPEC;
impl crate::RegisterSpec for I2S1_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s1_int_map::R`](R) reader structure"]
impl crate::Readable for I2S1_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2s1_int_map::W`](W) writer structure"]
impl crate::Writable for I2S1_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2S1_INT_MAP to value 0"]
impl crate::Resettable for I2S1_INT_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
