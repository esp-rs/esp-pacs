///Register `BLK7_W4` reader
pub type R = crate::R<BLK7_W4_SPEC>;
///Field `BLOCK7_W4` reader - Otp block7 word4 data.
pub type BLOCK7_W4_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Otp block7 word4 data.
    #[inline(always)]
    pub fn block7_w4(&self) -> BLOCK7_W4_R {
        BLOCK7_W4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK7_W4")
            .field("block7_w4", &self.block7_w4())
            .finish()
    }
}
/**Otp debuger block7 data register4.

You can [`read`](crate::generic::Reg::read) this register and get [`blk7_w4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BLK7_W4_SPEC;
impl crate::RegisterSpec for BLK7_W4_SPEC {
    type Ux = u32;
}
///`read()` method returns [`blk7_w4::R`](R) reader structure
impl crate::Readable for BLK7_W4_SPEC {}
///`reset()` method sets BLK7_W4 to value 0
impl crate::Resettable for BLK7_W4_SPEC {
    const RESET_VALUE: u32 = 0;
}
