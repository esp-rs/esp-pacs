#[doc = "Register `OUT_PUSH_CH%s` reader"]
pub type R = crate::R<OUT_PUSH_CH_SPEC>;
#[doc = "Register `OUT_PUSH_CH%s` writer"]
pub type W = crate::W<OUT_PUSH_CH_SPEC>;
#[doc = "Field `OUTFIFO_WDATA_CH` reader - This register stores the data that need to be pushed into AXI_DMA FIFO."]
pub type OUTFIFO_WDATA_CH_R = crate::FieldReader<u16>;
#[doc = "Field `OUTFIFO_WDATA_CH` writer - This register stores the data that need to be pushed into AXI_DMA FIFO."]
pub type OUTFIFO_WDATA_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `OUTFIFO_PUSH_CH` writer - Set this bit to push data into AXI_DMA FIFO."]
pub type OUTFIFO_PUSH_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - This register stores the data that need to be pushed into AXI_DMA FIFO."]
    #[inline(always)]
    pub fn outfifo_wdata_ch(&self) -> OUTFIFO_WDATA_CH_R {
        OUTFIFO_WDATA_CH_R::new((self.bits & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_PUSH_CH")
            .field(
                "outfifo_wdata_ch",
                &format_args!("{}", self.outfifo_wdata_ch().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_PUSH_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:8 - This register stores the data that need to be pushed into AXI_DMA FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_wdata_ch(&mut self) -> OUTFIFO_WDATA_CH_W<OUT_PUSH_CH_SPEC> {
        OUTFIFO_WDATA_CH_W::new(self, 0)
    }
    #[doc = "Bit 9 - Set this bit to push data into AXI_DMA FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn outfifo_push_ch(&mut self) -> OUTFIFO_PUSH_CH_W<OUT_PUSH_CH_SPEC> {
        OUTFIFO_PUSH_CH_W::new(self, 9)
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
#[doc = "Push control register of Tx channel0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_push_ch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_push_ch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_PUSH_CH_SPEC;
impl crate::RegisterSpec for OUT_PUSH_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_push_ch::R`](R) reader structure"]
impl crate::Readable for OUT_PUSH_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_push_ch::W`](W) writer structure"]
impl crate::Writable for OUT_PUSH_CH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_PUSH_CH%s to value 0"]
impl crate::Resettable for OUT_PUSH_CH_SPEC {
    const RESET_VALUE: u32 = 0;
}
