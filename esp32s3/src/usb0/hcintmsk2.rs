#[doc = "Register `HCINTMSK2` reader"]
pub struct R(crate::R<HCINTMSK2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCINTMSK2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCINTMSK2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCINTMSK2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCINTMSK2` writer"]
pub struct W(crate::W<HCINTMSK2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCINTMSK2_SPEC>;
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
impl From<crate::W<HCINTMSK2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCINTMSK2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H_XFERCOMPLMSK2` reader - "]
pub type H_XFERCOMPLMSK2_R = crate::BitReader;
#[doc = "Field `H_XFERCOMPLMSK2` writer - "]
pub type H_XFERCOMPLMSK2_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK2_SPEC, O>;
#[doc = "Field `H_CHHLTDMSK2` reader - "]
pub type H_CHHLTDMSK2_R = crate::BitReader;
#[doc = "Field `H_CHHLTDMSK2` writer - "]
pub type H_CHHLTDMSK2_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK2_SPEC, O>;
#[doc = "Field `H_AHBERRMSK2` reader - "]
pub type H_AHBERRMSK2_R = crate::BitReader;
#[doc = "Field `H_AHBERRMSK2` writer - "]
pub type H_AHBERRMSK2_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK2_SPEC, O>;
#[doc = "Field `H_STALLMSK2` reader - "]
pub type H_STALLMSK2_R = crate::BitReader;
#[doc = "Field `H_STALLMSK2` writer - "]
pub type H_STALLMSK2_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK2_SPEC, O>;
#[doc = "Field `H_NAKMSK2` reader - "]
pub type H_NAKMSK2_R = crate::BitReader;
#[doc = "Field `H_NAKMSK2` writer - "]
pub type H_NAKMSK2_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK2_SPEC, O>;
#[doc = "Field `H_ACKMSK2` reader - "]
pub type H_ACKMSK2_R = crate::BitReader;
#[doc = "Field `H_ACKMSK2` writer - "]
pub type H_ACKMSK2_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK2_SPEC, O>;
#[doc = "Field `H_NYETMSK2` reader - "]
pub type H_NYETMSK2_R = crate::BitReader;
#[doc = "Field `H_NYETMSK2` writer - "]
pub type H_NYETMSK2_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK2_SPEC, O>;
#[doc = "Field `H_XACTERRMSK2` reader - "]
pub type H_XACTERRMSK2_R = crate::BitReader;
#[doc = "Field `H_XACTERRMSK2` writer - "]
pub type H_XACTERRMSK2_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK2_SPEC, O>;
#[doc = "Field `H_BBLERRMSK2` reader - "]
pub type H_BBLERRMSK2_R = crate::BitReader;
#[doc = "Field `H_BBLERRMSK2` writer - "]
pub type H_BBLERRMSK2_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK2_SPEC, O>;
#[doc = "Field `H_FRMOVRUNMSK2` reader - "]
pub type H_FRMOVRUNMSK2_R = crate::BitReader;
#[doc = "Field `H_FRMOVRUNMSK2` writer - "]
pub type H_FRMOVRUNMSK2_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK2_SPEC, O>;
#[doc = "Field `H_DATATGLERRMSK2` reader - "]
pub type H_DATATGLERRMSK2_R = crate::BitReader;
#[doc = "Field `H_DATATGLERRMSK2` writer - "]
pub type H_DATATGLERRMSK2_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK2_SPEC, O>;
#[doc = "Field `H_BNAINTRMSK2` reader - "]
pub type H_BNAINTRMSK2_R = crate::BitReader;
#[doc = "Field `H_BNAINTRMSK2` writer - "]
pub type H_BNAINTRMSK2_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK2_SPEC, O>;
#[doc = "Field `H_DESC_LST_ROLLINTRMSK2` reader - "]
pub type H_DESC_LST_ROLLINTRMSK2_R = crate::BitReader;
#[doc = "Field `H_DESC_LST_ROLLINTRMSK2` writer - "]
pub type H_DESC_LST_ROLLINTRMSK2_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK2_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn h_xfercomplmsk2(&self) -> H_XFERCOMPLMSK2_R {
        H_XFERCOMPLMSK2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn h_chhltdmsk2(&self) -> H_CHHLTDMSK2_R {
        H_CHHLTDMSK2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn h_ahberrmsk2(&self) -> H_AHBERRMSK2_R {
        H_AHBERRMSK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn h_stallmsk2(&self) -> H_STALLMSK2_R {
        H_STALLMSK2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn h_nakmsk2(&self) -> H_NAKMSK2_R {
        H_NAKMSK2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn h_ackmsk2(&self) -> H_ACKMSK2_R {
        H_ACKMSK2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn h_nyetmsk2(&self) -> H_NYETMSK2_R {
        H_NYETMSK2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn h_xacterrmsk2(&self) -> H_XACTERRMSK2_R {
        H_XACTERRMSK2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn h_bblerrmsk2(&self) -> H_BBLERRMSK2_R {
        H_BBLERRMSK2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn h_frmovrunmsk2(&self) -> H_FRMOVRUNMSK2_R {
        H_FRMOVRUNMSK2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn h_datatglerrmsk2(&self) -> H_DATATGLERRMSK2_R {
        H_DATATGLERRMSK2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn h_bnaintrmsk2(&self) -> H_BNAINTRMSK2_R {
        H_BNAINTRMSK2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn h_desc_lst_rollintrmsk2(&self) -> H_DESC_LST_ROLLINTRMSK2_R {
        H_DESC_LST_ROLLINTRMSK2_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINTMSK2")
            .field(
                "h_xfercomplmsk2",
                &format_args!("{}", self.h_xfercomplmsk2().bit()),
            )
            .field(
                "h_chhltdmsk2",
                &format_args!("{}", self.h_chhltdmsk2().bit()),
            )
            .field(
                "h_ahberrmsk2",
                &format_args!("{}", self.h_ahberrmsk2().bit()),
            )
            .field("h_stallmsk2", &format_args!("{}", self.h_stallmsk2().bit()))
            .field("h_nakmsk2", &format_args!("{}", self.h_nakmsk2().bit()))
            .field("h_ackmsk2", &format_args!("{}", self.h_ackmsk2().bit()))
            .field("h_nyetmsk2", &format_args!("{}", self.h_nyetmsk2().bit()))
            .field(
                "h_xacterrmsk2",
                &format_args!("{}", self.h_xacterrmsk2().bit()),
            )
            .field(
                "h_bblerrmsk2",
                &format_args!("{}", self.h_bblerrmsk2().bit()),
            )
            .field(
                "h_frmovrunmsk2",
                &format_args!("{}", self.h_frmovrunmsk2().bit()),
            )
            .field(
                "h_datatglerrmsk2",
                &format_args!("{}", self.h_datatglerrmsk2().bit()),
            )
            .field(
                "h_bnaintrmsk2",
                &format_args!("{}", self.h_bnaintrmsk2().bit()),
            )
            .field(
                "h_desc_lst_rollintrmsk2",
                &format_args!("{}", self.h_desc_lst_rollintrmsk2().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCINTMSK2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn h_xfercomplmsk2(&mut self) -> H_XFERCOMPLMSK2_W<0> {
        H_XFERCOMPLMSK2_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn h_chhltdmsk2(&mut self) -> H_CHHLTDMSK2_W<1> {
        H_CHHLTDMSK2_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn h_ahberrmsk2(&mut self) -> H_AHBERRMSK2_W<2> {
        H_AHBERRMSK2_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn h_stallmsk2(&mut self) -> H_STALLMSK2_W<3> {
        H_STALLMSK2_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn h_nakmsk2(&mut self) -> H_NAKMSK2_W<4> {
        H_NAKMSK2_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn h_ackmsk2(&mut self) -> H_ACKMSK2_W<5> {
        H_ACKMSK2_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn h_nyetmsk2(&mut self) -> H_NYETMSK2_W<6> {
        H_NYETMSK2_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn h_xacterrmsk2(&mut self) -> H_XACTERRMSK2_W<7> {
        H_XACTERRMSK2_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn h_bblerrmsk2(&mut self) -> H_BBLERRMSK2_W<8> {
        H_BBLERRMSK2_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn h_frmovrunmsk2(&mut self) -> H_FRMOVRUNMSK2_W<9> {
        H_FRMOVRUNMSK2_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn h_datatglerrmsk2(&mut self) -> H_DATATGLERRMSK2_W<10> {
        H_DATATGLERRMSK2_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn h_bnaintrmsk2(&mut self) -> H_BNAINTRMSK2_W<11> {
        H_BNAINTRMSK2_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn h_desc_lst_rollintrmsk2(&mut self) -> H_DESC_LST_ROLLINTRMSK2_W<13> {
        H_DESC_LST_ROLLINTRMSK2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcintmsk2](index.html) module"]
pub struct HCINTMSK2_SPEC;
impl crate::RegisterSpec for HCINTMSK2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcintmsk2::R](R) reader structure"]
impl crate::Readable for HCINTMSK2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcintmsk2::W](W) writer structure"]
impl crate::Writable for HCINTMSK2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCINTMSK2 to value 0"]
impl crate::Resettable for HCINTMSK2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
