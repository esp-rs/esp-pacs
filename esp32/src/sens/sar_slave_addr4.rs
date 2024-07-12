#[doc = "Register `SAR_SLAVE_ADDR4` reader"]
pub type R = crate::R<SAR_SLAVE_ADDR4_SPEC>;
#[doc = "Register `SAR_SLAVE_ADDR4` writer"]
pub type W = crate::W<SAR_SLAVE_ADDR4_SPEC>;
#[doc = "Field `I2C_SLAVE_ADDR7` reader - "]
pub type I2C_SLAVE_ADDR7_R = crate::FieldReader<u16>;
#[doc = "Field `I2C_SLAVE_ADDR7` writer - "]
pub type I2C_SLAVE_ADDR7_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `I2C_SLAVE_ADDR6` reader - "]
pub type I2C_SLAVE_ADDR6_R = crate::FieldReader<u16>;
#[doc = "Field `I2C_SLAVE_ADDR6` writer - "]
pub type I2C_SLAVE_ADDR6_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `I2C_RDATA` reader - I2C read data"]
pub type I2C_RDATA_R = crate::FieldReader;
#[doc = "Field `I2C_DONE` reader - indicate I2C done"]
pub type I2C_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn i2c_slave_addr7(&self) -> I2C_SLAVE_ADDR7_R {
        I2C_SLAVE_ADDR7_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21"]
    #[inline(always)]
    pub fn i2c_slave_addr6(&self) -> I2C_SLAVE_ADDR6_R {
        I2C_SLAVE_ADDR6_R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bits 22:29 - I2C read data"]
    #[inline(always)]
    pub fn i2c_rdata(&self) -> I2C_RDATA_R {
        I2C_RDATA_R::new(((self.bits >> 22) & 0xff) as u8)
    }
    #[doc = "Bit 30 - indicate I2C done"]
    #[inline(always)]
    pub fn i2c_done(&self) -> I2C_DONE_R {
        I2C_DONE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_SLAVE_ADDR4")
            .field("i2c_slave_addr7", &self.i2c_slave_addr7())
            .field("i2c_slave_addr6", &self.i2c_slave_addr6())
            .field("i2c_rdata", &self.i2c_rdata())
            .field("i2c_done", &self.i2c_done())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_slave_addr7(&mut self) -> I2C_SLAVE_ADDR7_W<SAR_SLAVE_ADDR4_SPEC> {
        I2C_SLAVE_ADDR7_W::new(self, 0)
    }
    #[doc = "Bits 11:21"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_slave_addr6(&mut self) -> I2C_SLAVE_ADDR6_W<SAR_SLAVE_ADDR4_SPEC> {
        I2C_SLAVE_ADDR6_W::new(self, 11)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_slave_addr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_slave_addr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_SLAVE_ADDR4_SPEC;
impl crate::RegisterSpec for SAR_SLAVE_ADDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_slave_addr4::R`](R) reader structure"]
impl crate::Readable for SAR_SLAVE_ADDR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_slave_addr4::W`](W) writer structure"]
impl crate::Writable for SAR_SLAVE_ADDR4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_SLAVE_ADDR4 to value 0"]
impl crate::Resettable for SAR_SLAVE_ADDR4_SPEC {
    const RESET_VALUE: u32 = 0;
}
