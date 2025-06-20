#[doc = "Register `I2C_SDA_HOLD` reader"]
pub type R = crate::R<I2C_SDA_HOLD_SPEC>;
#[doc = "Register `I2C_SDA_HOLD` writer"]
pub type W = crate::W<I2C_SDA_HOLD_SPEC>;
#[doc = "Field `TIME` reader - This register is used to configure the time to hold the data after the negativeedge of SCL, in I2C module clock cycles."]
pub type TIME_R = crate::FieldReader<u16>;
#[doc = "Field `TIME` writer - This register is used to configure the time to hold the data after the negativeedge of SCL, in I2C module clock cycles."]
pub type TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - This register is used to configure the time to hold the data after the negativeedge of SCL, in I2C module clock cycles."]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new((self.bits & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_SDA_HOLD")
            .field("time", &self.time())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - This register is used to configure the time to hold the data after the negativeedge of SCL, in I2C module clock cycles."]
    #[inline(always)]
    pub fn time(&mut self) -> TIME_W<I2C_SDA_HOLD_SPEC> {
        TIME_W::new(self, 0)
    }
}
#[doc = "Configures the hold time after a negative SCL edge.\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_sda_hold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_sda_hold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_SDA_HOLD_SPEC;
impl crate::RegisterSpec for I2C_SDA_HOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_sda_hold::R`](R) reader structure"]
impl crate::Readable for I2C_SDA_HOLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_sda_hold::W`](W) writer structure"]
impl crate::Writable for I2C_SDA_HOLD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_SDA_HOLD to value 0"]
impl crate::Resettable for I2C_SDA_HOLD_SPEC {
    const RESET_VALUE: u32 = 0;
}
