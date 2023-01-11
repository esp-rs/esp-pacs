#[doc = "Register `BLK0_BACKUP2_W2` reader"]
pub struct R(crate::R<BLK0_BACKUP2_W2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK0_BACKUP2_W2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK0_BACKUP2_W2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK0_BACKUP2_W2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OTP_BEBUG_BLOCK0_BACKUP2_W2` reader - Otp block0 backup2 word2 data."]
pub type OTP_BEBUG_BLOCK0_BACKUP2_W2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block0 backup2 word2 data."]
    #[inline(always)]
    pub fn otp_bebug_block0_backup2_w2(&self) -> OTP_BEBUG_BLOCK0_BACKUP2_W2_R {
        OTP_BEBUG_BLOCK0_BACKUP2_W2_R::new(self.bits)
    }
}
#[doc = "Otp debuger block0 data register8.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk0_backup2_w2](index.html) module"]
pub struct BLK0_BACKUP2_W2_SPEC;
impl crate::RegisterSpec for BLK0_BACKUP2_W2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk0_backup2_w2::R](R) reader structure"]
impl crate::Readable for BLK0_BACKUP2_W2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK0_BACKUP2_W2 to value 0"]
impl crate::Resettable for BLK0_BACKUP2_W2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
