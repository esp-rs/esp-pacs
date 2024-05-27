///Register `BLK10_W9` reader
pub type R = crate::R<BLK10_W9_SPEC>;
///Field `BLOCK10_W9` reader - Otp block10 word9 data.
pub type BLOCK10_W9_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Otp block10 word9 data.
    #[inline(always)]
    pub fn block10_w9(&self) -> BLOCK10_W9_R {
        BLOCK10_W9_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK10_W9")
            .field("block10_w9", &self.block10_w9())
            .finish()
    }
}
/**Otp debuger block10 data register9.

You can [`read`](crate::generic::Reg::read) this register and get [`blk10_w9::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BLK10_W9_SPEC;
impl crate::RegisterSpec for BLK10_W9_SPEC {
    type Ux = u32;
}
///`read()` method returns [`blk10_w9::R`](R) reader structure
impl crate::Readable for BLK10_W9_SPEC {}
///`reset()` method sets BLK10_W9 to value 0
impl crate::Resettable for BLK10_W9_SPEC {
    const RESET_VALUE: u32 = 0;
}
