///Register `L2_UNALLOCATE_BUFFER_CLEAR` reader
pub type R = crate::R<L2_UNALLOCATE_BUFFER_CLEAR_SPEC>;
///Field `L2_CACHE_UNALLOC_CLR` reader - The bit is used to clear the unallocate request buffer of l2 icache where the unallocate request is responsed but not completed.
pub type L2_CACHE_UNALLOC_CLR_R = crate::BitReader;
impl R {
    ///Bit 5 - The bit is used to clear the unallocate request buffer of l2 icache where the unallocate request is responsed but not completed.
    #[inline(always)]
    pub fn l2_cache_unalloc_clr(&self) -> L2_CACHE_UNALLOC_CLR_R {
        L2_CACHE_UNALLOC_CLR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_UNALLOCATE_BUFFER_CLEAR")
            .field("l2_cache_unalloc_clr", &self.l2_cache_unalloc_clr())
            .finish()
    }
}
/**Unallocate request buffer clear registers

You can [`read`](crate::generic::Reg::read) this register and get [`l2_unallocate_buffer_clear::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L2_UNALLOCATE_BUFFER_CLEAR_SPEC;
impl crate::RegisterSpec for L2_UNALLOCATE_BUFFER_CLEAR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l2_unallocate_buffer_clear::R`](R) reader structure
impl crate::Readable for L2_UNALLOCATE_BUFFER_CLEAR_SPEC {}
///`reset()` method sets L2_UNALLOCATE_BUFFER_CLEAR to value 0
impl crate::Resettable for L2_UNALLOCATE_BUFFER_CLEAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
