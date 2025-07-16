#[doc = "Register `WORLD_PHASE` reader"]
pub type R = crate::R<WORLD_PHASE_SPEC>;
#[doc = "Field `WORLD_PHASE` reader - This bit indicates whether is preparing to switch to WORLD1, 1 means value."]
pub type WORLD_PHASE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit indicates whether is preparing to switch to WORLD1, 1 means value."]
    #[inline(always)]
    pub fn world_phase(&self) -> WORLD_PHASE_R {
        WORLD_PHASE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WORLD_PHASE")
            .field("world_phase", &self.world_phase())
            .finish()
    }
}
#[doc = "Core_0 world status register\n\nYou can [`read`](crate::Reg::read) this register and get [`world_phase::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WORLD_PHASE_SPEC;
impl crate::RegisterSpec for WORLD_PHASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`world_phase::R`](R) reader structure"]
impl crate::Readable for WORLD_PHASE_SPEC {}
#[doc = "`reset()` method sets WORLD_PHASE to value 0"]
impl crate::Resettable for WORLD_PHASE_SPEC {}
