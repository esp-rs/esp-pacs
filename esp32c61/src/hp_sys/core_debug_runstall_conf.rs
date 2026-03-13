#[doc = "Register `CORE_DEBUG_RUNSTALL_CONF` reader"]
pub type R = crate::R<CORE_DEBUG_RUNSTALL_CONF_SPEC>;
#[doc = "Field `CORE_RUNSTALLED` reader - Software can read this field to get the runstall status of hp-core. 1: stalled, 0: not stalled."]
pub type CORE_RUNSTALLED_R = crate::BitReader;
impl R {
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
            .field("core_runstalled", &self.core_runstalled())
            .finish()
    }
}
#[doc = "Core Debug RunStall configurion register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_debug_runstall_conf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_DEBUG_RUNSTALL_CONF_SPEC;
impl crate::RegisterSpec for CORE_DEBUG_RUNSTALL_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_debug_runstall_conf::R`](R) reader structure"]
impl crate::Readable for CORE_DEBUG_RUNSTALL_CONF_SPEC {}
#[doc = "`reset()` method sets CORE_DEBUG_RUNSTALL_CONF to value 0"]
impl crate::Resettable for CORE_DEBUG_RUNSTALL_CONF_SPEC {}
