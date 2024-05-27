#[doc = "Register `BLK2_RDATA3` reader"]
pub type R = crate::R<BLK2_RDATA3_SPEC>;
#[doc = "Field `RD_BLOCK2_3` reader - "]
pub type RD_BLOCK2_3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rd_block2_3(&self) -> RD_BLOCK2_3_R {
        RD_BLOCK2_3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK2_RDATA3")
            .field("rd_block2_3", &self.rd_block2_3())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk2_rdata3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK2_RDATA3_SPEC;
impl crate::RegisterSpec for BLK2_RDATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk2_rdata3::R`](R) reader structure"]
impl crate::Readable for BLK2_RDATA3_SPEC {}
#[doc = "`reset()` method sets BLK2_RDATA3 to value 0"]
impl crate::Resettable for BLK2_RDATA3_SPEC {
    const RESET_VALUE: u32 = 0;
}
