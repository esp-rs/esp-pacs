#[doc = "Register `BLK8_W2` reader"]
pub struct R(crate::R<BLK8_W2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK8_W2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK8_W2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK8_W2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLOCK8_W2` reader - Otp block8 word2 data."]
pub type BLOCK8_W2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block8 word2 data."]
    #[inline(always)]
    pub fn block8_w2(&self) -> BLOCK8_W2_R {
        BLOCK8_W2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK8_W2")
            .field("block8_w2", &format_args!("{}", self.block8_w2().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK8_W2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Otp debuger block8 data register2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk8_w2](index.html) module"]
pub struct BLK8_W2_SPEC;
impl crate::RegisterSpec for BLK8_W2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk8_w2::R](R) reader structure"]
impl crate::Readable for BLK8_W2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK8_W2 to value 0"]
impl crate::Resettable for BLK8_W2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
