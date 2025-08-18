#[doc = "Register `SAR_SLAVE_ADDR1` reader"]
pub type R = crate::R<SAR_SLAVE_ADDR1_SPEC>;
#[doc = "Register `SAR_SLAVE_ADDR1` writer"]
pub type W = crate::W<SAR_SLAVE_ADDR1_SPEC>;
#[doc = "Field `SAR_I2C_SLAVE_ADDR1` reader - configure i2c slave address1"]
pub type SAR_I2C_SLAVE_ADDR1_R = crate::FieldReader<u16>;
#[doc = "Field `SAR_I2C_SLAVE_ADDR1` writer - configure i2c slave address1"]
pub type SAR_I2C_SLAVE_ADDR1_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SAR_I2C_SLAVE_ADDR0` reader - configure i2c slave address0"]
pub type SAR_I2C_SLAVE_ADDR0_R = crate::FieldReader<u16>;
#[doc = "Field `SAR_I2C_SLAVE_ADDR0` writer - configure i2c slave address0"]
pub type SAR_I2C_SLAVE_ADDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SAR_SARADC_MEAS_STATUS` reader - no public"]
pub type SAR_SARADC_MEAS_STATUS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:10 - configure i2c slave address1"]
    #[inline(always)]
    pub fn sar_i2c_slave_addr1(&self) -> SAR_I2C_SLAVE_ADDR1_R {
        SAR_I2C_SLAVE_ADDR1_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - configure i2c slave address0"]
    #[inline(always)]
    pub fn sar_i2c_slave_addr0(&self) -> SAR_I2C_SLAVE_ADDR0_R {
        SAR_I2C_SLAVE_ADDR0_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bits 22:29 - no public"]
    #[inline(always)]
    pub fn sar_saradc_meas_status(&self) -> SAR_SARADC_MEAS_STATUS_R {
        SAR_SARADC_MEAS_STATUS_R::new(((self.bits >> 22) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_SLAVE_ADDR1")
            .field("sar_i2c_slave_addr1", &self.sar_i2c_slave_addr1())
            .field("sar_i2c_slave_addr0", &self.sar_i2c_slave_addr0())
            .field("sar_saradc_meas_status", &self.sar_saradc_meas_status())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - configure i2c slave address1"]
    #[inline(always)]
    pub fn sar_i2c_slave_addr1(&mut self) -> SAR_I2C_SLAVE_ADDR1_W<'_, SAR_SLAVE_ADDR1_SPEC> {
        SAR_I2C_SLAVE_ADDR1_W::new(self, 0)
    }
    #[doc = "Bits 11:21 - configure i2c slave address0"]
    #[inline(always)]
    pub fn sar_i2c_slave_addr0(&mut self) -> SAR_I2C_SLAVE_ADDR0_W<'_, SAR_SLAVE_ADDR1_SPEC> {
        SAR_I2C_SLAVE_ADDR0_W::new(self, 11)
    }
}
#[doc = "configure i2c slave address\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_slave_addr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_slave_addr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_SLAVE_ADDR1_SPEC;
impl crate::RegisterSpec for SAR_SLAVE_ADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_slave_addr1::R`](R) reader structure"]
impl crate::Readable for SAR_SLAVE_ADDR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_slave_addr1::W`](W) writer structure"]
impl crate::Writable for SAR_SLAVE_ADDR1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR_SLAVE_ADDR1 to value 0"]
impl crate::Resettable for SAR_SLAVE_ADDR1_SPEC {}
