///Register `BLK9_W8` reader
pub type R = crate::R<BLK9_W8_SPEC>;
///Field `BLOCK9_W8` reader - Otp block9 word8 data.
pub type BLOCK9_W8_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Otp block9 word8 data.
    #[inline(always)]
    pub fn block9_w8(&self) -> BLOCK9_W8_R {
        BLOCK9_W8_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK9_W8")
            .field("block9_w8", &self.block9_w8())
            .finish()
    }
}
/**Otp debuger block9 data register8.

You can [`read`](crate::generic::Reg::read) this register and get [`blk9_w8::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BLK9_W8_SPEC;
impl crate::RegisterSpec for BLK9_W8_SPEC {
    type Ux = u32;
}
///`read()` method returns [`blk9_w8::R`](R) reader structure
impl crate::Readable for BLK9_W8_SPEC {}
///`reset()` method sets BLK9_W8 to value 0
impl crate::Resettable for BLK9_W8_SPEC {
    const RESET_VALUE: u32 = 0;
}
