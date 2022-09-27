#[doc = "Register `CORE_0_AREA_PC` reader"]
pub struct R(crate::R<CORE_0_AREA_PC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_AREA_PC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_AREA_PC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_AREA_PC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_AREA_PC` reader - reg_core_0_area_pc"]
pub type CORE_0_AREA_PC_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - reg_core_0_area_pc"]
    #[inline(always)]
    pub fn core_0_area_pc(&self) -> CORE_0_AREA_PC_R {
        CORE_0_AREA_PC_R::new(self.bits)
    }
}
#[doc = "ASSIST_DEBUG_CORE_0_AREA_PC_REG\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_area_pc](index.html) module"]
pub struct CORE_0_AREA_PC_SPEC;
impl crate::RegisterSpec for CORE_0_AREA_PC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_area_pc::R](R) reader structure"]
impl crate::Readable for CORE_0_AREA_PC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_0_AREA_PC to value 0"]
impl crate::Resettable for CORE_0_AREA_PC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
