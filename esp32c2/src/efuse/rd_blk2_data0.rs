#[doc = "Register `RD_BLK2_DATA0` reader"]
pub type R = crate::R<RD_BLK2_DATA0_SPEC>;
#[doc = "Field `BLK2_DATA0` reader - Store the bit \\[0:31\\] of MAC."]
pub type BLK2_DATA0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Store the bit \\[0:31\\] of MAC."]
    #[inline(always)]
    pub fn blk2_data0(&self) -> BLK2_DATA0_R {
        BLK2_DATA0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_BLK2_DATA0")
            .field("blk2_data0", &self.blk2_data0())
            .finish()
    }
}
#[doc = "Register 0 of BLOCK2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_blk2_data0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_BLK2_DATA0_SPEC;
impl crate::RegisterSpec for RD_BLK2_DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_blk2_data0::R`](R) reader structure"]
impl crate::Readable for RD_BLK2_DATA0_SPEC {}
#[doc = "`reset()` method sets RD_BLK2_DATA0 to value 0"]
impl crate::Resettable for RD_BLK2_DATA0_SPEC {
    const RESET_VALUE: u32 = 0;
}
