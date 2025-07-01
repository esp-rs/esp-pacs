#[doc = "Register `SAR_SLAVE_ADDR2` reader"]
pub type R = crate::R<SAR_SLAVE_ADDR2_SPEC>;
#[doc = "Register `SAR_SLAVE_ADDR2` writer"]
pub type W = crate::W<SAR_SLAVE_ADDR2_SPEC>;
#[doc = "Field `SAR_I2C_SLAVE_ADDR3` reader - configure i2c slave address3"]
pub type SAR_I2C_SLAVE_ADDR3_R = crate::FieldReader<u16>;
#[doc = "Field `SAR_I2C_SLAVE_ADDR3` writer - configure i2c slave address3"]
pub type SAR_I2C_SLAVE_ADDR3_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SAR_I2C_SLAVE_ADDR2` reader - configure i2c slave address2"]
pub type SAR_I2C_SLAVE_ADDR2_R = crate::FieldReader<u16>;
#[doc = "Field `SAR_I2C_SLAVE_ADDR2` writer - configure i2c slave address2"]
pub type SAR_I2C_SLAVE_ADDR2_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - configure i2c slave address3"]
    #[inline(always)]
    pub fn sar_i2c_slave_addr3(&self) -> SAR_I2C_SLAVE_ADDR3_R {
        SAR_I2C_SLAVE_ADDR3_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - configure i2c slave address2"]
    #[inline(always)]
    pub fn sar_i2c_slave_addr2(&self) -> SAR_I2C_SLAVE_ADDR2_R {
        SAR_I2C_SLAVE_ADDR2_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_SLAVE_ADDR2")
            .field("sar_i2c_slave_addr3", &self.sar_i2c_slave_addr3())
            .field("sar_i2c_slave_addr2", &self.sar_i2c_slave_addr2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - configure i2c slave address3"]
    #[inline(always)]
    pub fn sar_i2c_slave_addr3(&mut self) -> SAR_I2C_SLAVE_ADDR3_W<SAR_SLAVE_ADDR2_SPEC> {
        SAR_I2C_SLAVE_ADDR3_W::new(self, 0)
    }
    #[doc = "Bits 11:21 - configure i2c slave address2"]
    #[inline(always)]
    pub fn sar_i2c_slave_addr2(&mut self) -> SAR_I2C_SLAVE_ADDR2_W<SAR_SLAVE_ADDR2_SPEC> {
        SAR_I2C_SLAVE_ADDR2_W::new(self, 11)
    }
}
#[doc = "configure i2c slave address\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_slave_addr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_slave_addr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_SLAVE_ADDR2_SPEC;
impl crate::RegisterSpec for SAR_SLAVE_ADDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_slave_addr2::R`](R) reader structure"]
impl crate::Readable for SAR_SLAVE_ADDR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_slave_addr2::W`](W) writer structure"]
impl crate::Writable for SAR_SLAVE_ADDR2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR_SLAVE_ADDR2 to value 0"]
impl crate::Resettable for SAR_SLAVE_ADDR2_SPEC {}
