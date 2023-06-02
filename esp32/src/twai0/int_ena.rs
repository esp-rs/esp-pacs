#[doc = "Register `INT_ENA` reader"]
pub struct R(crate::R<INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA` writer"]
pub struct W(crate::W<INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_SPEC>;
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
impl From<crate::W<INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_INT_ENA` reader - Set this bit to 1 to enable receive interrupt."]
pub type RX_INT_ENA_R = crate::BitReader;
#[doc = "Field `RX_INT_ENA` writer - Set this bit to 1 to enable receive interrupt."]
pub type RX_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `TX_INT_ENA` reader - Set this bit to 1 to enable transmit interrupt."]
pub type TX_INT_ENA_R = crate::BitReader;
#[doc = "Field `TX_INT_ENA` writer - Set this bit to 1 to enable transmit interrupt."]
pub type TX_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `ERR_WARN_INT_ENA` reader - Set this bit to 1 to enable error warning interrupt."]
pub type ERR_WARN_INT_ENA_R = crate::BitReader;
#[doc = "Field `ERR_WARN_INT_ENA` writer - Set this bit to 1 to enable error warning interrupt."]
pub type ERR_WARN_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `OVERRUN_INT_ENA` reader - Set this bit to 1 to enable data overrun interrupt."]
pub type OVERRUN_INT_ENA_R = crate::BitReader;
#[doc = "Field `OVERRUN_INT_ENA` writer - Set this bit to 1 to enable data overrun interrupt."]
pub type OVERRUN_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `ERR_PASSIVE_INT_ENA` reader - Set this bit to 1 to enable error passive interrupt."]
pub type ERR_PASSIVE_INT_ENA_R = crate::BitReader;
#[doc = "Field `ERR_PASSIVE_INT_ENA` writer - Set this bit to 1 to enable error passive interrupt."]
pub type ERR_PASSIVE_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `ARB_LOST_INT_ENA` reader - Set this bit to 1 to enable arbitration lost interrupt."]
pub type ARB_LOST_INT_ENA_R = crate::BitReader;
#[doc = "Field `ARB_LOST_INT_ENA` writer - Set this bit to 1 to enable arbitration lost interrupt."]
pub type ARB_LOST_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `BUS_ERR_INT_ENA` reader - Set this bit to 1 to enable error interrupt."]
pub type BUS_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `BUS_ERR_INT_ENA` writer - Set this bit to 1 to enable error interrupt."]
pub type BUS_ERR_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to 1 to enable receive interrupt."]
    #[inline(always)]
    pub fn rx_int_ena(&self) -> RX_INT_ENA_R {
        RX_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to 1 to enable transmit interrupt."]
    #[inline(always)]
    pub fn tx_int_ena(&self) -> TX_INT_ENA_R {
        TX_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to 1 to enable error warning interrupt."]
    #[inline(always)]
    pub fn err_warn_int_ena(&self) -> ERR_WARN_INT_ENA_R {
        ERR_WARN_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to 1 to enable data overrun interrupt."]
    #[inline(always)]
    pub fn overrun_int_ena(&self) -> OVERRUN_INT_ENA_R {
        OVERRUN_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to 1 to enable error passive interrupt."]
    #[inline(always)]
    pub fn err_passive_int_ena(&self) -> ERR_PASSIVE_INT_ENA_R {
        ERR_PASSIVE_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to 1 to enable arbitration lost interrupt."]
    #[inline(always)]
    pub fn arb_lost_int_ena(&self) -> ARB_LOST_INT_ENA_R {
        ARB_LOST_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to 1 to enable error interrupt."]
    #[inline(always)]
    pub fn bus_err_int_ena(&self) -> BUS_ERR_INT_ENA_R {
        BUS_ERR_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("rx_int_ena", &format_args!("{}", self.rx_int_ena().bit()))
            .field("tx_int_ena", &format_args!("{}", self.tx_int_ena().bit()))
            .field(
                "err_warn_int_ena",
                &format_args!("{}", self.err_warn_int_ena().bit()),
            )
            .field(
                "overrun_int_ena",
                &format_args!("{}", self.overrun_int_ena().bit()),
            )
            .field(
                "err_passive_int_ena",
                &format_args!("{}", self.err_passive_int_ena().bit()),
            )
            .field(
                "arb_lost_int_ena",
                &format_args!("{}", self.arb_lost_int_ena().bit()),
            )
            .field(
                "bus_err_int_ena",
                &format_args!("{}", self.bus_err_int_ena().bit()),
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
    #[doc = "Bit 0 - Set this bit to 1 to enable receive interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rx_int_ena(&mut self) -> RX_INT_ENA_W<0> {
        RX_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to 1 to enable transmit interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tx_int_ena(&mut self) -> TX_INT_ENA_W<1> {
        TX_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to 1 to enable error warning interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn err_warn_int_ena(&mut self) -> ERR_WARN_INT_ENA_W<2> {
        ERR_WARN_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to 1 to enable data overrun interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn overrun_int_ena(&mut self) -> OVERRUN_INT_ENA_W<3> {
        OVERRUN_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to 1 to enable error passive interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn err_passive_int_ena(&mut self) -> ERR_PASSIVE_INT_ENA_W<5> {
        ERR_PASSIVE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to 1 to enable arbitration lost interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn arb_lost_int_ena(&mut self) -> ARB_LOST_INT_ENA_W<6> {
        ARB_LOST_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to 1 to enable error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn bus_err_int_ena(&mut self) -> BUS_ERR_INT_ENA_W<7> {
        BUS_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](index.html) module"]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena::R](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena::W](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
