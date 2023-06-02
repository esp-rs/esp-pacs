#[doc = "Register `CONF_CHAN` reader"]
pub struct R(crate::R<CONF_CHAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_CHAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_CHAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_CHAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF_CHAN` writer"]
pub struct W(crate::W<CONF_CHAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_CHAN_SPEC>;
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
impl From<crate::W<CONF_CHAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_CHAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_CHAN_MOD` reader - "]
pub type TX_CHAN_MOD_R = crate::FieldReader;
#[doc = "Field `TX_CHAN_MOD` writer - "]
pub type TX_CHAN_MOD_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_CHAN_SPEC, 3, O>;
#[doc = "Field `RX_CHAN_MOD` reader - "]
pub type RX_CHAN_MOD_R = crate::FieldReader;
#[doc = "Field `RX_CHAN_MOD` writer - "]
pub type RX_CHAN_MOD_W<'a, const O: u8> = crate::FieldWriter<'a, CONF_CHAN_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tx_chan_mod(&self) -> TX_CHAN_MOD_R {
        TX_CHAN_MOD_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn rx_chan_mod(&self) -> RX_CHAN_MOD_R {
        RX_CHAN_MOD_R::new(((self.bits >> 3) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF_CHAN")
            .field(
                "tx_chan_mod",
                &format_args!("{}", self.tx_chan_mod().bits()),
            )
            .field(
                "rx_chan_mod",
                &format_args!("{}", self.rx_chan_mod().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF_CHAN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn tx_chan_mod(&mut self) -> TX_CHAN_MOD_W<0> {
        TX_CHAN_MOD_W::new(self)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    #[must_use]
    pub fn rx_chan_mod(&mut self) -> RX_CHAN_MOD_W<3> {
        RX_CHAN_MOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf_chan](index.html) module"]
pub struct CONF_CHAN_SPEC;
impl crate::RegisterSpec for CONF_CHAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf_chan::R](R) reader structure"]
impl crate::Readable for CONF_CHAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf_chan::W](W) writer structure"]
impl crate::Writable for CONF_CHAN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF_CHAN to value 0"]
impl crate::Resettable for CONF_CHAN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
