#[doc = "Register `SCL_I2C_FMP_TIME` reader"]
pub type R = crate::R<SCL_I2C_FMP_TIME_SPEC>;
#[doc = "Register `SCL_I2C_FMP_TIME` writer"]
pub type W = crate::W<SCL_I2C_FMP_TIME_SPEC>;
#[doc = "Field `REG_I2C_FMP_LOW_PERIOD` reader - NA"]
pub type REG_I2C_FMP_LOW_PERIOD_R = crate::FieldReader<u16>;
#[doc = "Field `REG_I2C_FMP_LOW_PERIOD` writer - NA"]
pub type REG_I2C_FMP_LOW_PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REG_I2C_FMP_HIGH_PERIOD` reader - NA"]
pub type REG_I2C_FMP_HIGH_PERIOD_R = crate::FieldReader;
#[doc = "Field `REG_I2C_FMP_HIGH_PERIOD` writer - NA"]
pub type REG_I2C_FMP_HIGH_PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    pub fn reg_i2c_fmp_low_period(&self) -> REG_I2C_FMP_LOW_PERIOD_R {
        REG_I2C_FMP_LOW_PERIOD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    pub fn reg_i2c_fmp_high_period(&self) -> REG_I2C_FMP_HIGH_PERIOD_R {
        REG_I2C_FMP_HIGH_PERIOD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_I2C_FMP_TIME")
            .field("reg_i2c_fmp_low_period", &self.reg_i2c_fmp_low_period())
            .field("reg_i2c_fmp_high_period", &self.reg_i2c_fmp_high_period())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    pub fn reg_i2c_fmp_low_period(&mut self) -> REG_I2C_FMP_LOW_PERIOD_W<SCL_I2C_FMP_TIME_SPEC> {
        REG_I2C_FMP_LOW_PERIOD_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    pub fn reg_i2c_fmp_high_period(&mut self) -> REG_I2C_FMP_HIGH_PERIOD_W<SCL_I2C_FMP_TIME_SPEC> {
        REG_I2C_FMP_HIGH_PERIOD_W::new(self, 16)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_i2c_fmp_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_i2c_fmp_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCL_I2C_FMP_TIME_SPEC;
impl crate::RegisterSpec for SCL_I2C_FMP_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_i2c_fmp_time::R`](R) reader structure"]
impl crate::Readable for SCL_I2C_FMP_TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scl_i2c_fmp_time::W`](W) writer structure"]
impl crate::Writable for SCL_I2C_FMP_TIME_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCL_I2C_FMP_TIME to value 0x0021_003f"]
impl crate::Resettable for SCL_I2C_FMP_TIME_SPEC {
    const RESET_VALUE: u32 = 0x0021_003f;
}
