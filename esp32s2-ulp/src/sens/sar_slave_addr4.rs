#[doc = "Register `SAR_SLAVE_ADDR4` reader"]
pub type R = crate::R<SAR_SLAVE_ADDR4_SPEC>;
#[doc = "Register `SAR_SLAVE_ADDR4` writer"]
pub type W = crate::W<SAR_SLAVE_ADDR4_SPEC>;
#[doc = "Field `I2C_SLAVE_ADDR7` reader - RTC I2C slave address 7"]
pub type I2C_SLAVE_ADDR7_R = crate::FieldReader<u16>;
#[doc = "Field `I2C_SLAVE_ADDR7` writer - RTC I2C slave address 7"]
pub type I2C_SLAVE_ADDR7_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `I2C_SLAVE_ADDR6` reader - RTC I2C slave address 6"]
pub type I2C_SLAVE_ADDR6_R = crate::FieldReader<u16>;
#[doc = "Field `I2C_SLAVE_ADDR6` writer - RTC I2C slave address 6"]
pub type I2C_SLAVE_ADDR6_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - RTC I2C slave address 7"]
    #[inline(always)]
    pub fn i2c_slave_addr7(&self) -> I2C_SLAVE_ADDR7_R {
        I2C_SLAVE_ADDR7_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - RTC I2C slave address 6"]
    #[inline(always)]
    pub fn i2c_slave_addr6(&self) -> I2C_SLAVE_ADDR6_R {
        I2C_SLAVE_ADDR6_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_SLAVE_ADDR4")
            .field("i2c_slave_addr7", &self.i2c_slave_addr7())
            .field("i2c_slave_addr6", &self.i2c_slave_addr6())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - RTC I2C slave address 7"]
    #[inline(always)]
    pub fn i2c_slave_addr7(&mut self) -> I2C_SLAVE_ADDR7_W<SAR_SLAVE_ADDR4_SPEC> {
        I2C_SLAVE_ADDR7_W::new(self, 0)
    }
    #[doc = "Bits 11:21 - RTC I2C slave address 6"]
    #[inline(always)]
    pub fn i2c_slave_addr6(&mut self) -> I2C_SLAVE_ADDR6_W<SAR_SLAVE_ADDR4_SPEC> {
        I2C_SLAVE_ADDR6_W::new(self, 11)
    }
}
#[doc = "Configure slave addresses 6-7 of RTC I2C\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_slave_addr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_slave_addr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_SLAVE_ADDR4_SPEC;
impl crate::RegisterSpec for SAR_SLAVE_ADDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_slave_addr4::R`](R) reader structure"]
impl crate::Readable for SAR_SLAVE_ADDR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_slave_addr4::W`](W) writer structure"]
impl crate::Writable for SAR_SLAVE_ADDR4_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR_SLAVE_ADDR4 to value 0"]
impl crate::Resettable for SAR_SLAVE_ADDR4_SPEC {}
