///Register `I2C1_CTRL1` reader
pub type R = crate::R<I2C1_CTRL1_SPEC>;
///Register `I2C1_CTRL1` writer
pub type W = crate::W<I2C1_CTRL1_SPEC>;
///Field `I2C1_SCL_PULSE_DUR` reader - need des
pub type I2C1_SCL_PULSE_DUR_R = crate::FieldReader;
///Field `I2C1_SCL_PULSE_DUR` writer - need des
pub type I2C1_SCL_PULSE_DUR_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `I2C1_SDA_SIDE_GUARD` reader - need des
pub type I2C1_SDA_SIDE_GUARD_R = crate::FieldReader;
///Field `I2C1_SDA_SIDE_GUARD` writer - need des
pub type I2C1_SDA_SIDE_GUARD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:5 - need des
    #[inline(always)]
    pub fn i2c1_scl_pulse_dur(&self) -> I2C1_SCL_PULSE_DUR_R {
        I2C1_SCL_PULSE_DUR_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:10 - need des
    #[inline(always)]
    pub fn i2c1_sda_side_guard(&self) -> I2C1_SDA_SIDE_GUARD_R {
        I2C1_SDA_SIDE_GUARD_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C1_CTRL1")
            .field("i2c1_scl_pulse_dur", &self.i2c1_scl_pulse_dur())
            .field("i2c1_sda_side_guard", &self.i2c1_sda_side_guard())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - need des
    #[inline(always)]
    #[must_use]
    pub fn i2c1_scl_pulse_dur(&mut self) -> I2C1_SCL_PULSE_DUR_W<I2C1_CTRL1_SPEC> {
        I2C1_SCL_PULSE_DUR_W::new(self, 0)
    }
    ///Bits 6:10 - need des
    #[inline(always)]
    #[must_use]
    pub fn i2c1_sda_side_guard(&mut self) -> I2C1_SDA_SIDE_GUARD_W<I2C1_CTRL1_SPEC> {
        I2C1_SDA_SIDE_GUARD_W::new(self, 6)
    }
}
/**need des

You can [`read`](crate::generic::Reg::read) this register and get [`i2c1_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c1_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct I2C1_CTRL1_SPEC;
impl crate::RegisterSpec for I2C1_CTRL1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`i2c1_ctrl1::R`](R) reader structure
impl crate::Readable for I2C1_CTRL1_SPEC {}
///`write(|w| ..)` method takes [`i2c1_ctrl1::W`](W) writer structure
impl crate::Writable for I2C1_CTRL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets I2C1_CTRL1 to value 0x42
impl crate::Resettable for I2C1_CTRL1_SPEC {
    const RESET_VALUE: u32 = 0x42;
}
