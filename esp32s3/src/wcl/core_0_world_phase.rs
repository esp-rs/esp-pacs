#[doc = "Register `Core_0_World_Phase` reader"]
pub type R = crate::R<CORE_0_WORLD_PHASE_SPEC>;
#[doc = "Field `CORE_0_WORLD_PHASE` reader - This bit indicates whether is preparing to switch to WORLD1, 1 means value."]
pub type CORE_0_WORLD_PHASE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit indicates whether is preparing to switch to WORLD1, 1 means value."]
    #[inline(always)]
    pub fn core_0_world_phase(&self) -> CORE_0_WORLD_PHASE_R {
        CORE_0_WORLD_PHASE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_0_World_Phase")
            .field("core_0_world_phase", &self.core_0_world_phase())
            .finish()
    }
}
#[doc = "Core_0 world status register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_world_phase::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_WORLD_PHASE_SPEC;
impl crate::RegisterSpec for CORE_0_WORLD_PHASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_world_phase::R`](R) reader structure"]
impl crate::Readable for CORE_0_WORLD_PHASE_SPEC {}
#[doc = "`reset()` method sets Core_0_World_Phase to value 0"]
impl crate::Resettable for CORE_0_WORLD_PHASE_SPEC {
    const RESET_VALUE: u32 = 0;
}
