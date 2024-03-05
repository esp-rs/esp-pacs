#[doc = "Register `RETENTION_CONF` reader"]
pub type R = crate::R<RETENTION_CONF_SPEC>;
#[doc = "Register `RETENTION_CONF` writer"]
pub type W = crate::W<RETENTION_CONF_SPEC>;
#[doc = "Field `RETENTION_DISABLE` reader - Set this bit as 1 to disable retention function. Not disable by default."]
pub type RETENTION_DISABLE_R = crate::BitReader;
#[doc = "Field `RETENTION_DISABLE` writer - Set this bit as 1 to disable retention function. Not disable by default."]
pub type RETENTION_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit as 1 to disable retention function. Not disable by default."]
    #[inline(always)]
    pub fn retention_disable(&self) -> RETENTION_DISABLE_R {
        RETENTION_DISABLE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RETENTION_CONF")
            .field(
                "retention_disable",
                &format_args!("{}", self.retention_disable().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RETENTION_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit as 1 to disable retention function. Not disable by default."]
    #[inline(always)]
    #[must_use]
    pub fn retention_disable(&mut self) -> RETENTION_DISABLE_W<RETENTION_CONF_SPEC> {
        RETENTION_DISABLE_W::new(self, 0)
    }
}
#[doc = "Retention configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`retention_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retention_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RETENTION_CONF_SPEC;
impl crate::RegisterSpec for RETENTION_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`retention_conf::R`](R) reader structure"]
impl crate::Readable for RETENTION_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`retention_conf::W`](W) writer structure"]
impl crate::Writable for RETENTION_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RETENTION_CONF to value 0"]
impl crate::Resettable for RETENTION_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
