#[doc = "Register `L2_CACHE_CTRL` reader"]
pub type R = crate::R<L2_CACHE_CTRL_SPEC>;
#[doc = "Register `L2_CACHE_CTRL` writer"]
pub type W = crate::W<L2_CACHE_CTRL_SPEC>;
#[doc = "Field `L2_CACHE_SHUT_DMA` reader - The bit is used to disable DMA access L2-Cache, 0: enable, 1: disable"]
pub type L2_CACHE_SHUT_DMA_R = crate::BitReader;
#[doc = "Field `L2_CACHE_SHUT_DMA` writer - The bit is used to disable DMA access L2-Cache, 0: enable, 1: disable"]
pub type L2_CACHE_SHUT_DMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_CACHE_UNDEF_OP` reader - Reserved"]
pub type L2_CACHE_UNDEF_OP_R = crate::FieldReader;
#[doc = "Field `L2_CACHE_UNDEF_OP` writer - Reserved"]
pub type L2_CACHE_UNDEF_OP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 4 - The bit is used to disable DMA access L2-Cache, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn l2_cache_shut_dma(&self) -> L2_CACHE_SHUT_DMA_R {
        L2_CACHE_SHUT_DMA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn l2_cache_undef_op(&self) -> L2_CACHE_UNDEF_OP_R {
        L2_CACHE_UNDEF_OP_R::new(((self.bits >> 8) & 0xff) as u8)
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
impl W {
    #[doc = "Bit 4 - The bit is used to disable DMA access L2-Cache, 0: enable, 1: disable"]
    #[inline(always)]
    pub fn l2_cache_shut_dma(&mut self) -> L2_CACHE_SHUT_DMA_W<L2_CACHE_CTRL_SPEC> {
        L2_CACHE_SHUT_DMA_W::new(self, 4)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    pub fn l2_cache_undef_op(&mut self) -> L2_CACHE_UNDEF_OP_W<L2_CACHE_CTRL_SPEC> {
        L2_CACHE_UNDEF_OP_W::new(self, 8)
    }
}
#[doc = "L2 Cache(L2-Cache) control register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_cache_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2_cache_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_ctrl::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_ctrl::W`](W) writer structure"]
impl crate::Writable for L2_CACHE_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L2_CACHE_CTRL to value 0x10"]
impl crate::Resettable for L2_CACHE_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
