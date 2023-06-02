#[doc = "Register `HCINTMSK5` reader"]
pub struct R(crate::R<HCINTMSK5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCINTMSK5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCINTMSK5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCINTMSK5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCINTMSK5` writer"]
pub struct W(crate::W<HCINTMSK5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCINTMSK5_SPEC>;
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
impl From<crate::W<HCINTMSK5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCINTMSK5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H_XFERCOMPLMSK5` reader - "]
pub type H_XFERCOMPLMSK5_R = crate::BitReader;
#[doc = "Field `H_XFERCOMPLMSK5` writer - "]
pub type H_XFERCOMPLMSK5_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK5_SPEC, O>;
#[doc = "Field `H_CHHLTDMSK5` reader - "]
pub type H_CHHLTDMSK5_R = crate::BitReader;
#[doc = "Field `H_CHHLTDMSK5` writer - "]
pub type H_CHHLTDMSK5_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK5_SPEC, O>;
#[doc = "Field `H_AHBERRMSK5` reader - "]
pub type H_AHBERRMSK5_R = crate::BitReader;
#[doc = "Field `H_AHBERRMSK5` writer - "]
pub type H_AHBERRMSK5_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK5_SPEC, O>;
#[doc = "Field `H_STALLMSK5` reader - "]
pub type H_STALLMSK5_R = crate::BitReader;
#[doc = "Field `H_STALLMSK5` writer - "]
pub type H_STALLMSK5_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK5_SPEC, O>;
#[doc = "Field `H_NAKMSK5` reader - "]
pub type H_NAKMSK5_R = crate::BitReader;
#[doc = "Field `H_NAKMSK5` writer - "]
pub type H_NAKMSK5_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK5_SPEC, O>;
#[doc = "Field `H_ACKMSK5` reader - "]
pub type H_ACKMSK5_R = crate::BitReader;
#[doc = "Field `H_ACKMSK5` writer - "]
pub type H_ACKMSK5_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK5_SPEC, O>;
#[doc = "Field `H_NYETMSK5` reader - "]
pub type H_NYETMSK5_R = crate::BitReader;
#[doc = "Field `H_NYETMSK5` writer - "]
pub type H_NYETMSK5_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK5_SPEC, O>;
#[doc = "Field `H_XACTERRMSK5` reader - "]
pub type H_XACTERRMSK5_R = crate::BitReader;
#[doc = "Field `H_XACTERRMSK5` writer - "]
pub type H_XACTERRMSK5_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK5_SPEC, O>;
#[doc = "Field `H_BBLERRMSK5` reader - "]
pub type H_BBLERRMSK5_R = crate::BitReader;
#[doc = "Field `H_BBLERRMSK5` writer - "]
pub type H_BBLERRMSK5_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK5_SPEC, O>;
#[doc = "Field `H_FRMOVRUNMSK5` reader - "]
pub type H_FRMOVRUNMSK5_R = crate::BitReader;
#[doc = "Field `H_FRMOVRUNMSK5` writer - "]
pub type H_FRMOVRUNMSK5_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK5_SPEC, O>;
#[doc = "Field `H_DATATGLERRMSK5` reader - "]
pub type H_DATATGLERRMSK5_R = crate::BitReader;
#[doc = "Field `H_DATATGLERRMSK5` writer - "]
pub type H_DATATGLERRMSK5_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK5_SPEC, O>;
#[doc = "Field `H_BNAINTRMSK5` reader - "]
pub type H_BNAINTRMSK5_R = crate::BitReader;
#[doc = "Field `H_BNAINTRMSK5` writer - "]
pub type H_BNAINTRMSK5_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK5_SPEC, O>;
#[doc = "Field `H_DESC_LST_ROLLINTRMSK5` reader - "]
pub type H_DESC_LST_ROLLINTRMSK5_R = crate::BitReader;
#[doc = "Field `H_DESC_LST_ROLLINTRMSK5` writer - "]
pub type H_DESC_LST_ROLLINTRMSK5_W<'a, const O: u8> = crate::BitWriter<'a, HCINTMSK5_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn h_xfercomplmsk5(&self) -> H_XFERCOMPLMSK5_R {
        H_XFERCOMPLMSK5_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn h_chhltdmsk5(&self) -> H_CHHLTDMSK5_R {
        H_CHHLTDMSK5_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn h_ahberrmsk5(&self) -> H_AHBERRMSK5_R {
        H_AHBERRMSK5_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn h_stallmsk5(&self) -> H_STALLMSK5_R {
        H_STALLMSK5_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn h_nakmsk5(&self) -> H_NAKMSK5_R {
        H_NAKMSK5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn h_ackmsk5(&self) -> H_ACKMSK5_R {
        H_ACKMSK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn h_nyetmsk5(&self) -> H_NYETMSK5_R {
        H_NYETMSK5_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn h_xacterrmsk5(&self) -> H_XACTERRMSK5_R {
        H_XACTERRMSK5_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn h_bblerrmsk5(&self) -> H_BBLERRMSK5_R {
        H_BBLERRMSK5_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn h_frmovrunmsk5(&self) -> H_FRMOVRUNMSK5_R {
        H_FRMOVRUNMSK5_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn h_datatglerrmsk5(&self) -> H_DATATGLERRMSK5_R {
        H_DATATGLERRMSK5_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn h_bnaintrmsk5(&self) -> H_BNAINTRMSK5_R {
        H_BNAINTRMSK5_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn h_desc_lst_rollintrmsk5(&self) -> H_DESC_LST_ROLLINTRMSK5_R {
        H_DESC_LST_ROLLINTRMSK5_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINTMSK5")
            .field(
                "h_xfercomplmsk5",
                &format_args!("{}", self.h_xfercomplmsk5().bit()),
            )
            .field(
                "h_chhltdmsk5",
                &format_args!("{}", self.h_chhltdmsk5().bit()),
            )
            .field(
                "h_ahberrmsk5",
                &format_args!("{}", self.h_ahberrmsk5().bit()),
            )
            .field("h_stallmsk5", &format_args!("{}", self.h_stallmsk5().bit()))
            .field("h_nakmsk5", &format_args!("{}", self.h_nakmsk5().bit()))
            .field("h_ackmsk5", &format_args!("{}", self.h_ackmsk5().bit()))
            .field("h_nyetmsk5", &format_args!("{}", self.h_nyetmsk5().bit()))
            .field(
                "h_xacterrmsk5",
                &format_args!("{}", self.h_xacterrmsk5().bit()),
            )
            .field(
                "h_bblerrmsk5",
                &format_args!("{}", self.h_bblerrmsk5().bit()),
            )
            .field(
                "h_frmovrunmsk5",
                &format_args!("{}", self.h_frmovrunmsk5().bit()),
            )
            .field(
                "h_datatglerrmsk5",
                &format_args!("{}", self.h_datatglerrmsk5().bit()),
            )
            .field(
                "h_bnaintrmsk5",
                &format_args!("{}", self.h_bnaintrmsk5().bit()),
            )
            .field(
                "h_desc_lst_rollintrmsk5",
                &format_args!("{}", self.h_desc_lst_rollintrmsk5().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCINTMSK5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn h_xfercomplmsk5(&mut self) -> H_XFERCOMPLMSK5_W<0> {
        H_XFERCOMPLMSK5_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn h_chhltdmsk5(&mut self) -> H_CHHLTDMSK5_W<1> {
        H_CHHLTDMSK5_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn h_ahberrmsk5(&mut self) -> H_AHBERRMSK5_W<2> {
        H_AHBERRMSK5_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn h_stallmsk5(&mut self) -> H_STALLMSK5_W<3> {
        H_STALLMSK5_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn h_nakmsk5(&mut self) -> H_NAKMSK5_W<4> {
        H_NAKMSK5_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn h_ackmsk5(&mut self) -> H_ACKMSK5_W<5> {
        H_ACKMSK5_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn h_nyetmsk5(&mut self) -> H_NYETMSK5_W<6> {
        H_NYETMSK5_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn h_xacterrmsk5(&mut self) -> H_XACTERRMSK5_W<7> {
        H_XACTERRMSK5_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn h_bblerrmsk5(&mut self) -> H_BBLERRMSK5_W<8> {
        H_BBLERRMSK5_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn h_frmovrunmsk5(&mut self) -> H_FRMOVRUNMSK5_W<9> {
        H_FRMOVRUNMSK5_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn h_datatglerrmsk5(&mut self) -> H_DATATGLERRMSK5_W<10> {
        H_DATATGLERRMSK5_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn h_bnaintrmsk5(&mut self) -> H_BNAINTRMSK5_W<11> {
        H_BNAINTRMSK5_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn h_desc_lst_rollintrmsk5(&mut self) -> H_DESC_LST_ROLLINTRMSK5_W<13> {
        H_DESC_LST_ROLLINTRMSK5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcintmsk5](index.html) module"]
pub struct HCINTMSK5_SPEC;
impl crate::RegisterSpec for HCINTMSK5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcintmsk5::R](R) reader structure"]
impl crate::Readable for HCINTMSK5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcintmsk5::W](W) writer structure"]
impl crate::Writable for HCINTMSK5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCINTMSK5 to value 0"]
impl crate::Resettable for HCINTMSK5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
