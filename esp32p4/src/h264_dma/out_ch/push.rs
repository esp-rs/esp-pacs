#[doc = "Register `PUSH` reader"]
pub type R = crate::R<PUSH_SPEC>;
#[doc = "Register `PUSH` writer"]
pub type W = crate::W<PUSH_SPEC>;
#[doc = "Field `OUTFIFO_WDATA_CH0` reader - This register stores the data that need to be pushed into DMA Tx FIFO."]
pub type OUTFIFO_WDATA_CH0_R = crate::FieldReader<u16>;
#[doc = "Field `OUTFIFO_WDATA_CH0` writer - This register stores the data that need to be pushed into DMA Tx FIFO."]
pub type OUTFIFO_WDATA_CH0_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `OUTFIFO_PUSH_CH0` reader - Set this bit to push data into DMA Tx FIFO."]
pub type OUTFIFO_PUSH_CH0_R = crate::BitReader;
#[doc = "Field `OUTFIFO_PUSH_CH0` writer - Set this bit to push data into DMA Tx FIFO."]
pub type OUTFIFO_PUSH_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - This register stores the data that need to be pushed into DMA Tx FIFO."]
    #[inline(always)]
    pub fn outfifo_wdata_ch0(&self) -> OUTFIFO_WDATA_CH0_R {
        OUTFIFO_WDATA_CH0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Set this bit to push data into DMA Tx FIFO."]
    #[inline(always)]
    pub fn outfifo_push_ch0(&self) -> OUTFIFO_PUSH_CH0_R {
        OUTFIFO_PUSH_CH0_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUSH")
            .field("outfifo_wdata_ch0", &self.outfifo_wdata_ch0().bits())
            .field("outfifo_push_ch0", &self.outfifo_push_ch0().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PUSH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:9 - This register stores the data that need to be pushed into DMA Tx FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_wdata_ch0(&mut self) -> OUTFIFO_WDATA_CH0_W<PUSH_SPEC> {
        OUTFIFO_WDATA_CH0_W::new(self, 0)
    }
    #[doc = "Bit 10 - Set this bit to push data into DMA Tx FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_push_ch0(&mut self) -> OUTFIFO_PUSH_CH0_W<PUSH_SPEC> {
        OUTFIFO_PUSH_CH0_W::new(self, 10)
    }
}
#[doc = "TX CHx outfifo push register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`push::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`push::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PUSH_SPEC;
impl crate::RegisterSpec for PUSH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`push::R`](R) reader structure"]
impl crate::Readable for PUSH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`push::W`](W) writer structure"]
impl crate::Writable for PUSH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PUSH to value 0"]
impl crate::Resettable for PUSH_SPEC {
    const RESET_VALUE: u32 = 0;
}
