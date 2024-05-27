#[doc = "Register `BLK1_RDATA1` reader"]
pub type R = crate::R<BLK1_RDATA1_SPEC>;
#[doc = "Field `RD_BLOCK1_1` reader - "]
pub type RD_BLOCK1_1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rd_block1_1(&self) -> RD_BLOCK1_1_R {
        RD_BLOCK1_1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK1_RDATA1")
            .field("rd_block1_1", &self.rd_block1_1())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk1_rdata1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK1_RDATA1_SPEC;
impl crate::RegisterSpec for BLK1_RDATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk1_rdata1::R`](R) reader structure"]
impl crate::Readable for BLK1_RDATA1_SPEC {}
#[doc = "`reset()` method sets BLK1_RDATA1 to value 0"]
impl crate::Resettable for BLK1_RDATA1_SPEC {
    const RESET_VALUE: u32 = 0;
}
