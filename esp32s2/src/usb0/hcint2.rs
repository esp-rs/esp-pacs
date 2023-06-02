#[doc = "Register `HCINT2` reader"]
pub struct R(crate::R<HCINT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCINT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCINT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCINT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCINT2` writer"]
pub struct W(crate::W<HCINT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCINT2_SPEC>;
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
impl From<crate::W<HCINT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCINT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H_XFERCOMPL2` reader - "]
pub type H_XFERCOMPL2_R = crate::BitReader;
#[doc = "Field `H_XFERCOMPL2` writer - "]
pub type H_XFERCOMPL2_W<'a, const O: u8> = crate::BitWriter<'a, HCINT2_SPEC, O>;
#[doc = "Field `H_CHHLTD2` reader - "]
pub type H_CHHLTD2_R = crate::BitReader;
#[doc = "Field `H_CHHLTD2` writer - "]
pub type H_CHHLTD2_W<'a, const O: u8> = crate::BitWriter<'a, HCINT2_SPEC, O>;
#[doc = "Field `H_AHBERR2` reader - "]
pub type H_AHBERR2_R = crate::BitReader;
#[doc = "Field `H_AHBERR2` writer - "]
pub type H_AHBERR2_W<'a, const O: u8> = crate::BitWriter<'a, HCINT2_SPEC, O>;
#[doc = "Field `H_STALL2` reader - "]
pub type H_STALL2_R = crate::BitReader;
#[doc = "Field `H_STALL2` writer - "]
pub type H_STALL2_W<'a, const O: u8> = crate::BitWriter<'a, HCINT2_SPEC, O>;
#[doc = "Field `H_NACK2` reader - "]
pub type H_NACK2_R = crate::BitReader;
#[doc = "Field `H_NACK2` writer - "]
pub type H_NACK2_W<'a, const O: u8> = crate::BitWriter<'a, HCINT2_SPEC, O>;
#[doc = "Field `H_ACK2` reader - "]
pub type H_ACK2_R = crate::BitReader;
#[doc = "Field `H_ACK2` writer - "]
pub type H_ACK2_W<'a, const O: u8> = crate::BitWriter<'a, HCINT2_SPEC, O>;
#[doc = "Field `H_NYET2` reader - "]
pub type H_NYET2_R = crate::BitReader;
#[doc = "Field `H_NYET2` writer - "]
pub type H_NYET2_W<'a, const O: u8> = crate::BitWriter<'a, HCINT2_SPEC, O>;
#[doc = "Field `H_XACTERR2` reader - "]
pub type H_XACTERR2_R = crate::BitReader;
#[doc = "Field `H_XACTERR2` writer - "]
pub type H_XACTERR2_W<'a, const O: u8> = crate::BitWriter<'a, HCINT2_SPEC, O>;
#[doc = "Field `H_BBLERR2` reader - "]
pub type H_BBLERR2_R = crate::BitReader;
#[doc = "Field `H_BBLERR2` writer - "]
pub type H_BBLERR2_W<'a, const O: u8> = crate::BitWriter<'a, HCINT2_SPEC, O>;
#[doc = "Field `H_FRMOVRUN2` reader - "]
pub type H_FRMOVRUN2_R = crate::BitReader;
#[doc = "Field `H_FRMOVRUN2` writer - "]
pub type H_FRMOVRUN2_W<'a, const O: u8> = crate::BitWriter<'a, HCINT2_SPEC, O>;
#[doc = "Field `H_DATATGLERR2` reader - "]
pub type H_DATATGLERR2_R = crate::BitReader;
#[doc = "Field `H_DATATGLERR2` writer - "]
pub type H_DATATGLERR2_W<'a, const O: u8> = crate::BitWriter<'a, HCINT2_SPEC, O>;
#[doc = "Field `H_BNAINTR2` reader - "]
pub type H_BNAINTR2_R = crate::BitReader;
#[doc = "Field `H_BNAINTR2` writer - "]
pub type H_BNAINTR2_W<'a, const O: u8> = crate::BitWriter<'a, HCINT2_SPEC, O>;
#[doc = "Field `H_XCS_XACT_ERR2` reader - "]
pub type H_XCS_XACT_ERR2_R = crate::BitReader;
#[doc = "Field `H_XCS_XACT_ERR2` writer - "]
pub type H_XCS_XACT_ERR2_W<'a, const O: u8> = crate::BitWriter<'a, HCINT2_SPEC, O>;
#[doc = "Field `H_DESC_LST_ROLLINTR2` reader - "]
pub type H_DESC_LST_ROLLINTR2_R = crate::BitReader;
#[doc = "Field `H_DESC_LST_ROLLINTR2` writer - "]
pub type H_DESC_LST_ROLLINTR2_W<'a, const O: u8> = crate::BitWriter<'a, HCINT2_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn h_xfercompl2(&self) -> H_XFERCOMPL2_R {
        H_XFERCOMPL2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn h_chhltd2(&self) -> H_CHHLTD2_R {
        H_CHHLTD2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn h_ahberr2(&self) -> H_AHBERR2_R {
        H_AHBERR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn h_stall2(&self) -> H_STALL2_R {
        H_STALL2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn h_nack2(&self) -> H_NACK2_R {
        H_NACK2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn h_ack2(&self) -> H_ACK2_R {
        H_ACK2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn h_nyet2(&self) -> H_NYET2_R {
        H_NYET2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn h_xacterr2(&self) -> H_XACTERR2_R {
        H_XACTERR2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn h_bblerr2(&self) -> H_BBLERR2_R {
        H_BBLERR2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn h_frmovrun2(&self) -> H_FRMOVRUN2_R {
        H_FRMOVRUN2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn h_datatglerr2(&self) -> H_DATATGLERR2_R {
        H_DATATGLERR2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn h_bnaintr2(&self) -> H_BNAINTR2_R {
        H_BNAINTR2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn h_xcs_xact_err2(&self) -> H_XCS_XACT_ERR2_R {
        H_XCS_XACT_ERR2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn h_desc_lst_rollintr2(&self) -> H_DESC_LST_ROLLINTR2_R {
        H_DESC_LST_ROLLINTR2_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINT2")
            .field(
                "h_xfercompl2",
                &format_args!("{}", self.h_xfercompl2().bit()),
            )
            .field("h_chhltd2", &format_args!("{}", self.h_chhltd2().bit()))
            .field("h_ahberr2", &format_args!("{}", self.h_ahberr2().bit()))
            .field("h_stall2", &format_args!("{}", self.h_stall2().bit()))
            .field("h_nack2", &format_args!("{}", self.h_nack2().bit()))
            .field("h_ack2", &format_args!("{}", self.h_ack2().bit()))
            .field("h_nyet2", &format_args!("{}", self.h_nyet2().bit()))
            .field("h_xacterr2", &format_args!("{}", self.h_xacterr2().bit()))
            .field("h_bblerr2", &format_args!("{}", self.h_bblerr2().bit()))
            .field("h_frmovrun2", &format_args!("{}", self.h_frmovrun2().bit()))
            .field(
                "h_datatglerr2",
                &format_args!("{}", self.h_datatglerr2().bit()),
            )
            .field("h_bnaintr2", &format_args!("{}", self.h_bnaintr2().bit()))
            .field(
                "h_xcs_xact_err2",
                &format_args!("{}", self.h_xcs_xact_err2().bit()),
            )
            .field(
                "h_desc_lst_rollintr2",
                &format_args!("{}", self.h_desc_lst_rollintr2().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCINT2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn h_xfercompl2(&mut self) -> H_XFERCOMPL2_W<0> {
        H_XFERCOMPL2_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn h_chhltd2(&mut self) -> H_CHHLTD2_W<1> {
        H_CHHLTD2_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn h_ahberr2(&mut self) -> H_AHBERR2_W<2> {
        H_AHBERR2_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn h_stall2(&mut self) -> H_STALL2_W<3> {
        H_STALL2_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn h_nack2(&mut self) -> H_NACK2_W<4> {
        H_NACK2_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn h_ack2(&mut self) -> H_ACK2_W<5> {
        H_ACK2_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn h_nyet2(&mut self) -> H_NYET2_W<6> {
        H_NYET2_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn h_xacterr2(&mut self) -> H_XACTERR2_W<7> {
        H_XACTERR2_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn h_bblerr2(&mut self) -> H_BBLERR2_W<8> {
        H_BBLERR2_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn h_frmovrun2(&mut self) -> H_FRMOVRUN2_W<9> {
        H_FRMOVRUN2_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn h_datatglerr2(&mut self) -> H_DATATGLERR2_W<10> {
        H_DATATGLERR2_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn h_bnaintr2(&mut self) -> H_BNAINTR2_W<11> {
        H_BNAINTR2_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn h_xcs_xact_err2(&mut self) -> H_XCS_XACT_ERR2_W<12> {
        H_XCS_XACT_ERR2_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn h_desc_lst_rollintr2(&mut self) -> H_DESC_LST_ROLLINTR2_W<13> {
        H_DESC_LST_ROLLINTR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcint2](index.html) module"]
pub struct HCINT2_SPEC;
impl crate::RegisterSpec for HCINT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcint2::R](R) reader structure"]
impl crate::Readable for HCINT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcint2::W](W) writer structure"]
impl crate::Writable for HCINT2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCINT2 to value 0"]
impl crate::Resettable for HCINT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
