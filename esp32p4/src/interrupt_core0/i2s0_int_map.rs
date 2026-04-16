#[doc = "Register `I2S0_INT_MAP` reader"]
pub type R = crate::R<I2S0_INT_MAP_SPEC>;
#[doc = "Register `I2S0_INT_MAP` writer"]
pub type W = crate::W<I2S0_INT_MAP_SPEC>;
#[doc = "Field `I2S0_INT_MAP` reader - Configures the interrupt source into one CPU interrupt."]
pub type I2S0_INT_MAP_R = crate::FieldReader;
#[doc = "Field `I2S0_INT_MAP` writer - Configures the interrupt source into one CPU interrupt."]
pub type I2S0_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `I2S0_INT_SRC_PASS_IN_SEC` reader - NA"]
pub type I2S0_INT_SRC_PASS_IN_SEC_R = crate::BitReader;
#[doc = "Field `I2S0_INT_SRC_PASS_IN_SEC` writer - NA"]
pub type I2S0_INT_SRC_PASS_IN_SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_INT_SRC_IN_SEC_FLAG` reader - NA"]
pub type I2S0_INT_SRC_IN_SEC_FLAG_R = crate::BitReader;
#[doc = "Field `I2S0_INT_SRC_IN_SEC_FLAG` writer - NA"]
pub type I2S0_INT_SRC_IN_SEC_FLAG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn i2s0_int_map(&self) -> I2S0_INT_MAP_R {
        I2S0_INT_MAP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn i2s0_int_src_pass_in_sec(&self) -> I2S0_INT_SRC_PASS_IN_SEC_R {
        I2S0_INT_SRC_PASS_IN_SEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn i2s0_int_src_in_sec_flag(&self) -> I2S0_INT_SRC_IN_SEC_FLAG_R {
        I2S0_INT_SRC_IN_SEC_FLAG_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2S0_INT_MAP")
            .field("i2s0_int_map", &self.i2s0_int_map())
            .field("i2s0_int_src_pass_in_sec", &self.i2s0_int_src_pass_in_sec())
            .field("i2s0_int_src_in_sec_flag", &self.i2s0_int_src_in_sec_flag())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Configures the interrupt source into one CPU interrupt."]
    #[inline(always)]
    pub fn i2s0_int_map(&mut self) -> I2S0_INT_MAP_W<'_, I2S0_INT_MAP_SPEC> {
        I2S0_INT_MAP_W::new(self, 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn i2s0_int_src_pass_in_sec(
        &mut self,
    ) -> I2S0_INT_SRC_PASS_IN_SEC_W<'_, I2S0_INT_MAP_SPEC> {
        I2S0_INT_SRC_PASS_IN_SEC_W::new(self, 6)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn i2s0_int_src_in_sec_flag(
        &mut self,
    ) -> I2S0_INT_SRC_IN_SEC_FLAG_W<'_, I2S0_INT_MAP_SPEC> {
        I2S0_INT_SRC_IN_SEC_FLAG_W::new(self, 7)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`i2s0_int_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2s0_int_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2S0_INT_MAP_SPEC;
impl crate::RegisterSpec for I2S0_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s0_int_map::R`](R) reader structure"]
impl crate::Readable for I2S0_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2s0_int_map::W`](W) writer structure"]
impl crate::Writable for I2S0_INT_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2S0_INT_MAP to value 0"]
impl crate::Resettable for I2S0_INT_MAP_SPEC {}
