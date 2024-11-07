#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `RX_START` reader - This is the interrupt raw bit. Triggered when a separator char has been sent."]
pub type RX_START_R = crate::BitReader;
#[doc = "Field `RX_START` writer - This is the interrupt raw bit. Triggered when a separator char has been sent."]
pub type RX_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_START` reader - This is the interrupt raw bit. Triggered when UHCI detects a separator char."]
pub type TX_START_R = crate::BitReader;
#[doc = "Field `TX_START` writer - This is the interrupt raw bit. Triggered when UHCI detects a separator char."]
pub type TX_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_HUNG` reader - This is the interrupt raw bit. Triggered when UHCI takes more time to receive data than configure value."]
pub type RX_HUNG_R = crate::BitReader;
#[doc = "Field `RX_HUNG` writer - This is the interrupt raw bit. Triggered when UHCI takes more time to receive data than configure value."]
pub type RX_HUNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_HUNG` reader - This is the interrupt raw bit. Triggered when UHCI takes more time to read data from RAM than the configured value."]
pub type TX_HUNG_R = crate::BitReader;
#[doc = "Field `TX_HUNG` writer - This is the interrupt raw bit. Triggered when UHCI takes more time to read data from RAM than the configured value."]
pub type TX_HUNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_S_REG_Q` reader - This is the interrupt raw bit. Triggered when UHCI has sent out a short packet using single_send registers."]
pub type SEND_S_REG_Q_R = crate::BitReader;
#[doc = "Field `SEND_S_REG_Q` writer - This is the interrupt raw bit. Triggered when UHCI has sent out a short packet using single_send registers."]
pub type SEND_S_REG_Q_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_A_REG_Q` reader - This is the interrupt raw bit. Triggered when UHCI has sent out a short packet using always_send registers."]
pub type SEND_A_REG_Q_R = crate::BitReader;
#[doc = "Field `SEND_A_REG_Q` writer - This is the interrupt raw bit. Triggered when UHCI has sent out a short packet using always_send registers."]
pub type SEND_A_REG_Q_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EOF` reader - This is the interrupt raw bit. Triggered when there are some errors in EOF in the transmit data."]
pub type OUT_EOF_R = crate::BitReader;
#[doc = "Field `OUT_EOF` writer - This is the interrupt raw bit. Triggered when there are some errors in EOF in the transmit data."]
pub type OUT_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CTRL0` reader - This is the interrupt raw bit. Triggered when set UHCI_APP_CTRL0_IN_SET."]
pub type APP_CTRL0_R = crate::BitReader;
#[doc = "Field `APP_CTRL0` writer - This is the interrupt raw bit. Triggered when set UHCI_APP_CTRL0_IN_SET."]
pub type APP_CTRL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP_CTRL1` reader - This is the interrupt raw bit. Triggered when set UHCI_APP_CTRL1_IN_SET."]
pub type APP_CTRL1_R = crate::BitReader;
#[doc = "Field `APP_CTRL1` writer - This is the interrupt raw bit. Triggered when set UHCI_APP_CTRL1_IN_SET."]
pub type APP_CTRL1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This is the interrupt raw bit. Triggered when a separator char has been sent."]
    #[inline(always)]
    pub fn rx_start(&self) -> RX_START_R {
        RX_START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the interrupt raw bit. Triggered when UHCI detects a separator char."]
    #[inline(always)]
    pub fn tx_start(&self) -> TX_START_R {
        TX_START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is the interrupt raw bit. Triggered when UHCI takes more time to receive data than configure value."]
    #[inline(always)]
    pub fn rx_hung(&self) -> RX_HUNG_R {
        RX_HUNG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This is the interrupt raw bit. Triggered when UHCI takes more time to read data from RAM than the configured value."]
    #[inline(always)]
    pub fn tx_hung(&self) -> TX_HUNG_R {
        TX_HUNG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the interrupt raw bit. Triggered when UHCI has sent out a short packet using single_send registers."]
    #[inline(always)]
    pub fn send_s_reg_q(&self) -> SEND_S_REG_Q_R {
        SEND_S_REG_Q_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This is the interrupt raw bit. Triggered when UHCI has sent out a short packet using always_send registers."]
    #[inline(always)]
    pub fn send_a_reg_q(&self) -> SEND_A_REG_Q_R {
        SEND_A_REG_Q_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the interrupt raw bit. Triggered when there are some errors in EOF in the transmit data."]
    #[inline(always)]
    pub fn out_eof(&self) -> OUT_EOF_R {
        OUT_EOF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the interrupt raw bit. Triggered when set UHCI_APP_CTRL0_IN_SET."]
    #[inline(always)]
    pub fn app_ctrl0(&self) -> APP_CTRL0_R {
        APP_CTRL0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This is the interrupt raw bit. Triggered when set UHCI_APP_CTRL1_IN_SET."]
    #[inline(always)]
    pub fn app_ctrl1(&self) -> APP_CTRL1_R {
        APP_CTRL1_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("rx_start", &self.rx_start())
            .field("tx_start", &self.tx_start())
            .field("rx_hung", &self.rx_hung())
            .field("tx_hung", &self.tx_hung())
            .field("send_s_reg_q", &self.send_s_reg_q())
            .field("send_a_reg_q", &self.send_a_reg_q())
            .field("out_eof", &self.out_eof())
            .field("app_ctrl0", &self.app_ctrl0())
            .field("app_ctrl1", &self.app_ctrl1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This is the interrupt raw bit. Triggered when a separator char has been sent."]
    #[inline(always)]
    pub fn rx_start(&mut self) -> RX_START_W<INT_RAW_SPEC> {
        RX_START_W::new(self, 0)
    }
    #[doc = "Bit 1 - This is the interrupt raw bit. Triggered when UHCI detects a separator char."]
    #[inline(always)]
    pub fn tx_start(&mut self) -> TX_START_W<INT_RAW_SPEC> {
        TX_START_W::new(self, 1)
    }
    #[doc = "Bit 2 - This is the interrupt raw bit. Triggered when UHCI takes more time to receive data than configure value."]
    #[inline(always)]
    pub fn rx_hung(&mut self) -> RX_HUNG_W<INT_RAW_SPEC> {
        RX_HUNG_W::new(self, 2)
    }
    #[doc = "Bit 3 - This is the interrupt raw bit. Triggered when UHCI takes more time to read data from RAM than the configured value."]
    #[inline(always)]
    pub fn tx_hung(&mut self) -> TX_HUNG_W<INT_RAW_SPEC> {
        TX_HUNG_W::new(self, 3)
    }
    #[doc = "Bit 4 - This is the interrupt raw bit. Triggered when UHCI has sent out a short packet using single_send registers."]
    #[inline(always)]
    pub fn send_s_reg_q(&mut self) -> SEND_S_REG_Q_W<INT_RAW_SPEC> {
        SEND_S_REG_Q_W::new(self, 4)
    }
    #[doc = "Bit 5 - This is the interrupt raw bit. Triggered when UHCI has sent out a short packet using always_send registers."]
    #[inline(always)]
    pub fn send_a_reg_q(&mut self) -> SEND_A_REG_Q_W<INT_RAW_SPEC> {
        SEND_A_REG_Q_W::new(self, 5)
    }
    #[doc = "Bit 6 - This is the interrupt raw bit. Triggered when there are some errors in EOF in the transmit data."]
    #[inline(always)]
    pub fn out_eof(&mut self) -> OUT_EOF_W<INT_RAW_SPEC> {
        OUT_EOF_W::new(self, 6)
    }
    #[doc = "Bit 7 - This is the interrupt raw bit. Triggered when set UHCI_APP_CTRL0_IN_SET."]
    #[inline(always)]
    pub fn app_ctrl0(&mut self) -> APP_CTRL0_W<INT_RAW_SPEC> {
        APP_CTRL0_W::new(self, 7)
    }
    #[doc = "Bit 8 - This is the interrupt raw bit. Triggered when set UHCI_APP_CTRL1_IN_SET."]
    #[inline(always)]
    pub fn app_ctrl1(&mut self) -> APP_CTRL1_W<INT_RAW_SPEC> {
        APP_CTRL1_W::new(self, 8)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
