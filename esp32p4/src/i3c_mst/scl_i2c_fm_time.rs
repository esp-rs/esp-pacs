#[doc = "Register `SCL_I2C_FM_TIME` reader"]
pub type R = crate::R<SCL_I2C_FM_TIME_SPEC>;
#[doc = "Register `SCL_I2C_FM_TIME` writer"]
pub type W = crate::W<SCL_I2C_FM_TIME_SPEC>;
#[doc = "Field `REG_I2C_FM_LOW_PERIOD` reader - NA"]
pub type REG_I2C_FM_LOW_PERIOD_R = crate::FieldReader<u16>;
#[doc = "Field `REG_I2C_FM_LOW_PERIOD` writer - NA"]
pub type REG_I2C_FM_LOW_PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REG_I2C_FM_HIGH_PERIOD` reader - The SCL open-drain low count timing for I2C Fast Mode transfers."]
pub type REG_I2C_FM_HIGH_PERIOD_R = crate::FieldReader<u16>;
#[doc = "Field `REG_I2C_FM_HIGH_PERIOD` writer - The SCL open-drain low count timing for I2C Fast Mode transfers."]
pub type REG_I2C_FM_HIGH_PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    pub fn reg_i2c_fm_low_period(&self) -> REG_I2C_FM_LOW_PERIOD_R {
        REG_I2C_FM_LOW_PERIOD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - The SCL open-drain low count timing for I2C Fast Mode transfers."]
    #[inline(always)]
    pub fn reg_i2c_fm_high_period(&self) -> REG_I2C_FM_HIGH_PERIOD_R {
        REG_I2C_FM_HIGH_PERIOD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_I2C_FM_TIME")
            .field(
                "reg_i2c_fm_low_period",
                &format_args!("{}", self.reg_i2c_fm_low_period().bits()),
            )
            .field(
                "reg_i2c_fm_high_period",
                &format_args!("{}", self.reg_i2c_fm_high_period().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SCL_I2C_FM_TIME_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_i2c_fm_low_period(&mut self) -> REG_I2C_FM_LOW_PERIOD_W<SCL_I2C_FM_TIME_SPEC> {
        REG_I2C_FM_LOW_PERIOD_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - The SCL open-drain low count timing for I2C Fast Mode transfers."]
    #[inline(always)]
    #[must_use]
    pub fn reg_i2c_fm_high_period(&mut self) -> REG_I2C_FM_HIGH_PERIOD_W<SCL_I2C_FM_TIME_SPEC> {
        REG_I2C_FM_HIGH_PERIOD_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_i2c_fm_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_i2c_fm_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCL_I2C_FM_TIME_SPEC;
impl crate::RegisterSpec for SCL_I2C_FM_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_i2c_fm_time::R`](R) reader structure"]
impl crate::Readable for SCL_I2C_FM_TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scl_i2c_fm_time::W`](W) writer structure"]
impl crate::Writable for SCL_I2C_FM_TIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCL_I2C_FM_TIME to value 0x004b_00a3"]
impl crate::Resettable for SCL_I2C_FM_TIME_SPEC {
    const RESET_VALUE: u32 = 0x004b_00a3;
}
