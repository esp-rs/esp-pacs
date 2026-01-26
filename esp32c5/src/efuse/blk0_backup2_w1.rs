#[doc = "Register `BLK0_BACKUP2_W1` reader"]
pub type R = crate::R<BLK0_BACKUP2_W1_SPEC>;
#[doc = "Field `BLOCK0_BACKUP2_W1` reader - Otp block0 backup2 word1 data."]
pub type BLOCK0_BACKUP2_W1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block0 backup2 word1 data."]
    #[inline(always)]
    pub fn block0_backup2_w1(&self) -> BLOCK0_BACKUP2_W1_R {
        BLOCK0_BACKUP2_W1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_BACKUP2_W1")
            .field("block0_backup2_w1", &self.block0_backup2_w1())
            .finish()
    }
}
#[doc = "eFuse apb2otp block0 data register7.\n\nYou can [`read`](crate::Reg::read) this register and get [`blk0_backup2_w1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK0_BACKUP2_W1_SPEC;
impl crate::RegisterSpec for BLK0_BACKUP2_W1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk0_backup2_w1::R`](R) reader structure"]
impl crate::Readable for BLK0_BACKUP2_W1_SPEC {}
#[doc = "`reset()` method sets BLK0_BACKUP2_W1 to value 0"]
impl crate::Resettable for BLK0_BACKUP2_W1_SPEC {}
