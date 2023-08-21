#[doc = "Register `HCINTMSK7` reader"]
pub type R = crate::R<HCINTMSK7_SPEC>;
#[doc = "Register `HCINTMSK7` writer"]
pub type W = crate::W<HCINTMSK7_SPEC>;
#[doc = "Field `H_XFERCOMPLMSK7` reader - "]
pub type H_XFERCOMPLMSK7_R = crate::BitReader;
#[doc = "Field `H_XFERCOMPLMSK7` writer - "]
pub type H_XFERCOMPLMSK7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `H_CHHLTDMSK7` reader - "]
pub type H_CHHLTDMSK7_R = crate::BitReader;
#[doc = "Field `H_CHHLTDMSK7` writer - "]
pub type H_CHHLTDMSK7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `H_AHBERRMSK7` reader - "]
pub type H_AHBERRMSK7_R = crate::BitReader;
#[doc = "Field `H_AHBERRMSK7` writer - "]
pub type H_AHBERRMSK7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `H_STALLMSK7` reader - "]
pub type H_STALLMSK7_R = crate::BitReader;
#[doc = "Field `H_STALLMSK7` writer - "]
pub type H_STALLMSK7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `H_NAKMSK7` reader - "]
pub type H_NAKMSK7_R = crate::BitReader;
#[doc = "Field `H_NAKMSK7` writer - "]
pub type H_NAKMSK7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `H_ACKMSK7` reader - "]
pub type H_ACKMSK7_R = crate::BitReader;
#[doc = "Field `H_ACKMSK7` writer - "]
pub type H_ACKMSK7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `H_NYETMSK7` reader - "]
pub type H_NYETMSK7_R = crate::BitReader;
#[doc = "Field `H_NYETMSK7` writer - "]
pub type H_NYETMSK7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `H_XACTERRMSK7` reader - "]
pub type H_XACTERRMSK7_R = crate::BitReader;
#[doc = "Field `H_XACTERRMSK7` writer - "]
pub type H_XACTERRMSK7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `H_BBLERRMSK7` reader - "]
pub type H_BBLERRMSK7_R = crate::BitReader;
#[doc = "Field `H_BBLERRMSK7` writer - "]
pub type H_BBLERRMSK7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `H_FRMOVRUNMSK7` reader - "]
pub type H_FRMOVRUNMSK7_R = crate::BitReader;
#[doc = "Field `H_FRMOVRUNMSK7` writer - "]
pub type H_FRMOVRUNMSK7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `H_DATATGLERRMSK7` reader - "]
pub type H_DATATGLERRMSK7_R = crate::BitReader;
#[doc = "Field `H_DATATGLERRMSK7` writer - "]
pub type H_DATATGLERRMSK7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `H_BNAINTRMSK7` reader - "]
pub type H_BNAINTRMSK7_R = crate::BitReader;
#[doc = "Field `H_BNAINTRMSK7` writer - "]
pub type H_BNAINTRMSK7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `H_DESC_LST_ROLLINTRMSK7` reader - "]
pub type H_DESC_LST_ROLLINTRMSK7_R = crate::BitReader;
#[doc = "Field `H_DESC_LST_ROLLINTRMSK7` writer - "]
pub type H_DESC_LST_ROLLINTRMSK7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn h_xfercomplmsk7(&self) -> H_XFERCOMPLMSK7_R {
        H_XFERCOMPLMSK7_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn h_chhltdmsk7(&self) -> H_CHHLTDMSK7_R {
        H_CHHLTDMSK7_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn h_ahberrmsk7(&self) -> H_AHBERRMSK7_R {
        H_AHBERRMSK7_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn h_stallmsk7(&self) -> H_STALLMSK7_R {
        H_STALLMSK7_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn h_nakmsk7(&self) -> H_NAKMSK7_R {
        H_NAKMSK7_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn h_ackmsk7(&self) -> H_ACKMSK7_R {
        H_ACKMSK7_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn h_nyetmsk7(&self) -> H_NYETMSK7_R {
        H_NYETMSK7_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn h_xacterrmsk7(&self) -> H_XACTERRMSK7_R {
        H_XACTERRMSK7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn h_bblerrmsk7(&self) -> H_BBLERRMSK7_R {
        H_BBLERRMSK7_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn h_frmovrunmsk7(&self) -> H_FRMOVRUNMSK7_R {
        H_FRMOVRUNMSK7_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn h_datatglerrmsk7(&self) -> H_DATATGLERRMSK7_R {
        H_DATATGLERRMSK7_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn h_bnaintrmsk7(&self) -> H_BNAINTRMSK7_R {
        H_BNAINTRMSK7_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn h_desc_lst_rollintrmsk7(&self) -> H_DESC_LST_ROLLINTRMSK7_R {
        H_DESC_LST_ROLLINTRMSK7_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINTMSK7")
            .field(
                "h_xfercomplmsk7",
                &format_args!("{}", self.h_xfercomplmsk7().bit()),
            )
            .field(
                "h_chhltdmsk7",
                &format_args!("{}", self.h_chhltdmsk7().bit()),
            )
            .field(
                "h_ahberrmsk7",
                &format_args!("{}", self.h_ahberrmsk7().bit()),
            )
            .field("h_stallmsk7", &format_args!("{}", self.h_stallmsk7().bit()))
            .field("h_nakmsk7", &format_args!("{}", self.h_nakmsk7().bit()))
            .field("h_ackmsk7", &format_args!("{}", self.h_ackmsk7().bit()))
            .field("h_nyetmsk7", &format_args!("{}", self.h_nyetmsk7().bit()))
            .field(
                "h_xacterrmsk7",
                &format_args!("{}", self.h_xacterrmsk7().bit()),
            )
            .field(
                "h_bblerrmsk7",
                &format_args!("{}", self.h_bblerrmsk7().bit()),
            )
            .field(
                "h_frmovrunmsk7",
                &format_args!("{}", self.h_frmovrunmsk7().bit()),
            )
            .field(
                "h_datatglerrmsk7",
                &format_args!("{}", self.h_datatglerrmsk7().bit()),
            )
            .field(
                "h_bnaintrmsk7",
                &format_args!("{}", self.h_bnaintrmsk7().bit()),
            )
            .field(
                "h_desc_lst_rollintrmsk7",
                &format_args!("{}", self.h_desc_lst_rollintrmsk7().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCINTMSK7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn h_xfercomplmsk7(&mut self) -> H_XFERCOMPLMSK7_W<HCINTMSK7_SPEC, 0> {
        H_XFERCOMPLMSK7_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn h_chhltdmsk7(&mut self) -> H_CHHLTDMSK7_W<HCINTMSK7_SPEC, 1> {
        H_CHHLTDMSK7_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn h_ahberrmsk7(&mut self) -> H_AHBERRMSK7_W<HCINTMSK7_SPEC, 2> {
        H_AHBERRMSK7_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn h_stallmsk7(&mut self) -> H_STALLMSK7_W<HCINTMSK7_SPEC, 3> {
        H_STALLMSK7_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn h_nakmsk7(&mut self) -> H_NAKMSK7_W<HCINTMSK7_SPEC, 4> {
        H_NAKMSK7_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn h_ackmsk7(&mut self) -> H_ACKMSK7_W<HCINTMSK7_SPEC, 5> {
        H_ACKMSK7_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn h_nyetmsk7(&mut self) -> H_NYETMSK7_W<HCINTMSK7_SPEC, 6> {
        H_NYETMSK7_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn h_xacterrmsk7(&mut self) -> H_XACTERRMSK7_W<HCINTMSK7_SPEC, 7> {
        H_XACTERRMSK7_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn h_bblerrmsk7(&mut self) -> H_BBLERRMSK7_W<HCINTMSK7_SPEC, 8> {
        H_BBLERRMSK7_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn h_frmovrunmsk7(&mut self) -> H_FRMOVRUNMSK7_W<HCINTMSK7_SPEC, 9> {
        H_FRMOVRUNMSK7_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn h_datatglerrmsk7(&mut self) -> H_DATATGLERRMSK7_W<HCINTMSK7_SPEC, 10> {
        H_DATATGLERRMSK7_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn h_bnaintrmsk7(&mut self) -> H_BNAINTRMSK7_W<HCINTMSK7_SPEC, 11> {
        H_BNAINTRMSK7_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn h_desc_lst_rollintrmsk7(&mut self) -> H_DESC_LST_ROLLINTRMSK7_W<HCINTMSK7_SPEC, 13> {
        H_DESC_LST_ROLLINTRMSK7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCINTMSK7_SPEC;
impl crate::RegisterSpec for HCINTMSK7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcintmsk7::R`](R) reader structure"]
impl crate::Readable for HCINTMSK7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcintmsk7::W`](W) writer structure"]
impl crate::Writable for HCINTMSK7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCINTMSK7 to value 0"]
impl crate::Resettable for HCINTMSK7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
