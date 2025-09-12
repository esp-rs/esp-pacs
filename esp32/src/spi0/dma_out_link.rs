#[doc = "Register `DMA_OUT_LINK` reader"]
pub type R = crate::R<DMA_OUT_LINK_SPEC>;
#[doc = "Register `DMA_OUT_LINK` writer"]
pub type W = crate::W<DMA_OUT_LINK_SPEC>;
#[doc = "Field `OUTLINK_ADDR` reader - The address of the first outlink descriptor."]
pub type OUTLINK_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `OUTLINK_ADDR` writer - The address of the first outlink descriptor."]
pub type OUTLINK_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `OUTLINK_STOP` reader - Set the bit to stop to use outlink descriptor."]
pub type OUTLINK_STOP_R = crate::BitReader;
#[doc = "Field `OUTLINK_STOP` writer - Set the bit to stop to use outlink descriptor."]
pub type OUTLINK_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_START` reader - Set the bit to start to use outlink descriptor."]
pub type OUTLINK_START_R = crate::BitReader;
#[doc = "Field `OUTLINK_START` writer - Set the bit to start to use outlink descriptor."]
pub type OUTLINK_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_RESTART` reader - Set the bit to mount on new outlink descriptors."]
pub type OUTLINK_RESTART_R = crate::BitReader;
#[doc = "Field `OUTLINK_RESTART` writer - Set the bit to mount on new outlink descriptors."]
pub type OUTLINK_RESTART_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:19 - The address of the first outlink descriptor."]
    #[inline(always)]
    pub fn outlink_addr(&self) -> OUTLINK_ADDR_R {
        OUTLINK_ADDR_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 28 - Set the bit to stop to use outlink descriptor."]
    #[inline(always)]
    pub fn outlink_stop(&self) -> OUTLINK_STOP_R {
        OUTLINK_STOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Set the bit to start to use outlink descriptor."]
    #[inline(always)]
    pub fn outlink_start(&self) -> OUTLINK_START_R {
        OUTLINK_START_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Set the bit to mount on new outlink descriptors."]
    #[inline(always)]
    pub fn outlink_restart(&self) -> OUTLINK_RESTART_R {
        OUTLINK_RESTART_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_OUT_LINK")
            .field("outlink_addr", &self.outlink_addr())
            .field("outlink_stop", &self.outlink_stop())
            .field("outlink_start", &self.outlink_start())
            .field("outlink_restart", &self.outlink_restart())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - The address of the first outlink descriptor."]
    #[inline(always)]
    pub fn outlink_addr(&mut self) -> OUTLINK_ADDR_W<'_, DMA_OUT_LINK_SPEC> {
        OUTLINK_ADDR_W::new(self, 0)
    }
    #[doc = "Bit 28 - Set the bit to stop to use outlink descriptor."]
    #[inline(always)]
    pub fn outlink_stop(&mut self) -> OUTLINK_STOP_W<'_, DMA_OUT_LINK_SPEC> {
        OUTLINK_STOP_W::new(self, 28)
    }
    #[doc = "Bit 29 - Set the bit to start to use outlink descriptor."]
    #[inline(always)]
    pub fn outlink_start(&mut self) -> OUTLINK_START_W<'_, DMA_OUT_LINK_SPEC> {
        OUTLINK_START_W::new(self, 29)
    }
    #[doc = "Bit 30 - Set the bit to mount on new outlink descriptors."]
    #[inline(always)]
    pub fn outlink_restart(&mut self) -> OUTLINK_RESTART_W<'_, DMA_OUT_LINK_SPEC> {
        OUTLINK_RESTART_W::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_out_link::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_out_link::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_OUT_LINK_SPEC;
impl crate::RegisterSpec for DMA_OUT_LINK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_out_link::R`](R) reader structure"]
impl crate::Readable for DMA_OUT_LINK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_out_link::W`](W) writer structure"]
impl crate::Writable for DMA_OUT_LINK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_OUT_LINK to value 0"]
impl crate::Resettable for DMA_OUT_LINK_SPEC {}
