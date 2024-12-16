#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `RX_START` reader - This is the interrupt enable bit for UHCI_RX_START_INT interrupt."]
pub type RX_START_R = crate::BitReader;
#[doc = "Field `RX_START` writer - This is the interrupt enable bit for UHCI_RX_START_INT interrupt."]
pub type RX_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_START` reader - This is the interrupt enable bit for UHCI_TX_START_INT interrupt."]
pub type TX_START_R = crate::BitReader;
#[doc = "Field `TX_START` writer - This is the interrupt enable bit for UHCI_TX_START_INT interrupt."]
pub type TX_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_HUNG` reader - This is the interrupt enable bit for UHCI_RX_HUNG_INT interrupt."]
pub type RX_HUNG_R = crate::BitReader;
#[doc = "Field `RX_HUNG` writer - This is the interrupt enable bit for UHCI_RX_HUNG_INT interrupt."]
pub type RX_HUNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_HUNG` reader - This is the interrupt enable bit for UHCI_TX_HUNG_INT interrupt."]
pub type TX_HUNG_R = crate::BitReader;
#[doc = "Field `TX_HUNG` writer - This is the interrupt enable bit for UHCI_TX_HUNG_INT interrupt."]
pub type TX_HUNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_S_REG_Q` reader - This is the interrupt enable bit for UHCI_SEND_S_REQ_Q_INT interrupt."]
pub type SEND_S_REG_Q_R = crate::BitReader;
#[doc = "Field `SEND_S_REG_Q` writer - This is the interrupt enable bit for UHCI_SEND_S_REQ_Q_INT interrupt."]
pub type SEND_S_REG_Q_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_A_REG_Q` reader - This is the interrupt enable bit for UHCI_SEND_A_REQ_Q_INT interrupt."]
pub type SEND_A_REG_Q_R = crate::BitReader;
#[doc = "Field `SEND_A_REG_Q` writer - This is the interrupt enable bit for UHCI_SEND_A_REQ_Q_INT interrupt."]
pub type SEND_A_REG_Q_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_EOF_ERR` reader - This is the interrupt enable bit for UHCI_OUTLINK_EOF_ERR_INT interrupt."]
pub type OUTLINK_EOF_ERR_R = crate::BitReader;
#[doc = "Field `OUTLINK_EOF_ERR` writer - This is the interrupt enable bit for UHCI_OUTLINK_EOF_ERR_INT interrupt."]
pub type OUTLINK_EOF_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CTRL0` reader - This is the interrupt enable bit for UHCI_APP_CTRL0_INT interrupt."]
pub type APP_CTRL0_R = crate::BitReader;
#[doc = "Field `APP_CTRL0` writer - This is the interrupt enable bit for UHCI_APP_CTRL0_INT interrupt."]
pub type APP_CTRL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CTRL1` reader - This is the interrupt enable bit for UHCI_APP_CTRL1_INT interrupt."]
pub type APP_CTRL1_R = crate::BitReader;
#[doc = "Field `APP_CTRL1` writer - This is the interrupt enable bit for UHCI_APP_CTRL1_INT interrupt."]
pub type APP_CTRL1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This is the interrupt enable bit for UHCI_RX_START_INT interrupt."]
    #[inline(always)]
    pub fn rx_start(&self) -> RX_START_R {
        RX_START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the interrupt enable bit for UHCI_TX_START_INT interrupt."]
    #[inline(always)]
    pub fn tx_start(&self) -> TX_START_R {
        TX_START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is the interrupt enable bit for UHCI_RX_HUNG_INT interrupt."]
    #[inline(always)]
    pub fn rx_hung(&self) -> RX_HUNG_R {
        RX_HUNG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This is the interrupt enable bit for UHCI_TX_HUNG_INT interrupt."]
    #[inline(always)]
    pub fn tx_hung(&self) -> TX_HUNG_R {
        TX_HUNG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the interrupt enable bit for UHCI_SEND_S_REQ_Q_INT interrupt."]
    #[inline(always)]
    pub fn send_s_reg_q(&self) -> SEND_S_REG_Q_R {
        SEND_S_REG_Q_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This is the interrupt enable bit for UHCI_SEND_A_REQ_Q_INT interrupt."]
    #[inline(always)]
    pub fn send_a_reg_q(&self) -> SEND_A_REG_Q_R {
        SEND_A_REG_Q_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the interrupt enable bit for UHCI_OUTLINK_EOF_ERR_INT interrupt."]
    #[inline(always)]
    pub fn outlink_eof_err(&self) -> OUTLINK_EOF_ERR_R {
        OUTLINK_EOF_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the interrupt enable bit for UHCI_APP_CTRL0_INT interrupt."]
    #[inline(always)]
    pub fn app_ctrl0(&self) -> APP_CTRL0_R {
        APP_CTRL0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This is the interrupt enable bit for UHCI_APP_CTRL1_INT interrupt."]
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
    #[doc = "Bit 0 - This is the interrupt enable bit for UHCI_RX_START_INT interrupt."]
    #[inline(always)]
    pub fn rx_start(&mut self) -> RX_START_W<INT_ENA_SPEC> {
        RX_START_W::new(self, 0)
    }
    #[doc = "Bit 1 - This is the interrupt enable bit for UHCI_TX_START_INT interrupt."]
    #[inline(always)]
    pub fn tx_start(&mut self) -> TX_START_W<INT_ENA_SPEC> {
        TX_START_W::new(self, 1)
    }
    #[doc = "Bit 2 - This is the interrupt enable bit for UHCI_RX_HUNG_INT interrupt."]
    #[inline(always)]
    pub fn rx_hung(&mut self) -> RX_HUNG_W<INT_ENA_SPEC> {
        RX_HUNG_W::new(self, 2)
    }
    #[doc = "Bit 3 - This is the interrupt enable bit for UHCI_TX_HUNG_INT interrupt."]
    #[inline(always)]
    pub fn tx_hung(&mut self) -> TX_HUNG_W<INT_ENA_SPEC> {
        TX_HUNG_W::new(self, 3)
    }
    #[doc = "Bit 4 - This is the interrupt enable bit for UHCI_SEND_S_REQ_Q_INT interrupt."]
    #[inline(always)]
    pub fn send_s_reg_q(&mut self) -> SEND_S_REG_Q_W<INT_ENA_SPEC> {
        SEND_S_REG_Q_W::new(self, 4)
    }
    #[doc = "Bit 5 - This is the interrupt enable bit for UHCI_SEND_A_REQ_Q_INT interrupt."]
    #[inline(always)]
    pub fn send_a_reg_q(&mut self) -> SEND_A_REG_Q_W<INT_ENA_SPEC> {
        SEND_A_REG_Q_W::new(self, 5)
    }
    #[doc = "Bit 6 - This is the interrupt enable bit for UHCI_OUTLINK_EOF_ERR_INT interrupt."]
    #[inline(always)]
    pub fn outlink_eof_err(&mut self) -> OUTLINK_EOF_ERR_W<INT_ENA_SPEC> {
        OUTLINK_EOF_ERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - This is the interrupt enable bit for UHCI_APP_CTRL0_INT interrupt."]
    #[inline(always)]
    pub fn app_ctrl0(&mut self) -> APP_CTRL0_W<INT_ENA_SPEC> {
        APP_CTRL0_W::new(self, 7)
    }
    #[doc = "Bit 8 - This is the interrupt enable bit for UHCI_APP_CTRL1_INT interrupt."]
    #[inline(always)]
    pub fn app_ctrl1(&mut self) -> APP_CTRL1_W<INT_ENA_SPEC> {
        APP_CTRL1_W::new(self, 8)
    }
}
#[doc = "Interrupt enable bits\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
