#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `TX_FIFO_REMPTY_INT_ENA` reader - Set this bit to enable TX_FIFO_REMPTY_INT."]
pub type TX_FIFO_REMPTY_INT_ENA_R = crate::BitReader;
#[doc = "Field `TX_FIFO_REMPTY_INT_ENA` writer - Set this bit to enable TX_FIFO_REMPTY_INT."]
pub type TX_FIFO_REMPTY_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RX_FIFO_WOVF_INT_ENA` reader - Set this bit to enable RX_FIFO_WOVF_INT."]
pub type RX_FIFO_WOVF_INT_ENA_R = crate::BitReader;
#[doc = "Field `RX_FIFO_WOVF_INT_ENA` writer - Set this bit to enable RX_FIFO_WOVF_INT."]
pub type RX_FIFO_WOVF_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_EOF_INT_ENA` reader - Set this bit to enable TX_EOF_INT."]
pub type TX_EOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `TX_EOF_INT_ENA` writer - Set this bit to enable TX_EOF_INT."]
pub type TX_EOF_INT_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable TX_FIFO_REMPTY_INT."]
    #[inline(always)]
    pub fn tx_fifo_rempty_int_ena(&self) -> TX_FIFO_REMPTY_INT_ENA_R {
        TX_FIFO_REMPTY_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable RX_FIFO_WOVF_INT."]
    #[inline(always)]
    pub fn rx_fifo_wovf_int_ena(&self) -> RX_FIFO_WOVF_INT_ENA_R {
        RX_FIFO_WOVF_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to enable TX_EOF_INT."]
    #[inline(always)]
    pub fn tx_eof_int_ena(&self) -> TX_EOF_INT_ENA_R {
        TX_EOF_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "tx_fifo_rempty_int_ena",
                &format_args!("{}", self.tx_fifo_rempty_int_ena().bit()),
            )
            .field(
                "rx_fifo_wovf_int_ena",
                &format_args!("{}", self.rx_fifo_wovf_int_ena().bit()),
            )
            .field(
                "tx_eof_int_ena",
                &format_args!("{}", self.tx_eof_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable TX_FIFO_REMPTY_INT."]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_rempty_int_ena(&mut self) -> TX_FIFO_REMPTY_INT_ENA_W<INT_ENA_SPEC, 0> {
        TX_FIFO_REMPTY_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to enable RX_FIFO_WOVF_INT."]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_wovf_int_ena(&mut self) -> RX_FIFO_WOVF_INT_ENA_W<INT_ENA_SPEC, 1> {
        RX_FIFO_WOVF_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to enable TX_EOF_INT."]
    #[inline(always)]
    #[must_use]
    pub fn tx_eof_int_ena(&mut self) -> TX_EOF_INT_ENA_W<INT_ENA_SPEC, 2> {
        TX_EOF_INT_ENA_W::new(self)
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
#[doc = "Parallel IO interrupt enable singal configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
