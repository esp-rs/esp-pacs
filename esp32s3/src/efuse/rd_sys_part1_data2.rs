#[doc = "Register `RD_SYS_PART1_DATA2` reader"]
pub struct R(crate::R<RD_SYS_PART1_DATA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_SYS_PART1_DATA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_SYS_PART1_DATA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_SYS_PART1_DATA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYS_DATA_PART1_2` reader - Stores the second 32 bits of the first part of system data."]
pub type SYS_DATA_PART1_2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the second 32 bits of the first part of system data."]
    #[inline(always)]
    pub fn sys_data_part1_2(&self) -> SYS_DATA_PART1_2_R {
        SYS_DATA_PART1_2_R::new(self.bits)
    }
}
#[doc = "Register 2 of BLOCK2 (system).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_sys_part1_data2](index.html) module"]
pub struct RD_SYS_PART1_DATA2_SPEC;
impl crate::RegisterSpec for RD_SYS_PART1_DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_sys_part1_data2::R](R) reader structure"]
impl crate::Readable for RD_SYS_PART1_DATA2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_SYS_PART1_DATA2 to value 0"]
impl crate::Resettable for RD_SYS_PART1_DATA2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
