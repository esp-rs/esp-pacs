#[doc = "Register `BLK7_W1` reader"]
pub struct R(crate::R<BLK7_W1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK7_W1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK7_W1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK7_W1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLOCK7_W1` reader - Otp block7 word1 data."]
pub type BLOCK7_W1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block7 word1 data."]
    #[inline(always)]
    pub fn block7_w1(&self) -> BLOCK7_W1_R {
        BLOCK7_W1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK7_W1")
            .field("block7_w1", &format_args!("{}", self.block7_w1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK7_W1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Otp debuger block7 data register1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk7_w1](index.html) module"]
pub struct BLK7_W1_SPEC;
impl crate::RegisterSpec for BLK7_W1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk7_w1::R](R) reader structure"]
impl crate::Readable for BLK7_W1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK7_W1 to value 0"]
impl crate::Resettable for BLK7_W1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
