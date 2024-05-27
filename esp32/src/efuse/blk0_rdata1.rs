#[doc = "Register `BLK0_RDATA1` reader"]
pub type R = crate::R<BLK0_RDATA1_SPEC>;
#[doc = "Field `RD_MAC` reader - "]
pub type RD_MAC_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rd_mac(&self) -> RD_MAC_R {
        RD_MAC_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_RDATA1")
            .field("rd_mac", &self.rd_mac())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk0_rdata1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK0_RDATA1_SPEC;
impl crate::RegisterSpec for BLK0_RDATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk0_rdata1::R`](R) reader structure"]
impl crate::Readable for BLK0_RDATA1_SPEC {}
#[doc = "`reset()` method sets BLK0_RDATA1 to value 0"]
impl crate::Resettable for BLK0_RDATA1_SPEC {
    const RESET_VALUE: u32 = 0;
}
