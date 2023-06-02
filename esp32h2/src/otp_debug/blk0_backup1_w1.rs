#[doc = "Register `BLK0_BACKUP1_W1` reader"]
pub struct R(crate::R<BLK0_BACKUP1_W1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK0_BACKUP1_W1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK0_BACKUP1_W1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK0_BACKUP1_W1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OTP_BEBUG_BLOCK0_BACKUP1_W1` reader - Otp block0 backup1 word1 data."]
pub type OTP_BEBUG_BLOCK0_BACKUP1_W1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block0 backup1 word1 data."]
    #[inline(always)]
    pub fn otp_bebug_block0_backup1_w1(&self) -> OTP_BEBUG_BLOCK0_BACKUP1_W1_R {
        OTP_BEBUG_BLOCK0_BACKUP1_W1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_BACKUP1_W1")
            .field(
                "otp_bebug_block0_backup1_w1",
                &format_args!("{}", self.otp_bebug_block0_backup1_w1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK0_BACKUP1_W1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Otp debuger block0 data register2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk0_backup1_w1](index.html) module"]
pub struct BLK0_BACKUP1_W1_SPEC;
impl crate::RegisterSpec for BLK0_BACKUP1_W1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk0_backup1_w1::R](R) reader structure"]
impl crate::Readable for BLK0_BACKUP1_W1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK0_BACKUP1_W1 to value 0"]
impl crate::Resettable for BLK0_BACKUP1_W1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
