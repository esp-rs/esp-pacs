#[doc = "Register `DMA_IN_LINK` reader"]
pub type R = crate::R<DMA_IN_LINK_SPEC>;
#[doc = "Register `DMA_IN_LINK` writer"]
pub type W = crate::W<DMA_IN_LINK_SPEC>;
#[doc = "Field `INLINK_ADDR` reader - The address of the first inlink descriptor."]
pub type INLINK_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `INLINK_ADDR` writer - The address of the first inlink descriptor."]
pub type INLINK_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `INLINK_AUTO_RET` reader - when the bit is set, the inlink descriptor returns to the first link node when a packet is error."]
pub type INLINK_AUTO_RET_R = crate::BitReader;
#[doc = "Field `INLINK_AUTO_RET` writer - when the bit is set, the inlink descriptor returns to the first link node when a packet is error."]
pub type INLINK_AUTO_RET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_STOP` reader - Set the bit to stop to use inlink descriptor."]
pub type INLINK_STOP_R = crate::BitReader;
#[doc = "Field `INLINK_STOP` writer - Set the bit to stop to use inlink descriptor."]
pub type INLINK_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_START` reader - Set the bit to start to use inlink descriptor."]
pub type INLINK_START_R = crate::BitReader;
#[doc = "Field `INLINK_START` writer - Set the bit to start to use inlink descriptor."]
pub type INLINK_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INLINK_RESTART` reader - Set the bit to mount on new inlink descriptors."]
pub type INLINK_RESTART_R = crate::BitReader;
#[doc = "Field `INLINK_RESTART` writer - Set the bit to mount on new inlink descriptors."]
pub type INLINK_RESTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_RX_ENA` reader - SPI DMA read data status bit."]
pub type DMA_RX_ENA_R = crate::BitReader;
#[doc = "Field `DMA_RX_ENA` writer - SPI DMA read data status bit."]
pub type DMA_RX_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:19 - The address of the first inlink descriptor."]
    #[inline(always)]
    pub fn inlink_addr(&self) -> INLINK_ADDR_R {
        INLINK_ADDR_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 20 - when the bit is set, the inlink descriptor returns to the first link node when a packet is error."]
    #[inline(always)]
    pub fn inlink_auto_ret(&self) -> INLINK_AUTO_RET_R {
        INLINK_AUTO_RET_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 28 - Set the bit to stop to use inlink descriptor."]
    #[inline(always)]
    pub fn inlink_stop(&self) -> INLINK_STOP_R {
        INLINK_STOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Set the bit to start to use inlink descriptor."]
    #[inline(always)]
    pub fn inlink_start(&self) -> INLINK_START_R {
        INLINK_START_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Set the bit to mount on new inlink descriptors."]
    #[inline(always)]
    pub fn inlink_restart(&self) -> INLINK_RESTART_R {
        INLINK_RESTART_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SPI DMA read data status bit."]
    #[inline(always)]
    pub fn dma_rx_ena(&self) -> DMA_RX_ENA_R {
        DMA_RX_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_IN_LINK")
            .field("inlink_addr", &self.inlink_addr())
            .field("inlink_auto_ret", &self.inlink_auto_ret())
            .field("inlink_stop", &self.inlink_stop())
            .field("inlink_start", &self.inlink_start())
            .field("inlink_restart", &self.inlink_restart())
            .field("dma_rx_ena", &self.dma_rx_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - The address of the first inlink descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_addr(&mut self) -> INLINK_ADDR_W<DMA_IN_LINK_SPEC> {
        INLINK_ADDR_W::new(self, 0)
    }
    #[doc = "Bit 20 - when the bit is set, the inlink descriptor returns to the first link node when a packet is error."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_auto_ret(&mut self) -> INLINK_AUTO_RET_W<DMA_IN_LINK_SPEC> {
        INLINK_AUTO_RET_W::new(self, 20)
    }
    #[doc = "Bit 28 - Set the bit to stop to use inlink descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_stop(&mut self) -> INLINK_STOP_W<DMA_IN_LINK_SPEC> {
        INLINK_STOP_W::new(self, 28)
    }
    #[doc = "Bit 29 - Set the bit to start to use inlink descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_start(&mut self) -> INLINK_START_W<DMA_IN_LINK_SPEC> {
        INLINK_START_W::new(self, 29)
    }
    #[doc = "Bit 30 - Set the bit to mount on new inlink descriptors."]
    #[inline(always)]
    #[must_use]
    pub fn inlink_restart(&mut self) -> INLINK_RESTART_W<DMA_IN_LINK_SPEC> {
        INLINK_RESTART_W::new(self, 30)
    }
    #[doc = "Bit 31 - SPI DMA read data status bit."]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_ena(&mut self) -> DMA_RX_ENA_W<DMA_IN_LINK_SPEC> {
        DMA_RX_ENA_W::new(self, 31)
    }
}
#[doc = "SPI DMA RX link configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_link::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_in_link::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_IN_LINK_SPEC;
impl crate::RegisterSpec for DMA_IN_LINK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_in_link::R`](R) reader structure"]
impl crate::Readable for DMA_IN_LINK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_in_link::W`](W) writer structure"]
impl crate::Writable for DMA_IN_LINK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_IN_LINK to value 0"]
impl crate::Resettable for DMA_IN_LINK_SPEC {
    const RESET_VALUE: u32 = 0;
}
