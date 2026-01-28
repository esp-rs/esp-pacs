#[doc = "Register `BLK0_BACKUP4_W3` reader"]
pub type R = crate::R<BLK0_BACKUP4_W3_SPEC>;
#[doc = "Field `BLOCK0_BACKUP4_W3` reader - Otp block0 backup4 word3 data."]
pub type BLOCK0_BACKUP4_W3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block0 backup4 word3 data."]
    #[inline(always)]
    pub fn block0_backup4_w3(&self) -> BLOCK0_BACKUP4_W3_R {
        BLOCK0_BACKUP4_W3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_BACKUP4_W3")
            .field("block0_backup4_w3", &self.block0_backup4_w3())
            .finish()
    }
}
#[doc = "eFuse apb2otp block0 data register19.\n\nYou can [`read`](crate::Reg::read) this register and get [`blk0_backup4_w3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK0_BACKUP4_W3_SPEC;
impl crate::RegisterSpec for BLK0_BACKUP4_W3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk0_backup4_w3::R`](R) reader structure"]
impl crate::Readable for BLK0_BACKUP4_W3_SPEC {}
#[doc = "`reset()` method sets BLK0_BACKUP4_W3 to value 0"]
impl crate::Resettable for BLK0_BACKUP4_W3_SPEC {}
