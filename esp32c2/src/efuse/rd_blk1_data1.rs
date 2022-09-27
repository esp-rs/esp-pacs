#[doc = "Register `RD_BLK1_DATA1` reader"]
pub struct R(crate::R<RD_BLK1_DATA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_BLK1_DATA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_BLK1_DATA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_BLK1_DATA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYSTEM_DATA1` reader - Stores the bits \\[32:63\\] of system data."]
pub type SYSTEM_DATA1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the bits \\[32:63\\] of system data."]
    #[inline(always)]
    pub fn system_data1(&self) -> SYSTEM_DATA1_R {
        SYSTEM_DATA1_R::new(self.bits)
    }
}
#[doc = "BLOCK1 data register 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_blk1_data1](index.html) module"]
pub struct RD_BLK1_DATA1_SPEC;
impl crate::RegisterSpec for RD_BLK1_DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_blk1_data1::R](R) reader structure"]
impl crate::Readable for RD_BLK1_DATA1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_BLK1_DATA1 to value 0"]
impl crate::Resettable for RD_BLK1_DATA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
