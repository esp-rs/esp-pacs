///Register `BLK4_W8` reader
pub type R = crate::R<BLK4_W8_SPEC>;
///Field `BLOCK4_W8` reader - Otp block4 word8 data.
pub type BLOCK4_W8_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Otp block4 word8 data.
    #[inline(always)]
    pub fn block4_w8(&self) -> BLOCK4_W8_R {
        BLOCK4_W8_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK4_W8")
            .field("block4_w8", &self.block4_w8())
            .finish()
    }
}
/**Otp debuger block4 data register8.

You can [`read`](crate::generic::Reg::read) this register and get [`blk4_w8::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BLK4_W8_SPEC;
impl crate::RegisterSpec for BLK4_W8_SPEC {
    type Ux = u32;
}
///`read()` method returns [`blk4_w8::R`](R) reader structure
impl crate::Readable for BLK4_W8_SPEC {}
///`reset()` method sets BLK4_W8 to value 0
impl crate::Resettable for BLK4_W8_SPEC {
    const RESET_VALUE: u32 = 0;
}
