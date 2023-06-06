#[doc = "Register `BLK3_W5` reader"]
pub struct R(crate::R<BLK3_W5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK3_W5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK3_W5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK3_W5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLOCK3_W5` reader - Otp block3 word5 data."]
pub type BLOCK3_W5_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block3 word5 data."]
    #[inline(always)]
    pub fn block3_w5(&self) -> BLOCK3_W5_R {
        BLOCK3_W5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK3_W5")
            .field("block3_w5", &format_args!("{}", self.block3_w5().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK3_W5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Otp debuger block3 data register5.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk3_w5](index.html) module"]
pub struct BLK3_W5_SPEC;
impl crate::RegisterSpec for BLK3_W5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk3_w5::R](R) reader structure"]
impl crate::Readable for BLK3_W5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK3_W5 to value 0"]
impl crate::Resettable for BLK3_W5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
