#[doc = "Register `CHNL1_FIFO_CTRL` writer"]
pub type W = crate::W<CHNL1_FIFO_CTRL_SPEC>;
#[doc = "Field `CHNL1_INFIFO_RESET` writer - Set this bit to reset outfifo."]
pub type CHNL1_INFIFO_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL1_OUTFIFO_RESET` writer - Set this bit to reset infifo."]
pub type CHNL1_OUTFIFO_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CHNL1_FIFO_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to reset outfifo."]
    #[inline(always)]
    pub fn chnl1_infifo_reset(&mut self) -> CHNL1_INFIFO_RESET_W<'_, CHNL1_FIFO_CTRL_SPEC> {
        CHNL1_INFIFO_RESET_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to reset infifo."]
    #[inline(always)]
    pub fn chnl1_outfifo_reset(&mut self) -> CHNL1_OUTFIFO_RESET_W<'_, CHNL1_FIFO_CTRL_SPEC> {
        CHNL1_OUTFIFO_RESET_W::new(self, 1)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl1_fifo_ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHNL1_FIFO_CTRL_SPEC;
impl crate::RegisterSpec for CHNL1_FIFO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chnl1_fifo_ctrl::W`](W) writer structure"]
impl crate::Writable for CHNL1_FIFO_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNL1_FIFO_CTRL to value 0"]
impl crate::Resettable for CHNL1_FIFO_CTRL_SPEC {}
