///Register `INT_CLR` writer
pub type W = crate::W<INT_CLR_SPEC>;
///Field `SLAVE_TRAN_COMP` writer - clear slave transit complete interrupt
pub type SLAVE_TRAN_COMP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `ARBITRATION_LOST` writer - clear arbitration lost interrupt
pub type ARBITRATION_LOST_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `MASTER_TRAN_COMP` writer - clear master transit complete interrupt
pub type MASTER_TRAN_COMP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `TRANS_COMPLETE` writer - clear transit complete interrupt
pub type TRANS_COMPLETE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `TIME_OUT` writer - clear time out interrupt
pub type TIME_OUT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `ACK_ERR` writer - clear ack error interrupt
pub type ACK_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `RX_DATA` writer - clear receive data interrupt
pub type RX_DATA_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `TX_DATA` writer - clear transit load data complete interrupt
pub type TX_DATA_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `DETECT_START` writer - clear detect start interrupt
pub type DETECT_START_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - clear slave transit complete interrupt
    #[inline(always)]
    #[must_use]
    pub fn slave_tran_comp(&mut self) -> SLAVE_TRAN_COMP_W<INT_CLR_SPEC> {
        SLAVE_TRAN_COMP_W::new(self, 0)
    }
    ///Bit 1 - clear arbitration lost interrupt
    #[inline(always)]
    #[must_use]
    pub fn arbitration_lost(&mut self) -> ARBITRATION_LOST_W<INT_CLR_SPEC> {
        ARBITRATION_LOST_W::new(self, 1)
    }
    ///Bit 2 - clear master transit complete interrupt
    #[inline(always)]
    #[must_use]
    pub fn master_tran_comp(&mut self) -> MASTER_TRAN_COMP_W<INT_CLR_SPEC> {
        MASTER_TRAN_COMP_W::new(self, 2)
    }
    ///Bit 3 - clear transit complete interrupt
    #[inline(always)]
    #[must_use]
    pub fn trans_complete(&mut self) -> TRANS_COMPLETE_W<INT_CLR_SPEC> {
        TRANS_COMPLETE_W::new(self, 3)
    }
    ///Bit 4 - clear time out interrupt
    #[inline(always)]
    #[must_use]
    pub fn time_out(&mut self) -> TIME_OUT_W<INT_CLR_SPEC> {
        TIME_OUT_W::new(self, 4)
    }
    ///Bit 5 - clear ack error interrupt
    #[inline(always)]
    #[must_use]
    pub fn ack_err(&mut self) -> ACK_ERR_W<INT_CLR_SPEC> {
        ACK_ERR_W::new(self, 5)
    }
    ///Bit 6 - clear receive data interrupt
    #[inline(always)]
    #[must_use]
    pub fn rx_data(&mut self) -> RX_DATA_W<INT_CLR_SPEC> {
        RX_DATA_W::new(self, 6)
    }
    ///Bit 7 - clear transit load data complete interrupt
    #[inline(always)]
    #[must_use]
    pub fn tx_data(&mut self) -> TX_DATA_W<INT_CLR_SPEC> {
        TX_DATA_W::new(self, 7)
    }
    ///Bit 8 - clear detect start interrupt
    #[inline(always)]
    #[must_use]
    pub fn detect_start(&mut self) -> DETECT_START_W<INT_CLR_SPEC> {
        DETECT_START_W::new(self, 8)
    }
}
/**interrupt clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`int_clr::W`](W) writer structure
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01ff;
}
///`reset()` method sets INT_CLR to value 0
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
