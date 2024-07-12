#[doc = "Register `RD_BLK2_DATA7` reader"]
pub type R = crate::R<RD_BLK2_DATA7_SPEC>;
#[doc = "Field `BLK2_RESERVED_DATA_1` reader - Store the bit \\[21:52\\] of block2 reserved data."]
pub type BLK2_RESERVED_DATA_1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Store the bit \\[21:52\\] of block2 reserved data."]
    #[inline(always)]
    pub fn blk2_reserved_data_1(&self) -> BLK2_RESERVED_DATA_1_R {
        BLK2_RESERVED_DATA_1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_BLK2_DATA7")
            .field("blk2_reserved_data_1", &self.blk2_reserved_data_1())
            .finish()
    }
}
#[doc = "Register 7 of BLOCK2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_blk2_data7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_BLK2_DATA7_SPEC;
impl crate::RegisterSpec for RD_BLK2_DATA7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_blk2_data7::R`](R) reader structure"]
impl crate::Readable for RD_BLK2_DATA7_SPEC {}
#[doc = "`reset()` method sets RD_BLK2_DATA7 to value 0"]
impl crate::Resettable for RD_BLK2_DATA7_SPEC {
    const RESET_VALUE: u32 = 0;
}
