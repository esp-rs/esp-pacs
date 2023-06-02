#[doc = "Register `HCINT0` reader"]
pub struct R(crate::R<HCINT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCINT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCINT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCINT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCINT0` writer"]
pub struct W(crate::W<HCINT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCINT0_SPEC>;
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
impl From<crate::W<HCINT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCINT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H_XFERCOMPL0` reader - "]
pub type H_XFERCOMPL0_R = crate::BitReader;
#[doc = "Field `H_XFERCOMPL0` writer - "]
pub type H_XFERCOMPL0_W<'a, const O: u8> = crate::BitWriter<'a, HCINT0_SPEC, O>;
#[doc = "Field `H_CHHLTD0` reader - "]
pub type H_CHHLTD0_R = crate::BitReader;
#[doc = "Field `H_CHHLTD0` writer - "]
pub type H_CHHLTD0_W<'a, const O: u8> = crate::BitWriter<'a, HCINT0_SPEC, O>;
#[doc = "Field `H_AHBERR0` reader - "]
pub type H_AHBERR0_R = crate::BitReader;
#[doc = "Field `H_AHBERR0` writer - "]
pub type H_AHBERR0_W<'a, const O: u8> = crate::BitWriter<'a, HCINT0_SPEC, O>;
#[doc = "Field `H_STALL0` reader - "]
pub type H_STALL0_R = crate::BitReader;
#[doc = "Field `H_STALL0` writer - "]
pub type H_STALL0_W<'a, const O: u8> = crate::BitWriter<'a, HCINT0_SPEC, O>;
#[doc = "Field `H_NACK0` reader - "]
pub type H_NACK0_R = crate::BitReader;
#[doc = "Field `H_NACK0` writer - "]
pub type H_NACK0_W<'a, const O: u8> = crate::BitWriter<'a, HCINT0_SPEC, O>;
#[doc = "Field `H_ACK0` reader - "]
pub type H_ACK0_R = crate::BitReader;
#[doc = "Field `H_ACK0` writer - "]
pub type H_ACK0_W<'a, const O: u8> = crate::BitWriter<'a, HCINT0_SPEC, O>;
#[doc = "Field `H_NYET0` reader - "]
pub type H_NYET0_R = crate::BitReader;
#[doc = "Field `H_NYET0` writer - "]
pub type H_NYET0_W<'a, const O: u8> = crate::BitWriter<'a, HCINT0_SPEC, O>;
#[doc = "Field `H_XACTERR0` reader - "]
pub type H_XACTERR0_R = crate::BitReader;
#[doc = "Field `H_XACTERR0` writer - "]
pub type H_XACTERR0_W<'a, const O: u8> = crate::BitWriter<'a, HCINT0_SPEC, O>;
#[doc = "Field `H_BBLERR0` reader - "]
pub type H_BBLERR0_R = crate::BitReader;
#[doc = "Field `H_BBLERR0` writer - "]
pub type H_BBLERR0_W<'a, const O: u8> = crate::BitWriter<'a, HCINT0_SPEC, O>;
#[doc = "Field `H_FRMOVRUN0` reader - "]
pub type H_FRMOVRUN0_R = crate::BitReader;
#[doc = "Field `H_FRMOVRUN0` writer - "]
pub type H_FRMOVRUN0_W<'a, const O: u8> = crate::BitWriter<'a, HCINT0_SPEC, O>;
#[doc = "Field `H_DATATGLERR0` reader - "]
pub type H_DATATGLERR0_R = crate::BitReader;
#[doc = "Field `H_DATATGLERR0` writer - "]
pub type H_DATATGLERR0_W<'a, const O: u8> = crate::BitWriter<'a, HCINT0_SPEC, O>;
#[doc = "Field `H_BNAINTR0` reader - "]
pub type H_BNAINTR0_R = crate::BitReader;
#[doc = "Field `H_BNAINTR0` writer - "]
pub type H_BNAINTR0_W<'a, const O: u8> = crate::BitWriter<'a, HCINT0_SPEC, O>;
#[doc = "Field `H_XCS_XACT_ERR0` reader - "]
pub type H_XCS_XACT_ERR0_R = crate::BitReader;
#[doc = "Field `H_XCS_XACT_ERR0` writer - "]
pub type H_XCS_XACT_ERR0_W<'a, const O: u8> = crate::BitWriter<'a, HCINT0_SPEC, O>;
#[doc = "Field `H_DESC_LST_ROLLINTR0` reader - "]
pub type H_DESC_LST_ROLLINTR0_R = crate::BitReader;
#[doc = "Field `H_DESC_LST_ROLLINTR0` writer - "]
pub type H_DESC_LST_ROLLINTR0_W<'a, const O: u8> = crate::BitWriter<'a, HCINT0_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn h_xfercompl0(&self) -> H_XFERCOMPL0_R {
        H_XFERCOMPL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn h_chhltd0(&self) -> H_CHHLTD0_R {
        H_CHHLTD0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn h_ahberr0(&self) -> H_AHBERR0_R {
        H_AHBERR0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn h_stall0(&self) -> H_STALL0_R {
        H_STALL0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn h_nack0(&self) -> H_NACK0_R {
        H_NACK0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn h_ack0(&self) -> H_ACK0_R {
        H_ACK0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn h_nyet0(&self) -> H_NYET0_R {
        H_NYET0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn h_xacterr0(&self) -> H_XACTERR0_R {
        H_XACTERR0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn h_bblerr0(&self) -> H_BBLERR0_R {
        H_BBLERR0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn h_frmovrun0(&self) -> H_FRMOVRUN0_R {
        H_FRMOVRUN0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn h_datatglerr0(&self) -> H_DATATGLERR0_R {
        H_DATATGLERR0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn h_bnaintr0(&self) -> H_BNAINTR0_R {
        H_BNAINTR0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn h_xcs_xact_err0(&self) -> H_XCS_XACT_ERR0_R {
        H_XCS_XACT_ERR0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn h_desc_lst_rollintr0(&self) -> H_DESC_LST_ROLLINTR0_R {
        H_DESC_LST_ROLLINTR0_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINT0")
            .field(
                "h_xfercompl0",
                &format_args!("{}", self.h_xfercompl0().bit()),
            )
            .field("h_chhltd0", &format_args!("{}", self.h_chhltd0().bit()))
            .field("h_ahberr0", &format_args!("{}", self.h_ahberr0().bit()))
            .field("h_stall0", &format_args!("{}", self.h_stall0().bit()))
            .field("h_nack0", &format_args!("{}", self.h_nack0().bit()))
            .field("h_ack0", &format_args!("{}", self.h_ack0().bit()))
            .field("h_nyet0", &format_args!("{}", self.h_nyet0().bit()))
            .field("h_xacterr0", &format_args!("{}", self.h_xacterr0().bit()))
            .field("h_bblerr0", &format_args!("{}", self.h_bblerr0().bit()))
            .field("h_frmovrun0", &format_args!("{}", self.h_frmovrun0().bit()))
            .field(
                "h_datatglerr0",
                &format_args!("{}", self.h_datatglerr0().bit()),
            )
            .field("h_bnaintr0", &format_args!("{}", self.h_bnaintr0().bit()))
            .field(
                "h_xcs_xact_err0",
                &format_args!("{}", self.h_xcs_xact_err0().bit()),
            )
            .field(
                "h_desc_lst_rollintr0",
                &format_args!("{}", self.h_desc_lst_rollintr0().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCINT0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn h_xfercompl0(&mut self) -> H_XFERCOMPL0_W<0> {
        H_XFERCOMPL0_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn h_chhltd0(&mut self) -> H_CHHLTD0_W<1> {
        H_CHHLTD0_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn h_ahberr0(&mut self) -> H_AHBERR0_W<2> {
        H_AHBERR0_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn h_stall0(&mut self) -> H_STALL0_W<3> {
        H_STALL0_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn h_nack0(&mut self) -> H_NACK0_W<4> {
        H_NACK0_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn h_ack0(&mut self) -> H_ACK0_W<5> {
        H_ACK0_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn h_nyet0(&mut self) -> H_NYET0_W<6> {
        H_NYET0_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn h_xacterr0(&mut self) -> H_XACTERR0_W<7> {
        H_XACTERR0_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn h_bblerr0(&mut self) -> H_BBLERR0_W<8> {
        H_BBLERR0_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn h_frmovrun0(&mut self) -> H_FRMOVRUN0_W<9> {
        H_FRMOVRUN0_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn h_datatglerr0(&mut self) -> H_DATATGLERR0_W<10> {
        H_DATATGLERR0_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn h_bnaintr0(&mut self) -> H_BNAINTR0_W<11> {
        H_BNAINTR0_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn h_xcs_xact_err0(&mut self) -> H_XCS_XACT_ERR0_W<12> {
        H_XCS_XACT_ERR0_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn h_desc_lst_rollintr0(&mut self) -> H_DESC_LST_ROLLINTR0_W<13> {
        H_DESC_LST_ROLLINTR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcint0](index.html) module"]
pub struct HCINT0_SPEC;
impl crate::RegisterSpec for HCINT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcint0::R](R) reader structure"]
impl crate::Readable for HCINT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcint0::W](W) writer structure"]
impl crate::Writable for HCINT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCINT0 to value 0"]
impl crate::Resettable for HCINT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
