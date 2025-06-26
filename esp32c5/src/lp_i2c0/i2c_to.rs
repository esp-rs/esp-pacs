#[doc = "Register `I2C_TO` reader"]
pub type R = crate::R<I2C_TO_SPEC>;
#[doc = "Register `I2C_TO` writer"]
pub type W = crate::W<I2C_TO_SPEC>;
#[doc = "Field `I2C_TIME_OUT_VALUE` reader - This register is used to configure the timeout for receiving a data bit in APBclock cycles."]
pub type I2C_TIME_OUT_VALUE_R = crate::FieldReader;
#[doc = "Field `I2C_TIME_OUT_VALUE` writer - This register is used to configure the timeout for receiving a data bit in APBclock cycles."]
pub type I2C_TIME_OUT_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `I2C_TIME_OUT_EN` reader - This is the enable bit for time out control."]
pub type I2C_TIME_OUT_EN_R = crate::BitReader;
#[doc = "Field `I2C_TIME_OUT_EN` writer - This is the enable bit for time out control."]
pub type I2C_TIME_OUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - This register is used to configure the timeout for receiving a data bit in APBclock cycles."]
    #[inline(always)]
    pub fn i2c_time_out_value(&self) -> I2C_TIME_OUT_VALUE_R {
        I2C_TIME_OUT_VALUE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - This is the enable bit for time out control."]
    #[inline(always)]
    pub fn i2c_time_out_en(&self) -> I2C_TIME_OUT_EN_R {
        I2C_TIME_OUT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_TO")
            .field("i2c_time_out_value", &self.i2c_time_out_value())
            .field("i2c_time_out_en", &self.i2c_time_out_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - This register is used to configure the timeout for receiving a data bit in APBclock cycles."]
    #[inline(always)]
    pub fn i2c_time_out_value(&mut self) -> I2C_TIME_OUT_VALUE_W<I2C_TO_SPEC> {
        I2C_TIME_OUT_VALUE_W::new(self, 0)
    }
    #[doc = "Bit 5 - This is the enable bit for time out control."]
    #[inline(always)]
    pub fn i2c_time_out_en(&mut self) -> I2C_TIME_OUT_EN_W<I2C_TO_SPEC> {
        I2C_TIME_OUT_EN_W::new(self, 5)
    }
}
#[doc = "Setting time out control for receiving data.\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_to::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_to::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_TO_SPEC;
impl crate::RegisterSpec for I2C_TO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_to::R`](R) reader structure"]
impl crate::Readable for I2C_TO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_to::W`](W) writer structure"]
impl crate::Writable for I2C_TO_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_TO to value 0x10"]
impl crate::Resettable for I2C_TO_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
