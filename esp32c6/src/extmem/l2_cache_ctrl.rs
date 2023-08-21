#[doc = "Register `L2_CACHE_CTRL` reader"]
pub type R = crate::R<L2_CACHE_CTRL_SPEC>;
#[doc = "Field `L2_CACHE_SHUT_DMA` reader - The bit is used to disable DMA access L2-Cache, 0: enable, 1: disable"]
pub type L2_CACHE_SHUT_DMA_R = crate::BitReader;
#[doc = "Field `L2_CACHE_UNDEF_OP` reader - Reserved"]
pub type L2_CACHE_UNDEF_OP_R = crate::FieldReader;
impl R {
    #[doc = "Bit 4 - The bit is used to disable DMA access L2-Cache, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn l2_cache_shut_dma(&self) -> L2_CACHE_SHUT_DMA_R {
        L2_CACHE_SHUT_DMA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8 - Reserved"]
    #[inline(always)]
    pub fn l2_cache_undef_op(&self) -> L2_CACHE_UNDEF_OP_R {
        L2_CACHE_UNDEF_OP_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_CTRL")
            .field(
                "l2_cache_shut_dma",
                &format_args!("{}", self.l2_cache_shut_dma().bit()),
            )
            .field(
                "l2_cache_undef_op",
                &format_args!("{}", self.l2_cache_undef_op().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "L2 Cache(L2-Cache) control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_ctrl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_ctrl::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_CTRL_SPEC {}
#[doc = "`reset()` method sets L2_CACHE_CTRL to value 0"]
impl crate::Resettable for L2_CACHE_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
