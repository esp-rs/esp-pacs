#[doc = "Register `RD_BLK3_DATA2` reader"]
pub struct R(crate::R<RD_BLK3_DATA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_BLK3_DATA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_BLK3_DATA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_BLK3_DATA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLK3_DATA2` reader - Store the third 32-bit of Block3."]
pub type BLK3_DATA2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Store the third 32-bit of Block3."]
    #[inline(always)]
    pub fn blk3_data2(&self) -> BLK3_DATA2_R {
        BLK3_DATA2_R::new(self.bits)
    }
}
#[doc = "Register 2 of BLOCK3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_blk3_data2](index.html) module"]
pub struct RD_BLK3_DATA2_SPEC;
impl crate::RegisterSpec for RD_BLK3_DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_blk3_data2::R](R) reader structure"]
impl crate::Readable for RD_BLK3_DATA2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_BLK3_DATA2 to value 0"]
impl crate::Resettable for RD_BLK3_DATA2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
