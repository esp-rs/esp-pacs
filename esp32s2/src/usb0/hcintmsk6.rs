#[doc = "Register `HCINTMSK6` reader"]
pub struct R(crate::R<HCINTMSK6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCINTMSK6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCINTMSK6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCINTMSK6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCINTMSK6` writer"]
pub struct W(crate::W<HCINTMSK6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCINTMSK6_SPEC>;
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
impl From<crate::W<HCINTMSK6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCINTMSK6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H_XFERCOMPLMSK6` reader - "]
pub type H_XFERCOMPLMSK6_R = crate::BitReader;
#[doc = "Field `H_XFERCOMPLMSK6` writer - "]
pub type H_XFERCOMPLMSK6_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK6_SPEC, O>;
#[doc = "Field `H_CHHLTDMSK6` reader - "]
pub type H_CHHLTDMSK6_R = crate::BitReader;
#[doc = "Field `H_CHHLTDMSK6` writer - "]
pub type H_CHHLTDMSK6_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK6_SPEC, O>;
#[doc = "Field `H_AHBERRMSK6` reader - "]
pub type H_AHBERRMSK6_R = crate::BitReader;
#[doc = "Field `H_AHBERRMSK6` writer - "]
pub type H_AHBERRMSK6_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK6_SPEC, O>;
#[doc = "Field `H_STALLMSK6` reader - "]
pub type H_STALLMSK6_R = crate::BitReader;
#[doc = "Field `H_STALLMSK6` writer - "]
pub type H_STALLMSK6_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK6_SPEC, O>;
#[doc = "Field `H_NAKMSK6` reader - "]
pub type H_NAKMSK6_R = crate::BitReader;
#[doc = "Field `H_NAKMSK6` writer - "]
pub type H_NAKMSK6_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK6_SPEC, O>;
#[doc = "Field `H_ACKMSK6` reader - "]
pub type H_ACKMSK6_R = crate::BitReader;
#[doc = "Field `H_ACKMSK6` writer - "]
pub type H_ACKMSK6_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK6_SPEC, O>;
#[doc = "Field `H_NYETMSK6` reader - "]
pub type H_NYETMSK6_R = crate::BitReader;
#[doc = "Field `H_NYETMSK6` writer - "]
pub type H_NYETMSK6_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK6_SPEC, O>;
#[doc = "Field `H_XACTERRMSK6` reader - "]
pub type H_XACTERRMSK6_R = crate::BitReader;
#[doc = "Field `H_XACTERRMSK6` writer - "]
pub type H_XACTERRMSK6_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK6_SPEC, O>;
#[doc = "Field `H_BBLERRMSK6` reader - "]
pub type H_BBLERRMSK6_R = crate::BitReader;
#[doc = "Field `H_BBLERRMSK6` writer - "]
pub type H_BBLERRMSK6_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK6_SPEC, O>;
#[doc = "Field `H_FRMOVRUNMSK6` reader - "]
pub type H_FRMOVRUNMSK6_R = crate::BitReader;
#[doc = "Field `H_FRMOVRUNMSK6` writer - "]
pub type H_FRMOVRUNMSK6_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK6_SPEC, O>;
#[doc = "Field `H_DATATGLERRMSK6` reader - "]
pub type H_DATATGLERRMSK6_R = crate::BitReader;
#[doc = "Field `H_DATATGLERRMSK6` writer - "]
pub type H_DATATGLERRMSK6_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK6_SPEC, O>;
#[doc = "Field `H_BNAINTRMSK6` reader - "]
pub type H_BNAINTRMSK6_R = crate::BitReader;
#[doc = "Field `H_BNAINTRMSK6` writer - "]
pub type H_BNAINTRMSK6_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK6_SPEC, O>;
#[doc = "Field `H_DESC_LST_ROLLINTRMSK6` reader - "]
pub type H_DESC_LST_ROLLINTRMSK6_R = crate::BitReader;
#[doc = "Field `H_DESC_LST_ROLLINTRMSK6` writer - "]
pub type H_DESC_LST_ROLLINTRMSK6_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK6_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn h_xfercomplmsk6(&self) -> H_XFERCOMPLMSK6_R {
        H_XFERCOMPLMSK6_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn h_chhltdmsk6(&self) -> H_CHHLTDMSK6_R {
        H_CHHLTDMSK6_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn h_ahberrmsk6(&self) -> H_AHBERRMSK6_R {
        H_AHBERRMSK6_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn h_stallmsk6(&self) -> H_STALLMSK6_R {
        H_STALLMSK6_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn h_nakmsk6(&self) -> H_NAKMSK6_R {
        H_NAKMSK6_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn h_ackmsk6(&self) -> H_ACKMSK6_R {
        H_ACKMSK6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn h_nyetmsk6(&self) -> H_NYETMSK6_R {
        H_NYETMSK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn h_xacterrmsk6(&self) -> H_XACTERRMSK6_R {
        H_XACTERRMSK6_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn h_bblerrmsk6(&self) -> H_BBLERRMSK6_R {
        H_BBLERRMSK6_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn h_frmovrunmsk6(&self) -> H_FRMOVRUNMSK6_R {
        H_FRMOVRUNMSK6_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn h_datatglerrmsk6(&self) -> H_DATATGLERRMSK6_R {
        H_DATATGLERRMSK6_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn h_bnaintrmsk6(&self) -> H_BNAINTRMSK6_R {
        H_BNAINTRMSK6_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn h_desc_lst_rollintrmsk6(&self) -> H_DESC_LST_ROLLINTRMSK6_R {
        H_DESC_LST_ROLLINTRMSK6_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINTMSK6")
            .field(
                "h_xfercomplmsk6",
                &format_args!("{}", self.h_xfercomplmsk6().bit()),
            )
            .field(
                "h_chhltdmsk6",
                &format_args!("{}", self.h_chhltdmsk6().bit()),
            )
            .field(
                "h_ahberrmsk6",
                &format_args!("{}", self.h_ahberrmsk6().bit()),
            )
            .field("h_stallmsk6", &format_args!("{}", self.h_stallmsk6().bit()))
            .field("h_nakmsk6", &format_args!("{}", self.h_nakmsk6().bit()))
            .field("h_ackmsk6", &format_args!("{}", self.h_ackmsk6().bit()))
            .field("h_nyetmsk6", &format_args!("{}", self.h_nyetmsk6().bit()))
            .field(
                "h_xacterrmsk6",
                &format_args!("{}", self.h_xacterrmsk6().bit()),
            )
            .field(
                "h_bblerrmsk6",
                &format_args!("{}", self.h_bblerrmsk6().bit()),
            )
            .field(
                "h_frmovrunmsk6",
                &format_args!("{}", self.h_frmovrunmsk6().bit()),
            )
            .field(
                "h_datatglerrmsk6",
                &format_args!("{}", self.h_datatglerrmsk6().bit()),
            )
            .field(
                "h_bnaintrmsk6",
                &format_args!("{}", self.h_bnaintrmsk6().bit()),
            )
            .field(
                "h_desc_lst_rollintrmsk6",
                &format_args!("{}", self.h_desc_lst_rollintrmsk6().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCINTMSK6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn h_xfercomplmsk6(&mut self) -> H_XFERCOMPLMSK6_W<0> {
        H_XFERCOMPLMSK6_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn h_chhltdmsk6(&mut self) -> H_CHHLTDMSK6_W<1> {
        H_CHHLTDMSK6_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn h_ahberrmsk6(&mut self) -> H_AHBERRMSK6_W<2> {
        H_AHBERRMSK6_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn h_stallmsk6(&mut self) -> H_STALLMSK6_W<3> {
        H_STALLMSK6_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn h_nakmsk6(&mut self) -> H_NAKMSK6_W<4> {
        H_NAKMSK6_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn h_ackmsk6(&mut self) -> H_ACKMSK6_W<5> {
        H_ACKMSK6_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn h_nyetmsk6(&mut self) -> H_NYETMSK6_W<6> {
        H_NYETMSK6_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn h_xacterrmsk6(&mut self) -> H_XACTERRMSK6_W<7> {
        H_XACTERRMSK6_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn h_bblerrmsk6(&mut self) -> H_BBLERRMSK6_W<8> {
        H_BBLERRMSK6_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn h_frmovrunmsk6(&mut self) -> H_FRMOVRUNMSK6_W<9> {
        H_FRMOVRUNMSK6_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn h_datatglerrmsk6(&mut self) -> H_DATATGLERRMSK6_W<10> {
        H_DATATGLERRMSK6_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn h_bnaintrmsk6(&mut self) -> H_BNAINTRMSK6_W<11> {
        H_BNAINTRMSK6_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn h_desc_lst_rollintrmsk6(&mut self) -> H_DESC_LST_ROLLINTRMSK6_W<13> {
        H_DESC_LST_ROLLINTRMSK6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcintmsk6](index.html) module"]
pub struct HCINTMSK6_SPEC;
impl crate::RegisterSpec for HCINTMSK6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcintmsk6::R](R) reader structure"]
impl crate::Readable for HCINTMSK6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcintmsk6::W](W) writer structure"]
impl crate::Writable for HCINTMSK6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCINTMSK6 to value 0"]
impl crate::Resettable for HCINTMSK6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
