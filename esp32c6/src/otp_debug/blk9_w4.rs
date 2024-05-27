///Register `BLK9_W4` reader
pub type R = crate::R<BLK9_W4_SPEC>;
///Field `BLOCK9_W4` reader - Otp block9 word4 data.
pub type BLOCK9_W4_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Otp block9 word4 data.
    #[inline(always)]
    pub fn block9_w4(&self) -> BLOCK9_W4_R {
        BLOCK9_W4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK9_W4")
            .field("block9_w4", &self.block9_w4())
            .finish()
    }
}
/**Otp debuger block9 data register4.

You can [`read`](crate::generic::Reg::read) this register and get [`blk9_w4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BLK9_W4_SPEC;
impl crate::RegisterSpec for BLK9_W4_SPEC {
    type Ux = u32;
}
///`read()` method returns [`blk9_w4::R`](R) reader structure
impl crate::Readable for BLK9_W4_SPEC {}
///`reset()` method sets BLK9_W4 to value 0
impl crate::Resettable for BLK9_W4_SPEC {
    const RESET_VALUE: u32 = 0;
}
