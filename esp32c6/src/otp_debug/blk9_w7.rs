///Register `BLK9_W7` reader
pub type R = crate::R<BLK9_W7_SPEC>;
///Field `BLOCK9_W7` reader - Otp block9 word7 data.
pub type BLOCK9_W7_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Otp block9 word7 data.
    #[inline(always)]
    pub fn block9_w7(&self) -> BLOCK9_W7_R {
        BLOCK9_W7_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK9_W7")
            .field("block9_w7", &self.block9_w7())
            .finish()
    }
}
/**Otp debuger block9 data register7.

You can [`read`](crate::generic::Reg::read) this register and get [`blk9_w7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BLK9_W7_SPEC;
impl crate::RegisterSpec for BLK9_W7_SPEC {
    type Ux = u32;
}
///`read()` method returns [`blk9_w7::R`](R) reader structure
impl crate::Readable for BLK9_W7_SPEC {}
///`reset()` method sets BLK9_W7 to value 0
impl crate::Resettable for BLK9_W7_SPEC {
    const RESET_VALUE: u32 = 0;
}
