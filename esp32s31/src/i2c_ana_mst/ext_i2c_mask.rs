#[doc = "Register `EXT_I2C_MASK` reader"]
pub type R = crate::R<EXT_I2C_MASK_SPEC>;
#[doc = "Register `EXT_I2C_MASK` writer"]
pub type W = crate::W<EXT_I2C_MASK_SPEC>;
#[doc = "Field `EXT_I2C_SDA_I_MASK` reader - "]
pub type EXT_I2C_SDA_I_MASK_R = crate::FieldReader<u16>;
#[doc = "Field `EXT_I2C_SDA_I_MASK` writer - "]
pub type EXT_I2C_SDA_I_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EXT_I2C_SDA_O_MASK` reader - "]
pub type EXT_I2C_SDA_O_MASK_R = crate::FieldReader<u16>;
#[doc = "Field `EXT_I2C_SDA_O_MASK` writer - "]
pub type EXT_I2C_SDA_O_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ext_i2c_sda_i_mask(&self) -> EXT_I2C_SDA_I_MASK_R {
        EXT_I2C_SDA_I_MASK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn ext_i2c_sda_o_mask(&self) -> EXT_I2C_SDA_O_MASK_R {
        EXT_I2C_SDA_O_MASK_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_I2C_MASK")
            .field("ext_i2c_sda_i_mask", &self.ext_i2c_sda_i_mask())
            .field("ext_i2c_sda_o_mask", &self.ext_i2c_sda_o_mask())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ext_i2c_sda_i_mask(&mut self) -> EXT_I2C_SDA_I_MASK_W<'_, EXT_I2C_MASK_SPEC> {
        EXT_I2C_SDA_I_MASK_W::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn ext_i2c_sda_o_mask(&mut self) -> EXT_I2C_SDA_O_MASK_W<'_, EXT_I2C_MASK_SPEC> {
        EXT_I2C_SDA_O_MASK_W::new(self, 16)
    }
}
#[doc = "EXT_I2C_MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_i2c_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_i2c_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_I2C_MASK_SPEC;
impl crate::RegisterSpec for EXT_I2C_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_i2c_mask::R`](R) reader structure"]
impl crate::Readable for EXT_I2C_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_i2c_mask::W`](W) writer structure"]
impl crate::Writable for EXT_I2C_MASK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXT_I2C_MASK to value 0"]
impl crate::Resettable for EXT_I2C_MASK_SPEC {}
