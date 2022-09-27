#[doc = "Register `CORE_1_SP_PC` reader"]
pub struct R(crate::R<CORE_1_SP_PC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_SP_PC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_SP_PC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_SP_PC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_1_SP_PC` reader - the PC when first touch stack monitor interrupt"]
pub type CORE_1_SP_PC_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - the PC when first touch stack monitor interrupt"]
    #[inline(always)]
    pub fn core_1_sp_pc(&self) -> CORE_1_SP_PC_R {
        CORE_1_SP_PC_R::new(self.bits)
    }
}
#[doc = "Core1 sp pc status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_sp_pc](index.html) module"]
pub struct CORE_1_SP_PC_SPEC;
impl crate::RegisterSpec for CORE_1_SP_PC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_sp_pc::R](R) reader structure"]
impl crate::Readable for CORE_1_SP_PC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_1_SP_PC to value 0"]
impl crate::Resettable for CORE_1_SP_PC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
