#[doc = "Register `HCINTMSK3` reader"]
pub struct R(crate::R<HCINTMSK3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCINTMSK3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCINTMSK3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCINTMSK3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCINTMSK3` writer"]
pub struct W(crate::W<HCINTMSK3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCINTMSK3_SPEC>;
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
impl From<crate::W<HCINTMSK3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCINTMSK3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H_XFERCOMPLMSK3` reader - "]
pub type H_XFERCOMPLMSK3_R = crate::BitReader;
#[doc = "Field `H_XFERCOMPLMSK3` writer - "]
pub type H_XFERCOMPLMSK3_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK3_SPEC, O>;
#[doc = "Field `H_CHHLTDMSK3` reader - "]
pub type H_CHHLTDMSK3_R = crate::BitReader;
#[doc = "Field `H_CHHLTDMSK3` writer - "]
pub type H_CHHLTDMSK3_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK3_SPEC, O>;
#[doc = "Field `H_AHBERRMSK3` reader - "]
pub type H_AHBERRMSK3_R = crate::BitReader;
#[doc = "Field `H_AHBERRMSK3` writer - "]
pub type H_AHBERRMSK3_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK3_SPEC, O>;
#[doc = "Field `H_STALLMSK3` reader - "]
pub type H_STALLMSK3_R = crate::BitReader;
#[doc = "Field `H_STALLMSK3` writer - "]
pub type H_STALLMSK3_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK3_SPEC, O>;
#[doc = "Field `H_NAKMSK3` reader - "]
pub type H_NAKMSK3_R = crate::BitReader;
#[doc = "Field `H_NAKMSK3` writer - "]
pub type H_NAKMSK3_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK3_SPEC, O>;
#[doc = "Field `H_ACKMSK3` reader - "]
pub type H_ACKMSK3_R = crate::BitReader;
#[doc = "Field `H_ACKMSK3` writer - "]
pub type H_ACKMSK3_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK3_SPEC, O>;
#[doc = "Field `H_NYETMSK3` reader - "]
pub type H_NYETMSK3_R = crate::BitReader;
#[doc = "Field `H_NYETMSK3` writer - "]
pub type H_NYETMSK3_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK3_SPEC, O>;
#[doc = "Field `H_XACTERRMSK3` reader - "]
pub type H_XACTERRMSK3_R = crate::BitReader;
#[doc = "Field `H_XACTERRMSK3` writer - "]
pub type H_XACTERRMSK3_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK3_SPEC, O>;
#[doc = "Field `H_BBLERRMSK3` reader - "]
pub type H_BBLERRMSK3_R = crate::BitReader;
#[doc = "Field `H_BBLERRMSK3` writer - "]
pub type H_BBLERRMSK3_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK3_SPEC, O>;
#[doc = "Field `H_FRMOVRUNMSK3` reader - "]
pub type H_FRMOVRUNMSK3_R = crate::BitReader;
#[doc = "Field `H_FRMOVRUNMSK3` writer - "]
pub type H_FRMOVRUNMSK3_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK3_SPEC, O>;
#[doc = "Field `H_DATATGLERRMSK3` reader - "]
pub type H_DATATGLERRMSK3_R = crate::BitReader;
#[doc = "Field `H_DATATGLERRMSK3` writer - "]
pub type H_DATATGLERRMSK3_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK3_SPEC, O>;
#[doc = "Field `H_BNAINTRMSK3` reader - "]
pub type H_BNAINTRMSK3_R = crate::BitReader;
#[doc = "Field `H_BNAINTRMSK3` writer - "]
pub type H_BNAINTRMSK3_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK3_SPEC, O>;
#[doc = "Field `H_DESC_LST_ROLLINTRMSK3` reader - "]
pub type H_DESC_LST_ROLLINTRMSK3_R = crate::BitReader;
#[doc = "Field `H_DESC_LST_ROLLINTRMSK3` writer - "]
pub type H_DESC_LST_ROLLINTRMSK3_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK3_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn h_xfercomplmsk3(&self) -> H_XFERCOMPLMSK3_R {
        H_XFERCOMPLMSK3_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn h_chhltdmsk3(&self) -> H_CHHLTDMSK3_R {
        H_CHHLTDMSK3_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn h_ahberrmsk3(&self) -> H_AHBERRMSK3_R {
        H_AHBERRMSK3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn h_stallmsk3(&self) -> H_STALLMSK3_R {
        H_STALLMSK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn h_nakmsk3(&self) -> H_NAKMSK3_R {
        H_NAKMSK3_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn h_ackmsk3(&self) -> H_ACKMSK3_R {
        H_ACKMSK3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn h_nyetmsk3(&self) -> H_NYETMSK3_R {
        H_NYETMSK3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn h_xacterrmsk3(&self) -> H_XACTERRMSK3_R {
        H_XACTERRMSK3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn h_bblerrmsk3(&self) -> H_BBLERRMSK3_R {
        H_BBLERRMSK3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn h_frmovrunmsk3(&self) -> H_FRMOVRUNMSK3_R {
        H_FRMOVRUNMSK3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn h_datatglerrmsk3(&self) -> H_DATATGLERRMSK3_R {
        H_DATATGLERRMSK3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn h_bnaintrmsk3(&self) -> H_BNAINTRMSK3_R {
        H_BNAINTRMSK3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn h_desc_lst_rollintrmsk3(&self) -> H_DESC_LST_ROLLINTRMSK3_R {
        H_DESC_LST_ROLLINTRMSK3_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINTMSK3")
            .field(
                "h_xfercomplmsk3",
                &format_args!("{}", self.h_xfercomplmsk3().bit()),
            )
            .field(
                "h_chhltdmsk3",
                &format_args!("{}", self.h_chhltdmsk3().bit()),
            )
            .field(
                "h_ahberrmsk3",
                &format_args!("{}", self.h_ahberrmsk3().bit()),
            )
            .field("h_stallmsk3", &format_args!("{}", self.h_stallmsk3().bit()))
            .field("h_nakmsk3", &format_args!("{}", self.h_nakmsk3().bit()))
            .field("h_ackmsk3", &format_args!("{}", self.h_ackmsk3().bit()))
            .field("h_nyetmsk3", &format_args!("{}", self.h_nyetmsk3().bit()))
            .field(
                "h_xacterrmsk3",
                &format_args!("{}", self.h_xacterrmsk3().bit()),
            )
            .field(
                "h_bblerrmsk3",
                &format_args!("{}", self.h_bblerrmsk3().bit()),
            )
            .field(
                "h_frmovrunmsk3",
                &format_args!("{}", self.h_frmovrunmsk3().bit()),
            )
            .field(
                "h_datatglerrmsk3",
                &format_args!("{}", self.h_datatglerrmsk3().bit()),
            )
            .field(
                "h_bnaintrmsk3",
                &format_args!("{}", self.h_bnaintrmsk3().bit()),
            )
            .field(
                "h_desc_lst_rollintrmsk3",
                &format_args!("{}", self.h_desc_lst_rollintrmsk3().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCINTMSK3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn h_xfercomplmsk3(&mut self) -> H_XFERCOMPLMSK3_W<0> {
        H_XFERCOMPLMSK3_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn h_chhltdmsk3(&mut self) -> H_CHHLTDMSK3_W<1> {
        H_CHHLTDMSK3_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn h_ahberrmsk3(&mut self) -> H_AHBERRMSK3_W<2> {
        H_AHBERRMSK3_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn h_stallmsk3(&mut self) -> H_STALLMSK3_W<3> {
        H_STALLMSK3_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn h_nakmsk3(&mut self) -> H_NAKMSK3_W<4> {
        H_NAKMSK3_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn h_ackmsk3(&mut self) -> H_ACKMSK3_W<5> {
        H_ACKMSK3_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn h_nyetmsk3(&mut self) -> H_NYETMSK3_W<6> {
        H_NYETMSK3_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn h_xacterrmsk3(&mut self) -> H_XACTERRMSK3_W<7> {
        H_XACTERRMSK3_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn h_bblerrmsk3(&mut self) -> H_BBLERRMSK3_W<8> {
        H_BBLERRMSK3_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn h_frmovrunmsk3(&mut self) -> H_FRMOVRUNMSK3_W<9> {
        H_FRMOVRUNMSK3_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn h_datatglerrmsk3(&mut self) -> H_DATATGLERRMSK3_W<10> {
        H_DATATGLERRMSK3_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn h_bnaintrmsk3(&mut self) -> H_BNAINTRMSK3_W<11> {
        H_BNAINTRMSK3_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn h_desc_lst_rollintrmsk3(&mut self) -> H_DESC_LST_ROLLINTRMSK3_W<13> {
        H_DESC_LST_ROLLINTRMSK3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcintmsk3](index.html) module"]
pub struct HCINTMSK3_SPEC;
impl crate::RegisterSpec for HCINTMSK3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcintmsk3::R](R) reader structure"]
impl crate::Readable for HCINTMSK3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcintmsk3::W](W) writer structure"]
impl crate::Writable for HCINTMSK3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCINTMSK3 to value 0"]
impl crate::Resettable for HCINTMSK3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
