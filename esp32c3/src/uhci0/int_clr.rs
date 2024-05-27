///Register `INT_CLR` writer
pub type W = crate::W<INT_CLR_SPEC>;
///Field `RX_START` writer - a
pub type RX_START_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `TX_START` writer - a
pub type TX_START_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `RX_HUNG` writer - a
pub type RX_HUNG_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `TX_HUNG` writer - a
pub type TX_HUNG_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `SEND_S_REG_Q` writer - a
pub type SEND_S_REG_Q_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `SEND_A_REG_Q` writer - a
pub type SEND_A_REG_Q_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `OUTLINK_EOF_ERR` writer - a
pub type OUTLINK_EOF_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `APP_CTRL0` writer - a
pub type APP_CTRL0_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `APP_CTRL1` writer - a
pub type APP_CTRL1_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - a
    #[inline(always)]
    #[must_use]
    pub fn rx_start(&mut self) -> RX_START_W<INT_CLR_SPEC> {
        RX_START_W::new(self, 0)
    }
    ///Bit 1 - a
    #[inline(always)]
    #[must_use]
    pub fn tx_start(&mut self) -> TX_START_W<INT_CLR_SPEC> {
        TX_START_W::new(self, 1)
    }
    ///Bit 2 - a
    #[inline(always)]
    #[must_use]
    pub fn rx_hung(&mut self) -> RX_HUNG_W<INT_CLR_SPEC> {
        RX_HUNG_W::new(self, 2)
    }
    ///Bit 3 - a
    #[inline(always)]
    #[must_use]
    pub fn tx_hung(&mut self) -> TX_HUNG_W<INT_CLR_SPEC> {
        TX_HUNG_W::new(self, 3)
    }
    ///Bit 4 - a
    #[inline(always)]
    #[must_use]
    pub fn send_s_reg_q(&mut self) -> SEND_S_REG_Q_W<INT_CLR_SPEC> {
        SEND_S_REG_Q_W::new(self, 4)
    }
    ///Bit 5 - a
    #[inline(always)]
    #[must_use]
    pub fn send_a_reg_q(&mut self) -> SEND_A_REG_Q_W<INT_CLR_SPEC> {
        SEND_A_REG_Q_W::new(self, 5)
    }
    ///Bit 6 - a
    #[inline(always)]
    #[must_use]
    pub fn outlink_eof_err(&mut self) -> OUTLINK_EOF_ERR_W<INT_CLR_SPEC> {
        OUTLINK_EOF_ERR_W::new(self, 6)
    }
    ///Bit 7 - a
    #[inline(always)]
    #[must_use]
    pub fn app_ctrl0(&mut self) -> APP_CTRL0_W<INT_CLR_SPEC> {
        APP_CTRL0_W::new(self, 7)
    }
    ///Bit 8 - a
    #[inline(always)]
    #[must_use]
    pub fn app_ctrl1(&mut self) -> APP_CTRL1_W<INT_CLR_SPEC> {
        APP_CTRL1_W::new(self, 8)
    }
}
/**a

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
