#[doc = "Register `COMMAND` writer"]
pub type W = crate::W<COMMAND_SPEC>;
#[doc = "Field `RXRPMV` writer - RX Buffer read pointer move."]
pub type RXRPMV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRB` writer - Release RX Buffer. This command flushes RX buffer and resets its memory pointers. 0: invalid 1: delete"]
pub type RRB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDO` writer - Clear Data Overrun flag in RX buffer. 0: invalid 1: clear"]
pub type CDO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERCRST` writer - Error Counters Reset. When unit is bus off, issuing this command will request erasing TEC, REC counters after 128 consecutive ocurrences of 11 recessive bits. Upon completion, TEC and REC are erased and fault confinement State is set to error-active. When unit is not bus-off, or when unit is bus-off due to being disabled (SETTINGS\\[ENA\\] = ’0’), this command has no effect."]
pub type ERCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFCRST` writer - Clear RX bus traffic counter (RX_COUNTER register)."]
pub type RXFCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFCRST` writer - Clear TX bus traffic counter (TX_COUNTER register)."]
pub type TXFCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPEXS` writer - Clear Protocol exception status (STATUS\\[PEXS\\])."]
pub type CPEXS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRXPE` writer - Clear STATUS\\[RXPE\\] flag."]
pub type CRXPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTXPE` writer - Clear STATUS\\[TXPE\\] flag."]
pub type CTXPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTXDPE` writer - Clear STATUS\\[TXDPE\\] flag."]
pub type CTXDPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COMMAND_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 1 - RX Buffer read pointer move."]
    #[inline(always)]
    pub fn rxrpmv(&mut self) -> RXRPMV_W<COMMAND_SPEC> {
        RXRPMV_W::new(self, 1)
    }
    #[doc = "Bit 2 - Release RX Buffer. This command flushes RX buffer and resets its memory pointers. 0: invalid 1: delete"]
    #[inline(always)]
    pub fn rrb(&mut self) -> RRB_W<COMMAND_SPEC> {
        RRB_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Data Overrun flag in RX buffer. 0: invalid 1: clear"]
    #[inline(always)]
    pub fn cdo(&mut self) -> CDO_W<COMMAND_SPEC> {
        CDO_W::new(self, 3)
    }
    #[doc = "Bit 4 - Error Counters Reset. When unit is bus off, issuing this command will request erasing TEC, REC counters after 128 consecutive ocurrences of 11 recessive bits. Upon completion, TEC and REC are erased and fault confinement State is set to error-active. When unit is not bus-off, or when unit is bus-off due to being disabled (SETTINGS\\[ENA\\] = ’0’), this command has no effect."]
    #[inline(always)]
    pub fn ercrst(&mut self) -> ERCRST_W<COMMAND_SPEC> {
        ERCRST_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear RX bus traffic counter (RX_COUNTER register)."]
    #[inline(always)]
    pub fn rxfcrst(&mut self) -> RXFCRST_W<COMMAND_SPEC> {
        RXFCRST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear TX bus traffic counter (TX_COUNTER register)."]
    #[inline(always)]
    pub fn txfcrst(&mut self) -> TXFCRST_W<COMMAND_SPEC> {
        TXFCRST_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear Protocol exception status (STATUS\\[PEXS\\])."]
    #[inline(always)]
    pub fn cpexs(&mut self) -> CPEXS_W<COMMAND_SPEC> {
        CPEXS_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear STATUS\\[RXPE\\] flag."]
    #[inline(always)]
    pub fn crxpe(&mut self) -> CRXPE_W<COMMAND_SPEC> {
        CRXPE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear STATUS\\[TXPE\\] flag."]
    #[inline(always)]
    pub fn ctxpe(&mut self) -> CTXPE_W<COMMAND_SPEC> {
        CTXPE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear STATUS\\[TXDPE\\] flag."]
    #[inline(always)]
    pub fn ctxdpe(&mut self) -> CTXDPE_W<COMMAND_SPEC> {
        CTXDPE_W::new(self, 10)
    }
}
#[doc = "TWAI FD command register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COMMAND_SPEC;
impl crate::RegisterSpec for COMMAND_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`command::W`](W) writer structure"]
impl crate::Writable for COMMAND_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMMAND to value 0"]
impl crate::Resettable for COMMAND_SPEC {}
