///Register `INT_CLR` writer
pub type W = crate::W<INT_CLR_SPEC>;
///Field `RX_DONE` writer - Set this bit to clear the i2s_rx_done_int interrupt
pub type RX_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `TX_DONE` writer - Set this bit to clear the i2s_tx_done_int interrupt
pub type TX_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `RX_HUNG` writer - Set this bit to clear the i2s_rx_hung_int interrupt
pub type RX_HUNG_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `TX_HUNG` writer - Set this bit to clear the i2s_tx_hung_int interrupt
pub type TX_HUNG_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Set this bit to clear the i2s_rx_done_int interrupt
    #[inline(always)]
    #[must_use]
    pub fn rx_done(&mut self) -> RX_DONE_W<INT_CLR_SPEC> {
        RX_DONE_W::new(self, 0)
    }
    ///Bit 1 - Set this bit to clear the i2s_tx_done_int interrupt
    #[inline(always)]
    #[must_use]
    pub fn tx_done(&mut self) -> TX_DONE_W<INT_CLR_SPEC> {
        TX_DONE_W::new(self, 1)
    }
    ///Bit 2 - Set this bit to clear the i2s_rx_hung_int interrupt
    #[inline(always)]
    #[must_use]
    pub fn rx_hung(&mut self) -> RX_HUNG_W<INT_CLR_SPEC> {
        RX_HUNG_W::new(self, 2)
    }
    ///Bit 3 - Set this bit to clear the i2s_tx_hung_int interrupt
    #[inline(always)]
    #[must_use]
    pub fn tx_hung(&mut self) -> TX_HUNG_W<INT_CLR_SPEC> {
        TX_HUNG_W::new(self, 3)
    }
}
/**I2S interrupt clear register.

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`int_clr::W`](W) writer structure
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
///`reset()` method sets INT_CLR to value 0
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
