#[doc = "Register `BLK7_W2` reader"]
pub struct R(crate::R<BLK7_W2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK7_W2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK7_W2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK7_W2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLOCK7_W2` reader - Otp block7 word2 data."]
pub type BLOCK7_W2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block7 word2 data."]
    #[inline(always)]
    pub fn block7_w2(&self) -> BLOCK7_W2_R {
        BLOCK7_W2_R::new(self.bits)
    }
}
#[doc = "Otp debuger block7 data register2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk7_w2](index.html) module"]
pub struct BLK7_W2_SPEC;
impl crate::RegisterSpec for BLK7_W2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk7_w2::R](R) reader structure"]
impl crate::Readable for BLK7_W2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK7_W2 to value 0"]
impl crate::Resettable for BLK7_W2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
