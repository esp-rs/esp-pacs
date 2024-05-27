///Register `INT_ENA` reader
pub type R = crate::R<INT_ENA_SPEC>;
///Register `INT_ENA` writer
pub type W = crate::W<INT_ENA_SPEC>;
///Field `RX_START` reader - Set this bit to enable the interrupt of UHCI_RX_START_INT.
pub type RX_START_R = crate::BitReader;
///Field `RX_START` writer - Set this bit to enable the interrupt of UHCI_RX_START_INT.
pub type RX_START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_START` reader - Set this bit to enable the interrupt of UHCI_TX_START_INT.
pub type TX_START_R = crate::BitReader;
///Field `TX_START` writer - Set this bit to enable the interrupt of UHCI_TX_START_INT.
pub type TX_START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_HUNG` reader - Set this bit to enable the interrupt of UHCI_RX_HUNG_INT.
pub type RX_HUNG_R = crate::BitReader;
///Field `RX_HUNG` writer - Set this bit to enable the interrupt of UHCI_RX_HUNG_INT.
pub type RX_HUNG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_HUNG` reader - Set this bit to enable the interrupt of UHCI_TX_HUNG_INT.
pub type TX_HUNG_R = crate::BitReader;
///Field `TX_HUNG` writer - Set this bit to enable the interrupt of UHCI_TX_HUNG_INT.
pub type TX_HUNG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEND_S_REG_Q` reader - Set this bit to enable the interrupt of UHCI_SEND_S_REG_Q_INT.
pub type SEND_S_REG_Q_R = crate::BitReader;
///Field `SEND_S_REG_Q` writer - Set this bit to enable the interrupt of UHCI_SEND_S_REG_Q_INT.
pub type SEND_S_REG_Q_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEND_A_REG_Q` reader - Set this bit to enable the interrupt of UHCI_SEND_A_REG_Q_INT.
pub type SEND_A_REG_Q_R = crate::BitReader;
///Field `SEND_A_REG_Q` writer - Set this bit to enable the interrupt of UHCI_SEND_A_REG_Q_INT.
pub type SEND_A_REG_Q_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OUTLINK_EOF_ERR` reader - Set this bit to enable the interrupt of UHCI_OUT_EOF_INT.
pub type OUTLINK_EOF_ERR_R = crate::BitReader;
///Field `OUTLINK_EOF_ERR` writer - Set this bit to enable the interrupt of UHCI_OUT_EOF_INT.
pub type OUTLINK_EOF_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APP_CTRL0` reader - Set this bit to enable the interrupt of UHCI_APP_CTRL0_INT.
pub type APP_CTRL0_R = crate::BitReader;
///Field `APP_CTRL0` writer - Set this bit to enable the interrupt of UHCI_APP_CTRL0_INT.
pub type APP_CTRL0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APP_CTRL1` reader - Set this bit to enable the interrupt of UHCI_APP_CTRL1_INT.
pub type APP_CTRL1_R = crate::BitReader;
///Field `APP_CTRL1` writer - Set this bit to enable the interrupt of UHCI_APP_CTRL1_INT.
pub type APP_CTRL1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set this bit to enable the interrupt of UHCI_RX_START_INT.
    #[inline(always)]
    pub fn rx_start(&self) -> RX_START_R {
        RX_START_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set this bit to enable the interrupt of UHCI_TX_START_INT.
    #[inline(always)]
    pub fn tx_start(&self) -> TX_START_R {
        TX_START_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Set this bit to enable the interrupt of UHCI_RX_HUNG_INT.
    #[inline(always)]
    pub fn rx_hung(&self) -> RX_HUNG_R {
        RX_HUNG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Set this bit to enable the interrupt of UHCI_TX_HUNG_INT.
    #[inline(always)]
    pub fn tx_hung(&self) -> TX_HUNG_R {
        TX_HUNG_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Set this bit to enable the interrupt of UHCI_SEND_S_REG_Q_INT.
    #[inline(always)]
    pub fn send_s_reg_q(&self) -> SEND_S_REG_Q_R {
        SEND_S_REG_Q_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Set this bit to enable the interrupt of UHCI_SEND_A_REG_Q_INT.
    #[inline(always)]
    pub fn send_a_reg_q(&self) -> SEND_A_REG_Q_R {
        SEND_A_REG_Q_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Set this bit to enable the interrupt of UHCI_OUT_EOF_INT.
    #[inline(always)]
    pub fn outlink_eof_err(&self) -> OUTLINK_EOF_ERR_R {
        OUTLINK_EOF_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Set this bit to enable the interrupt of UHCI_APP_CTRL0_INT.
    #[inline(always)]
    pub fn app_ctrl0(&self) -> APP_CTRL0_R {
        APP_CTRL0_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Set this bit to enable the interrupt of UHCI_APP_CTRL1_INT.
    #[inline(always)]
    pub fn app_ctrl1(&self) -> APP_CTRL1_R {
        APP_CTRL1_R::new(((self.bits >> 8) & 1) != 0)
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
            .field("send_s_reg_q", &self.send_s_reg_q())
            .field("send_a_reg_q", &self.send_a_reg_q())
            .field("outlink_eof_err", &self.outlink_eof_err())
            .field("app_ctrl0", &self.app_ctrl0())
            .field("app_ctrl1", &self.app_ctrl1())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set this bit to enable the interrupt of UHCI_RX_START_INT.
    #[inline(always)]
    #[must_use]
    pub fn rx_start(&mut self) -> RX_START_W<INT_ENA_SPEC> {
        RX_START_W::new(self, 0)
    }
    ///Bit 1 - Set this bit to enable the interrupt of UHCI_TX_START_INT.
    #[inline(always)]
    #[must_use]
    pub fn tx_start(&mut self) -> TX_START_W<INT_ENA_SPEC> {
        TX_START_W::new(self, 1)
    }
    ///Bit 2 - Set this bit to enable the interrupt of UHCI_RX_HUNG_INT.
    #[inline(always)]
    #[must_use]
    pub fn rx_hung(&mut self) -> RX_HUNG_W<INT_ENA_SPEC> {
        RX_HUNG_W::new(self, 2)
    }
    ///Bit 3 - Set this bit to enable the interrupt of UHCI_TX_HUNG_INT.
    #[inline(always)]
    #[must_use]
    pub fn tx_hung(&mut self) -> TX_HUNG_W<INT_ENA_SPEC> {
        TX_HUNG_W::new(self, 3)
    }
    ///Bit 4 - Set this bit to enable the interrupt of UHCI_SEND_S_REG_Q_INT.
    #[inline(always)]
    #[must_use]
    pub fn send_s_reg_q(&mut self) -> SEND_S_REG_Q_W<INT_ENA_SPEC> {
        SEND_S_REG_Q_W::new(self, 4)
    }
    ///Bit 5 - Set this bit to enable the interrupt of UHCI_SEND_A_REG_Q_INT.
    #[inline(always)]
    #[must_use]
    pub fn send_a_reg_q(&mut self) -> SEND_A_REG_Q_W<INT_ENA_SPEC> {
        SEND_A_REG_Q_W::new(self, 5)
    }
    ///Bit 6 - Set this bit to enable the interrupt of UHCI_OUT_EOF_INT.
    #[inline(always)]
    #[must_use]
    pub fn outlink_eof_err(&mut self) -> OUTLINK_EOF_ERR_W<INT_ENA_SPEC> {
        OUTLINK_EOF_ERR_W::new(self, 6)
    }
    ///Bit 7 - Set this bit to enable the interrupt of UHCI_APP_CTRL0_INT.
    #[inline(always)]
    #[must_use]
    pub fn app_ctrl0(&mut self) -> APP_CTRL0_W<INT_ENA_SPEC> {
        APP_CTRL0_W::new(self, 7)
    }
    ///Bit 8 - Set this bit to enable the interrupt of UHCI_APP_CTRL1_INT.
    #[inline(always)]
    #[must_use]
    pub fn app_ctrl1(&mut self) -> APP_CTRL1_W<INT_ENA_SPEC> {
        APP_CTRL1_W::new(self, 8)
    }
}
/**UHCI Interrupt Enable Register

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
