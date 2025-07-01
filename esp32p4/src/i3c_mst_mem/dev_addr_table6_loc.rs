#[doc = "Register `DEV_ADDR_TABLE6_LOC` reader"]
pub type R = crate::R<DEV_ADDR_TABLE6_LOC_SPEC>;
#[doc = "Register `DEV_ADDR_TABLE6_LOC` writer"]
pub type W = crate::W<DEV_ADDR_TABLE6_LOC_SPEC>;
#[doc = "Field `REG_DAT_DEV6_STATIC_ADDR` reader - NA"]
pub type REG_DAT_DEV6_STATIC_ADDR_R = crate::FieldReader;
#[doc = "Field `REG_DAT_DEV6_STATIC_ADDR` writer - NA"]
pub type REG_DAT_DEV6_STATIC_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `REG_DAT_DEV6_DYNAMIC_ADDR` reader - Device Dynamic Address with parity, The MSB,bit\\[23\\], should be programmed with parity of dynamic address."]
pub type REG_DAT_DEV6_DYNAMIC_ADDR_R = crate::FieldReader;
#[doc = "Field `REG_DAT_DEV6_DYNAMIC_ADDR` writer - Device Dynamic Address with parity, The MSB,bit\\[23\\], should be programmed with parity of dynamic address."]
pub type REG_DAT_DEV6_DYNAMIC_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REG_DAT_DEV6_NACK_RETRY_CNT` reader - This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state."]
pub type REG_DAT_DEV6_NACK_RETRY_CNT_R = crate::FieldReader;
#[doc = "Field `REG_DAT_DEV6_NACK_RETRY_CNT` writer - This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state."]
pub type REG_DAT_DEV6_NACK_RETRY_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_DAT_DEV6_I2C` reader - Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device."]
pub type REG_DAT_DEV6_I2C_R = crate::BitReader;
#[doc = "Field `REG_DAT_DEV6_I2C` writer - Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device."]
pub type REG_DAT_DEV6_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - NA"]
    #[inline(always)]
    pub fn reg_dat_dev6_static_addr(&self) -> REG_DAT_DEV6_STATIC_ADDR_R {
        REG_DAT_DEV6_STATIC_ADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:23 - Device Dynamic Address with parity, The MSB,bit\\[23\\], should be programmed with parity of dynamic address."]
    #[inline(always)]
    pub fn reg_dat_dev6_dynamic_addr(&self) -> REG_DAT_DEV6_DYNAMIC_ADDR_R {
        REG_DAT_DEV6_DYNAMIC_ADDR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 29:30 - This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state."]
    #[inline(always)]
    pub fn reg_dat_dev6_nack_retry_cnt(&self) -> REG_DAT_DEV6_NACK_RETRY_CNT_R {
        REG_DAT_DEV6_NACK_RETRY_CNT_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device."]
    #[inline(always)]
    pub fn reg_dat_dev6_i2c(&self) -> REG_DAT_DEV6_I2C_R {
        REG_DAT_DEV6_I2C_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEV_ADDR_TABLE6_LOC")
            .field("reg_dat_dev6_static_addr", &self.reg_dat_dev6_static_addr())
            .field(
                "reg_dat_dev6_dynamic_addr",
                &self.reg_dat_dev6_dynamic_addr(),
            )
            .field(
                "reg_dat_dev6_nack_retry_cnt",
                &self.reg_dat_dev6_nack_retry_cnt(),
            )
            .field("reg_dat_dev6_i2c", &self.reg_dat_dev6_i2c())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - NA"]
    #[inline(always)]
    pub fn reg_dat_dev6_static_addr(
        &mut self,
    ) -> REG_DAT_DEV6_STATIC_ADDR_W<DEV_ADDR_TABLE6_LOC_SPEC> {
        REG_DAT_DEV6_STATIC_ADDR_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Device Dynamic Address with parity, The MSB,bit\\[23\\], should be programmed with parity of dynamic address."]
    #[inline(always)]
    pub fn reg_dat_dev6_dynamic_addr(
        &mut self,
    ) -> REG_DAT_DEV6_DYNAMIC_ADDR_W<DEV_ADDR_TABLE6_LOC_SPEC> {
        REG_DAT_DEV6_DYNAMIC_ADDR_W::new(self, 16)
    }
    #[doc = "Bits 29:30 - This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state."]
    #[inline(always)]
    pub fn reg_dat_dev6_nack_retry_cnt(
        &mut self,
    ) -> REG_DAT_DEV6_NACK_RETRY_CNT_W<DEV_ADDR_TABLE6_LOC_SPEC> {
        REG_DAT_DEV6_NACK_RETRY_CNT_W::new(self, 29)
    }
    #[doc = "Bit 31 - Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device."]
    #[inline(always)]
    pub fn reg_dat_dev6_i2c(&mut self) -> REG_DAT_DEV6_I2C_W<DEV_ADDR_TABLE6_LOC_SPEC> {
        REG_DAT_DEV6_I2C_W::new(self, 31)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_addr_table6_loc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_addr_table6_loc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEV_ADDR_TABLE6_LOC_SPEC;
impl crate::RegisterSpec for DEV_ADDR_TABLE6_LOC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_addr_table6_loc::R`](R) reader structure"]
impl crate::Readable for DEV_ADDR_TABLE6_LOC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dev_addr_table6_loc::W`](W) writer structure"]
impl crate::Writable for DEV_ADDR_TABLE6_LOC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV_ADDR_TABLE6_LOC to value 0"]
impl crate::Resettable for DEV_ADDR_TABLE6_LOC_SPEC {}
