#[doc = "Register `RX_STATE` reader"]
pub type R = crate::R<RX_STATE_SPEC>;
#[doc = "Register `RX_STATE` writer"]
pub type W = crate::W<RX_STATE_SPEC>;
#[doc = "Field `RX_IN_IDLE` reader - represents the bitscrambler rx core in halt mode"]
pub type RX_IN_IDLE_R = crate::BitReader;
#[doc = "Field `RX_IN_RUN` reader - represents the bitscrambler rx core in run mode"]
pub type RX_IN_RUN_R = crate::BitReader;
#[doc = "Field `RX_IN_WAIT` reader - represents the bitscrambler rx core in wait mode to wait write back done"]
pub type RX_IN_WAIT_R = crate::BitReader;
#[doc = "Field `RX_IN_PAUSE` reader - represents the bitscrambler rx core in pause mode"]
pub type RX_IN_PAUSE_R = crate::BitReader;
#[doc = "Field `RX_FIFO_FULL` reader - represents the bitscrambler rx fifo in full state"]
pub type RX_FIFO_FULL_R = crate::BitReader;
#[doc = "Field `RX_EOF_GET_CNT` reader - represents the bytes numbers of bitscrambler rx core when get EOF"]
pub type RX_EOF_GET_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `RX_EOF_OVERLOAD` reader - represents the some EOFs will be lost for bitscrambler rx core"]
pub type RX_EOF_OVERLOAD_R = crate::BitReader;
#[doc = "Field `RX_EOF_TRACE_CLR` writer - write this bit to clear reg_bitscrambler_rx_eof_overload and reg_bitscrambler_rx_eof_get_cnt registers"]
pub type RX_EOF_TRACE_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - represents the bitscrambler rx core in halt mode"]
    #[inline(always)]
    pub fn rx_in_idle(&self) -> RX_IN_IDLE_R {
        RX_IN_IDLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - represents the bitscrambler rx core in run mode"]
    #[inline(always)]
    pub fn rx_in_run(&self) -> RX_IN_RUN_R {
        RX_IN_RUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - represents the bitscrambler rx core in wait mode to wait write back done"]
    #[inline(always)]
    pub fn rx_in_wait(&self) -> RX_IN_WAIT_R {
        RX_IN_WAIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - represents the bitscrambler rx core in pause mode"]
    #[inline(always)]
    pub fn rx_in_pause(&self) -> RX_IN_PAUSE_R {
        RX_IN_PAUSE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - represents the bitscrambler rx fifo in full state"]
    #[inline(always)]
    pub fn rx_fifo_full(&self) -> RX_FIFO_FULL_R {
        RX_FIFO_FULL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:29 - represents the bytes numbers of bitscrambler rx core when get EOF"]
    #[inline(always)]
    pub fn rx_eof_get_cnt(&self) -> RX_EOF_GET_CNT_R {
        RX_EOF_GET_CNT_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 30 - represents the some EOFs will be lost for bitscrambler rx core"]
    #[inline(always)]
    pub fn rx_eof_overload(&self) -> RX_EOF_OVERLOAD_R {
        RX_EOF_OVERLOAD_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_STATE")
            .field("rx_in_idle", &self.rx_in_idle())
            .field("rx_in_run", &self.rx_in_run())
            .field("rx_in_wait", &self.rx_in_wait())
            .field("rx_in_pause", &self.rx_in_pause())
            .field("rx_fifo_full", &self.rx_fifo_full())
            .field("rx_eof_get_cnt", &self.rx_eof_get_cnt())
            .field("rx_eof_overload", &self.rx_eof_overload())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - write this bit to clear reg_bitscrambler_rx_eof_overload and reg_bitscrambler_rx_eof_get_cnt registers"]
    #[inline(always)]
    pub fn rx_eof_trace_clr(&mut self) -> RX_EOF_TRACE_CLR_W<RX_STATE_SPEC> {
        RX_EOF_TRACE_CLR_W::new(self, 31)
    }
}
#[doc = "Status registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_state::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_state::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_STATE_SPEC;
impl crate::RegisterSpec for RX_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_state::R`](R) reader structure"]
impl crate::Readable for RX_STATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_state::W`](W) writer structure"]
impl crate::Writable for RX_STATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX_STATE to value 0x01"]
impl crate::Resettable for RX_STATE_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
