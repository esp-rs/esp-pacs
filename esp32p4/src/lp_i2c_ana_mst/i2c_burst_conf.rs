#[doc = "Register `I2C_BURST_CONF` reader"]
pub type R = crate::R<I2C_BURST_CONF_SPEC>;
#[doc = "Register `I2C_BURST_CONF` writer"]
pub type W = crate::W<I2C_BURST_CONF_SPEC>;
#[doc = "Field `I2C_MST_BURST_CTRL` reader - need des"]
pub type I2C_MST_BURST_CTRL_R = crate::FieldReader<u32>;
#[doc = "Field `I2C_MST_BURST_CTRL` writer - need des"]
pub type I2C_MST_BURST_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need des"]
    #[inline(always)]
    pub fn i2c_mst_burst_ctrl(&self) -> I2C_MST_BURST_CTRL_R {
        I2C_MST_BURST_CTRL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_BURST_CONF")
            .field("i2c_mst_burst_ctrl", &self.i2c_mst_burst_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mst_burst_ctrl(&mut self) -> I2C_MST_BURST_CTRL_W<I2C_BURST_CONF_SPEC> {
        I2C_MST_BURST_CTRL_W::new(self, 0)
    }
}
#[doc = "need des\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_burst_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_burst_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_BURST_CONF_SPEC;
impl crate::RegisterSpec for I2C_BURST_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_burst_conf::R`](R) reader structure"]
impl crate::Readable for I2C_BURST_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_burst_conf::W`](W) writer structure"]
impl crate::Writable for I2C_BURST_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_BURST_CONF to value 0"]
impl crate::Resettable for I2C_BURST_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
