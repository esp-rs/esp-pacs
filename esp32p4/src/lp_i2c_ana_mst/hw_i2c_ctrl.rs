#[doc = "Register `HW_I2C_CTRL` reader"]
pub type R = crate::R<HW_I2C_CTRL_SPEC>;
#[doc = "Register `HW_I2C_CTRL` writer"]
pub type W = crate::W<HW_I2C_CTRL_SPEC>;
#[doc = "Field `HW_I2C_SCL_PULSE_DUR` reader - need des"]
pub type HW_I2C_SCL_PULSE_DUR_R = crate::FieldReader;
#[doc = "Field `HW_I2C_SCL_PULSE_DUR` writer - need des"]
pub type HW_I2C_SCL_PULSE_DUR_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `HW_I2C_SDA_SIDE_GUARD` reader - need des"]
pub type HW_I2C_SDA_SIDE_GUARD_R = crate::FieldReader;
#[doc = "Field `HW_I2C_SDA_SIDE_GUARD` writer - need des"]
pub type HW_I2C_SDA_SIDE_GUARD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ARBITER_DIS` reader - need des"]
pub type ARBITER_DIS_R = crate::BitReader;
#[doc = "Field `ARBITER_DIS` writer - need des"]
pub type ARBITER_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - need des"]
    #[inline(always)]
    pub fn hw_i2c_scl_pulse_dur(&self) -> HW_I2C_SCL_PULSE_DUR_R {
        HW_I2C_SCL_PULSE_DUR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:10 - need des"]
    #[inline(always)]
    pub fn hw_i2c_sda_side_guard(&self) -> HW_I2C_SDA_SIDE_GUARD_R {
        HW_I2C_SDA_SIDE_GUARD_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn arbiter_dis(&self) -> ARBITER_DIS_R {
        ARBITER_DIS_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HW_I2C_CTRL")
            .field(
                "hw_i2c_scl_pulse_dur",
                &format_args!("{}", self.hw_i2c_scl_pulse_dur().bits()),
            )
            .field(
                "hw_i2c_sda_side_guard",
                &format_args!("{}", self.hw_i2c_sda_side_guard().bits()),
            )
            .field("arbiter_dis", &format_args!("{}", self.arbiter_dis().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HW_I2C_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:5 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn hw_i2c_scl_pulse_dur(&mut self) -> HW_I2C_SCL_PULSE_DUR_W<HW_I2C_CTRL_SPEC> {
        HW_I2C_SCL_PULSE_DUR_W::new(self, 0)
    }
    #[doc = "Bits 6:10 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn hw_i2c_sda_side_guard(&mut self) -> HW_I2C_SDA_SIDE_GUARD_W<HW_I2C_CTRL_SPEC> {
        HW_I2C_SDA_SIDE_GUARD_W::new(self, 6)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn arbiter_dis(&mut self) -> ARBITER_DIS_W<HW_I2C_CTRL_SPEC> {
        ARBITER_DIS_W::new(self, 11)
    }
}
#[doc = "need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hw_i2c_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hw_i2c_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HW_I2C_CTRL_SPEC;
impl crate::RegisterSpec for HW_I2C_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_i2c_ctrl::R`](R) reader structure"]
impl crate::Readable for HW_I2C_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hw_i2c_ctrl::W`](W) writer structure"]
impl crate::Writable for HW_I2C_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HW_I2C_CTRL to value 0x42"]
impl crate::Resettable for HW_I2C_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x42;
}
