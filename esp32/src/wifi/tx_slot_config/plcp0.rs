#[doc = "Register `PLCP0` reader"]
pub type R = crate::R<PLCP0_SPEC>;
#[doc = "Register `PLCP0` writer"]
pub type W = crate::W<PLCP0_SPEC>;
#[doc = "Field `DMA_ADDR` reader - Bottom bits of address of dma_item"]
pub type DMA_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DMA_ADDR` writer - Bottom bits of address of dma_item"]
pub type DMA_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `FLAGS` reader - Flags for the SLOT"]
pub type FLAGS_R = crate::FieldReader<u16>;
#[doc = "Field `FLAGS` writer - Flags for the SLOT"]
pub type FLAGS_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - Bottom bits of address of dma_item"]
    #[inline(always)]
    pub fn dma_addr(&self) -> DMA_ADDR_R {
        DMA_ADDR_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - Flags for the SLOT"]
    #[inline(always)]
    pub fn flags(&self) -> FLAGS_R {
        FLAGS_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLCP0")
            .field("dma_addr", &self.dma_addr())
            .field("flags", &self.flags())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - Bottom bits of address of dma_item"]
    #[inline(always)]
    pub fn dma_addr(&mut self) -> DMA_ADDR_W<PLCP0_SPEC> {
        DMA_ADDR_W::new(self, 0)
    }
    #[doc = "Bits 20:31 - Flags for the SLOT"]
    #[inline(always)]
    pub fn flags(&mut self) -> FLAGS_W<PLCP0_SPEC> {
        FLAGS_W::new(self, 20)
    }
}
#[doc = "PLCP0\n\nYou can [`read`](crate::Reg::read) this register and get [`plcp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`plcp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLCP0_SPEC;
impl crate::RegisterSpec for PLCP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`plcp0::R`](R) reader structure"]
impl crate::Readable for PLCP0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`plcp0::W`](W) writer structure"]
impl crate::Writable for PLCP0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLCP0 to value 0"]
impl crate::Resettable for PLCP0_SPEC {
    const RESET_VALUE: u32 = 0;
}
