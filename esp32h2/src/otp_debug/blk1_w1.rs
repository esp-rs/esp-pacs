///Register `BLK1_W1` reader
pub type R = crate::R<BLK1_W1_SPEC>;
///Field `BLOCK1_W1` reader - Otp block1 word1 data.
pub type BLOCK1_W1_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Otp block1 word1 data.
    #[inline(always)]
    pub fn block1_w1(&self) -> BLOCK1_W1_R {
        BLOCK1_W1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK1_W1")
            .field("block1_w1", &self.block1_w1())
            .finish()
    }
}
/**Otp debuger block1 data register1.

You can [`read`](crate::generic::Reg::read) this register and get [`blk1_w1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BLK1_W1_SPEC;
impl crate::RegisterSpec for BLK1_W1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`blk1_w1::R`](R) reader structure
impl crate::Readable for BLK1_W1_SPEC {}
///`reset()` method sets BLK1_W1 to value 0
impl crate::Resettable for BLK1_W1_SPEC {
    const RESET_VALUE: u32 = 0;
}
