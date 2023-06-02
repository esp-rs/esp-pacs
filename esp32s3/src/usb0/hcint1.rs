#[doc = "Register `HCINT1` reader"]
pub struct R(crate::R<HCINT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCINT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCINT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCINT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCINT1` writer"]
pub struct W(crate::W<HCINT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCINT1_SPEC>;
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
impl From<crate::W<HCINT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCINT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H_XFERCOMPL1` reader - "]
pub type H_XFERCOMPL1_R = crate::BitReader;
#[doc = "Field `H_XFERCOMPL1` writer - "]
pub type H_XFERCOMPL1_W<'a, const O: u8> = crate::BitWriter<'a, HCINT1_SPEC, O>;
#[doc = "Field `H_CHHLTD1` reader - "]
pub type H_CHHLTD1_R = crate::BitReader;
#[doc = "Field `H_CHHLTD1` writer - "]
pub type H_CHHLTD1_W<'a, const O: u8> = crate::BitWriter<'a, HCINT1_SPEC, O>;
#[doc = "Field `H_AHBERR1` reader - "]
pub type H_AHBERR1_R = crate::BitReader;
#[doc = "Field `H_AHBERR1` writer - "]
pub type H_AHBERR1_W<'a, const O: u8> = crate::BitWriter<'a, HCINT1_SPEC, O>;
#[doc = "Field `H_STALL1` reader - "]
pub type H_STALL1_R = crate::BitReader;
#[doc = "Field `H_STALL1` writer - "]
pub type H_STALL1_W<'a, const O: u8> = crate::BitWriter<'a, HCINT1_SPEC, O>;
#[doc = "Field `H_NACK1` reader - "]
pub type H_NACK1_R = crate::BitReader;
#[doc = "Field `H_NACK1` writer - "]
pub type H_NACK1_W<'a, const O: u8> = crate::BitWriter<'a, HCINT1_SPEC, O>;
#[doc = "Field `H_ACK1` reader - "]
pub type H_ACK1_R = crate::BitReader;
#[doc = "Field `H_ACK1` writer - "]
pub type H_ACK1_W<'a, const O: u8> = crate::BitWriter<'a, HCINT1_SPEC, O>;
#[doc = "Field `H_NYET1` reader - "]
pub type H_NYET1_R = crate::BitReader;
#[doc = "Field `H_NYET1` writer - "]
pub type H_NYET1_W<'a, const O: u8> = crate::BitWriter<'a, HCINT1_SPEC, O>;
#[doc = "Field `H_XACTERR1` reader - "]
pub type H_XACTERR1_R = crate::BitReader;
#[doc = "Field `H_XACTERR1` writer - "]
pub type H_XACTERR1_W<'a, const O: u8> = crate::BitWriter<'a, HCINT1_SPEC, O>;
#[doc = "Field `H_BBLERR1` reader - "]
pub type H_BBLERR1_R = crate::BitReader;
#[doc = "Field `H_BBLERR1` writer - "]
pub type H_BBLERR1_W<'a, const O: u8> = crate::BitWriter<'a, HCINT1_SPEC, O>;
#[doc = "Field `H_FRMOVRUN1` reader - "]
pub type H_FRMOVRUN1_R = crate::BitReader;
#[doc = "Field `H_FRMOVRUN1` writer - "]
pub type H_FRMOVRUN1_W<'a, const O: u8> = crate::BitWriter<'a, HCINT1_SPEC, O>;
#[doc = "Field `H_DATATGLERR1` reader - "]
pub type H_DATATGLERR1_R = crate::BitReader;
#[doc = "Field `H_DATATGLERR1` writer - "]
pub type H_DATATGLERR1_W<'a, const O: u8> = crate::BitWriter<'a, HCINT1_SPEC, O>;
#[doc = "Field `H_BNAINTR1` reader - "]
pub type H_BNAINTR1_R = crate::BitReader;
#[doc = "Field `H_BNAINTR1` writer - "]
pub type H_BNAINTR1_W<'a, const O: u8> = crate::BitWriter<'a, HCINT1_SPEC, O>;
#[doc = "Field `H_XCS_XACT_ERR1` reader - "]
pub type H_XCS_XACT_ERR1_R = crate::BitReader;
#[doc = "Field `H_XCS_XACT_ERR1` writer - "]
pub type H_XCS_XACT_ERR1_W<'a, const O: u8> = crate::BitWriter<'a, HCINT1_SPEC, O>;
#[doc = "Field `H_DESC_LST_ROLLINTR1` reader - "]
pub type H_DESC_LST_ROLLINTR1_R = crate::BitReader;
#[doc = "Field `H_DESC_LST_ROLLINTR1` writer - "]
pub type H_DESC_LST_ROLLINTR1_W<'a, const O: u8> = crate::BitWriter<'a, HCINT1_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn h_xfercompl1(&self) -> H_XFERCOMPL1_R {
        H_XFERCOMPL1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn h_chhltd1(&self) -> H_CHHLTD1_R {
        H_CHHLTD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn h_ahberr1(&self) -> H_AHBERR1_R {
        H_AHBERR1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn h_stall1(&self) -> H_STALL1_R {
        H_STALL1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn h_nack1(&self) -> H_NACK1_R {
        H_NACK1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn h_ack1(&self) -> H_ACK1_R {
        H_ACK1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn h_nyet1(&self) -> H_NYET1_R {
        H_NYET1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn h_xacterr1(&self) -> H_XACTERR1_R {
        H_XACTERR1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn h_bblerr1(&self) -> H_BBLERR1_R {
        H_BBLERR1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn h_frmovrun1(&self) -> H_FRMOVRUN1_R {
        H_FRMOVRUN1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn h_datatglerr1(&self) -> H_DATATGLERR1_R {
        H_DATATGLERR1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn h_bnaintr1(&self) -> H_BNAINTR1_R {
        H_BNAINTR1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn h_xcs_xact_err1(&self) -> H_XCS_XACT_ERR1_R {
        H_XCS_XACT_ERR1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn h_desc_lst_rollintr1(&self) -> H_DESC_LST_ROLLINTR1_R {
        H_DESC_LST_ROLLINTR1_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINT1")
            .field(
                "h_xfercompl1",
                &format_args!("{}", self.h_xfercompl1().bit()),
            )
            .field("h_chhltd1", &format_args!("{}", self.h_chhltd1().bit()))
            .field("h_ahberr1", &format_args!("{}", self.h_ahberr1().bit()))
            .field("h_stall1", &format_args!("{}", self.h_stall1().bit()))
            .field("h_nack1", &format_args!("{}", self.h_nack1().bit()))
            .field("h_ack1", &format_args!("{}", self.h_ack1().bit()))
            .field("h_nyet1", &format_args!("{}", self.h_nyet1().bit()))
            .field("h_xacterr1", &format_args!("{}", self.h_xacterr1().bit()))
            .field("h_bblerr1", &format_args!("{}", self.h_bblerr1().bit()))
            .field("h_frmovrun1", &format_args!("{}", self.h_frmovrun1().bit()))
            .field(
                "h_datatglerr1",
                &format_args!("{}", self.h_datatglerr1().bit()),
            )
            .field("h_bnaintr1", &format_args!("{}", self.h_bnaintr1().bit()))
            .field(
                "h_xcs_xact_err1",
                &format_args!("{}", self.h_xcs_xact_err1().bit()),
            )
            .field(
                "h_desc_lst_rollintr1",
                &format_args!("{}", self.h_desc_lst_rollintr1().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCINT1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn h_xfercompl1(&mut self) -> H_XFERCOMPL1_W<0> {
        H_XFERCOMPL1_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn h_chhltd1(&mut self) -> H_CHHLTD1_W<1> {
        H_CHHLTD1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn h_ahberr1(&mut self) -> H_AHBERR1_W<2> {
        H_AHBERR1_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn h_stall1(&mut self) -> H_STALL1_W<3> {
        H_STALL1_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn h_nack1(&mut self) -> H_NACK1_W<4> {
        H_NACK1_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn h_ack1(&mut self) -> H_ACK1_W<5> {
        H_ACK1_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn h_nyet1(&mut self) -> H_NYET1_W<6> {
        H_NYET1_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn h_xacterr1(&mut self) -> H_XACTERR1_W<7> {
        H_XACTERR1_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn h_bblerr1(&mut self) -> H_BBLERR1_W<8> {
        H_BBLERR1_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn h_frmovrun1(&mut self) -> H_FRMOVRUN1_W<9> {
        H_FRMOVRUN1_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn h_datatglerr1(&mut self) -> H_DATATGLERR1_W<10> {
        H_DATATGLERR1_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn h_bnaintr1(&mut self) -> H_BNAINTR1_W<11> {
        H_BNAINTR1_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn h_xcs_xact_err1(&mut self) -> H_XCS_XACT_ERR1_W<12> {
        H_XCS_XACT_ERR1_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn h_desc_lst_rollintr1(&mut self) -> H_DESC_LST_ROLLINTR1_W<13> {
        H_DESC_LST_ROLLINTR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcint1](index.html) module"]
pub struct HCINT1_SPEC;
impl crate::RegisterSpec for HCINT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcint1::R](R) reader structure"]
impl crate::Readable for HCINT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcint1::W](W) writer structure"]
impl crate::Writable for HCINT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCINT1 to value 0"]
impl crate::Resettable for HCINT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
