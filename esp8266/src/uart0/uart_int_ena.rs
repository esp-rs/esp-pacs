#[doc = "Register `UART_INT_ENA` reader"]
pub type R = crate::R<UART_INT_ENA_SPEC>;
#[doc = "Register `UART_INT_ENA` writer"]
pub type W = crate::W<UART_INT_ENA_SPEC>;
#[doc = "Field `rxfifo_full_int_ena` reader - The interrupt enable bit for rx fifo full event"]
pub type RXFIFO_FULL_INT_ENA_R = crate::BitReader;
#[doc = "Field `rxfifo_full_int_ena` writer - The interrupt enable bit for rx fifo full event"]
pub type RXFIFO_FULL_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `txfifo_empty_int_ena` reader - The interrupt enable bit for tx fifo empty event"]
pub type TXFIFO_EMPTY_INT_ENA_R = crate::BitReader;
#[doc = "Field `txfifo_empty_int_ena` writer - The interrupt enable bit for tx fifo empty event"]
pub type TXFIFO_EMPTY_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `parity_err_int_ena` reader - The interrupt enable bit for parity error"]
pub type PARITY_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `parity_err_int_ena` writer - The interrupt enable bit for parity error"]
pub type PARITY_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `frm_err_int_ena` reader - The interrupt enable bit for other rx error"]
pub type FRM_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `frm_err_int_ena` writer - The interrupt enable bit for other rx error"]
pub type FRM_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rxfifo_ovf_int_ena` reader - The interrupt enable bit for rx fifo overflow"]
pub type RXFIFO_OVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `rxfifo_ovf_int_ena` writer - The interrupt enable bit for rx fifo overflow"]
pub type RXFIFO_OVF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dsr_chg_int_ena` reader - The interrupt enable bit for DSR changing level"]
pub type DSR_CHG_INT_ENA_R = crate::BitReader;
#[doc = "Field `dsr_chg_int_ena` writer - The interrupt enable bit for DSR changing level"]
pub type DSR_CHG_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cts_chg_int_ena` reader - The interrupt enable bit for CTS changing level"]
pub type CTS_CHG_INT_ENA_R = crate::BitReader;
#[doc = "Field `cts_chg_int_ena` writer - The interrupt enable bit for CTS changing level"]
pub type CTS_CHG_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `brk_det_int_ena` reader - The interrupt enable bit for rx byte start error"]
pub type BRK_DET_INT_ENA_R = crate::BitReader;
#[doc = "Field `brk_det_int_ena` writer - The interrupt enable bit for rx byte start error"]
pub type BRK_DET_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rxfifo_tout_int_ena` reader - The interrupt enable bit for rx time-out interrupt"]
pub type RXFIFO_TOUT_INT_ENA_R = crate::BitReader;
#[doc = "Field `rxfifo_tout_int_ena` writer - The interrupt enable bit for rx time-out interrupt"]
pub type RXFIFO_TOUT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for rx fifo full event"]
    #[inline(always)]
    pub fn rxfifo_full_int_ena(&self) -> RXFIFO_FULL_INT_ENA_R {
        RXFIFO_FULL_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for tx fifo empty event"]
    #[inline(always)]
    pub fn txfifo_empty_int_ena(&self) -> TXFIFO_EMPTY_INT_ENA_R {
        TXFIFO_EMPTY_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for parity error"]
    #[inline(always)]
    pub fn parity_err_int_ena(&self) -> PARITY_ERR_INT_ENA_R {
        PARITY_ERR_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for other rx error"]
    #[inline(always)]
    pub fn frm_err_int_ena(&self) -> FRM_ERR_INT_ENA_R {
        FRM_ERR_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for rx fifo overflow"]
    #[inline(always)]
    pub fn rxfifo_ovf_int_ena(&self) -> RXFIFO_OVF_INT_ENA_R {
        RXFIFO_OVF_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for DSR changing level"]
    #[inline(always)]
    pub fn dsr_chg_int_ena(&self) -> DSR_CHG_INT_ENA_R {
        DSR_CHG_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for CTS changing level"]
    #[inline(always)]
    pub fn cts_chg_int_ena(&self) -> CTS_CHG_INT_ENA_R {
        CTS_CHG_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for rx byte start error"]
    #[inline(always)]
    pub fn brk_det_int_ena(&self) -> BRK_DET_INT_ENA_R {
        BRK_DET_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for rx time-out interrupt"]
    #[inline(always)]
    pub fn rxfifo_tout_int_ena(&self) -> RXFIFO_TOUT_INT_ENA_R {
        RXFIFO_TOUT_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_INT_ENA")
            .field(
                "rxfifo_tout_int_ena",
                &format_args!("{}", self.rxfifo_tout_int_ena().bit()),
            )
            .field(
                "brk_det_int_ena",
                &format_args!("{}", self.brk_det_int_ena().bit()),
            )
            .field(
                "cts_chg_int_ena",
                &format_args!("{}", self.cts_chg_int_ena().bit()),
            )
            .field(
                "dsr_chg_int_ena",
                &format_args!("{}", self.dsr_chg_int_ena().bit()),
            )
            .field(
                "rxfifo_ovf_int_ena",
                &format_args!("{}", self.rxfifo_ovf_int_ena().bit()),
            )
            .field(
                "frm_err_int_ena",
                &format_args!("{}", self.frm_err_int_ena().bit()),
            )
            .field(
                "parity_err_int_ena",
                &format_args!("{}", self.parity_err_int_ena().bit()),
            )
            .field(
                "txfifo_empty_int_ena",
                &format_args!("{}", self.txfifo_empty_int_ena().bit()),
            )
            .field(
                "rxfifo_full_int_ena",
                &format_args!("{}", self.rxfifo_full_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UART_INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for rx fifo full event"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_full_int_ena(&mut self) -> RXFIFO_FULL_INT_ENA_W<UART_INT_ENA_SPEC> {
        RXFIFO_FULL_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for tx fifo empty event"]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_empty_int_ena(&mut self) -> TXFIFO_EMPTY_INT_ENA_W<UART_INT_ENA_SPEC> {
        TXFIFO_EMPTY_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for parity error"]
    #[inline(always)]
    #[must_use]
    pub fn parity_err_int_ena(&mut self) -> PARITY_ERR_INT_ENA_W<UART_INT_ENA_SPEC> {
        PARITY_ERR_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for other rx error"]
    #[inline(always)]
    #[must_use]
    pub fn frm_err_int_ena(&mut self) -> FRM_ERR_INT_ENA_W<UART_INT_ENA_SPEC> {
        FRM_ERR_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - The interrupt enable bit for rx fifo overflow"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_ovf_int_ena(&mut self) -> RXFIFO_OVF_INT_ENA_W<UART_INT_ENA_SPEC> {
        RXFIFO_OVF_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt enable bit for DSR changing level"]
    #[inline(always)]
    #[must_use]
    pub fn dsr_chg_int_ena(&mut self) -> DSR_CHG_INT_ENA_W<UART_INT_ENA_SPEC> {
        DSR_CHG_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6 - The interrupt enable bit for CTS changing level"]
    #[inline(always)]
    #[must_use]
    pub fn cts_chg_int_ena(&mut self) -> CTS_CHG_INT_ENA_W<UART_INT_ENA_SPEC> {
        CTS_CHG_INT_ENA_W::new(self, 6)
    }
    #[doc = "Bit 7 - The interrupt enable bit for rx byte start error"]
    #[inline(always)]
    #[must_use]
    pub fn brk_det_int_ena(&mut self) -> BRK_DET_INT_ENA_W<UART_INT_ENA_SPEC> {
        BRK_DET_INT_ENA_W::new(self, 7)
    }
    #[doc = "Bit 8 - The interrupt enable bit for rx time-out interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_tout_int_ena(&mut self) -> RXFIFO_TOUT_INT_ENA_W<UART_INT_ENA_SPEC> {
        RXFIFO_TOUT_INT_ENA_W::new(self, 8)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "UART INTERRUPT ENABLE REGISTER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_INT_ENA_SPEC;
impl crate::RegisterSpec for UART_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_int_ena::R`](R) reader structure"]
impl crate::Readable for UART_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart_int_ena::W`](W) writer structure"]
impl crate::Writable for UART_INT_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART_INT_ENA to value 0"]
impl crate::Resettable for UART_INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
