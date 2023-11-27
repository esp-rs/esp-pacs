#[doc = "Register `I2C0_CTRL1` reader"]
pub type R = crate::R<I2C0_CTRL1_SPEC>;
#[doc = "Register `I2C0_CTRL1` writer"]
pub type W = crate::W<I2C0_CTRL1_SPEC>;
#[doc = "Field `I2C0_SCL_PULSE_DUR` reader - need des"]
pub type I2C0_SCL_PULSE_DUR_R = crate::FieldReader;
#[doc = "Field `I2C0_SCL_PULSE_DUR` writer - need des"]
pub type I2C0_SCL_PULSE_DUR_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `I2C0_SDA_SIDE_GUARD` reader - need des"]
pub type I2C0_SDA_SIDE_GUARD_R = crate::FieldReader;
#[doc = "Field `I2C0_SDA_SIDE_GUARD` writer - need des"]
pub type I2C0_SDA_SIDE_GUARD_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:5 - need des"]
    #[inline(always)]
    pub fn i2c0_scl_pulse_dur(&self) -> I2C0_SCL_PULSE_DUR_R {
        I2C0_SCL_PULSE_DUR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:10 - need des"]
    #[inline(always)]
    pub fn i2c0_sda_side_guard(&self) -> I2C0_SDA_SIDE_GUARD_R {
        I2C0_SDA_SIDE_GUARD_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C0_CTRL1")
            .field(
                "i2c0_scl_pulse_dur",
                &format_args!("{}", self.i2c0_scl_pulse_dur().bits()),
            )
            .field(
                "i2c0_sda_side_guard",
                &format_args!("{}", self.i2c0_sda_side_guard().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C0_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:5 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0_scl_pulse_dur(&mut self) -> I2C0_SCL_PULSE_DUR_W<I2C0_CTRL1_SPEC> {
        I2C0_SCL_PULSE_DUR_W::new(self, 0)
    }
    #[doc = "Bits 6:10 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0_sda_side_guard(&mut self) -> I2C0_SDA_SIDE_GUARD_W<I2C0_CTRL1_SPEC> {
        I2C0_SDA_SIDE_GUARD_W::new(self, 6)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c0_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c0_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C0_CTRL1_SPEC;
impl crate::RegisterSpec for I2C0_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c0_ctrl1::R`](R) reader structure"]
impl crate::Readable for I2C0_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c0_ctrl1::W`](W) writer structure"]
impl crate::Writable for I2C0_CTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C0_CTRL1 to value 0x42"]
impl crate::Resettable for I2C0_CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x42;
}
