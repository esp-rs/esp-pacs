#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `RXFIFO_FULL_INT_RAW` reader - The raw interrupt status of LP_UART_RXFIFO_FULL_INT."]
pub type RXFIFO_FULL_INT_RAW_R = crate::BitReader;
#[doc = "Field `RXFIFO_FULL_INT_RAW` writer - The raw interrupt status of LP_UART_RXFIFO_FULL_INT."]
pub type RXFIFO_FULL_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFO_EMPTY_INT_RAW` reader - The raw interrupt status of LP_UART_TXFIFO_EMPTY_INT."]
pub type TXFIFO_EMPTY_INT_RAW_R = crate::BitReader;
#[doc = "Field `TXFIFO_EMPTY_INT_RAW` writer - The raw interrupt status of LP_UART_TXFIFO_EMPTY_INT."]
pub type TXFIFO_EMPTY_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARITY_ERR_INT_RAW` reader - The raw interrupt status of LP_UART_PARITY_ERR_INT."]
pub type PARITY_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `PARITY_ERR_INT_RAW` writer - The raw interrupt status of LP_UART_PARITY_ERR_INT."]
pub type PARITY_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRM_ERR_INT_RAW` reader - The raw interrupt status of LP_UART_FRM_ERR_INT."]
pub type FRM_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `FRM_ERR_INT_RAW` writer - The raw interrupt status of LP_UART_FRM_ERR_INT."]
pub type FRM_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFO_OVF_INT_RAW` reader - The raw interrupt status of LP_UART_RXFIFO_OVF_INT."]
pub type RXFIFO_OVF_INT_RAW_R = crate::BitReader;
#[doc = "Field `RXFIFO_OVF_INT_RAW` writer - The raw interrupt status of LP_UART_RXFIFO_OVF_INT."]
pub type RXFIFO_OVF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSR_CHG_INT_RAW` reader - The raw interrupt status of LP_UART_DSR_CHG_INT."]
pub type DSR_CHG_INT_RAW_R = crate::BitReader;
#[doc = "Field `DSR_CHG_INT_RAW` writer - The raw interrupt status of LP_UART_DSR_CHG_INT."]
pub type DSR_CHG_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS_CHG_INT_RAW` reader - The raw interrupt status of LP_UART_CTS_CHG_INT."]
pub type CTS_CHG_INT_RAW_R = crate::BitReader;
#[doc = "Field `CTS_CHG_INT_RAW` writer - The raw interrupt status of LP_UART_CTS_CHG_INT."]
pub type CTS_CHG_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRK_DET_INT_RAW` reader - The raw interrupt status of LP_UART_BRK_DET_INT."]
pub type BRK_DET_INT_RAW_R = crate::BitReader;
#[doc = "Field `BRK_DET_INT_RAW` writer - The raw interrupt status of LP_UART_BRK_DET_INT."]
pub type BRK_DET_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFO_TOUT_INT_RAW` reader - The raw interrupt status of LP_UART_RXFIFO_TOUT_INT."]
pub type RXFIFO_TOUT_INT_RAW_R = crate::BitReader;
#[doc = "Field `RXFIFO_TOUT_INT_RAW` writer - The raw interrupt status of LP_UART_RXFIFO_TOUT_INT."]
pub type RXFIFO_TOUT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_XON_INT_RAW` reader - The raw interrupt status of LP_UART_SW_XON_INT."]
pub type SW_XON_INT_RAW_R = crate::BitReader;
#[doc = "Field `SW_XON_INT_RAW` writer - The raw interrupt status of LP_UART_SW_XON_INT."]
pub type SW_XON_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_XOFF_INT_RAW` reader - LP_UART_SW_XOFF_INT."]
pub type SW_XOFF_INT_RAW_R = crate::BitReader;
#[doc = "Field `SW_XOFF_INT_RAW` writer - LP_UART_SW_XOFF_INT."]
pub type SW_XOFF_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLITCH_DET_INT_RAW` reader - The raw interrupt status of LP_UART_GLITCH_DET_INT."]
pub type GLITCH_DET_INT_RAW_R = crate::BitReader;
#[doc = "Field `GLITCH_DET_INT_RAW` writer - The raw interrupt status of LP_UART_GLITCH_DET_INT."]
pub type GLITCH_DET_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_BRK_DONE_INT_RAW` reader - The raw interrupt status of LP_UART_TX_BRK_DONE_INT."]
pub type TX_BRK_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_BRK_DONE_INT_RAW` writer - The raw interrupt status of LP_UART_TX_BRK_DONE_INT."]
pub type TX_BRK_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_BRK_IDLE_DONE_INT_RAW` reader - The raw interrupt status of LP_UART_TX_BRK_IDLE_DONE_INT."]
pub type TX_BRK_IDLE_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_BRK_IDLE_DONE_INT_RAW` writer - The raw interrupt status of LP_UART_TX_BRK_IDLE_DONE_INT."]
pub type TX_BRK_IDLE_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DONE_INT_RAW` reader - The raw interrupt status of LP_UART_TX_DONE_INT."]
pub type TX_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_DONE_INT_RAW` writer - The raw interrupt status of LP_UART_TX_DONE_INT."]
pub type TX_DONE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AT_CMD_CHAR_DET_INT_RAW` reader - The raw interrupt status of LP_UART_AT_CMD_CHAR_DET_INT."]
pub type AT_CMD_CHAR_DET_INT_RAW_R = crate::BitReader;
#[doc = "Field `AT_CMD_CHAR_DET_INT_RAW` writer - The raw interrupt status of LP_UART_AT_CMD_CHAR_DET_INT."]
pub type AT_CMD_CHAR_DET_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP_INT_RAW` reader - The raw interrupt status of LP_UART_WAKEUP_INT."]
pub type WAKEUP_INT_RAW_R = crate::BitReader;
#[doc = "Field `WAKEUP_INT_RAW` writer - The raw interrupt status of LP_UART_WAKEUP_INT."]
pub type WAKEUP_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The raw interrupt status of LP_UART_RXFIFO_FULL_INT."]
    #[inline(always)]
    pub fn rxfifo_full_int_raw(&self) -> RXFIFO_FULL_INT_RAW_R {
        RXFIFO_FULL_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status of LP_UART_TXFIFO_EMPTY_INT."]
    #[inline(always)]
    pub fn txfifo_empty_int_raw(&self) -> TXFIFO_EMPTY_INT_RAW_R {
        TXFIFO_EMPTY_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status of LP_UART_PARITY_ERR_INT."]
    #[inline(always)]
    pub fn parity_err_int_raw(&self) -> PARITY_ERR_INT_RAW_R {
        PARITY_ERR_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status of LP_UART_FRM_ERR_INT."]
    #[inline(always)]
    pub fn frm_err_int_raw(&self) -> FRM_ERR_INT_RAW_R {
        FRM_ERR_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw interrupt status of LP_UART_RXFIFO_OVF_INT."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_raw(&self) -> RXFIFO_OVF_INT_RAW_R {
        RXFIFO_OVF_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw interrupt status of LP_UART_DSR_CHG_INT."]
    #[inline(always)]
    pub fn dsr_chg_int_raw(&self) -> DSR_CHG_INT_RAW_R {
        DSR_CHG_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw interrupt status of LP_UART_CTS_CHG_INT."]
    #[inline(always)]
    pub fn cts_chg_int_raw(&self) -> CTS_CHG_INT_RAW_R {
        CTS_CHG_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw interrupt status of LP_UART_BRK_DET_INT."]
    #[inline(always)]
    pub fn brk_det_int_raw(&self) -> BRK_DET_INT_RAW_R {
        BRK_DET_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw interrupt status of LP_UART_RXFIFO_TOUT_INT."]
    #[inline(always)]
    pub fn rxfifo_tout_int_raw(&self) -> RXFIFO_TOUT_INT_RAW_R {
        RXFIFO_TOUT_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The raw interrupt status of LP_UART_SW_XON_INT."]
    #[inline(always)]
    pub fn sw_xon_int_raw(&self) -> SW_XON_INT_RAW_R {
        SW_XON_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LP_UART_SW_XOFF_INT."]
    #[inline(always)]
    pub fn sw_xoff_int_raw(&self) -> SW_XOFF_INT_RAW_R {
        SW_XOFF_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The raw interrupt status of LP_UART_GLITCH_DET_INT."]
    #[inline(always)]
    pub fn glitch_det_int_raw(&self) -> GLITCH_DET_INT_RAW_R {
        GLITCH_DET_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The raw interrupt status of LP_UART_TX_BRK_DONE_INT."]
    #[inline(always)]
    pub fn tx_brk_done_int_raw(&self) -> TX_BRK_DONE_INT_RAW_R {
        TX_BRK_DONE_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The raw interrupt status of LP_UART_TX_BRK_IDLE_DONE_INT."]
    #[inline(always)]
    pub fn tx_brk_idle_done_int_raw(&self) -> TX_BRK_IDLE_DONE_INT_RAW_R {
        TX_BRK_IDLE_DONE_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The raw interrupt status of LP_UART_TX_DONE_INT."]
    #[inline(always)]
    pub fn tx_done_int_raw(&self) -> TX_DONE_INT_RAW_R {
        TX_DONE_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - The raw interrupt status of LP_UART_AT_CMD_CHAR_DET_INT."]
    #[inline(always)]
    pub fn at_cmd_char_det_int_raw(&self) -> AT_CMD_CHAR_DET_INT_RAW_R {
        AT_CMD_CHAR_DET_INT_RAW_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The raw interrupt status of LP_UART_WAKEUP_INT."]
    #[inline(always)]
    pub fn wakeup_int_raw(&self) -> WAKEUP_INT_RAW_R {
        WAKEUP_INT_RAW_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("rxfifo_full_int_raw", &self.rxfifo_full_int_raw())
            .field("txfifo_empty_int_raw", &self.txfifo_empty_int_raw())
            .field("parity_err_int_raw", &self.parity_err_int_raw())
            .field("frm_err_int_raw", &self.frm_err_int_raw())
            .field("rxfifo_ovf_int_raw", &self.rxfifo_ovf_int_raw())
            .field("dsr_chg_int_raw", &self.dsr_chg_int_raw())
            .field("cts_chg_int_raw", &self.cts_chg_int_raw())
            .field("brk_det_int_raw", &self.brk_det_int_raw())
            .field("rxfifo_tout_int_raw", &self.rxfifo_tout_int_raw())
            .field("sw_xon_int_raw", &self.sw_xon_int_raw())
            .field("sw_xoff_int_raw", &self.sw_xoff_int_raw())
            .field("glitch_det_int_raw", &self.glitch_det_int_raw())
            .field("tx_brk_done_int_raw", &self.tx_brk_done_int_raw())
            .field("tx_brk_idle_done_int_raw", &self.tx_brk_idle_done_int_raw())
            .field("tx_done_int_raw", &self.tx_done_int_raw())
            .field("at_cmd_char_det_int_raw", &self.at_cmd_char_det_int_raw())
            .field("wakeup_int_raw", &self.wakeup_int_raw())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The raw interrupt status of LP_UART_RXFIFO_FULL_INT."]
    #[inline(always)]
    pub fn rxfifo_full_int_raw(&mut self) -> RXFIFO_FULL_INT_RAW_W<INT_RAW_SPEC> {
        RXFIFO_FULL_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - The raw interrupt status of LP_UART_TXFIFO_EMPTY_INT."]
    #[inline(always)]
    pub fn txfifo_empty_int_raw(&mut self) -> TXFIFO_EMPTY_INT_RAW_W<INT_RAW_SPEC> {
        TXFIFO_EMPTY_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 2 - The raw interrupt status of LP_UART_PARITY_ERR_INT."]
    #[inline(always)]
    pub fn parity_err_int_raw(&mut self) -> PARITY_ERR_INT_RAW_W<INT_RAW_SPEC> {
        PARITY_ERR_INT_RAW_W::new(self, 2)
    }
    #[doc = "Bit 3 - The raw interrupt status of LP_UART_FRM_ERR_INT."]
    #[inline(always)]
    pub fn frm_err_int_raw(&mut self) -> FRM_ERR_INT_RAW_W<INT_RAW_SPEC> {
        FRM_ERR_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4 - The raw interrupt status of LP_UART_RXFIFO_OVF_INT."]
    #[inline(always)]
    pub fn rxfifo_ovf_int_raw(&mut self) -> RXFIFO_OVF_INT_RAW_W<INT_RAW_SPEC> {
        RXFIFO_OVF_INT_RAW_W::new(self, 4)
    }
    #[doc = "Bit 5 - The raw interrupt status of LP_UART_DSR_CHG_INT."]
    #[inline(always)]
    pub fn dsr_chg_int_raw(&mut self) -> DSR_CHG_INT_RAW_W<INT_RAW_SPEC> {
        DSR_CHG_INT_RAW_W::new(self, 5)
    }
    #[doc = "Bit 6 - The raw interrupt status of LP_UART_CTS_CHG_INT."]
    #[inline(always)]
    pub fn cts_chg_int_raw(&mut self) -> CTS_CHG_INT_RAW_W<INT_RAW_SPEC> {
        CTS_CHG_INT_RAW_W::new(self, 6)
    }
    #[doc = "Bit 7 - The raw interrupt status of LP_UART_BRK_DET_INT."]
    #[inline(always)]
    pub fn brk_det_int_raw(&mut self) -> BRK_DET_INT_RAW_W<INT_RAW_SPEC> {
        BRK_DET_INT_RAW_W::new(self, 7)
    }
    #[doc = "Bit 8 - The raw interrupt status of LP_UART_RXFIFO_TOUT_INT."]
    #[inline(always)]
    pub fn rxfifo_tout_int_raw(&mut self) -> RXFIFO_TOUT_INT_RAW_W<INT_RAW_SPEC> {
        RXFIFO_TOUT_INT_RAW_W::new(self, 8)
    }
    #[doc = "Bit 9 - The raw interrupt status of LP_UART_SW_XON_INT."]
    #[inline(always)]
    pub fn sw_xon_int_raw(&mut self) -> SW_XON_INT_RAW_W<INT_RAW_SPEC> {
        SW_XON_INT_RAW_W::new(self, 9)
    }
    #[doc = "Bit 10 - LP_UART_SW_XOFF_INT."]
    #[inline(always)]
    pub fn sw_xoff_int_raw(&mut self) -> SW_XOFF_INT_RAW_W<INT_RAW_SPEC> {
        SW_XOFF_INT_RAW_W::new(self, 10)
    }
    #[doc = "Bit 11 - The raw interrupt status of LP_UART_GLITCH_DET_INT."]
    #[inline(always)]
    pub fn glitch_det_int_raw(&mut self) -> GLITCH_DET_INT_RAW_W<INT_RAW_SPEC> {
        GLITCH_DET_INT_RAW_W::new(self, 11)
    }
    #[doc = "Bit 12 - The raw interrupt status of LP_UART_TX_BRK_DONE_INT."]
    #[inline(always)]
    pub fn tx_brk_done_int_raw(&mut self) -> TX_BRK_DONE_INT_RAW_W<INT_RAW_SPEC> {
        TX_BRK_DONE_INT_RAW_W::new(self, 12)
    }
    #[doc = "Bit 13 - The raw interrupt status of LP_UART_TX_BRK_IDLE_DONE_INT."]
    #[inline(always)]
    pub fn tx_brk_idle_done_int_raw(&mut self) -> TX_BRK_IDLE_DONE_INT_RAW_W<INT_RAW_SPEC> {
        TX_BRK_IDLE_DONE_INT_RAW_W::new(self, 13)
    }
    #[doc = "Bit 14 - The raw interrupt status of LP_UART_TX_DONE_INT."]
    #[inline(always)]
    pub fn tx_done_int_raw(&mut self) -> TX_DONE_INT_RAW_W<INT_RAW_SPEC> {
        TX_DONE_INT_RAW_W::new(self, 14)
    }
    #[doc = "Bit 18 - The raw interrupt status of LP_UART_AT_CMD_CHAR_DET_INT."]
    #[inline(always)]
    pub fn at_cmd_char_det_int_raw(&mut self) -> AT_CMD_CHAR_DET_INT_RAW_W<INT_RAW_SPEC> {
        AT_CMD_CHAR_DET_INT_RAW_W::new(self, 18)
    }
    #[doc = "Bit 19 - The raw interrupt status of LP_UART_WAKEUP_INT."]
    #[inline(always)]
    pub fn wakeup_int_raw(&mut self) -> WAKEUP_INT_RAW_W<INT_RAW_SPEC> {
        WAKEUP_INT_RAW_W::new(self, 19)
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
#[doc = "`reset()` method sets INT_RAW to value 0x02"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
