///Register `INT_CLR` writer
pub type W = crate::W<INT_CLR_SPEC>;
///Field `TAKE_DATA` writer - Set this bit to clear I2S_RX_TAKE_DATA_INT interrupt.
pub type TAKE_DATA_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `PUT_DATA` writer - Set this bit to clear I2S_TX_PUT_DATA_INT interrupt.
pub type PUT_DATA_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `RX_WFULL` writer - Set this bit to clear I2S_RX_WFULL_INT interrupt.
pub type RX_WFULL_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `RX_REMPTY` writer - Set this bit to clear I2S_RX_REMPTY_INT interrupt.
pub type RX_REMPTY_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `TX_WFULL` writer - Set this bit to clear I2S_TX_WFULL_INT interrupt.
pub type TX_WFULL_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `TX_REMPTY` writer - Set this bit to clear I2S_TX_REMPTY_INT interrupt.
pub type TX_REMPTY_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `RX_HUNG` writer - Set this bit to clear I2S_RX_HUNG_INT interrupt.
pub type RX_HUNG_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `TX_HUNG` writer - Set this bit to clear I2S_TX_HUNG_INT interrupt.
pub type TX_HUNG_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `IN_DONE` writer - Set this bit to clear I2S_IN_DONE_INT interrupt.
pub type IN_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `IN_SUC_EOF` writer - Set this bit to clear I2S_IN_SUC_EOF_INT interrupt.
pub type IN_SUC_EOF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `IN_ERR_EOF` writer - Reserved.
pub type IN_ERR_EOF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `OUT_DONE` writer - Set this bit to clear I2S_OUT_DONE_INT interrupt.
pub type OUT_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `OUT_EOF` writer - Set this bit to clear I2S_OUT_EOF_INT interrupt.
pub type OUT_EOF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `IN_DSCR_ERR` writer - Set this bit to clear I2S_IN_DSCR_ERR_INT interrupt.
pub type IN_DSCR_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `OUT_DSCR_ERR` writer - Set this bit to clear I2S_OUT_DSCR_ERR_INT interrupt.
pub type OUT_DSCR_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `IN_DSCR_EMPTY` writer - Set this bit to clear I2S_IN_DSCR_EMPTY_INT interrupt.
pub type IN_DSCR_EMPTY_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `OUT_TOTAL_EOF` writer - Set this bit to clear I2S_OUT_TOTAL_EOF_INT interrupt.
pub type OUT_TOTAL_EOF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `V_SYNC` writer - Set this bit to clear I2S_V_SYNC_INT interrupt.
pub type V_SYNC_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Set this bit to clear I2S_RX_TAKE_DATA_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn take_data(&mut self) -> TAKE_DATA_W<INT_CLR_SPEC> {
        TAKE_DATA_W::new(self, 0)
    }
    ///Bit 1 - Set this bit to clear I2S_TX_PUT_DATA_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn put_data(&mut self) -> PUT_DATA_W<INT_CLR_SPEC> {
        PUT_DATA_W::new(self, 1)
    }
    ///Bit 2 - Set this bit to clear I2S_RX_WFULL_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn rx_wfull(&mut self) -> RX_WFULL_W<INT_CLR_SPEC> {
        RX_WFULL_W::new(self, 2)
    }
    ///Bit 3 - Set this bit to clear I2S_RX_REMPTY_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn rx_rempty(&mut self) -> RX_REMPTY_W<INT_CLR_SPEC> {
        RX_REMPTY_W::new(self, 3)
    }
    ///Bit 4 - Set this bit to clear I2S_TX_WFULL_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn tx_wfull(&mut self) -> TX_WFULL_W<INT_CLR_SPEC> {
        TX_WFULL_W::new(self, 4)
    }
    ///Bit 5 - Set this bit to clear I2S_TX_REMPTY_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn tx_rempty(&mut self) -> TX_REMPTY_W<INT_CLR_SPEC> {
        TX_REMPTY_W::new(self, 5)
    }
    ///Bit 6 - Set this bit to clear I2S_RX_HUNG_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn rx_hung(&mut self) -> RX_HUNG_W<INT_CLR_SPEC> {
        RX_HUNG_W::new(self, 6)
    }
    ///Bit 7 - Set this bit to clear I2S_TX_HUNG_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn tx_hung(&mut self) -> TX_HUNG_W<INT_CLR_SPEC> {
        TX_HUNG_W::new(self, 7)
    }
    ///Bit 8 - Set this bit to clear I2S_IN_DONE_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn in_done(&mut self) -> IN_DONE_W<INT_CLR_SPEC> {
        IN_DONE_W::new(self, 8)
    }
    ///Bit 9 - Set this bit to clear I2S_IN_SUC_EOF_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn in_suc_eof(&mut self) -> IN_SUC_EOF_W<INT_CLR_SPEC> {
        IN_SUC_EOF_W::new(self, 9)
    }
    ///Bit 10 - Reserved.
    #[inline(always)]
    #[must_use]
    pub fn in_err_eof(&mut self) -> IN_ERR_EOF_W<INT_CLR_SPEC> {
        IN_ERR_EOF_W::new(self, 10)
    }
    ///Bit 11 - Set this bit to clear I2S_OUT_DONE_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn out_done(&mut self) -> OUT_DONE_W<INT_CLR_SPEC> {
        OUT_DONE_W::new(self, 11)
    }
    ///Bit 12 - Set this bit to clear I2S_OUT_EOF_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn out_eof(&mut self) -> OUT_EOF_W<INT_CLR_SPEC> {
        OUT_EOF_W::new(self, 12)
    }
    ///Bit 13 - Set this bit to clear I2S_IN_DSCR_ERR_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_err(&mut self) -> IN_DSCR_ERR_W<INT_CLR_SPEC> {
        IN_DSCR_ERR_W::new(self, 13)
    }
    ///Bit 14 - Set this bit to clear I2S_OUT_DSCR_ERR_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn out_dscr_err(&mut self) -> OUT_DSCR_ERR_W<INT_CLR_SPEC> {
        OUT_DSCR_ERR_W::new(self, 14)
    }
    ///Bit 15 - Set this bit to clear I2S_IN_DSCR_EMPTY_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_empty(&mut self) -> IN_DSCR_EMPTY_W<INT_CLR_SPEC> {
        IN_DSCR_EMPTY_W::new(self, 15)
    }
    ///Bit 16 - Set this bit to clear I2S_OUT_TOTAL_EOF_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn out_total_eof(&mut self) -> OUT_TOTAL_EOF_W<INT_CLR_SPEC> {
        OUT_TOTAL_EOF_W::new(self, 16)
    }
    ///Bit 17 - Set this bit to clear I2S_V_SYNC_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn v_sync(&mut self) -> V_SYNC_W<INT_CLR_SPEC> {
        V_SYNC_W::new(self, 17)
    }
}
/**Interrupt clear bits

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`int_clr::W`](W) writer structure
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0003_ffff;
}
///`reset()` method sets INT_CLR to value 0
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
