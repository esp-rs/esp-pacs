#[doc = "Register `TOKEN_CON` writer"]
pub struct W(crate::W<TOKEN_CON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOKEN_CON_SPEC>;
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
impl From<crate::W<TOKEN_CON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOKEN_CON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC0HOST_TOKEN0_DEC` writer - *******Description***********"]
pub type SLC0HOST_TOKEN0_DEC_W<'a, const O: u8> = crate::BitWriter<'a, TOKEN_CON_SPEC, O>;
#[doc = "Field `SLC0HOST_TOKEN1_DEC` writer - *******Description***********"]
pub type SLC0HOST_TOKEN1_DEC_W<'a, const O: u8> = crate::BitWriter<'a, TOKEN_CON_SPEC, O>;
#[doc = "Field `SLC0HOST_TOKEN0_WR` writer - *******Description***********"]
pub type SLC0HOST_TOKEN0_WR_W<'a, const O: u8> = crate::BitWriter<'a, TOKEN_CON_SPEC, O>;
#[doc = "Field `SLC0HOST_TOKEN1_WR` writer - *******Description***********"]
pub type SLC0HOST_TOKEN1_WR_W<'a, const O: u8> = crate::BitWriter<'a, TOKEN_CON_SPEC, O>;
#[doc = "Field `SLC1HOST_TOKEN0_DEC` writer - *******Description***********"]
pub type SLC1HOST_TOKEN0_DEC_W<'a, const O: u8> = crate::BitWriter<'a, TOKEN_CON_SPEC, O>;
#[doc = "Field `SLC1HOST_TOKEN1_DEC` writer - *******Description***********"]
pub type SLC1HOST_TOKEN1_DEC_W<'a, const O: u8> = crate::BitWriter<'a, TOKEN_CON_SPEC, O>;
#[doc = "Field `SLC1HOST_TOKEN0_WR` writer - *******Description***********"]
pub type SLC1HOST_TOKEN0_WR_W<'a, const O: u8> = crate::BitWriter<'a, TOKEN_CON_SPEC, O>;
#[doc = "Field `SLC1HOST_TOKEN1_WR` writer - *******Description***********"]
pub type SLC1HOST_TOKEN1_WR_W<'a, const O: u8> = crate::BitWriter<'a, TOKEN_CON_SPEC, O>;
#[doc = "Field `SLC0HOST_LEN_WR` writer - *******Description***********"]
pub type SLC0HOST_LEN_WR_W<'a, const O: u8> = crate::BitWriter<'a, TOKEN_CON_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOKEN_CON_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0host_token0_dec(&mut self) -> SLC0HOST_TOKEN0_DEC_W<0> {
        SLC0HOST_TOKEN0_DEC_W::new(self)
    }
    #[doc = "Bit 1 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0host_token1_dec(&mut self) -> SLC0HOST_TOKEN1_DEC_W<1> {
        SLC0HOST_TOKEN1_DEC_W::new(self)
    }
    #[doc = "Bit 2 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0host_token0_wr(&mut self) -> SLC0HOST_TOKEN0_WR_W<2> {
        SLC0HOST_TOKEN0_WR_W::new(self)
    }
    #[doc = "Bit 3 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0host_token1_wr(&mut self) -> SLC0HOST_TOKEN1_WR_W<3> {
        SLC0HOST_TOKEN1_WR_W::new(self)
    }
    #[doc = "Bit 4 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1host_token0_dec(&mut self) -> SLC1HOST_TOKEN0_DEC_W<4> {
        SLC1HOST_TOKEN0_DEC_W::new(self)
    }
    #[doc = "Bit 5 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1host_token1_dec(&mut self) -> SLC1HOST_TOKEN1_DEC_W<5> {
        SLC1HOST_TOKEN1_DEC_W::new(self)
    }
    #[doc = "Bit 6 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1host_token0_wr(&mut self) -> SLC1HOST_TOKEN0_WR_W<6> {
        SLC1HOST_TOKEN0_WR_W::new(self)
    }
    #[doc = "Bit 7 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc1host_token1_wr(&mut self) -> SLC1HOST_TOKEN1_WR_W<7> {
        SLC1HOST_TOKEN1_WR_W::new(self)
    }
    #[doc = "Bit 8 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0host_len_wr(&mut self) -> SLC0HOST_LEN_WR_W<8> {
        SLC0HOST_LEN_WR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "*******Description***********\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [token_con](index.html) module"]
pub struct TOKEN_CON_SPEC;
impl crate::RegisterSpec for TOKEN_CON_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [token_con::W](W) writer structure"]
impl crate::Writable for TOKEN_CON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOKEN_CON to value 0"]
impl crate::Resettable for TOKEN_CON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
