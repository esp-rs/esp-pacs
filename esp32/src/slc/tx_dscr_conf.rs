#[doc = "Register `TX_DSCR_CONF` reader"]
pub struct R(crate::R<TX_DSCR_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_DSCR_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_DSCR_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_DSCR_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_DSCR_CONF` writer"]
pub struct W(crate::W<TX_DSCR_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_DSCR_CONF_SPEC>;
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
impl From<crate::W<TX_DSCR_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_DSCR_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WR_RETRY_THRESHOLD` reader - "]
pub type WR_RETRY_THRESHOLD_R = crate::FieldReader<u16>;
#[doc = "Field `WR_RETRY_THRESHOLD` writer - "]
pub type WR_RETRY_THRESHOLD_W<'a, const O: u8> =
    crate::FieldWriter<'a, TX_DSCR_CONF_SPEC, 11, O, u16>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn wr_retry_threshold(&self) -> WR_RETRY_THRESHOLD_R {
        WR_RETRY_THRESHOLD_R::new((self.bits & 0x07ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_DSCR_CONF")
            .field(
                "wr_retry_threshold",
                &format_args!("{}", self.wr_retry_threshold().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TX_DSCR_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn wr_retry_threshold(&mut self) -> WR_RETRY_THRESHOLD_W<0> {
        WR_RETRY_THRESHOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_dscr_conf](index.html) module"]
pub struct TX_DSCR_CONF_SPEC;
impl crate::RegisterSpec for TX_DSCR_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_dscr_conf::R](R) reader structure"]
impl crate::Readable for TX_DSCR_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_dscr_conf::W](W) writer structure"]
impl crate::Writable for TX_DSCR_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_DSCR_CONF to value 0x80"]
impl crate::Resettable for TX_DSCR_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
