#[doc = "Register `_0_RXPKT_E_DSCR` reader"]
pub struct R(crate::R<_0_RXPKT_E_DSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_0_RXPKT_E_DSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<_0_RXPKT_E_DSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<_0_RXPKT_E_DSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_0_RXPKT_E_DSCR` writer"]
pub struct W(crate::W<_0_RXPKT_E_DSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_0_RXPKT_E_DSCR_SPEC>;
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
impl From<crate::W<_0_RXPKT_E_DSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<_0_RXPKT_E_DSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC0_RX_PKT_E_DSCR_ADDR` reader - "]
pub type SLC0_RX_PKT_E_DSCR_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `SLC0_RX_PKT_E_DSCR_ADDR` writer - "]
pub type SLC0_RX_PKT_E_DSCR_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, _0_RXPKT_E_DSCR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc0_rx_pkt_e_dscr_addr(&self) -> SLC0_RX_PKT_E_DSCR_ADDR_R {
        SLC0_RX_PKT_E_DSCR_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_0_RXPKT_E_DSCR")
            .field(
                "slc0_rx_pkt_e_dscr_addr",
                &format_args!("{}", self.slc0_rx_pkt_e_dscr_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<_0_RXPKT_E_DSCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_rx_pkt_e_dscr_addr(&mut self) -> SLC0_RX_PKT_E_DSCR_ADDR_W<0> {
        SLC0_RX_PKT_E_DSCR_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0_rxpkt_e_dscr](index.html) module"]
pub struct _0_RXPKT_E_DSCR_SPEC;
impl crate::RegisterSpec for _0_RXPKT_E_DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_0_rxpkt_e_dscr::R](R) reader structure"]
impl crate::Readable for _0_RXPKT_E_DSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_0_rxpkt_e_dscr::W](W) writer structure"]
impl crate::Writable for _0_RXPKT_E_DSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets _0_RXPKT_E_DSCR to value 0"]
impl crate::Resettable for _0_RXPKT_E_DSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
