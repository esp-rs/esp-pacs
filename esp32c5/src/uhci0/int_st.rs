#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `RX_START_INT_ST` reader - The masked interrupt status of UHCI_RX_START_INT."]
pub type RX_START_INT_ST_R = crate::BitReader;
#[doc = "Field `TX_START_INT_ST` reader - The masked interrupt status of UHCI_TX_START_INT."]
pub type TX_START_INT_ST_R = crate::BitReader;
#[doc = "Field `RX_HUNG_INT_ST` reader - The masked interrupt status of UHCI_RX_HUNG_INT."]
pub type RX_HUNG_INT_ST_R = crate::BitReader;
#[doc = "Field `TX_HUNG_INT_ST` reader - The masked interrupt status of UHCI_TX_HUNG_INT."]
pub type TX_HUNG_INT_ST_R = crate::BitReader;
#[doc = "Field `SEND_S_REG_Q_INT_ST` reader - The masked interrupt status of UHCI_SEND_S_REG_Q_INT."]
pub type SEND_S_REG_Q_INT_ST_R = crate::BitReader;
#[doc = "Field `SEND_A_REG_Q_INT_ST` reader - The masked interrupt status of UHCI_SEND_A_REG_Q_INT."]
pub type SEND_A_REG_Q_INT_ST_R = crate::BitReader;
#[doc = "Field `OUTLINK_EOF_ERR_INT_ST` reader - The masked interrupt status of UHCI_OUTLINK_EOF_ERR_INT."]
pub type OUTLINK_EOF_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `APP_CTRL0_INT_ST` reader - The masked interrupt status of UHCI_APP_CTRL0_INT."]
pub type APP_CTRL0_INT_ST_R = crate::BitReader;
#[doc = "Field `APP_CTRL1_INT_ST` reader - The masked interrupt status of UHCI_APP_CTRL1_INT."]
pub type APP_CTRL1_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The masked interrupt status of UHCI_RX_START_INT."]
    #[inline(always)]
    pub fn rx_start_int_st(&self) -> RX_START_INT_ST_R {
        RX_START_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The masked interrupt status of UHCI_TX_START_INT."]
    #[inline(always)]
    pub fn tx_start_int_st(&self) -> TX_START_INT_ST_R {
        TX_START_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The masked interrupt status of UHCI_RX_HUNG_INT."]
    #[inline(always)]
    pub fn rx_hung_int_st(&self) -> RX_HUNG_INT_ST_R {
        RX_HUNG_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status of UHCI_TX_HUNG_INT."]
    #[inline(always)]
    pub fn tx_hung_int_st(&self) -> TX_HUNG_INT_ST_R {
        TX_HUNG_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The masked interrupt status of UHCI_SEND_S_REG_Q_INT."]
    #[inline(always)]
    pub fn send_s_reg_q_int_st(&self) -> SEND_S_REG_Q_INT_ST_R {
        SEND_S_REG_Q_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The masked interrupt status of UHCI_SEND_A_REG_Q_INT."]
    #[inline(always)]
    pub fn send_a_reg_q_int_st(&self) -> SEND_A_REG_Q_INT_ST_R {
        SEND_A_REG_Q_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The masked interrupt status of UHCI_OUTLINK_EOF_ERR_INT."]
    #[inline(always)]
    pub fn outlink_eof_err_int_st(&self) -> OUTLINK_EOF_ERR_INT_ST_R {
        OUTLINK_EOF_ERR_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The masked interrupt status of UHCI_APP_CTRL0_INT."]
    #[inline(always)]
    pub fn app_ctrl0_int_st(&self) -> APP_CTRL0_INT_ST_R {
        APP_CTRL0_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The masked interrupt status of UHCI_APP_CTRL1_INT."]
    #[inline(always)]
    pub fn app_ctrl1_int_st(&self) -> APP_CTRL1_INT_ST_R {
        APP_CTRL1_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("rx_start_int_st", &self.rx_start_int_st())
            .field("tx_start_int_st", &self.tx_start_int_st())
            .field("rx_hung_int_st", &self.rx_hung_int_st())
            .field("tx_hung_int_st", &self.tx_hung_int_st())
            .field("send_s_reg_q_int_st", &self.send_s_reg_q_int_st())
            .field("send_a_reg_q_int_st", &self.send_a_reg_q_int_st())
            .field("outlink_eof_err_int_st", &self.outlink_eof_err_int_st())
            .field("app_ctrl0_int_st", &self.app_ctrl0_int_st())
            .field("app_ctrl1_int_st", &self.app_ctrl1_int_st())
            .finish()
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {}
