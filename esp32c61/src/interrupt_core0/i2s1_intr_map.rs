#[doc = "Register `I2S1_INTR_MAP` reader"]
pub type R = crate::R<I2S1_INTR_MAP_SPEC>;
#[doc = "Register `I2S1_INTR_MAP` writer"]
pub type W = crate::W<I2S1_INTR_MAP_SPEC>;
#[doc = "Field `I2S1_INTR_MAP` reader - Configures the interrupt source into one CPU interrupt."]
pub type I2S1_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `I2S1_INTR_MAP` writer - Configures the interrupt source into one CPU interrupt."]
pub type I2S1_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `I2S1_INTR_PASS_IN_SEC` reader - Configures the PASS_IN_SEC flag of the interrupt source."]
pub type I2S1_INTR_PASS_IN_SEC_R = crate::BitReader;
#[doc = "Field `I2S1_INTR_PASS_IN_SEC` writer - Configures the PASS_IN_SEC flag of the interrupt source."]
pub type I2S1_INTR_PASS_IN_SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn i2s1_intr_map(&self) -> I2S1_INTR_MAP_R {
        I2S1_INTR_MAP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Configures the PASS_IN_SEC flag of the interrupt source."]
    #[inline(always)]
    pub fn i2s1_intr_pass_in_sec(&self) -> I2S1_INTR_PASS_IN_SEC_R {
        I2S1_INTR_PASS_IN_SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2S1_INTR_MAP")
            .field("i2s1_intr_map", &self.i2s1_intr_map())
            .field("i2s1_intr_pass_in_sec", &self.i2s1_intr_pass_in_sec())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn i2s1_intr_map(&mut self) -> I2S1_INTR_MAP_W<I2S1_INTR_MAP_SPEC> {
        I2S1_INTR_MAP_W::new(self, 0)
    }
    #[doc = "Bit 8 - Configures the PASS_IN_SEC flag of the interrupt source."]
    #[inline(always)]
    pub fn i2s1_intr_pass_in_sec(&mut self) -> I2S1_INTR_PASS_IN_SEC_W<I2S1_INTR_MAP_SPEC> {
        I2S1_INTR_PASS_IN_SEC_W::new(self, 8)
    }
}
#[doc = "I2S1_INTR mapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s1_intr_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s1_intr_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2S1_INTR_MAP_SPEC;
impl crate::RegisterSpec for I2S1_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s1_intr_map::R`](R) reader structure"]
impl crate::Readable for I2S1_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2s1_intr_map::W`](W) writer structure"]
impl crate::Writable for I2S1_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2S1_INTR_MAP to value 0"]
impl crate::Resettable for I2S1_INTR_MAP_SPEC {}
