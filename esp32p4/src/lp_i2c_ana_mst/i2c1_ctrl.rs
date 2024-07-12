#[doc = "Register `I2C1_CTRL` reader"]
pub type R = crate::R<I2C1_CTRL_SPEC>;
#[doc = "Register `I2C1_CTRL` writer"]
pub type W = crate::W<I2C1_CTRL_SPEC>;
#[doc = "Field `I2C1_CTRL` reader - need des"]
pub type I2C1_CTRL_R = crate::FieldReader<u32>;
#[doc = "Field `I2C1_CTRL` writer - need des"]
pub type I2C1_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
#[doc = "Field `I2C1_BUSY` reader - need des"]
pub type I2C1_BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:24 - need des"]
    #[inline(always)]
    pub fn i2c1_ctrl(&self) -> I2C1_CTRL_R {
        I2C1_CTRL_R::new(self.bits & 0x01ff_ffff)
    }
    #[doc = "Bit 25 - need des"]
    #[inline(always)]
    pub fn i2c1_busy(&self) -> I2C1_BUSY_R {
        I2C1_BUSY_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C1_CTRL")
            .field("i2c1_ctrl", &self.i2c1_ctrl())
            .field("i2c1_busy", &self.i2c1_busy())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:24 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_ctrl(&mut self) -> I2C1_CTRL_W<I2C1_CTRL_SPEC> {
        I2C1_CTRL_W::new(self, 0)
    }
}
#[doc = "need des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C1_CTRL_SPEC;
impl crate::RegisterSpec for I2C1_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c1_ctrl::R`](R) reader structure"]
impl crate::Readable for I2C1_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c1_ctrl::W`](W) writer structure"]
impl crate::Writable for I2C1_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C1_CTRL to value 0"]
impl crate::Resettable for I2C1_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
