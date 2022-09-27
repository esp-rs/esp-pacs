#[doc = "Register `UART_INT_ENA` reader"]
pub struct R(crate::R<UART_INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_INT_ENA` writer"]
pub struct W(crate::W<UART_INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<UART_INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rxfifo_full_int_ena` reader - The interrupt enable bit for rx fifo full event"]
pub type RXFIFO_FULL_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `rxfifo_full_int_ena` writer - The interrupt enable bit for rx fifo full event"]
pub type RXFIFO_FULL_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART_INT_ENA_SPEC, bool, O>;
#[doc = "Field `txfifo_empty_int_ena` reader - The interrupt enable bit for tx fifo empty event"]
pub type TXFIFO_EMPTY_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `txfifo_empty_int_ena` writer - The interrupt enable bit for tx fifo empty event"]
pub type TXFIFO_EMPTY_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART_INT_ENA_SPEC, bool, O>;
#[doc = "Field `parity_err_int_ena` reader - The interrupt enable bit for parity error"]
pub type PARITY_ERR_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `parity_err_int_ena` writer - The interrupt enable bit for parity error"]
pub type PARITY_ERR_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART_INT_ENA_SPEC, bool, O>;
#[doc = "Field `frm_err_int_ena` reader - The interrupt enable bit for other rx error"]
pub type FRM_ERR_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `frm_err_int_ena` writer - The interrupt enable bit for other rx error"]
pub type FRM_ERR_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_INT_ENA_SPEC, bool, O>;
#[doc = "Field `rxfifo_ovf_int_ena` reader - The interrupt enable bit for rx fifo overflow"]
pub type RXFIFO_OVF_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `rxfifo_ovf_int_ena` writer - The interrupt enable bit for rx fifo overflow"]
pub type RXFIFO_OVF_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART_INT_ENA_SPEC, bool, O>;
#[doc = "Field `dsr_chg_int_ena` reader - The interrupt enable bit for DSR changing level"]
pub type DSR_CHG_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `dsr_chg_int_ena` writer - The interrupt enable bit for DSR changing level"]
pub type DSR_CHG_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_INT_ENA_SPEC, bool, O>;
#[doc = "Field `cts_chg_int_ena` reader - The interrupt enable bit for CTS changing level"]
pub type CTS_CHG_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `cts_chg_int_ena` writer - The interrupt enable bit for CTS changing level"]
pub type CTS_CHG_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_INT_ENA_SPEC, bool, O>;
#[doc = "Field `brk_det_int_ena` reader - The interrupt enable bit for rx byte start error"]
pub type BRK_DET_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `brk_det_int_ena` writer - The interrupt enable bit for rx byte start error"]
pub type BRK_DET_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, UART_INT_ENA_SPEC, bool, O>;
#[doc = "Field `rxfifo_tout_int_ena` reader - The interrupt enable bit for rx time-out interrupt"]
pub type RXFIFO_TOUT_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `rxfifo_tout_int_ena` writer - The interrupt enable bit for rx time-out interrupt"]
pub type RXFIFO_TOUT_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, UART_INT_ENA_SPEC, bool, O>;
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
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for rx fifo full event"]
    #[inline(always)]
    pub fn rxfifo_full_int_ena(&mut self) -> RXFIFO_FULL_INT_ENA_W<0> {
        RXFIFO_FULL_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - The interrupt enable bit for tx fifo empty event"]
    #[inline(always)]
    pub fn txfifo_empty_int_ena(&mut self) -> TXFIFO_EMPTY_INT_ENA_W<1> {
        TXFIFO_EMPTY_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - The interrupt enable bit for parity error"]
    #[inline(always)]
    pub fn parity_err_int_ena(&mut self) -> PARITY_ERR_INT_ENA_W<2> {
        PARITY_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - The interrupt enable bit for other rx error"]
    #[inline(always)]
    pub fn frm_err_int_ena(&mut self) -> FRM_ERR_INT_ENA_W<3> {
        FRM_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4 - The interrupt enable bit for rx fifo overflow"]
    #[inline(always)]
    pub fn rxfifo_ovf_int_ena(&mut self) -> RXFIFO_OVF_INT_ENA_W<4> {
        RXFIFO_OVF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - The interrupt enable bit for DSR changing level"]
    #[inline(always)]
    pub fn dsr_chg_int_ena(&mut self) -> DSR_CHG_INT_ENA_W<5> {
        DSR_CHG_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6 - The interrupt enable bit for CTS changing level"]
    #[inline(always)]
    pub fn cts_chg_int_ena(&mut self) -> CTS_CHG_INT_ENA_W<6> {
        CTS_CHG_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7 - The interrupt enable bit for rx byte start error"]
    #[inline(always)]
    pub fn brk_det_int_ena(&mut self) -> BRK_DET_INT_ENA_W<7> {
        BRK_DET_INT_ENA_W::new(self)
    }
    #[doc = "Bit 8 - The interrupt enable bit for rx time-out interrupt"]
    #[inline(always)]
    pub fn rxfifo_tout_int_ena(&mut self) -> RXFIFO_TOUT_INT_ENA_W<8> {
        RXFIFO_TOUT_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART INTERRUPT ENABLE REGISTER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_int_ena](index.html) module"]
pub struct UART_INT_ENA_SPEC;
impl crate::RegisterSpec for UART_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_int_ena::R](R) reader structure"]
impl crate::Readable for UART_INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_int_ena::W](W) writer structure"]
impl crate::Writable for UART_INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_INT_ENA to value 0"]
impl crate::Resettable for UART_INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
