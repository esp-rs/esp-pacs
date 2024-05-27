///Register `RD_KEY1_DATA5` reader
pub type R = crate::R<RD_KEY1_DATA5_SPEC>;
///Field `KEY1_DATA5` reader - Stores the fifth 32 bits of KEY1.
pub type KEY1_DATA5_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Stores the fifth 32 bits of KEY1.
    #[inline(always)]
    pub fn key1_data5(&self) -> KEY1_DATA5_R {
        KEY1_DATA5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_KEY1_DATA5")
            .field("key1_data5", &self.key1_data5())
            .finish()
    }
}
/**Register $n of BLOCK5 (KEY1).

You can [`read`](crate::generic::Reg::read) this register and get [`rd_key1_data5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RD_KEY1_DATA5_SPEC;
impl crate::RegisterSpec for RD_KEY1_DATA5_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rd_key1_data5::R`](R) reader structure
impl crate::Readable for RD_KEY1_DATA5_SPEC {}
///`reset()` method sets RD_KEY1_DATA5 to value 0
impl crate::Resettable for RD_KEY1_DATA5_SPEC {
    const RESET_VALUE: u32 = 0;
}
