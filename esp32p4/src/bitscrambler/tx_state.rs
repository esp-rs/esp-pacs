#[doc = "Register `TX_STATE` reader"]
pub type R = crate::R<TX_STATE_SPEC>;
#[doc = "Register `TX_STATE` writer"]
pub type W = crate::W<TX_STATE_SPEC>;
#[doc = "Field `TX_IN_IDLE` reader - represents the bitscrambler tx core in halt mode"]
pub type TX_IN_IDLE_R = crate::BitReader;
#[doc = "Field `TX_IN_RUN` reader - represents the bitscrambler tx core in run mode"]
pub type TX_IN_RUN_R = crate::BitReader;
#[doc = "Field `TX_IN_WAIT` reader - represents the bitscrambler tx core in wait mode to wait write back done"]
pub type TX_IN_WAIT_R = crate::BitReader;
#[doc = "Field `TX_IN_PAUSE` reader - represents the bitscrambler tx core in pause mode"]
pub type TX_IN_PAUSE_R = crate::BitReader;
#[doc = "Field `TX_FIFO_EMPTY` reader - represents the bitscrambler tx fifo in empty state"]
pub type TX_FIFO_EMPTY_R = crate::BitReader;
#[doc = "Field `TX_EOF_GET_CNT` reader - represents the bytes numbers of bitscrambler tx core when get EOF"]
pub type TX_EOF_GET_CNT_R = crate::FieldReader<u16>;
#[doc = "Field `TX_EOF_OVERLOAD` reader - represents the some EOFs will be lost for bitscrambler tx core"]
pub type TX_EOF_OVERLOAD_R = crate::BitReader;
#[doc = "Field `TX_EOF_TRACE_CLR` writer - write this bit to clear reg_bitscrambler_tx_eof_overload and reg_bitscrambler_tx_eof_get_cnt registers"]
pub type TX_EOF_TRACE_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - represents the bitscrambler tx core in halt mode"]
    #[inline(always)]
    pub fn tx_in_idle(&self) -> TX_IN_IDLE_R {
        TX_IN_IDLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - represents the bitscrambler tx core in run mode"]
    #[inline(always)]
    pub fn tx_in_run(&self) -> TX_IN_RUN_R {
        TX_IN_RUN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - represents the bitscrambler tx core in wait mode to wait write back done"]
    #[inline(always)]
    pub fn tx_in_wait(&self) -> TX_IN_WAIT_R {
        TX_IN_WAIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - represents the bitscrambler tx core in pause mode"]
    #[inline(always)]
    pub fn tx_in_pause(&self) -> TX_IN_PAUSE_R {
        TX_IN_PAUSE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - represents the bitscrambler tx fifo in empty state"]
    #[inline(always)]
    pub fn tx_fifo_empty(&self) -> TX_FIFO_EMPTY_R {
        TX_FIFO_EMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:29 - represents the bytes numbers of bitscrambler tx core when get EOF"]
    #[inline(always)]
    pub fn tx_eof_get_cnt(&self) -> TX_EOF_GET_CNT_R {
        TX_EOF_GET_CNT_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 30 - represents the some EOFs will be lost for bitscrambler tx core"]
    #[inline(always)]
    pub fn tx_eof_overload(&self) -> TX_EOF_OVERLOAD_R {
        TX_EOF_OVERLOAD_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_STATE")
            .field("tx_in_idle", &format_args!("{}", self.tx_in_idle().bit()))
            .field("tx_in_run", &format_args!("{}", self.tx_in_run().bit()))
            .field("tx_in_wait", &format_args!("{}", self.tx_in_wait().bit()))
            .field("tx_in_pause", &format_args!("{}", self.tx_in_pause().bit()))
            .field(
                "tx_fifo_empty",
                &format_args!("{}", self.tx_fifo_empty().bit()),
            )
            .field(
                "tx_eof_get_cnt",
                &format_args!("{}", self.tx_eof_get_cnt().bits()),
            )
            .field(
                "tx_eof_overload",
                &format_args!("{}", self.tx_eof_overload().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TX_STATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 31 - write this bit to clear reg_bitscrambler_tx_eof_overload and reg_bitscrambler_tx_eof_get_cnt registers"]
    #[inline(always)]
    #[must_use]
    pub fn tx_eof_trace_clr(&mut self) -> TX_EOF_TRACE_CLR_W<TX_STATE_SPEC> {
        TX_EOF_TRACE_CLR_W::new(self, 31)
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
#[doc = "Status registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_state::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_state::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_STATE_SPEC;
impl crate::RegisterSpec for TX_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_state::R`](R) reader structure"]
impl crate::Readable for TX_STATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_state::W`](W) writer structure"]
impl crate::Writable for TX_STATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_STATE to value 0x01"]
impl crate::Resettable for TX_STATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
