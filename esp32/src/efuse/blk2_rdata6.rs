///Register `BLK2_RDATA6` reader
pub type R = crate::R<BLK2_RDATA6_SPEC>;
///Field `RD_BLOCK2_6` reader -
pub type RD_BLOCK2_6_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn rd_block2_6(&self) -> RD_BLOCK2_6_R {
        RD_BLOCK2_6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK2_RDATA6")
            .field("rd_block2_6", &self.rd_block2_6())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`blk2_rdata6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BLK2_RDATA6_SPEC;
impl crate::RegisterSpec for BLK2_RDATA6_SPEC {
    type Ux = u32;
}
///`read()` method returns [`blk2_rdata6::R`](R) reader structure
impl crate::Readable for BLK2_RDATA6_SPEC {}
///`reset()` method sets BLK2_RDATA6 to value 0
impl crate::Resettable for BLK2_RDATA6_SPEC {
    const RESET_VALUE: u32 = 0;
}
