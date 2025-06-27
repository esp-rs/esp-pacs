#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `RX_START` writer - "]
pub type RX_START_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TX_START` writer - "]
pub type TX_START_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RX_HUNG` writer - "]
pub type RX_HUNG_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TX_HUNG` writer - "]
pub type TX_HUNG_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IN_DONE` writer - "]
pub type IN_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IN_SUC_EOF` writer - "]
pub type IN_SUC_EOF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IN_ERR_EOF` writer - "]
pub type IN_ERR_EOF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUT_DONE` writer - "]
pub type OUT_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUT_EOF` writer - "]
pub type OUT_EOF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IN_DSCR_ERR` writer - "]
pub type IN_DSCR_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUT_DSCR_ERR` writer - "]
pub type OUT_DSCR_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `IN_DSCR_EMPTY` writer - "]
pub type IN_DSCR_EMPTY_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUTLINK_EOF_ERR` writer - "]
pub type OUTLINK_EOF_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `OUT_TOTAL_EOF` writer - "]
pub type OUT_TOTAL_EOF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SEND_S_Q` writer - "]
pub type SEND_S_Q_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SEND_A_Q` writer - "]
pub type SEND_A_Q_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DMA_INFIFO_FULL_WM` writer - "]
pub type DMA_INFIFO_FULL_WM_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_start(&mut self) -> RX_START_W<INT_CLR_SPEC> {
        RX_START_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_start(&mut self) -> TX_START_W<INT_CLR_SPEC> {
        TX_START_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rx_hung(&mut self) -> RX_HUNG_W<INT_CLR_SPEC> {
        RX_HUNG_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tx_hung(&mut self) -> TX_HUNG_W<INT_CLR_SPEC> {
        TX_HUNG_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn in_done(&mut self) -> IN_DONE_W<INT_CLR_SPEC> {
        IN_DONE_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn in_suc_eof(&mut self) -> IN_SUC_EOF_W<INT_CLR_SPEC> {
        IN_SUC_EOF_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn in_err_eof(&mut self) -> IN_ERR_EOF_W<INT_CLR_SPEC> {
        IN_ERR_EOF_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn out_done(&mut self) -> OUT_DONE_W<INT_CLR_SPEC> {
        OUT_DONE_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn out_eof(&mut self) -> OUT_EOF_W<INT_CLR_SPEC> {
        OUT_EOF_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn in_dscr_err(&mut self) -> IN_DSCR_ERR_W<INT_CLR_SPEC> {
        IN_DSCR_ERR_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn out_dscr_err(&mut self) -> OUT_DSCR_ERR_W<INT_CLR_SPEC> {
        OUT_DSCR_ERR_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn in_dscr_empty(&mut self) -> IN_DSCR_EMPTY_W<INT_CLR_SPEC> {
        IN_DSCR_EMPTY_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn outlink_eof_err(&mut self) -> OUTLINK_EOF_ERR_W<INT_CLR_SPEC> {
        OUTLINK_EOF_ERR_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn out_total_eof(&mut self) -> OUT_TOTAL_EOF_W<INT_CLR_SPEC> {
        OUT_TOTAL_EOF_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn send_s_q(&mut self) -> SEND_S_Q_W<INT_CLR_SPEC> {
        SEND_S_Q_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn send_a_q(&mut self) -> SEND_A_Q_W<INT_CLR_SPEC> {
        SEND_A_Q_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn dma_infifo_full_wm(&mut self) -> DMA_INFIFO_FULL_WM_W<INT_CLR_SPEC> {
        DMA_INFIFO_FULL_WM_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0001_ffff;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {}
