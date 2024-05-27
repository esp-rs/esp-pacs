#[doc = "Register `BLK3_RDATA6` reader"]
pub type R = crate::R<BLK3_RDATA6_SPEC>;
#[doc = "Field `RD_BLK3_RESERVED_6` reader - "]
pub type RD_BLK3_RESERVED_6_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rd_blk3_reserved_6(&self) -> RD_BLK3_RESERVED_6_R {
        RD_BLK3_RESERVED_6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK3_RDATA6")
            .field("rd_blk3_reserved_6", &self.rd_blk3_reserved_6())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_rdata6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK3_RDATA6_SPEC;
impl crate::RegisterSpec for BLK3_RDATA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk3_rdata6::R`](R) reader structure"]
impl crate::Readable for BLK3_RDATA6_SPEC {}
#[doc = "`reset()` method sets BLK3_RDATA6 to value 0"]
impl crate::Resettable for BLK3_RDATA6_SPEC {
    const RESET_VALUE: u32 = 0;
}
