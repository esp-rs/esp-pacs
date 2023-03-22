#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_RAW` writer"]
pub struct W(crate::W<INT_RAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_RAW_SPEC>;
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
impl From<crate::W<INT_RAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_RAW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_FIFO_REMPTY_INT_RAW` reader - The raw interrupt status of TX_FIFO_REMPTY_INT."]
pub type TX_FIFO_REMPTY_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `TX_FIFO_REMPTY_INT_RAW` writer - The raw interrupt status of TX_FIFO_REMPTY_INT."]
pub type TX_FIFO_REMPTY_INT_RAW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_RAW_SPEC, bool, O>;
#[doc = "Field `RX_FIFO_WOVF_INT_RAW` reader - The raw interrupt status of RX_FIFO_WOVF_INT."]
pub type RX_FIFO_WOVF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `RX_FIFO_WOVF_INT_RAW` writer - The raw interrupt status of RX_FIFO_WOVF_INT."]
pub type RX_FIFO_WOVF_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_RAW_SPEC, bool, O>;
#[doc = "Field `TX_EOF_INT_RAW` reader - The raw interrupt status of TX_EOF_INT."]
pub type TX_EOF_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `TX_EOF_INT_RAW` writer - The raw interrupt status of TX_EOF_INT."]
pub type TX_EOF_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_RAW_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - The raw interrupt status of TX_FIFO_REMPTY_INT."]
    #[inline(always)]
    pub fn tx_fifo_rempty_int_raw(&self) -> TX_FIFO_REMPTY_INT_RAW_R {
        TX_FIFO_REMPTY_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status of RX_FIFO_WOVF_INT."]
    #[inline(always)]
    pub fn rx_fifo_wovf_int_raw(&self) -> RX_FIFO_WOVF_INT_RAW_R {
        RX_FIFO_WOVF_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status of TX_EOF_INT."]
    #[inline(always)]
    pub fn tx_eof_int_raw(&self) -> TX_EOF_INT_RAW_R {
        TX_EOF_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The raw interrupt status of TX_FIFO_REMPTY_INT."]
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_rempty_int_raw(&mut self) -> TX_FIFO_REMPTY_INT_RAW_W<0> {
        TX_FIFO_REMPTY_INT_RAW_W::new(self)
    }
    #[doc = "Bit 1 - The raw interrupt status of RX_FIFO_WOVF_INT."]
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_wovf_int_raw(&mut self) -> RX_FIFO_WOVF_INT_RAW_W<1> {
        RX_FIFO_WOVF_INT_RAW_W::new(self)
    }
    #[doc = "Bit 2 - The raw interrupt status of TX_EOF_INT."]
    #[inline(always)]
    #[must_use]
    pub fn tx_eof_int_raw(&mut self) -> TX_EOF_INT_RAW_W<2> {
        TX_EOF_INT_RAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Parallel IO interrupt raw singal status register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_raw::W](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
