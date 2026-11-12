#[doc = "Register `TICK_CONF` reader"]
pub type R = crate::R<TICK_CONF_SPEC>;
#[doc = "Field `PWR_TICK_TARGET` reader - "]
pub type PWR_TICK_TARGET_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 6:31"]
    #[inline(always)]
    pub fn pwr_tick_target(&self) -> PWR_TICK_TARGET_R {
        PWR_TICK_TARGET_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TICK_CONF")
            .field("pwr_tick_target", &self.pwr_tick_target())
            .finish()
    }
}
#[doc = "TICK_CONF\n\nYou can [`read`](crate::Reg::read) this register and get [`tick_conf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TICK_CONF_SPEC;
impl crate::RegisterSpec for TICK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tick_conf::R`](R) reader structure"]
impl crate::Readable for TICK_CONF_SPEC {}
#[doc = "`reset()` method sets TICK_CONF to value 0"]
impl crate::Resettable for TICK_CONF_SPEC {}
