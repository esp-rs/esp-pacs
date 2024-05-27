///Register `TXRX_STATUS` reader
pub type R = crate::R<TXRX_STATUS_SPEC>;
///Register `TXRX_STATUS` writer
pub type W = crate::W<TXRX_STATUS_SPEC>;
///Field `TXRX_STATE` reader -
pub type TXRX_STATE_R = crate::FieldReader;
///Field `TXRX_STATE` writer -
pub type TXRX_STATE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `TX_PROC` reader -
pub type TX_PROC_R = crate::BitReader;
///Field `TX_PROC` writer -
pub type TX_PROC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_PROC` reader -
pub type RX_PROC_R = crate::BitReader;
///Field `RX_PROC` writer -
pub type RX_PROC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ED_PROC` reader -
pub type ED_PROC_R = crate::BitReader;
///Field `ED_PROC` writer -
pub type ED_PROC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ED_TRIGGER_TX_PROC` reader -
pub type ED_TRIGGER_TX_PROC_R = crate::BitReader;
///Field `ED_TRIGGER_TX_PROC` writer -
pub type ED_TRIGGER_TX_PROC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RF_CTRL_STATE` reader -
pub type RF_CTRL_STATE_R = crate::FieldReader;
///Field `RF_CTRL_STATE` writer -
pub type RF_CTRL_STATE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3
    #[inline(always)]
    pub fn txrx_state(&self) -> TXRX_STATE_R {
        TXRX_STATE_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 8
    #[inline(always)]
    pub fn tx_proc(&self) -> TX_PROC_R {
        TX_PROC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9
    #[inline(always)]
    pub fn rx_proc(&self) -> RX_PROC_R {
        RX_PROC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10
    #[inline(always)]
    pub fn ed_proc(&self) -> ED_PROC_R {
        ED_PROC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11
    #[inline(always)]
    pub fn ed_trigger_tx_proc(&self) -> ED_TRIGGER_TX_PROC_R {
        ED_TRIGGER_TX_PROC_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 16:19
    #[inline(always)]
    pub fn rf_ctrl_state(&self) -> RF_CTRL_STATE_R {
        RF_CTRL_STATE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TXRX_STATUS")
            .field("txrx_state", &self.txrx_state())
            .field("tx_proc", &self.tx_proc())
            .field("rx_proc", &self.rx_proc())
            .field("ed_proc", &self.ed_proc())
            .field("ed_trigger_tx_proc", &self.ed_trigger_tx_proc())
            .field("rf_ctrl_state", &self.rf_ctrl_state())
            .finish()
    }
}
impl W {
    ///Bits 0:3
    #[inline(always)]
    #[must_use]
    pub fn txrx_state(&mut self) -> TXRX_STATE_W<TXRX_STATUS_SPEC> {
        TXRX_STATE_W::new(self, 0)
    }
    ///Bit 8
    #[inline(always)]
    #[must_use]
    pub fn tx_proc(&mut self) -> TX_PROC_W<TXRX_STATUS_SPEC> {
        TX_PROC_W::new(self, 8)
    }
    ///Bit 9
    #[inline(always)]
    #[must_use]
    pub fn rx_proc(&mut self) -> RX_PROC_W<TXRX_STATUS_SPEC> {
        RX_PROC_W::new(self, 9)
    }
    ///Bit 10
    #[inline(always)]
    #[must_use]
    pub fn ed_proc(&mut self) -> ED_PROC_W<TXRX_STATUS_SPEC> {
        ED_PROC_W::new(self, 10)
    }
    ///Bit 11
    #[inline(always)]
    #[must_use]
    pub fn ed_trigger_tx_proc(&mut self) -> ED_TRIGGER_TX_PROC_W<TXRX_STATUS_SPEC> {
        ED_TRIGGER_TX_PROC_W::new(self, 11)
    }
    ///Bits 16:19
    #[inline(always)]
    #[must_use]
    pub fn rf_ctrl_state(&mut self) -> RF_CTRL_STATE_W<TXRX_STATUS_SPEC> {
        RF_CTRL_STATE_W::new(self, 16)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`txrx_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txrx_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TXRX_STATUS_SPEC;
impl crate::RegisterSpec for TXRX_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`txrx_status::R`](R) reader structure
impl crate::Readable for TXRX_STATUS_SPEC {}
///`write(|w| ..)` method takes [`txrx_status::W`](W) writer structure
impl crate::Writable for TXRX_STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TXRX_STATUS to value 0
impl crate::Resettable for TXRX_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
