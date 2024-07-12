#[doc = "Register `SAR_SLAVE_ADDR3` reader"]
pub type R = crate::R<SAR_SLAVE_ADDR3_SPEC>;
#[doc = "Register `SAR_SLAVE_ADDR3` writer"]
pub type W = crate::W<SAR_SLAVE_ADDR3_SPEC>;
#[doc = "Field `I2C_SLAVE_ADDR5` reader - RTC I2C slave address 5"]
pub type I2C_SLAVE_ADDR5_R = crate::FieldReader<u16>;
#[doc = "Field `I2C_SLAVE_ADDR5` writer - RTC I2C slave address 5"]
pub type I2C_SLAVE_ADDR5_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `I2C_SLAVE_ADDR4` reader - RTC I2C slave address 4"]
pub type I2C_SLAVE_ADDR4_R = crate::FieldReader<u16>;
#[doc = "Field `I2C_SLAVE_ADDR4` writer - RTC I2C slave address 4"]
pub type I2C_SLAVE_ADDR4_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - RTC I2C slave address 5"]
    #[inline(always)]
    pub fn i2c_slave_addr5(&self) -> I2C_SLAVE_ADDR5_R {
        I2C_SLAVE_ADDR5_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - RTC I2C slave address 4"]
    #[inline(always)]
    pub fn i2c_slave_addr4(&self) -> I2C_SLAVE_ADDR4_R {
        I2C_SLAVE_ADDR4_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_SLAVE_ADDR3")
            .field("i2c_slave_addr5", &self.i2c_slave_addr5())
            .field("i2c_slave_addr4", &self.i2c_slave_addr4())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - RTC I2C slave address 5"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_slave_addr5(&mut self) -> I2C_SLAVE_ADDR5_W<SAR_SLAVE_ADDR3_SPEC> {
        I2C_SLAVE_ADDR5_W::new(self, 0)
    }
    #[doc = "Bits 11:21 - RTC I2C slave address 4"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_slave_addr4(&mut self) -> I2C_SLAVE_ADDR4_W<SAR_SLAVE_ADDR3_SPEC> {
        I2C_SLAVE_ADDR4_W::new(self, 11)
    }
}
#[doc = "Configure slave addresses 4-5 of RTC I2C\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_slave_addr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_slave_addr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_SLAVE_ADDR3_SPEC;
impl crate::RegisterSpec for SAR_SLAVE_ADDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_slave_addr3::R`](R) reader structure"]
impl crate::Readable for SAR_SLAVE_ADDR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_slave_addr3::W`](W) writer structure"]
impl crate::Writable for SAR_SLAVE_ADDR3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_SLAVE_ADDR3 to value 0"]
impl crate::Resettable for SAR_SLAVE_ADDR3_SPEC {
    const RESET_VALUE: u32 = 0;
}
