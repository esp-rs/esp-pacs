///Register `INT_ENA` reader
pub type R = crate::R<INT_ENA_SPEC>;
///Register `INT_ENA` writer
pub type W = crate::W<INT_ENA_SPEC>;
///Field `RX_START` reader -
pub type RX_START_R = crate::BitReader;
///Field `RX_START` writer -
pub type RX_START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_START` reader -
pub type TX_START_R = crate::BitReader;
///Field `TX_START` writer -
pub type TX_START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_HUNG` reader -
pub type RX_HUNG_R = crate::BitReader;
///Field `RX_HUNG` writer -
pub type RX_HUNG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_HUNG` reader -
pub type TX_HUNG_R = crate::BitReader;
///Field `TX_HUNG` writer -
pub type TX_HUNG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IN_DONE` reader -
pub type IN_DONE_R = crate::BitReader;
///Field `IN_DONE` writer -
pub type IN_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IN_SUC_EOF` reader -
pub type IN_SUC_EOF_R = crate::BitReader;
///Field `IN_SUC_EOF` writer -
pub type IN_SUC_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IN_ERR_EOF` reader -
pub type IN_ERR_EOF_R = crate::BitReader;
///Field `IN_ERR_EOF` writer -
pub type IN_ERR_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_DONE` reader -
pub type OUT_DONE_R = crate::BitReader;
///Field `OUT_DONE` writer -
pub type OUT_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_EOF` reader -
pub type OUT_EOF_R = crate::BitReader;
///Field `OUT_EOF` writer -
pub type OUT_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IN_DSCR_ERR` reader -
pub type IN_DSCR_ERR_R = crate::BitReader;
///Field `IN_DSCR_ERR` writer -
pub type IN_DSCR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_DSCR_ERR` reader -
pub type OUT_DSCR_ERR_R = crate::BitReader;
///Field `OUT_DSCR_ERR` writer -
pub type OUT_DSCR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IN_DSCR_EMPTY` reader -
pub type IN_DSCR_EMPTY_R = crate::BitReader;
///Field `IN_DSCR_EMPTY` writer -
pub type IN_DSCR_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUTLINK_EOF_ERR` reader -
pub type OUTLINK_EOF_ERR_R = crate::BitReader;
///Field `OUTLINK_EOF_ERR` writer -
pub type OUTLINK_EOF_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUT_TOTAL_EOF` reader -
pub type OUT_TOTAL_EOF_R = crate::BitReader;
///Field `OUT_TOTAL_EOF` writer -
pub type OUT_TOTAL_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEND_S_Q` reader -
pub type SEND_S_Q_R = crate::BitReader;
///Field `SEND_S_Q` writer -
pub type SEND_S_Q_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEND_A_Q` reader -
pub type SEND_A_Q_R = crate::BitReader;
///Field `SEND_A_Q` writer -
pub type SEND_A_Q_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA_INFIFO_FULL_WM` reader -
pub type DMA_INFIFO_FULL_WM_R = crate::BitReader;
///Field `DMA_INFIFO_FULL_WM` writer -
pub type DMA_INFIFO_FULL_WM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn rx_start(&self) -> RX_START_R {
        RX_START_R::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn tx_start(&self) -> TX_START_R {
        TX_START_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn rx_hung(&self) -> RX_HUNG_R {
        RX_HUNG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3
    #[inline(always)]
    pub fn tx_hung(&self) -> TX_HUNG_R {
        TX_HUNG_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4
    #[inline(always)]
    pub fn in_done(&self) -> IN_DONE_R {
        IN_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5
    #[inline(always)]
    pub fn in_suc_eof(&self) -> IN_SUC_EOF_R {
        IN_SUC_EOF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6
    #[inline(always)]
    pub fn in_err_eof(&self) -> IN_ERR_EOF_R {
        IN_ERR_EOF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7
    #[inline(always)]
    pub fn out_done(&self) -> OUT_DONE_R {
        OUT_DONE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8
    #[inline(always)]
    pub fn out_eof(&self) -> OUT_EOF_R {
        OUT_EOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9
    #[inline(always)]
    pub fn in_dscr_err(&self) -> IN_DSCR_ERR_R {
        IN_DSCR_ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10
    #[inline(always)]
    pub fn out_dscr_err(&self) -> OUT_DSCR_ERR_R {
        OUT_DSCR_ERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11
    #[inline(always)]
    pub fn in_dscr_empty(&self) -> IN_DSCR_EMPTY_R {
        IN_DSCR_EMPTY_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12
    #[inline(always)]
    pub fn outlink_eof_err(&self) -> OUTLINK_EOF_ERR_R {
        OUTLINK_EOF_ERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13
    #[inline(always)]
    pub fn out_total_eof(&self) -> OUT_TOTAL_EOF_R {
        OUT_TOTAL_EOF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14
    #[inline(always)]
    pub fn send_s_q(&self) -> SEND_S_Q_R {
        SEND_S_Q_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15
    #[inline(always)]
    pub fn send_a_q(&self) -> SEND_A_Q_R {
        SEND_A_Q_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16
    #[inline(always)]
    pub fn dma_infifo_full_wm(&self) -> DMA_INFIFO_FULL_WM_R {
        DMA_INFIFO_FULL_WM_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("rx_start", &self.rx_start())
            .field("tx_start", &self.tx_start())
            .field("rx_hung", &self.rx_hung())
            .field("tx_hung", &self.tx_hung())
            .field("in_done", &self.in_done())
            .field("in_suc_eof", &self.in_suc_eof())
            .field("in_err_eof", &self.in_err_eof())
            .field("out_done", &self.out_done())
            .field("out_eof", &self.out_eof())
            .field("in_dscr_err", &self.in_dscr_err())
            .field("out_dscr_err", &self.out_dscr_err())
            .field("in_dscr_empty", &self.in_dscr_empty())
            .field("outlink_eof_err", &self.outlink_eof_err())
            .field("out_total_eof", &self.out_total_eof())
            .field("send_s_q", &self.send_s_q())
            .field("send_a_q", &self.send_a_q())
            .field("dma_infifo_full_wm", &self.dma_infifo_full_wm())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    #[must_use]
    pub fn rx_start(&mut self) -> RX_START_W<INT_ENA_SPEC> {
        RX_START_W::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    #[must_use]
    pub fn tx_start(&mut self) -> TX_START_W<INT_ENA_SPEC> {
        TX_START_W::new(self, 1)
    }
    ///Bit 2
    #[inline(always)]
    #[must_use]
    pub fn rx_hung(&mut self) -> RX_HUNG_W<INT_ENA_SPEC> {
        RX_HUNG_W::new(self, 2)
    }
    ///Bit 3
    #[inline(always)]
    #[must_use]
    pub fn tx_hung(&mut self) -> TX_HUNG_W<INT_ENA_SPEC> {
        TX_HUNG_W::new(self, 3)
    }
    ///Bit 4
    #[inline(always)]
    #[must_use]
    pub fn in_done(&mut self) -> IN_DONE_W<INT_ENA_SPEC> {
        IN_DONE_W::new(self, 4)
    }
    ///Bit 5
    #[inline(always)]
    #[must_use]
    pub fn in_suc_eof(&mut self) -> IN_SUC_EOF_W<INT_ENA_SPEC> {
        IN_SUC_EOF_W::new(self, 5)
    }
    ///Bit 6
    #[inline(always)]
    #[must_use]
    pub fn in_err_eof(&mut self) -> IN_ERR_EOF_W<INT_ENA_SPEC> {
        IN_ERR_EOF_W::new(self, 6)
    }
    ///Bit 7
    #[inline(always)]
    #[must_use]
    pub fn out_done(&mut self) -> OUT_DONE_W<INT_ENA_SPEC> {
        OUT_DONE_W::new(self, 7)
    }
    ///Bit 8
    #[inline(always)]
    #[must_use]
    pub fn out_eof(&mut self) -> OUT_EOF_W<INT_ENA_SPEC> {
        OUT_EOF_W::new(self, 8)
    }
    ///Bit 9
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_err(&mut self) -> IN_DSCR_ERR_W<INT_ENA_SPEC> {
        IN_DSCR_ERR_W::new(self, 9)
    }
    ///Bit 10
    #[inline(always)]
    #[must_use]
    pub fn out_dscr_err(&mut self) -> OUT_DSCR_ERR_W<INT_ENA_SPEC> {
        OUT_DSCR_ERR_W::new(self, 10)
    }
    ///Bit 11
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_empty(&mut self) -> IN_DSCR_EMPTY_W<INT_ENA_SPEC> {
        IN_DSCR_EMPTY_W::new(self, 11)
    }
    ///Bit 12
    #[inline(always)]
    #[must_use]
    pub fn outlink_eof_err(&mut self) -> OUTLINK_EOF_ERR_W<INT_ENA_SPEC> {
        OUTLINK_EOF_ERR_W::new(self, 12)
    }
    ///Bit 13
    #[inline(always)]
    #[must_use]
    pub fn out_total_eof(&mut self) -> OUT_TOTAL_EOF_W<INT_ENA_SPEC> {
        OUT_TOTAL_EOF_W::new(self, 13)
    }
    ///Bit 14
    #[inline(always)]
    #[must_use]
    pub fn send_s_q(&mut self) -> SEND_S_Q_W<INT_ENA_SPEC> {
        SEND_S_Q_W::new(self, 14)
    }
    ///Bit 15
    #[inline(always)]
    #[must_use]
    pub fn send_a_q(&mut self) -> SEND_A_Q_W<INT_ENA_SPEC> {
        SEND_A_Q_W::new(self, 15)
    }
    ///Bit 16
    #[inline(always)]
    #[must_use]
    pub fn dma_infifo_full_wm(&mut self) -> DMA_INFIFO_FULL_WM_W<INT_ENA_SPEC> {
        DMA_INFIFO_FULL_WM_W::new(self, 16)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_ena::R`](R) reader structure
impl crate::Readable for INT_ENA_SPEC {}
///`write(|w| ..)` method takes [`int_ena::W`](W) writer structure
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_ENA to value 0
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
