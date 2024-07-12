#[doc = "Register `RD_BLK3_DATA6` reader"]
pub type R = crate::R<RD_BLK3_DATA6_SPEC>;
#[doc = "Field `BLK3_DATA6` reader - Store the seventh 32-bit of Block3."]
pub type BLK3_DATA6_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Store the seventh 32-bit of Block3."]
    #[inline(always)]
    pub fn blk3_data6(&self) -> BLK3_DATA6_R {
        BLK3_DATA6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_BLK3_DATA6")
            .field("blk3_data6", &self.blk3_data6())
            .finish()
    }
}
#[doc = "Register 6 of BLOCK3.\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_blk3_data6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_BLK3_DATA6_SPEC;
impl crate::RegisterSpec for RD_BLK3_DATA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_blk3_data6::R`](R) reader structure"]
impl crate::Readable for RD_BLK3_DATA6_SPEC {}
#[doc = "`reset()` method sets RD_BLK3_DATA6 to value 0"]
impl crate::Resettable for RD_BLK3_DATA6_SPEC {
    const RESET_VALUE: u32 = 0;
}
