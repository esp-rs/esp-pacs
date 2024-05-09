#[doc = "Register `TXRX_STATUS` reader"]
pub type R = crate::R<TXRX_STATUS_SPEC>;
#[doc = "Register `TXRX_STATUS` writer"]
pub type W = crate::W<TXRX_STATUS_SPEC>;
#[doc = "Field `TXRX_STATE` reader - "]
pub type TXRX_STATE_R = crate::FieldReader;
#[doc = "Field `TXRX_STATE` writer - "]
pub type TXRX_STATE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TX_PROC` reader - "]
pub type TX_PROC_R = crate::BitReader;
#[doc = "Field `TX_PROC` writer - "]
pub type TX_PROC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PROC` reader - "]
pub type RX_PROC_R = crate::BitReader;
#[doc = "Field `RX_PROC` writer - "]
pub type RX_PROC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ED_PROC` reader - "]
pub type ED_PROC_R = crate::BitReader;
#[doc = "Field `ED_PROC` writer - "]
pub type ED_PROC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ED_TRIGGER_TX_PROC` reader - "]
pub type ED_TRIGGER_TX_PROC_R = crate::BitReader;
#[doc = "Field `ED_TRIGGER_TX_PROC` writer - "]
pub type ED_TRIGGER_TX_PROC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF_CTRL_STATE` reader - "]
pub type RF_CTRL_STATE_R = crate::FieldReader;
#[doc = "Field `RF_CTRL_STATE` writer - "]
pub type RF_CTRL_STATE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn txrx_state(&self) -> TXRX_STATE_R {
        TXRX_STATE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tx_proc(&self) -> TX_PROC_R {
        TX_PROC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rx_proc(&self) -> RX_PROC_R {
        RX_PROC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ed_proc(&self) -> ED_PROC_R {
        ED_PROC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ed_trigger_tx_proc(&self) -> ED_TRIGGER_TX_PROC_R {
        ED_TRIGGER_TX_PROC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn rf_ctrl_state(&self) -> RF_CTRL_STATE_R {
        RF_CTRL_STATE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXRX_STATUS")
            .field("txrx_state", &format_args!("{}", self.txrx_state().bits()))
            .field("tx_proc", &format_args!("{}", self.tx_proc().bit()))
            .field("rx_proc", &format_args!("{}", self.rx_proc().bit()))
            .field("ed_proc", &format_args!("{}", self.ed_proc().bit()))
            .field(
                "ed_trigger_tx_proc",
                &format_args!("{}", self.ed_trigger_tx_proc().bit()),
            )
            .field(
                "rf_ctrl_state",
                &format_args!("{}", self.rf_ctrl_state().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TXRX_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn txrx_state(&mut self) -> TXRX_STATE_W<TXRX_STATUS_SPEC> {
        TXRX_STATE_W::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn tx_proc(&mut self) -> TX_PROC_W<TXRX_STATUS_SPEC> {
        TX_PROC_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn rx_proc(&mut self) -> RX_PROC_W<TXRX_STATUS_SPEC> {
        RX_PROC_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn ed_proc(&mut self) -> ED_PROC_W<TXRX_STATUS_SPEC> {
        ED_PROC_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn ed_trigger_tx_proc(&mut self) -> ED_TRIGGER_TX_PROC_W<TXRX_STATUS_SPEC> {
        ED_TRIGGER_TX_PROC_W::new(self, 11)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn rf_ctrl_state(&mut self) -> RF_CTRL_STATE_W<TXRX_STATUS_SPEC> {
        RF_CTRL_STATE_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txrx_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txrx_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXRX_STATUS_SPEC;
impl crate::RegisterSpec for TXRX_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txrx_status::R`](R) reader structure"]
impl crate::Readable for TXRX_STATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txrx_status::W`](W) writer structure"]
impl crate::Writable for TXRX_STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXRX_STATUS to value 0"]
impl crate::Resettable for TXRX_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
