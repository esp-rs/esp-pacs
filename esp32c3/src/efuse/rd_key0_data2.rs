///Register `RD_KEY0_DATA2` reader
pub type R = crate::R<RD_KEY0_DATA2_SPEC>;
///Field `KEY0_DATA2` reader - Stores the second 32 bits of KEY0.
pub type KEY0_DATA2_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Stores the second 32 bits of KEY0.
    #[inline(always)]
    pub fn key0_data2(&self) -> KEY0_DATA2_R {
        KEY0_DATA2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_KEY0_DATA2")
            .field("key0_data2", &self.key0_data2())
            .finish()
    }
}
/**Register 2 of BLOCK4 (KEY0).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key0_data2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RD_KEY0_DATA2_SPEC;
impl crate::RegisterSpec for RD_KEY0_DATA2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rd_key0_data2::R`](R) reader structure
impl crate::Readable for RD_KEY0_DATA2_SPEC {}
///`reset()` method sets RD_KEY0_DATA2 to value 0
impl crate::Resettable for RD_KEY0_DATA2_SPEC {
    const RESET_VALUE: u32 = 0;
}
