///Register `L2_CACHE_CTRL` reader
pub type R = crate::R<L2_CACHE_CTRL_SPEC>;
///Field `L2_CACHE_SHUT_DMA` reader - The bit is used to disable DMA access L2-Cache, 0: enable, 1: disable
pub type L2_CACHE_SHUT_DMA_R = crate::BitReader;
///Field `L2_CACHE_UNDEF_OP` reader - Reserved
pub type L2_CACHE_UNDEF_OP_R = crate::FieldReader;
impl R {
    ///Bit 4 - The bit is used to disable DMA access L2-Cache, 0: enable, 1: disable
    #[inline(always)]
    pub fn l2_cache_shut_dma(&self) -> L2_CACHE_SHUT_DMA_R {
        L2_CACHE_SHUT_DMA_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:8 - Reserved
    #[inline(always)]
    pub fn l2_cache_undef_op(&self) -> L2_CACHE_UNDEF_OP_R {
        L2_CACHE_UNDEF_OP_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_CTRL")
            .field("l2_cache_shut_dma", &self.l2_cache_shut_dma())
            .field("l2_cache_undef_op", &self.l2_cache_undef_op())
            .finish()
    }
}
/**L2 Cache(L2-Cache) control register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_ctrl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L2_CACHE_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l2_cache_ctrl::R`](R) reader structure
impl crate::Readable for L2_CACHE_CTRL_SPEC {}
///`reset()` method sets L2_CACHE_CTRL to value 0
impl crate::Resettable for L2_CACHE_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
