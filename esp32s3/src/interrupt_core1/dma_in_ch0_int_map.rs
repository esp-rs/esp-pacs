#[doc = "Register `DMA_IN_CH0_INT_MAP` reader"]
pub type R = crate::R<DMA_IN_CH0_INT_MAP_SPEC>;
#[doc = "Register `DMA_IN_CH0_INT_MAP` writer"]
pub type W = crate::W<DMA_IN_CH0_INT_MAP_SPEC>;
#[doc = "Field `DMA_IN_CH0_INT_MAP` reader - this register used to map dma_in_ch0 interrupt to one of core1's external interrupt"]
pub type DMA_IN_CH0_INT_MAP_R = crate::FieldReader;
#[doc = "Field `DMA_IN_CH0_INT_MAP` writer - this register used to map dma_in_ch0 interrupt to one of core1's external interrupt"]
pub type DMA_IN_CH0_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - this register used to map dma_in_ch0 interrupt to one of core1's external interrupt"]
    #[inline(always)]
    pub fn dma_in_ch0_int_map(&self) -> DMA_IN_CH0_INT_MAP_R {
        DMA_IN_CH0_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_IN_CH0_INT_MAP")
            .field(
                "dma_in_ch0_int_map",
                &format_args!("{}", self.dma_in_ch0_int_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_IN_CH0_INT_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - this register used to map dma_in_ch0 interrupt to one of core1's external interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dma_in_ch0_int_map(&mut self) -> DMA_IN_CH0_INT_MAP_W<DMA_IN_CH0_INT_MAP_SPEC> {
        DMA_IN_CH0_INT_MAP_W::new(self, 0)
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
#[doc = "dma_in_ch0 interrupt configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_ch0_int_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_in_ch0_int_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_IN_CH0_INT_MAP_SPEC;
impl crate::RegisterSpec for DMA_IN_CH0_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_in_ch0_int_map::R`](R) reader structure"]
impl crate::Readable for DMA_IN_CH0_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_in_ch0_int_map::W`](W) writer structure"]
impl crate::Writable for DMA_IN_CH0_INT_MAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_IN_CH0_INT_MAP to value 0x10"]
impl crate::Resettable for DMA_IN_CH0_INT_MAP_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
