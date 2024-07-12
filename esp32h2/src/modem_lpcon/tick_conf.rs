#[doc = "Register `TICK_CONF` reader"]
pub type R = crate::R<TICK_CONF_SPEC>;
#[doc = "Register `TICK_CONF` writer"]
pub type W = crate::W<TICK_CONF_SPEC>;
#[doc = "Field `PWR_TICK_TARGET` reader - "]
pub type PWR_TICK_TARGET_R = crate::FieldReader;
#[doc = "Field `PWR_TICK_TARGET` writer - "]
pub type PWR_TICK_TARGET_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn pwr_tick_target(&self) -> PWR_TICK_TARGET_R {
        PWR_TICK_TARGET_R::new((self.bits & 0x3f) as u8)
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
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn pwr_tick_target(&mut self) -> PWR_TICK_TARGET_W<TICK_CONF_SPEC> {
        PWR_TICK_TARGET_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tick_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tick_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TICK_CONF_SPEC;
impl crate::RegisterSpec for TICK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tick_conf::R`](R) reader structure"]
impl crate::Readable for TICK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tick_conf::W`](W) writer structure"]
impl crate::Writable for TICK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TICK_CONF to value 0x1f"]
impl crate::Resettable for TICK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x1f;
}
