#[doc = "Register `Core_1_World_Phase` reader"]
pub struct R(crate::R<CORE_1_WORLD_PHASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_WORLD_PHASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_WORLD_PHASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_WORLD_PHASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_1_WORLD_PHASE` reader - This bit indicates whether is preparing to switch to WORLD1,1 means value."]
pub type CORE_1_WORLD_PHASE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - This bit indicates whether is preparing to switch to WORLD1,1 means value."]
    #[inline(always)]
    pub fn core_1_world_phase(&self) -> CORE_1_WORLD_PHASE_R {
        CORE_1_WORLD_PHASE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Core_0 world status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_world_phase](index.html) module"]
pub struct CORE_1_WORLD_PHASE_SPEC;
impl crate::RegisterSpec for CORE_1_WORLD_PHASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_world_phase::R](R) reader structure"]
impl crate::Readable for CORE_1_WORLD_PHASE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets Core_1_World_Phase to value 0"]
impl crate::Resettable for CORE_1_WORLD_PHASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
