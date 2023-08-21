#[doc = "Register `SLC_RX_DSCR_CONF` reader"]
pub type R = crate::R<SLC_RX_DSCR_CONF_SPEC>;
#[doc = "Register `SLC_RX_DSCR_CONF` writer"]
pub type W = crate::W<SLC_RX_DSCR_CONF_SPEC>;
#[doc = "Field `SLC_TOKEN_NO_REPLACE` reader - "]
pub type SLC_TOKEN_NO_REPLACE_R = crate::BitReader;
#[doc = "Field `SLC_TOKEN_NO_REPLACE` writer - "]
pub type SLC_TOKEN_NO_REPLACE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLC_INFOR_NO_REPLACE` reader - "]
pub type SLC_INFOR_NO_REPLACE_R = crate::BitReader;
#[doc = "Field `SLC_INFOR_NO_REPLACE` writer - "]
pub type SLC_INFOR_NO_REPLACE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn slc_token_no_replace(&mut self) -> SLC_TOKEN_NO_REPLACE_W<SLC_RX_DSCR_CONF_SPEC, 8> {
        SLC_TOKEN_NO_REPLACE_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn slc_infor_no_replace(&mut self) -> SLC_INFOR_NO_REPLACE_W<SLC_RX_DSCR_CONF_SPEC, 9> {
        SLC_INFOR_NO_REPLACE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SLC_RX_DSCR_CONF\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_rx_dscr_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_rx_dscr_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC_RX_DSCR_CONF_SPEC;
impl crate::RegisterSpec for SLC_RX_DSCR_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc_rx_dscr_conf::R`](R) reader structure"]
impl crate::Readable for SLC_RX_DSCR_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc_rx_dscr_conf::W`](W) writer structure"]
impl crate::Writable for SLC_RX_DSCR_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLC_RX_DSCR_CONF to value 0"]
impl crate::Resettable for SLC_RX_DSCR_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
