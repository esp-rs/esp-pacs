///Register `BLK7_W8` reader
pub type R = crate::R<BLK7_W8_SPEC>;
///Field `BLOCK7_W8` reader - Otp block7 word8 data.
pub type BLOCK7_W8_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Otp block7 word8 data.
    #[inline(always)]
    pub fn block7_w8(&self) -> BLOCK7_W8_R {
        BLOCK7_W8_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK7_W8")
            .field("block7_w8", &self.block7_w8())
            .finish()
    }
}
/**Otp debuger block7 data register8.

You can [`read`](crate::generic::Reg::read) this register and get [`blk7_w8::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BLK7_W8_SPEC;
impl crate::RegisterSpec for BLK7_W8_SPEC {
    type Ux = u32;
}
///`read()` method returns [`blk7_w8::R`](R) reader structure
impl crate::Readable for BLK7_W8_SPEC {}
///`reset()` method sets BLK7_W8 to value 0
impl crate::Resettable for BLK7_W8_SPEC {
    const RESET_VALUE: u32 = 0;
}
