#[doc = "Register `CACHE_L2_CACHE_CTRL` reader"]
pub type R = crate::R<CACHE_L2_CACHE_CTRL_SPEC>;
#[doc = "Register `CACHE_L2_CACHE_CTRL` writer"]
pub type W = crate::W<CACHE_L2_CACHE_CTRL_SPEC>;
#[doc = "Field `CACHE_L2_CACHE_SHUT_DMA` reader - The bit is used to disable DMA access L2-Cache, 0: enable, 1: disable"]
pub type CACHE_L2_CACHE_SHUT_DMA_R = crate::BitReader;
#[doc = "Field `CACHE_L2_CACHE_UNDEF_OP` reader - Reserved"]
pub type CACHE_L2_CACHE_UNDEF_OP_R = crate::FieldReader;
#[doc = "Field `CACHE_L2_CACHE_UNDEF_OP` writer - Reserved"]
pub type CACHE_L2_CACHE_UNDEF_OP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 4 - The bit is used to disable DMA access L2-Cache, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn cache_l2_cache_shut_dma(&self) -> CACHE_L2_CACHE_SHUT_DMA_R {
        CACHE_L2_CACHE_SHUT_DMA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn cache_l2_cache_undef_op(&self) -> CACHE_L2_CACHE_UNDEF_OP_R {
        CACHE_L2_CACHE_UNDEF_OP_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_L2_CACHE_CTRL")
            .field("cache_l2_cache_shut_dma", &self.cache_l2_cache_shut_dma())
            .field("cache_l2_cache_undef_op", &self.cache_l2_cache_undef_op())
            .finish()
    }
}
impl W {
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn cache_l2_cache_undef_op(
        &mut self,
    ) -> CACHE_L2_CACHE_UNDEF_OP_W<'_, CACHE_L2_CACHE_CTRL_SPEC> {
        CACHE_L2_CACHE_UNDEF_OP_W::new(self, 8)
    }
}
#[doc = "L2 Cache(L2-Cache) control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_l2_cache_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_L2_CACHE_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_L2_CACHE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_l2_cache_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_L2_CACHE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_l2_cache_ctrl::W`](W) writer structure"]
impl crate::Writable for CACHE_L2_CACHE_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_L2_CACHE_CTRL to value 0"]
impl crate::Resettable for CACHE_L2_CACHE_CTRL_SPEC {}
