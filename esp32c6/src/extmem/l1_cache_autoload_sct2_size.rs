///Register `L1_CACHE_AUTOLOAD_SCT2_SIZE` reader
pub type R = crate::R<L1_CACHE_AUTOLOAD_SCT2_SIZE_SPEC>;
///Field `L1_CACHE_AUTOLOAD_SCT2_SIZE` reader - Those bits are used to configure the size of the third section for autoload operation on L1-Cache. Note that it should be used together with L1_CACHE_AUTOLOAD_SCT2_ADDR and L1_CACHE_AUTOLOAD_SCT2_ENA.
pub type L1_CACHE_AUTOLOAD_SCT2_SIZE_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:27 - Those bits are used to configure the size of the third section for autoload operation on L1-Cache. Note that it should be used together with L1_CACHE_AUTOLOAD_SCT2_ADDR and L1_CACHE_AUTOLOAD_SCT2_ENA.
    #[inline(always)]
    pub fn l1_cache_autoload_sct2_size(&self) -> L1_CACHE_AUTOLOAD_SCT2_SIZE_R {
        L1_CACHE_AUTOLOAD_SCT2_SIZE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_CACHE_AUTOLOAD_SCT2_SIZE")
            .field(
                "l1_cache_autoload_sct2_size",
                &self.l1_cache_autoload_sct2_size(),
            )
            .finish()
    }
}
/**L1 Cache autoload section 2 size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_autoload_sct2_size::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L1_CACHE_AUTOLOAD_SCT2_SIZE_SPEC;
impl crate::RegisterSpec for L1_CACHE_AUTOLOAD_SCT2_SIZE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l1_cache_autoload_sct2_size::R`](R) reader structure
impl crate::Readable for L1_CACHE_AUTOLOAD_SCT2_SIZE_SPEC {}
///`reset()` method sets L1_CACHE_AUTOLOAD_SCT2_SIZE to value 0
impl crate::Resettable for L1_CACHE_AUTOLOAD_SCT2_SIZE_SPEC {
    const RESET_VALUE: u32 = 0;
}
