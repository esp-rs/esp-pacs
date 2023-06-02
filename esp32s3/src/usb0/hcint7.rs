#[doc = "Register `HCINT7` reader"]
pub struct R(crate::R<HCINT7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCINT7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCINT7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCINT7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCINT7` writer"]
pub struct W(crate::W<HCINT7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCINT7_SPEC>;
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
impl From<crate::W<HCINT7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCINT7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H_XFERCOMPL7` reader - "]
pub type H_XFERCOMPL7_R = crate::BitReader;
#[doc = "Field `H_XFERCOMPL7` writer - "]
pub type H_XFERCOMPL7_W<'a, const O: u8> = crate::BitWriter<'a, HCINT7_SPEC, O>;
#[doc = "Field `H_CHHLTD7` reader - "]
pub type H_CHHLTD7_R = crate::BitReader;
#[doc = "Field `H_CHHLTD7` writer - "]
pub type H_CHHLTD7_W<'a, const O: u8> = crate::BitWriter<'a, HCINT7_SPEC, O>;
#[doc = "Field `H_AHBERR7` reader - "]
pub type H_AHBERR7_R = crate::BitReader;
#[doc = "Field `H_AHBERR7` writer - "]
pub type H_AHBERR7_W<'a, const O: u8> = crate::BitWriter<'a, HCINT7_SPEC, O>;
#[doc = "Field `H_STALL7` reader - "]
pub type H_STALL7_R = crate::BitReader;
#[doc = "Field `H_STALL7` writer - "]
pub type H_STALL7_W<'a, const O: u8> = crate::BitWriter<'a, HCINT7_SPEC, O>;
#[doc = "Field `H_NACK7` reader - "]
pub type H_NACK7_R = crate::BitReader;
#[doc = "Field `H_NACK7` writer - "]
pub type H_NACK7_W<'a, const O: u8> = crate::BitWriter<'a, HCINT7_SPEC, O>;
#[doc = "Field `H_ACK7` reader - "]
pub type H_ACK7_R = crate::BitReader;
#[doc = "Field `H_ACK7` writer - "]
pub type H_ACK7_W<'a, const O: u8> = crate::BitWriter<'a, HCINT7_SPEC, O>;
#[doc = "Field `H_NYET7` reader - "]
pub type H_NYET7_R = crate::BitReader;
#[doc = "Field `H_NYET7` writer - "]
pub type H_NYET7_W<'a, const O: u8> = crate::BitWriter<'a, HCINT7_SPEC, O>;
#[doc = "Field `H_XACTERR7` reader - "]
pub type H_XACTERR7_R = crate::BitReader;
#[doc = "Field `H_XACTERR7` writer - "]
pub type H_XACTERR7_W<'a, const O: u8> = crate::BitWriter<'a, HCINT7_SPEC, O>;
#[doc = "Field `H_BBLERR7` reader - "]
pub type H_BBLERR7_R = crate::BitReader;
#[doc = "Field `H_BBLERR7` writer - "]
pub type H_BBLERR7_W<'a, const O: u8> = crate::BitWriter<'a, HCINT7_SPEC, O>;
#[doc = "Field `H_FRMOVRUN7` reader - "]
pub type H_FRMOVRUN7_R = crate::BitReader;
#[doc = "Field `H_FRMOVRUN7` writer - "]
pub type H_FRMOVRUN7_W<'a, const O: u8> = crate::BitWriter<'a, HCINT7_SPEC, O>;
#[doc = "Field `H_DATATGLERR7` reader - "]
pub type H_DATATGLERR7_R = crate::BitReader;
#[doc = "Field `H_DATATGLERR7` writer - "]
pub type H_DATATGLERR7_W<'a, const O: u8> = crate::BitWriter<'a, HCINT7_SPEC, O>;
#[doc = "Field `H_BNAINTR7` reader - "]
pub type H_BNAINTR7_R = crate::BitReader;
#[doc = "Field `H_BNAINTR7` writer - "]
pub type H_BNAINTR7_W<'a, const O: u8> = crate::BitWriter<'a, HCINT7_SPEC, O>;
#[doc = "Field `H_XCS_XACT_ERR7` reader - "]
pub type H_XCS_XACT_ERR7_R = crate::BitReader;
#[doc = "Field `H_XCS_XACT_ERR7` writer - "]
pub type H_XCS_XACT_ERR7_W<'a, const O: u8> = crate::BitWriter<'a, HCINT7_SPEC, O>;
#[doc = "Field `H_DESC_LST_ROLLINTR7` reader - "]
pub type H_DESC_LST_ROLLINTR7_R = crate::BitReader;
#[doc = "Field `H_DESC_LST_ROLLINTR7` writer - "]
pub type H_DESC_LST_ROLLINTR7_W<'a, const O: u8> = crate::BitWriter<'a, HCINT7_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn h_xfercompl7(&self) -> H_XFERCOMPL7_R {
        H_XFERCOMPL7_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn h_chhltd7(&self) -> H_CHHLTD7_R {
        H_CHHLTD7_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn h_ahberr7(&self) -> H_AHBERR7_R {
        H_AHBERR7_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn h_stall7(&self) -> H_STALL7_R {
        H_STALL7_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn h_nack7(&self) -> H_NACK7_R {
        H_NACK7_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn h_ack7(&self) -> H_ACK7_R {
        H_ACK7_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn h_nyet7(&self) -> H_NYET7_R {
        H_NYET7_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn h_xacterr7(&self) -> H_XACTERR7_R {
        H_XACTERR7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn h_bblerr7(&self) -> H_BBLERR7_R {
        H_BBLERR7_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn h_frmovrun7(&self) -> H_FRMOVRUN7_R {
        H_FRMOVRUN7_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn h_datatglerr7(&self) -> H_DATATGLERR7_R {
        H_DATATGLERR7_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn h_bnaintr7(&self) -> H_BNAINTR7_R {
        H_BNAINTR7_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn h_xcs_xact_err7(&self) -> H_XCS_XACT_ERR7_R {
        H_XCS_XACT_ERR7_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn h_desc_lst_rollintr7(&self) -> H_DESC_LST_ROLLINTR7_R {
        H_DESC_LST_ROLLINTR7_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINT7")
            .field(
                "h_xfercompl7",
                &format_args!("{}", self.h_xfercompl7().bit()),
            )
            .field("h_chhltd7", &format_args!("{}", self.h_chhltd7().bit()))
            .field("h_ahberr7", &format_args!("{}", self.h_ahberr7().bit()))
            .field("h_stall7", &format_args!("{}", self.h_stall7().bit()))
            .field("h_nack7", &format_args!("{}", self.h_nack7().bit()))
            .field("h_ack7", &format_args!("{}", self.h_ack7().bit()))
            .field("h_nyet7", &format_args!("{}", self.h_nyet7().bit()))
            .field("h_xacterr7", &format_args!("{}", self.h_xacterr7().bit()))
            .field("h_bblerr7", &format_args!("{}", self.h_bblerr7().bit()))
            .field("h_frmovrun7", &format_args!("{}", self.h_frmovrun7().bit()))
            .field(
                "h_datatglerr7",
                &format_args!("{}", self.h_datatglerr7().bit()),
            )
            .field("h_bnaintr7", &format_args!("{}", self.h_bnaintr7().bit()))
            .field(
                "h_xcs_xact_err7",
                &format_args!("{}", self.h_xcs_xact_err7().bit()),
            )
            .field(
                "h_desc_lst_rollintr7",
                &format_args!("{}", self.h_desc_lst_rollintr7().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCINT7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn h_xfercompl7(&mut self) -> H_XFERCOMPL7_W<0> {
        H_XFERCOMPL7_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn h_chhltd7(&mut self) -> H_CHHLTD7_W<1> {
        H_CHHLTD7_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn h_ahberr7(&mut self) -> H_AHBERR7_W<2> {
        H_AHBERR7_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn h_stall7(&mut self) -> H_STALL7_W<3> {
        H_STALL7_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn h_nack7(&mut self) -> H_NACK7_W<4> {
        H_NACK7_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn h_ack7(&mut self) -> H_ACK7_W<5> {
        H_ACK7_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn h_nyet7(&mut self) -> H_NYET7_W<6> {
        H_NYET7_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn h_xacterr7(&mut self) -> H_XACTERR7_W<7> {
        H_XACTERR7_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn h_bblerr7(&mut self) -> H_BBLERR7_W<8> {
        H_BBLERR7_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn h_frmovrun7(&mut self) -> H_FRMOVRUN7_W<9> {
        H_FRMOVRUN7_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn h_datatglerr7(&mut self) -> H_DATATGLERR7_W<10> {
        H_DATATGLERR7_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn h_bnaintr7(&mut self) -> H_BNAINTR7_W<11> {
        H_BNAINTR7_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn h_xcs_xact_err7(&mut self) -> H_XCS_XACT_ERR7_W<12> {
        H_XCS_XACT_ERR7_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn h_desc_lst_rollintr7(&mut self) -> H_DESC_LST_ROLLINTR7_W<13> {
        H_DESC_LST_ROLLINTR7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcint7](index.html) module"]
pub struct HCINT7_SPEC;
impl crate::RegisterSpec for HCINT7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcint7::R](R) reader structure"]
impl crate::Readable for HCINT7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcint7::W](W) writer structure"]
impl crate::Writable for HCINT7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCINT7 to value 0"]
impl crate::Resettable for HCINT7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
