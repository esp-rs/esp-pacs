#[doc = "Register `PLCP0` reader"]
pub type R = crate::R<PLCP0_SPEC>;
#[doc = "Register `PLCP0` writer"]
pub type W = crate::W<PLCP0_SPEC>;
#[doc = "Field `DMA_ADDR` reader - Bottom bits of address of dma_item"]
pub type DMA_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DMA_ADDR` writer - Bottom bits of address of dma_item"]
pub type DMA_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `WAIT_FOR_ACK` reader - Enables ACK timeouts"]
pub type WAIT_FOR_ACK_R = crate::BitReader;
#[doc = "Field `WAIT_FOR_ACK` writer - Enables ACK timeouts"]
pub type WAIT_FOR_ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOT_VALID` reader - Marks this slot as valid"]
pub type SLOT_VALID_R = crate::BitReader;
#[doc = "Field `SLOT_VALID` writer - Marks this slot as valid"]
pub type SLOT_VALID_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOT_ENABLED` reader - Marks this slot as ready for transmission"]
pub type SLOT_ENABLED_R = crate::BitReader;
#[doc = "Field `SLOT_ENABLED` writer - Marks this slot as ready for transmission"]
pub type SLOT_ENABLED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:19 - Bottom bits of address of dma_item"]
    #[inline(always)]
    pub fn dma_addr(&self) -> DMA_ADDR_R {
        DMA_ADDR_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 24 - Enables ACK timeouts"]
    #[inline(always)]
    pub fn wait_for_ack(&self) -> WAIT_FOR_ACK_R {
        WAIT_FOR_ACK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 30 - Marks this slot as valid"]
    #[inline(always)]
    pub fn slot_valid(&self) -> SLOT_VALID_R {
        SLOT_VALID_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Marks this slot as ready for transmission"]
    #[inline(always)]
    pub fn slot_enabled(&self) -> SLOT_ENABLED_R {
        SLOT_ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLCP0")
            .field("dma_addr", &self.dma_addr())
            .field("wait_for_ack", &self.wait_for_ack())
            .field("slot_valid", &self.slot_valid())
            .field("slot_enabled", &self.slot_enabled())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - Bottom bits of address of dma_item"]
    #[inline(always)]
    pub fn dma_addr(&mut self) -> DMA_ADDR_W<PLCP0_SPEC> {
        DMA_ADDR_W::new(self, 0)
    }
    #[doc = "Bit 24 - Enables ACK timeouts"]
    #[inline(always)]
    pub fn wait_for_ack(&mut self) -> WAIT_FOR_ACK_W<PLCP0_SPEC> {
        WAIT_FOR_ACK_W::new(self, 24)
    }
    #[doc = "Bit 30 - Marks this slot as valid"]
    #[inline(always)]
    pub fn slot_valid(&mut self) -> SLOT_VALID_W<PLCP0_SPEC> {
        SLOT_VALID_W::new(self, 30)
    }
    #[doc = "Bit 31 - Marks this slot as ready for transmission"]
    #[inline(always)]
    pub fn slot_enabled(&mut self) -> SLOT_ENABLED_W<PLCP0_SPEC> {
        SLOT_ENABLED_W::new(self, 31)
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
}
#[doc = "`reset()` method sets PLCP0 to value 0"]
impl crate::Resettable for PLCP0_SPEC {}
