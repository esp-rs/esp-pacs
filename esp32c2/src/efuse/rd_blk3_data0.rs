#[doc = "Register `RD_BLK3_DATA0` reader"]
pub type R = crate::R<RD_BLK3_DATA0_SPEC>;
#[doc = "Field `BLK3_DATA0` reader - Store the first 32-bit of Block3."]
pub type BLK3_DATA0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Store the first 32-bit of Block3."]
    #[inline(always)]
    pub fn blk3_data0(&self) -> BLK3_DATA0_R {
        BLK3_DATA0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_BLK3_DATA0")
            .field("blk3_data0", &self.blk3_data0())
            .finish()
    }
}
#[doc = "Register 0 of BLOCK3.\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_blk3_data0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_BLK3_DATA0_SPEC;
impl crate::RegisterSpec for RD_BLK3_DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_blk3_data0::R`](R) reader structure"]
impl crate::Readable for RD_BLK3_DATA0_SPEC {}
#[doc = "`reset()` method sets RD_BLK3_DATA0 to value 0"]
impl crate::Resettable for RD_BLK3_DATA0_SPEC {}
