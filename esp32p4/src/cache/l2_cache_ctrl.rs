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
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 4 - The bit is used to disable DMA access L2-Cache, 0: enable, 1: disable"]
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_shut_dma(&mut self) -> L2_CACHE_SHUT_DMA_W<L2_CACHE_CTRL_SPEC> {
        L2_CACHE_SHUT_DMA_W::new(self, 4)
    }
    #[doc = "Bits 8:15 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_undef_op(&mut self) -> L2_CACHE_UNDEF_OP_W<L2_CACHE_CTRL_SPEC> {
        L2_CACHE_UNDEF_OP_W::new(self, 8)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "L2 Cache(L2-Cache) control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_ctrl::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_ctrl::W`](W) writer structure"]
impl crate::Writable for L2_CACHE_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L2_CACHE_CTRL to value 0x10"]
impl crate::Resettable for L2_CACHE_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
