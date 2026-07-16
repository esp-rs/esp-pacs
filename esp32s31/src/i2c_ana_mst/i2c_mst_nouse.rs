#[doc = "Register `I2C_MST_NOUSE` reader"]
pub type R = crate::R<I2C_MST_NOUSE_SPEC>;
#[doc = "Register `I2C_MST_NOUSE` writer"]
pub type W = crate::W<I2C_MST_NOUSE_SPEC>;
#[doc = "Field `I2C_MST_NOUSE` reader - "]
pub type I2C_MST_NOUSE_R = crate::FieldReader<u32>;
#[doc = "Field `I2C_MST_NOUSE` writer - "]
pub type I2C_MST_NOUSE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn i2c_mst_nouse(&self) -> I2C_MST_NOUSE_R {
        I2C_MST_NOUSE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_MST_NOUSE")
            .field("i2c_mst_nouse", &self.i2c_mst_nouse())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn i2c_mst_nouse(&mut self) -> I2C_MST_NOUSE_W<'_, I2C_MST_NOUSE_SPEC> {
        I2C_MST_NOUSE_W::new(self, 0)
    }
}
#[doc = "I2C_MST_NOUSE\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_mst_nouse::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_mst_nouse::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_MST_NOUSE_SPEC;
impl crate::RegisterSpec for I2C_MST_NOUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_mst_nouse::R`](R) reader structure"]
impl crate::Readable for I2C_MST_NOUSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_mst_nouse::W`](W) writer structure"]
impl crate::Writable for I2C_MST_NOUSE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_MST_NOUSE to value 0"]
impl crate::Resettable for I2C_MST_NOUSE_SPEC {}
