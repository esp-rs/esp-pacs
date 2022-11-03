#[doc = "Register `BLK10_W2` reader"]
pub struct R(crate::R<BLK10_W2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK10_W2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK10_W2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK10_W2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLOCK10_W2` reader - Otp block10 word2 data."]
pub type BLOCK10_W2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block10 word2 data."]
    #[inline(always)]
    pub fn block10_w2(&self) -> BLOCK10_W2_R {
        BLOCK10_W2_R::new(self.bits)
    }
}
#[doc = "Otp debuger block10 data register2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk10_w2](index.html) module"]
pub struct BLK10_W2_SPEC;
impl crate::RegisterSpec for BLK10_W2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk10_w2::R](R) reader structure"]
impl crate::Readable for BLK10_W2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK10_W2 to value 0"]
impl crate::Resettable for BLK10_W2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
