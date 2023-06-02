#[doc = "Register `SLC_RX_DSCR_CONF` reader"]
pub struct R(crate::R<SLC_RX_DSCR_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLC_RX_DSCR_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLC_RX_DSCR_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLC_RX_DSCR_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLC_RX_DSCR_CONF` writer"]
pub struct W(crate::W<SLC_RX_DSCR_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLC_RX_DSCR_CONF_SPEC>;
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
impl From<crate::W<SLC_RX_DSCR_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLC_RX_DSCR_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC_TOKEN_NO_REPLACE` reader - "]
pub type SLC_TOKEN_NO_REPLACE_R = crate::BitReader;
#[doc = "Field `SLC_TOKEN_NO_REPLACE` writer - "]
pub type SLC_TOKEN_NO_REPLACE_W<'a, const O: u8> = crate::BitWriter<'a, SLC_RX_DSCR_CONF_SPEC, O>;
#[doc = "Field `SLC_INFOR_NO_REPLACE` reader - "]
pub type SLC_INFOR_NO_REPLACE_R = crate::BitReader;
#[doc = "Field `SLC_INFOR_NO_REPLACE` writer - "]
pub type SLC_INFOR_NO_REPLACE_W<'a, const O: u8> = crate::BitWriter<'a, SLC_RX_DSCR_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn slc_token_no_replace(&self) -> SLC_TOKEN_NO_REPLACE_R {
        SLC_TOKEN_NO_REPLACE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn slc_infor_no_replace(&self) -> SLC_INFOR_NO_REPLACE_R {
        SLC_INFOR_NO_REPLACE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC_RX_DSCR_CONF")
            .field(
                "slc_infor_no_replace",
                &format_args!("{}", self.slc_infor_no_replace().bit()),
            )
            .field(
                "slc_token_no_replace",
                &format_args!("{}", self.slc_token_no_replace().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC_RX_DSCR_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn slc_token_no_replace(&mut self) -> SLC_TOKEN_NO_REPLACE_W<8> {
        SLC_TOKEN_NO_REPLACE_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn slc_infor_no_replace(&mut self) -> SLC_INFOR_NO_REPLACE_W<9> {
        SLC_INFOR_NO_REPLACE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SLC_RX_DSCR_CONF\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slc_rx_dscr_conf](index.html) module"]
pub struct SLC_RX_DSCR_CONF_SPEC;
impl crate::RegisterSpec for SLC_RX_DSCR_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slc_rx_dscr_conf::R](R) reader structure"]
impl crate::Readable for SLC_RX_DSCR_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slc_rx_dscr_conf::W](W) writer structure"]
impl crate::Writable for SLC_RX_DSCR_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLC_RX_DSCR_CONF to value 0"]
impl crate::Resettable for SLC_RX_DSCR_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
