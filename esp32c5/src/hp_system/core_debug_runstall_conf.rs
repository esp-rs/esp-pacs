#[doc = "Register `CORE_DEBUG_RUNSTALL_CONF` reader"]
pub type R = crate::R<CORE_DEBUG_RUNSTALL_CONF_SPEC>;
#[doc = "Register `CORE_DEBUG_RUNSTALL_CONF` writer"]
pub type W = crate::W<CORE_DEBUG_RUNSTALL_CONF_SPEC>;
#[doc = "Field `CORE_DEBUG_RUNSTALL_ENABLE` reader - Configures whether or not to enable debug RunStall functionality between HP CPU and LP CPU.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type CORE_DEBUG_RUNSTALL_ENABLE_R = crate::BitReader;
#[doc = "Field `CORE_DEBUG_RUNSTALL_ENABLE` writer - Configures whether or not to enable debug RunStall functionality between HP CPU and LP CPU.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
pub type CORE_DEBUG_RUNSTALL_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE_RUNSTALLED` reader - Software can read this field to get the runstall status of hp-core. 1: stalled, 0: not stalled."]
pub type CORE_RUNSTALLED_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enable debug RunStall functionality between HP CPU and LP CPU.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn core_debug_runstall_enable(&self) -> CORE_DEBUG_RUNSTALL_ENABLE_R {
        CORE_DEBUG_RUNSTALL_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software can read this field to get the runstall status of hp-core. 1: stalled, 0: not stalled."]
    #[inline(always)]
    pub fn core_runstalled(&self) -> CORE_RUNSTALLED_R {
        CORE_RUNSTALLED_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_DEBUG_RUNSTALL_CONF")
            .field(
                "core_debug_runstall_enable",
                &self.core_debug_runstall_enable(),
            )
            .field("core_runstalled", &self.core_runstalled())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable debug RunStall functionality between HP CPU and LP CPU.\\\\ 0: Disable\\\\ 1: Enable\\\\"]
    #[inline(always)]
    pub fn core_debug_runstall_enable(
        &mut self,
    ) -> CORE_DEBUG_RUNSTALL_ENABLE_W<CORE_DEBUG_RUNSTALL_CONF_SPEC> {
        CORE_DEBUG_RUNSTALL_ENABLE_W::new(self, 0)
    }
}
#[doc = "Core Debug RunStall configurion register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_debug_runstall_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_debug_runstall_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_DEBUG_RUNSTALL_CONF_SPEC;
impl crate::RegisterSpec for CORE_DEBUG_RUNSTALL_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_debug_runstall_conf::R`](R) reader structure"]
impl crate::Readable for CORE_DEBUG_RUNSTALL_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_debug_runstall_conf::W`](W) writer structure"]
impl crate::Writable for CORE_DEBUG_RUNSTALL_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE_DEBUG_RUNSTALL_CONF to value 0"]
impl crate::Resettable for CORE_DEBUG_RUNSTALL_CONF_SPEC {}
