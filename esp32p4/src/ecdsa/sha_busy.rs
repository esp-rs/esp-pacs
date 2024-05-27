///Register `SHA_BUSY` reader
pub type R = crate::R<SHA_BUSY_SPEC>;
///Field `SHA_BUSY` reader - The busy status bit of SHA Calculator in ECDSA Accelerator. 1:SHA is in calculation. 0: SHA is idle.
pub type SHA_BUSY_R = crate::BitReader;
impl R {
    ///Bit 0 - The busy status bit of SHA Calculator in ECDSA Accelerator. 1:SHA is in calculation. 0: SHA is idle.
    #[inline(always)]
    pub fn sha_busy(&self) -> SHA_BUSY_R {
        SHA_BUSY_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHA_BUSY")
            .field("sha_busy", &self.sha_busy())
            .finish()
    }
}
/**ECDSA status register

You can [`read`](crate::generic::Reg::read) this register and get [`sha_busy::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SHA_BUSY_SPEC;
impl crate::RegisterSpec for SHA_BUSY_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sha_busy::R`](R) reader structure
impl crate::Readable for SHA_BUSY_SPEC {}
///`reset()` method sets SHA_BUSY to value 0
impl crate::Resettable for SHA_BUSY_SPEC {
    const RESET_VALUE: u32 = 0;
}
