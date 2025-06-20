#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `RX_START_INT_CLR` writer - Write 1 to clear UHCI_RX_START_INT."]
pub type RX_START_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_START_INT_CLR` writer - Write 1 to clear UHCI_TX_START_INT."]
pub type TX_START_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_HUNG_INT_CLR` writer - Write 1 to clear UHCI_RX_HUNG_INT."]
pub type RX_HUNG_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_HUNG_INT_CLR` writer - Write 1 to clear UHCI_TX_HUNG_INT."]
pub type TX_HUNG_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_S_REG_Q_INT_CLR` writer - Write 1 to clear UHCI_SEND_S_REG_Q_INT."]
pub type SEND_S_REG_Q_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_A_REG_Q_INT_CLR` writer - Write 1 to clear UHCI_SEND_A_REG_Q_INT."]
pub type SEND_A_REG_Q_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_EOF_ERR_INT_CLR` writer - Write 1 to clear UHCI_OUTLINK_EOF_ERR_INT."]
pub type OUTLINK_EOF_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CTRL0_INT_CLR` writer - Write 1 to clear UHCI_APP_CTRL0_INT."]
pub type APP_CTRL0_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CTRL1_INT_CLR` writer - Write 1 to clear UHCI_APP_CTRL1_INT."]
pub type APP_CTRL1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to clear UHCI_RX_START_INT."]
    #[inline(always)]
    pub fn rx_start_int_clr(&mut self) -> RX_START_INT_CLR_W<INT_CLR_SPEC> {
        RX_START_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to clear UHCI_TX_START_INT."]
    #[inline(always)]
    pub fn tx_start_int_clr(&mut self) -> TX_START_INT_CLR_W<INT_CLR_SPEC> {
        TX_START_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to clear UHCI_RX_HUNG_INT."]
    #[inline(always)]
    pub fn rx_hung_int_clr(&mut self) -> RX_HUNG_INT_CLR_W<INT_CLR_SPEC> {
        RX_HUNG_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to clear UHCI_TX_HUNG_INT."]
    #[inline(always)]
    pub fn tx_hung_int_clr(&mut self) -> TX_HUNG_INT_CLR_W<INT_CLR_SPEC> {
        TX_HUNG_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to clear UHCI_SEND_S_REG_Q_INT."]
    #[inline(always)]
    pub fn send_s_reg_q_int_clr(&mut self) -> SEND_S_REG_Q_INT_CLR_W<INT_CLR_SPEC> {
        SEND_S_REG_Q_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to clear UHCI_SEND_A_REG_Q_INT."]
    #[inline(always)]
    pub fn send_a_reg_q_int_clr(&mut self) -> SEND_A_REG_Q_INT_CLR_W<INT_CLR_SPEC> {
        SEND_A_REG_Q_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Write 1 to clear UHCI_OUTLINK_EOF_ERR_INT."]
    #[inline(always)]
    pub fn outlink_eof_err_int_clr(&mut self) -> OUTLINK_EOF_ERR_INT_CLR_W<INT_CLR_SPEC> {
        OUTLINK_EOF_ERR_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Write 1 to clear UHCI_APP_CTRL0_INT."]
    #[inline(always)]
    pub fn app_ctrl0_int_clr(&mut self) -> APP_CTRL0_INT_CLR_W<INT_CLR_SPEC> {
        APP_CTRL0_INT_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Write 1 to clear UHCI_APP_CTRL1_INT."]
    #[inline(always)]
    pub fn app_ctrl1_int_clr(&mut self) -> APP_CTRL1_INT_CLR_W<INT_CLR_SPEC> {
        APP_CTRL1_INT_CLR_W::new(self, 8)
    }
}
#[doc = "Interrupt clear bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
