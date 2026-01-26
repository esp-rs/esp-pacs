#[doc = "Register `BLK0_BACKUP3_W2` reader"]
pub type R = crate::R<BLK0_BACKUP3_W2_SPEC>;
#[doc = "Field `BLOCK0_BACKUP3_W2` reader - Otp block0 backup3 word2 data."]
pub type BLOCK0_BACKUP3_W2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block0 backup3 word2 data."]
    #[inline(always)]
    pub fn block0_backup3_w2(&self) -> BLOCK0_BACKUP3_W2_R {
        BLOCK0_BACKUP3_W2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_BACKUP3_W2")
            .field("block0_backup3_w2", &self.block0_backup3_w2())
            .finish()
    }
}
#[doc = "eFuse apb2otp block0 data register13.\n\nYou can [`read`](crate::Reg::read) this register and get [`blk0_backup3_w2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK0_BACKUP3_W2_SPEC;
impl crate::RegisterSpec for BLK0_BACKUP3_W2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk0_backup3_w2::R`](R) reader structure"]
impl crate::Readable for BLK0_BACKUP3_W2_SPEC {}
#[doc = "`reset()` method sets BLK0_BACKUP3_W2 to value 0"]
impl crate::Resettable for BLK0_BACKUP3_W2_SPEC {}
