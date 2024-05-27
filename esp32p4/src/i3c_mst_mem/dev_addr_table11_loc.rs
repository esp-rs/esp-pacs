///Register `DEV_ADDR_TABLE11_LOC` reader
pub type R = crate::R<DEV_ADDR_TABLE11_LOC_SPEC>;
///Register `DEV_ADDR_TABLE11_LOC` writer
pub type W = crate::W<DEV_ADDR_TABLE11_LOC_SPEC>;
///Field `REG_DAT_DEV11_STATIC_ADDR` reader - NA
pub type REG_DAT_DEV11_STATIC_ADDR_R = crate::FieldReader;
///Field `REG_DAT_DEV11_STATIC_ADDR` writer - NA
pub type REG_DAT_DEV11_STATIC_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `REG_DAT_DEV11_DYNAMIC_ADDR` reader - Device Dynamic Address with parity, The MSB,bit\[23\], should be programmed with parity of dynamic address.
pub type REG_DAT_DEV11_DYNAMIC_ADDR_R = crate::FieldReader;
///Field `REG_DAT_DEV11_DYNAMIC_ADDR` writer - Device Dynamic Address with parity, The MSB,bit\[23\], should be programmed with parity of dynamic address.
pub type REG_DAT_DEV11_DYNAMIC_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `REG_DAT_DEV11_NACK_RETRY_CNT` reader - This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
pub type REG_DAT_DEV11_NACK_RETRY_CNT_R = crate::FieldReader;
///Field `REG_DAT_DEV11_NACK_RETRY_CNT` writer - This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
pub type REG_DAT_DEV11_NACK_RETRY_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `REG_DAT_DEV11_I2C` reader - Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
pub type REG_DAT_DEV11_I2C_R = crate::BitReader;
///Field `REG_DAT_DEV11_I2C` writer - Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
pub type REG_DAT_DEV11_I2C_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:6 - NA
    #[inline(always)]
    pub fn reg_dat_dev11_static_addr(&self) -> REG_DAT_DEV11_STATIC_ADDR_R {
        REG_DAT_DEV11_STATIC_ADDR_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 16:23 - Device Dynamic Address with parity, The MSB,bit\[23\], should be programmed with parity of dynamic address.
    #[inline(always)]
    pub fn reg_dat_dev11_dynamic_addr(&self) -> REG_DAT_DEV11_DYNAMIC_ADDR_R {
        REG_DAT_DEV11_DYNAMIC_ADDR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 29:30 - This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    pub fn reg_dat_dev11_nack_retry_cnt(&self) -> REG_DAT_DEV11_NACK_RETRY_CNT_R {
        REG_DAT_DEV11_NACK_RETRY_CNT_R::new(((self.bits >> 29) & 3) as u8)
    }
    ///Bit 31 - Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    pub fn reg_dat_dev11_i2c(&self) -> REG_DAT_DEV11_I2C_R {
        REG_DAT_DEV11_I2C_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEV_ADDR_TABLE11_LOC")
            .field(
                "reg_dat_dev11_static_addr",
                &self.reg_dat_dev11_static_addr(),
            )
            .field(
                "reg_dat_dev11_dynamic_addr",
                &self.reg_dat_dev11_dynamic_addr(),
            )
            .field(
                "reg_dat_dev11_nack_retry_cnt",
                &self.reg_dat_dev11_nack_retry_cnt(),
            )
            .field("reg_dat_dev11_i2c", &self.reg_dat_dev11_i2c())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - NA
    #[inline(always)]
    #[must_use]
    pub fn reg_dat_dev11_static_addr(
        &mut self,
    ) -> REG_DAT_DEV11_STATIC_ADDR_W<DEV_ADDR_TABLE11_LOC_SPEC> {
        REG_DAT_DEV11_STATIC_ADDR_W::new(self, 0)
    }
    ///Bits 16:23 - Device Dynamic Address with parity, The MSB,bit\[23\], should be programmed with parity of dynamic address.
    #[inline(always)]
    #[must_use]
    pub fn reg_dat_dev11_dynamic_addr(
        &mut self,
    ) -> REG_DAT_DEV11_DYNAMIC_ADDR_W<DEV_ADDR_TABLE11_LOC_SPEC> {
        REG_DAT_DEV11_DYNAMIC_ADDR_W::new(self, 16)
    }
    ///Bits 29:30 - This field is used to set the Device NACK Retry count for the particular device. If the Device NACK's for the device address, the controller automatically retries the same device until this count expires. If the Slave does not ACK for the mentioned number of retries, then controller generates an error response and move to the Halt state.
    #[inline(always)]
    #[must_use]
    pub fn reg_dat_dev11_nack_retry_cnt(
        &mut self,
    ) -> REG_DAT_DEV11_NACK_RETRY_CNT_W<DEV_ADDR_TABLE11_LOC_SPEC> {
        REG_DAT_DEV11_NACK_RETRY_CNT_W::new(self, 29)
    }
    ///Bit 31 - Legacy I2C device or not. This bit should be set to 1 if the device is a legacy I2C device.
    #[inline(always)]
    #[must_use]
    pub fn reg_dat_dev11_i2c(&mut self) -> REG_DAT_DEV11_I2C_W<DEV_ADDR_TABLE11_LOC_SPEC> {
        REG_DAT_DEV11_I2C_W::new(self, 31)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`dev_addr_table11_loc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dev_addr_table11_loc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DEV_ADDR_TABLE11_LOC_SPEC;
impl crate::RegisterSpec for DEV_ADDR_TABLE11_LOC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dev_addr_table11_loc::R`](R) reader structure
impl crate::Readable for DEV_ADDR_TABLE11_LOC_SPEC {}
///`write(|w| ..)` method takes [`dev_addr_table11_loc::W`](W) writer structure
impl crate::Writable for DEV_ADDR_TABLE11_LOC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DEV_ADDR_TABLE11_LOC to value 0
impl crate::Resettable for DEV_ADDR_TABLE11_LOC_SPEC {
    const RESET_VALUE: u32 = 0;
}
