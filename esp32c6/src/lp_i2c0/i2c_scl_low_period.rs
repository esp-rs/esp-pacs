#[doc = "Register `I2C_SCL_LOW_PERIOD` reader"]
pub type R = crate::R<I2C_SCL_LOW_PERIOD_SPEC>;
#[doc = "Register `I2C_SCL_LOW_PERIOD` writer"]
pub type W = crate::W<I2C_SCL_LOW_PERIOD_SPEC>;
#[doc = "Field `I2C_SCL_LOW_PERIOD` reader - This register is used to configure for how long SCL remains low in master mode, in I2C module clock cycles."]
pub type I2C_SCL_LOW_PERIOD_R = crate::FieldReader<u16>;
#[doc = "Field `I2C_SCL_LOW_PERIOD` writer - This register is used to configure for how long SCL remains low in master mode, in I2C module clock cycles."]
pub type I2C_SCL_LOW_PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - This register is used to configure for how long SCL remains low in master mode, in I2C module clock cycles."]
    #[inline(always)]
    pub fn i2c_scl_low_period(&self) -> I2C_SCL_LOW_PERIOD_R {
        I2C_SCL_LOW_PERIOD_R::new((self.bits & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_SCL_LOW_PERIOD")
            .field(
                "i2c_scl_low_period",
                &format_args!("{}", self.i2c_scl_low_period().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C_SCL_LOW_PERIOD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:8 - This register is used to configure for how long SCL remains low in master mode, in I2C module clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_scl_low_period(&mut self) -> I2C_SCL_LOW_PERIOD_W<I2C_SCL_LOW_PERIOD_SPEC> {
        I2C_SCL_LOW_PERIOD_W::new(self, 0)
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
#[doc = "Configures the low level width of the SCL Clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_scl_low_period::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_scl_low_period::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_SCL_LOW_PERIOD_SPEC;
impl crate::RegisterSpec for I2C_SCL_LOW_PERIOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_scl_low_period::R`](R) reader structure"]
impl crate::Readable for I2C_SCL_LOW_PERIOD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_scl_low_period::W`](W) writer structure"]
impl crate::Writable for I2C_SCL_LOW_PERIOD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_SCL_LOW_PERIOD to value 0"]
impl crate::Resettable for I2C_SCL_LOW_PERIOD_SPEC {
    const RESET_VALUE: u32 = 0;
}
