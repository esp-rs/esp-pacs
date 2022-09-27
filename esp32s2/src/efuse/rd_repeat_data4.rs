#[doc = "Register `RD_REPEAT_DATA4` reader"]
pub struct R(crate::R<RD_REPEAT_DATA4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_REPEAT_DATA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_REPEAT_DATA4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_REPEAT_DATA4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RPT4_RESERVED4` reader - Reserved (used for four backups method)."]
pub type RPT4_RESERVED4_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Reserved (used for four backups method)."]
    #[inline(always)]
    pub fn rpt4_reserved4(&self) -> RPT4_RESERVED4_R {
        RPT4_RESERVED4_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
#[doc = "Register 5 of BLOCK0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_data4](index.html) module"]
pub struct RD_REPEAT_DATA4_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_repeat_data4::R](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_REPEAT_DATA4 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
