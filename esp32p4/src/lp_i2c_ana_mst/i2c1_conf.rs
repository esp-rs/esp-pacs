#[doc = "Register `I2C1_CONF` reader"]
pub type R = crate::R<I2C1_CONF_SPEC>;
#[doc = "Register `I2C1_CONF` writer"]
pub type W = crate::W<I2C1_CONF_SPEC>;
#[doc = "Field `I2C1_CONF` reader - need des"]
pub type I2C1_CONF_R = crate::FieldReader<u32>;
#[doc = "Field `I2C1_CONF` writer - need des"]
pub type I2C1_CONF_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `I2C1_STATUS` reader - need des"]
pub type I2C1_STATUS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - need des"]
    #[inline(always)]
    pub fn i2c1_conf(&self) -> I2C1_CONF_R {
        I2C1_CONF_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - need des"]
    #[inline(always)]
    pub fn i2c1_status(&self) -> I2C1_STATUS_R {
        I2C1_STATUS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C1_CONF")
            .field("i2c1_conf", &self.i2c1_conf().bits())
            .field("i2c1_status", &self.i2c1_status().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C1_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:23 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_conf(&mut self) -> I2C1_CONF_W<I2C1_CONF_SPEC> {
        I2C1_CONF_W::new(self, 0)
    }
}
#[doc = "need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c1_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c1_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C1_CONF_SPEC;
impl crate::RegisterSpec for I2C1_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c1_conf::R`](R) reader structure"]
impl crate::Readable for I2C1_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c1_conf::W`](W) writer structure"]
impl crate::Writable for I2C1_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C1_CONF to value 0"]
impl crate::Resettable for I2C1_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
