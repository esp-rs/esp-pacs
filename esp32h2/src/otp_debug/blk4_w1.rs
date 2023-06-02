#[doc = "Register `BLK4_W1` reader"]
pub struct R(crate::R<BLK4_W1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK4_W1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK4_W1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK4_W1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLOCK4_W1` reader - Otp block4 word1 data."]
pub type BLOCK4_W1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block4 word1 data."]
    #[inline(always)]
    pub fn block4_w1(&self) -> BLOCK4_W1_R {
        BLOCK4_W1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK4_W1")
            .field("block4_w1", &format_args!("{}", self.block4_w1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK4_W1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Otp debuger block4 data register1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk4_w1](index.html) module"]
pub struct BLK4_W1_SPEC;
impl crate::RegisterSpec for BLK4_W1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk4_w1::R](R) reader structure"]
impl crate::Readable for BLK4_W1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK4_W1 to value 0"]
impl crate::Resettable for BLK4_W1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
