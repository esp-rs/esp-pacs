#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<STATUS_SPEC>;
#[doc = "Field `STNOTSTOP` reader - Is 1 if bus is busy(activity) and 0 when in a STOP condition. Other bits may also set when busy. Note that this can also be true from an S0 or S1 error, which waits for an Exit Pattern."]
pub type STNOTSTOP_R = crate::BitReader;
#[doc = "Field `STMSG` reader - Is 1 if this bus Slave is listening to the bus traffic or repsonding, If STNOSTOP=1, then this will be 0 when a non-matching address seen until next respeated START it STOP."]
pub type STMSG_R = crate::BitReader;
#[doc = "Field `STCCCH` reader - Is 1 if a CCC message is being handled automatically."]
pub type STCCCH_R = crate::BitReader;
#[doc = "Field `STREQRD` reader - 1 if the req in process is an sdr read from this slave or an IBI is being pushed out,"]
pub type STREQRD_R = crate::BitReader;
#[doc = "Field `STREQWR` reader - NA"]
pub type STREQWR_R = crate::BitReader;
#[doc = "Field `STDAA` reader - NA"]
pub type STDAA_R = crate::BitReader;
#[doc = "Field `STHDR` reader - NA"]
pub type STHDR_R = crate::BitReader;
#[doc = "Field `START` reader - NA"]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - NA"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCHED` reader - NA"]
pub type MATCHED_R = crate::BitReader;
#[doc = "Field `MATCHED` writer - NA"]
pub type MATCHED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - NA"]
pub type STOP_R = crate::BitReader;
#[doc = "Field `STOP` writer - NA"]
pub type STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPEND` reader - Receiving a message from master,which is not being handled by block(not a CCC internally processed). For all but External FIFO, this uses DATACTRL RXTRIG, which defaults to not-empty. If DMA is enabled for RX, DMA will be signaled as well. Will self-clear if data is read(FIFO and non-FIFO)"]
pub type RXPEND_R = crate::BitReader;
#[doc = "Field `TXNOTFULL` reader - Is 1 when the To-bus buffer/FIFO can accept more data to go out. Defau:1. For all but External FIFO, this uses DATACTRL TXTRIG,which defaults to not-full. If DMA is enabled for TX, it will also be signaled to provide more."]
pub type TXNOTFULL_R = crate::BitReader;
#[doc = "Field `DACHG` reader - The Slv Dynamic Address has been assigned, reassigned, or reset(lost) and is now in that state of being valid or none. Actual DA can be seen in the DYNADDR register. Note that this will also be used when MAP Auto feature is configured. This will be changing one or more MAP items. See DYNADDR and/or MAPCTRLn. DYNAADDR for the main DA(0) will indicate if last change was due to Auto MAP."]
pub type DACHG_R = crate::BitReader;
#[doc = "Field `DACHG` writer - The Slv Dynamic Address has been assigned, reassigned, or reset(lost) and is now in that state of being valid or none. Actual DA can be seen in the DYNADDR register. Note that this will also be used when MAP Auto feature is configured. This will be changing one or more MAP items. See DYNADDR and/or MAPCTRLn. DYNAADDR for the main DA(0) will indicate if last change was due to Auto MAP."]
pub type DACHG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCC` reader - A common -command-code(CCC), not handled by block, has been received. This acts differently between: *Broadcasted ones, which will then also correspond with RXPEND and the 1st byte will be the CCC(command) . *Direct ones, which may never be directed to this device. If it is, then the TXSEND or RXPEND will be triggered with this end the RXPEND will contain the command."]
pub type CCC_R = crate::BitReader;
#[doc = "Field `CCC` writer - A common -command-code(CCC), not handled by block, has been received. This acts differently between: *Broadcasted ones, which will then also correspond with RXPEND and the 1st byte will be the CCC(command) . *Direct ones, which may never be directed to this device. If it is, then the TXSEND or RXPEND will be triggered with this end the RXPEND will contain the command."]
pub type CCC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRWARN` reader - NA"]
pub type ERRWARN_R = crate::BitReader;
#[doc = "Field `HDRMATCH` reader - NA"]
pub type HDRMATCH_R = crate::BitReader;
#[doc = "Field `HDRMATCH` writer - NA"]
pub type HDRMATCH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Is 1 if bus is busy(activity) and 0 when in a STOP condition. Other bits may also set when busy. Note that this can also be true from an S0 or S1 error, which waits for an Exit Pattern."]
    #[inline(always)]
    pub fn stnotstop(&self) -> STNOTSTOP_R {
        STNOTSTOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Is 1 if this bus Slave is listening to the bus traffic or repsonding, If STNOSTOP=1, then this will be 0 when a non-matching address seen until next respeated START it STOP."]
    #[inline(always)]
    pub fn stmsg(&self) -> STMSG_R {
        STMSG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Is 1 if a CCC message is being handled automatically."]
    #[inline(always)]
    pub fn stccch(&self) -> STCCCH_R {
        STCCCH_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1 if the req in process is an sdr read from this slave or an IBI is being pushed out,"]
    #[inline(always)]
    pub fn streqrd(&self) -> STREQRD_R {
        STREQRD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn streqwr(&self) -> STREQWR_R {
        STREQWR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn stdaa(&self) -> STDAA_R {
        STDAA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn sthdr(&self) -> STHDR_R {
        STHDR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn matched(&self) -> MATCHED_R {
        MATCHED_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receiving a message from master,which is not being handled by block(not a CCC internally processed). For all but External FIFO, this uses DATACTRL RXTRIG, which defaults to not-empty. If DMA is enabled for RX, DMA will be signaled as well. Will self-clear if data is read(FIFO and non-FIFO)"]
    #[inline(always)]
    pub fn rxpend(&self) -> RXPEND_R {
        RXPEND_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Is 1 when the To-bus buffer/FIFO can accept more data to go out. Defau:1. For all but External FIFO, this uses DATACTRL TXTRIG,which defaults to not-full. If DMA is enabled for TX, it will also be signaled to provide more."]
    #[inline(always)]
    pub fn txnotfull(&self) -> TXNOTFULL_R {
        TXNOTFULL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The Slv Dynamic Address has been assigned, reassigned, or reset(lost) and is now in that state of being valid or none. Actual DA can be seen in the DYNADDR register. Note that this will also be used when MAP Auto feature is configured. This will be changing one or more MAP items. See DYNADDR and/or MAPCTRLn. DYNAADDR for the main DA(0) will indicate if last change was due to Auto MAP."]
    #[inline(always)]
    pub fn dachg(&self) -> DACHG_R {
        DACHG_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - A common -command-code(CCC), not handled by block, has been received. This acts differently between: *Broadcasted ones, which will then also correspond with RXPEND and the 1st byte will be the CCC(command) . *Direct ones, which may never be directed to this device. If it is, then the TXSEND or RXPEND will be triggered with this end the RXPEND will contain the command."]
    #[inline(always)]
    pub fn ccc(&self) -> CCC_R {
        CCC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NA"]
    #[inline(always)]
    pub fn errwarn(&self) -> ERRWARN_R {
        ERRWARN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn hdrmatch(&self) -> HDRMATCH_R {
        HDRMATCH_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("stnotstop", &self.stnotstop())
            .field("stmsg", &self.stmsg())
            .field("stccch", &self.stccch())
            .field("streqrd", &self.streqrd())
            .field("streqwr", &self.streqwr())
            .field("stdaa", &self.stdaa())
            .field("sthdr", &self.sthdr())
            .field("start", &self.start())
            .field("matched", &self.matched())
            .field("stop", &self.stop())
            .field("rxpend", &self.rxpend())
            .field("txnotfull", &self.txnotfull())
            .field("dachg", &self.dachg())
            .field("ccc", &self.ccc())
            .field("errwarn", &self.errwarn())
            .field("hdrmatch", &self.hdrmatch())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<STATUS_SPEC> {
        START_W::new(self, 8)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn matched(&mut self) -> MATCHED_W<STATUS_SPEC> {
        MATCHED_W::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<STATUS_SPEC> {
        STOP_W::new(self, 10)
    }
    #[doc = "Bit 13 - The Slv Dynamic Address has been assigned, reassigned, or reset(lost) and is now in that state of being valid or none. Actual DA can be seen in the DYNADDR register. Note that this will also be used when MAP Auto feature is configured. This will be changing one or more MAP items. See DYNADDR and/or MAPCTRLn. DYNAADDR for the main DA(0) will indicate if last change was due to Auto MAP."]
    #[inline(always)]
    pub fn dachg(&mut self) -> DACHG_W<STATUS_SPEC> {
        DACHG_W::new(self, 13)
    }
    #[doc = "Bit 14 - A common -command-code(CCC), not handled by block, has been received. This acts differently between: *Broadcasted ones, which will then also correspond with RXPEND and the 1st byte will be the CCC(command) . *Direct ones, which may never be directed to this device. If it is, then the TXSEND or RXPEND will be triggered with this end the RXPEND will contain the command."]
    #[inline(always)]
    pub fn ccc(&mut self) -> CCC_W<STATUS_SPEC> {
        CCC_W::new(self, 14)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn hdrmatch(&mut self) -> HDRMATCH_W<STATUS_SPEC> {
        HDRMATCH_W::new(self, 16)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {}
