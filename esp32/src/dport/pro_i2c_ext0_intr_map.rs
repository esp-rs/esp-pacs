#[doc = "Register `PRO_I2C_EXT0_INTR_MAP` reader"]
pub type R = crate::R<PRO_I2C_EXT0_INTR_MAP_SPEC>;
#[doc = "Register `PRO_I2C_EXT0_INTR_MAP` writer"]
pub type W = crate::W<PRO_I2C_EXT0_INTR_MAP_SPEC>;
#[doc = "Field `PRO_I2C_EXT0_INTR_MAP` reader - "]
pub type PRO_I2C_EXT0_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `PRO_I2C_EXT0_INTR_MAP` writer - "]
pub type PRO_I2C_EXT0_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn pro_i2c_ext0_intr_map(&self) -> PRO_I2C_EXT0_INTR_MAP_R {
        PRO_I2C_EXT0_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_I2C_EXT0_INTR_MAP")
            .field(
                "pro_i2c_ext0_intr_map",
                &self.pro_i2c_ext0_intr_map().bits(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_I2C_EXT0_INTR_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn pro_i2c_ext0_intr_map(&mut self) -> PRO_I2C_EXT0_INTR_MAP_W<PRO_I2C_EXT0_INTR_MAP_SPEC> {
        PRO_I2C_EXT0_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_i2c_ext0_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_i2c_ext0_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_I2C_EXT0_INTR_MAP_SPEC;
impl crate::RegisterSpec for PRO_I2C_EXT0_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_i2c_ext0_intr_map::R`](R) reader structure"]
impl crate::Readable for PRO_I2C_EXT0_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_i2c_ext0_intr_map::W`](W) writer structure"]
impl crate::Writable for PRO_I2C_EXT0_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_I2C_EXT0_INTR_MAP to value 0x10"]
impl crate::Resettable for PRO_I2C_EXT0_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
