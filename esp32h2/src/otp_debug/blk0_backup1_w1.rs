///Register `BLK0_BACKUP1_W1` reader
pub type R = crate::R<BLK0_BACKUP1_W1_SPEC>;
///Field `OTP_BEBUG_BLOCK0_BACKUP1_W1` reader - Otp block0 backup1 word1 data.
pub type OTP_BEBUG_BLOCK0_BACKUP1_W1_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Otp block0 backup1 word1 data.
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
                &self.otp_bebug_block0_backup1_w1(),
            )
            .finish()
    }
}
/**Otp debuger block0 data register2.

You can [`read`](crate::generic::Reg::read) this register and get [`blk0_backup1_w1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BLK0_BACKUP1_W1_SPEC;
impl crate::RegisterSpec for BLK0_BACKUP1_W1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`blk0_backup1_w1::R`](R) reader structure
impl crate::Readable for BLK0_BACKUP1_W1_SPEC {}
///`reset()` method sets BLK0_BACKUP1_W1 to value 0
impl crate::Resettable for BLK0_BACKUP1_W1_SPEC {
    const RESET_VALUE: u32 = 0;
}
