///Register `SAR_I2C_CTRL` reader
pub type R = crate::R<SAR_I2C_CTRL_SPEC>;
///Register `SAR_I2C_CTRL` writer
pub type W = crate::W<SAR_I2C_CTRL_SPEC>;
///Field `SAR_I2C_CTRL` reader - I2C control data only active when reg_sar_i2c_start_force = 1
pub type SAR_I2C_CTRL_R = crate::FieldReader<u32>;
///Field `SAR_I2C_CTRL` writer - I2C control data only active when reg_sar_i2c_start_force = 1
pub type SAR_I2C_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
///Field `SAR_I2C_START` reader - start I2C only active when reg_sar_i2c_start_force = 1
pub type SAR_I2C_START_R = crate::BitReader;
///Field `SAR_I2C_START` writer - start I2C only active when reg_sar_i2c_start_force = 1
pub type SAR_I2C_START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAR_I2C_START_FORCE` reader - 1: I2C started by SW 0: I2C started by FSM
pub type SAR_I2C_START_FORCE_R = crate::BitReader;
///Field `SAR_I2C_START_FORCE` writer - 1: I2C started by SW 0: I2C started by FSM
pub type SAR_I2C_START_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:27 - I2C control data only active when reg_sar_i2c_start_force = 1
    #[inline(always)]
    pub fn sar_i2c_ctrl(&self) -> SAR_I2C_CTRL_R {
        SAR_I2C_CTRL_R::new(self.bits & 0x0fff_ffff)
    }
    ///Bit 28 - start I2C only active when reg_sar_i2c_start_force = 1
    #[inline(always)]
    pub fn sar_i2c_start(&self) -> SAR_I2C_START_R {
        SAR_I2C_START_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - 1: I2C started by SW 0: I2C started by FSM
    #[inline(always)]
    pub fn sar_i2c_start_force(&self) -> SAR_I2C_START_FORCE_R {
        SAR_I2C_START_FORCE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_I2C_CTRL")
            .field("sar_i2c_ctrl", &self.sar_i2c_ctrl())
            .field("sar_i2c_start", &self.sar_i2c_start())
            .field("sar_i2c_start_force", &self.sar_i2c_start_force())
            .finish()
    }
}
impl W {
    ///Bits 0:27 - I2C control data only active when reg_sar_i2c_start_force = 1
    #[inline(always)]
    #[must_use]
    pub fn sar_i2c_ctrl(&mut self) -> SAR_I2C_CTRL_W<SAR_I2C_CTRL_SPEC> {
        SAR_I2C_CTRL_W::new(self, 0)
    }
    ///Bit 28 - start I2C only active when reg_sar_i2c_start_force = 1
    #[inline(always)]
    #[must_use]
    pub fn sar_i2c_start(&mut self) -> SAR_I2C_START_W<SAR_I2C_CTRL_SPEC> {
        SAR_I2C_START_W::new(self, 28)
    }
    ///Bit 29 - 1: I2C started by SW 0: I2C started by FSM
    #[inline(always)]
    #[must_use]
    pub fn sar_i2c_start_force(&mut self) -> SAR_I2C_START_FORCE_W<SAR_I2C_CTRL_SPEC> {
        SAR_I2C_START_FORCE_W::new(self, 29)
    }
}
/**configure rtc i2c controller by sw

You can [`read`](crate::generic::Reg::read) this register and get [`sar_i2c_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_i2c_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR_I2C_CTRL_SPEC;
impl crate::RegisterSpec for SAR_I2C_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar_i2c_ctrl::R`](R) reader structure
impl crate::Readable for SAR_I2C_CTRL_SPEC {}
///`write(|w| ..)` method takes [`sar_i2c_ctrl::W`](W) writer structure
impl crate::Writable for SAR_I2C_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SAR_I2C_CTRL to value 0
impl crate::Resettable for SAR_I2C_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
