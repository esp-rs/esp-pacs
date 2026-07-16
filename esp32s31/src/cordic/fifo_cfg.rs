#[doc = "Register `FIFO_CFG` writer"]
pub type W = crate::W<FIFO_CFG_SPEC>;
#[doc = "Field `INFIFO_RESET` writer - infifo_reset_register"]
pub type INFIFO_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTFIFO_RESET` writer - outfifo_reset_register"]
pub type OUTFIFO_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FIFO_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - infifo_reset_register"]
    #[inline(always)]
    pub fn infifo_reset(&mut self) -> INFIFO_RESET_W<'_, FIFO_CFG_SPEC> {
        INFIFO_RESET_W::new(self, 0)
    }
    #[doc = "Bit 1 - outfifo_reset_register"]
    #[inline(always)]
    pub fn outfifo_reset(&mut self) -> OUTFIFO_RESET_W<'_, FIFO_CFG_SPEC> {
        OUTFIFO_RESET_W::new(self, 1)
    }
}
#[doc = "Cordic FIFO configuration register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_cfg::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_CFG_SPEC;
impl crate::RegisterSpec for FIFO_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fifo_cfg::W`](W) writer structure"]
impl crate::Writable for FIFO_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO_CFG to value 0"]
impl crate::Resettable for FIFO_CFG_SPEC {}
