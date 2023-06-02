#[doc = "Register `HCINTMSK1` reader"]
pub struct R(crate::R<HCINTMSK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCINTMSK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCINTMSK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCINTMSK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCINTMSK1` writer"]
pub struct W(crate::W<HCINTMSK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCINTMSK1_SPEC>;
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
impl From<crate::W<HCINTMSK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCINTMSK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H_XFERCOMPLMSK1` reader - "]
pub type H_XFERCOMPLMSK1_R = crate::BitReader;
#[doc = "Field `H_XFERCOMPLMSK1` writer - "]
pub type H_XFERCOMPLMSK1_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK1_SPEC, O>;
#[doc = "Field `H_CHHLTDMSK1` reader - "]
pub type H_CHHLTDMSK1_R = crate::BitReader;
#[doc = "Field `H_CHHLTDMSK1` writer - "]
pub type H_CHHLTDMSK1_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK1_SPEC, O>;
#[doc = "Field `H_AHBERRMSK1` reader - "]
pub type H_AHBERRMSK1_R = crate::BitReader;
#[doc = "Field `H_AHBERRMSK1` writer - "]
pub type H_AHBERRMSK1_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK1_SPEC, O>;
#[doc = "Field `H_STALLMSK1` reader - "]
pub type H_STALLMSK1_R = crate::BitReader;
#[doc = "Field `H_STALLMSK1` writer - "]
pub type H_STALLMSK1_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK1_SPEC, O>;
#[doc = "Field `H_NAKMSK1` reader - "]
pub type H_NAKMSK1_R = crate::BitReader;
#[doc = "Field `H_NAKMSK1` writer - "]
pub type H_NAKMSK1_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK1_SPEC, O>;
#[doc = "Field `H_ACKMSK1` reader - "]
pub type H_ACKMSK1_R = crate::BitReader;
#[doc = "Field `H_ACKMSK1` writer - "]
pub type H_ACKMSK1_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK1_SPEC, O>;
#[doc = "Field `H_NYETMSK1` reader - "]
pub type H_NYETMSK1_R = crate::BitReader;
#[doc = "Field `H_NYETMSK1` writer - "]
pub type H_NYETMSK1_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK1_SPEC, O>;
#[doc = "Field `H_XACTERRMSK1` reader - "]
pub type H_XACTERRMSK1_R = crate::BitReader;
#[doc = "Field `H_XACTERRMSK1` writer - "]
pub type H_XACTERRMSK1_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK1_SPEC, O>;
#[doc = "Field `H_BBLERRMSK1` reader - "]
pub type H_BBLERRMSK1_R = crate::BitReader;
#[doc = "Field `H_BBLERRMSK1` writer - "]
pub type H_BBLERRMSK1_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK1_SPEC, O>;
#[doc = "Field `H_FRMOVRUNMSK1` reader - "]
pub type H_FRMOVRUNMSK1_R = crate::BitReader;
#[doc = "Field `H_FRMOVRUNMSK1` writer - "]
pub type H_FRMOVRUNMSK1_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK1_SPEC, O>;
#[doc = "Field `H_DATATGLERRMSK1` reader - "]
pub type H_DATATGLERRMSK1_R = crate::BitReader;
#[doc = "Field `H_DATATGLERRMSK1` writer - "]
pub type H_DATATGLERRMSK1_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK1_SPEC, O>;
#[doc = "Field `H_BNAINTRMSK1` reader - "]
pub type H_BNAINTRMSK1_R = crate::BitReader;
#[doc = "Field `H_BNAINTRMSK1` writer - "]
pub type H_BNAINTRMSK1_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK1_SPEC, O>;
#[doc = "Field `H_DESC_LST_ROLLINTRMSK1` reader - "]
pub type H_DESC_LST_ROLLINTRMSK1_R = crate::BitReader;
#[doc = "Field `H_DESC_LST_ROLLINTRMSK1` writer - "]
pub type H_DESC_LST_ROLLINTRMSK1_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK1_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn h_xfercomplmsk1(&self) -> H_XFERCOMPLMSK1_R {
        H_XFERCOMPLMSK1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn h_chhltdmsk1(&self) -> H_CHHLTDMSK1_R {
        H_CHHLTDMSK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn h_ahberrmsk1(&self) -> H_AHBERRMSK1_R {
        H_AHBERRMSK1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn h_stallmsk1(&self) -> H_STALLMSK1_R {
        H_STALLMSK1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn h_nakmsk1(&self) -> H_NAKMSK1_R {
        H_NAKMSK1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn h_ackmsk1(&self) -> H_ACKMSK1_R {
        H_ACKMSK1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn h_nyetmsk1(&self) -> H_NYETMSK1_R {
        H_NYETMSK1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn h_xacterrmsk1(&self) -> H_XACTERRMSK1_R {
        H_XACTERRMSK1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn h_bblerrmsk1(&self) -> H_BBLERRMSK1_R {
        H_BBLERRMSK1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn h_frmovrunmsk1(&self) -> H_FRMOVRUNMSK1_R {
        H_FRMOVRUNMSK1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn h_datatglerrmsk1(&self) -> H_DATATGLERRMSK1_R {
        H_DATATGLERRMSK1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn h_bnaintrmsk1(&self) -> H_BNAINTRMSK1_R {
        H_BNAINTRMSK1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn h_desc_lst_rollintrmsk1(&self) -> H_DESC_LST_ROLLINTRMSK1_R {
        H_DESC_LST_ROLLINTRMSK1_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINTMSK1")
            .field(
                "h_xfercomplmsk1",
                &format_args!("{}", self.h_xfercomplmsk1().bit()),
            )
            .field(
                "h_chhltdmsk1",
                &format_args!("{}", self.h_chhltdmsk1().bit()),
            )
            .field(
                "h_ahberrmsk1",
                &format_args!("{}", self.h_ahberrmsk1().bit()),
            )
            .field("h_stallmsk1", &format_args!("{}", self.h_stallmsk1().bit()))
            .field("h_nakmsk1", &format_args!("{}", self.h_nakmsk1().bit()))
            .field("h_ackmsk1", &format_args!("{}", self.h_ackmsk1().bit()))
            .field("h_nyetmsk1", &format_args!("{}", self.h_nyetmsk1().bit()))
            .field(
                "h_xacterrmsk1",
                &format_args!("{}", self.h_xacterrmsk1().bit()),
            )
            .field(
                "h_bblerrmsk1",
                &format_args!("{}", self.h_bblerrmsk1().bit()),
            )
            .field(
                "h_frmovrunmsk1",
                &format_args!("{}", self.h_frmovrunmsk1().bit()),
            )
            .field(
                "h_datatglerrmsk1",
                &format_args!("{}", self.h_datatglerrmsk1().bit()),
            )
            .field(
                "h_bnaintrmsk1",
                &format_args!("{}", self.h_bnaintrmsk1().bit()),
            )
            .field(
                "h_desc_lst_rollintrmsk1",
                &format_args!("{}", self.h_desc_lst_rollintrmsk1().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCINTMSK1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn h_xfercomplmsk1(&mut self) -> H_XFERCOMPLMSK1_W<0> {
        H_XFERCOMPLMSK1_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn h_chhltdmsk1(&mut self) -> H_CHHLTDMSK1_W<1> {
        H_CHHLTDMSK1_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn h_ahberrmsk1(&mut self) -> H_AHBERRMSK1_W<2> {
        H_AHBERRMSK1_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn h_stallmsk1(&mut self) -> H_STALLMSK1_W<3> {
        H_STALLMSK1_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn h_nakmsk1(&mut self) -> H_NAKMSK1_W<4> {
        H_NAKMSK1_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn h_ackmsk1(&mut self) -> H_ACKMSK1_W<5> {
        H_ACKMSK1_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn h_nyetmsk1(&mut self) -> H_NYETMSK1_W<6> {
        H_NYETMSK1_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn h_xacterrmsk1(&mut self) -> H_XACTERRMSK1_W<7> {
        H_XACTERRMSK1_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn h_bblerrmsk1(&mut self) -> H_BBLERRMSK1_W<8> {
        H_BBLERRMSK1_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn h_frmovrunmsk1(&mut self) -> H_FRMOVRUNMSK1_W<9> {
        H_FRMOVRUNMSK1_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn h_datatglerrmsk1(&mut self) -> H_DATATGLERRMSK1_W<10> {
        H_DATATGLERRMSK1_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn h_bnaintrmsk1(&mut self) -> H_BNAINTRMSK1_W<11> {
        H_BNAINTRMSK1_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn h_desc_lst_rollintrmsk1(&mut self) -> H_DESC_LST_ROLLINTRMSK1_W<13> {
        H_DESC_LST_ROLLINTRMSK1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcintmsk1](index.html) module"]
pub struct HCINTMSK1_SPEC;
impl crate::RegisterSpec for HCINTMSK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcintmsk1::R](R) reader structure"]
impl crate::Readable for HCINTMSK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcintmsk1::W](W) writer structure"]
impl crate::Writable for HCINTMSK1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCINTMSK1 to value 0"]
impl crate::Resettable for HCINTMSK1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
