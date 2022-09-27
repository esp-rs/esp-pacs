#[doc = "Register `BLK3_RDATA1` reader"]
pub struct R(crate::R<BLK3_RDATA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK3_RDATA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK3_RDATA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK3_RDATA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLK3_DOUT1` reader - read for BLOCK3"]
pub type BLK3_DOUT1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - read for BLOCK3"]
    #[inline(always)]
    pub fn blk3_dout1(&self) -> BLK3_DOUT1_R {
        BLK3_DOUT1_R::new(self.bits)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk3_rdata1](index.html) module"]
pub struct BLK3_RDATA1_SPEC;
impl crate::RegisterSpec for BLK3_RDATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk3_rdata1::R](R) reader structure"]
impl crate::Readable for BLK3_RDATA1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK3_RDATA1 to value 0"]
impl crate::Resettable for BLK3_RDATA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
