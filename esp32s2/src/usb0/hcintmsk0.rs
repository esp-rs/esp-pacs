#[doc = "Register `HCINTMSK0` reader"]
pub type R = crate::R<HCINTMSK0_SPEC>;
#[doc = "Register `HCINTMSK0` writer"]
pub type W = crate::W<HCINTMSK0_SPEC>;
#[doc = "Field `H_XFERCOMPLMSK0` reader - "]
pub type H_XFERCOMPLMSK0_R = crate::BitReader;
#[doc = "Field `H_XFERCOMPLMSK0` writer - "]
pub type H_XFERCOMPLMSK0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_CHHLTDMSK0` reader - "]
pub type H_CHHLTDMSK0_R = crate::BitReader;
#[doc = "Field `H_CHHLTDMSK0` writer - "]
pub type H_CHHLTDMSK0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_AHBERRMSK0` reader - "]
pub type H_AHBERRMSK0_R = crate::BitReader;
#[doc = "Field `H_AHBERRMSK0` writer - "]
pub type H_AHBERRMSK0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_STALLMSK0` reader - "]
pub type H_STALLMSK0_R = crate::BitReader;
#[doc = "Field `H_STALLMSK0` writer - "]
pub type H_STALLMSK0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_NAKMSK0` reader - "]
pub type H_NAKMSK0_R = crate::BitReader;
#[doc = "Field `H_NAKMSK0` writer - "]
pub type H_NAKMSK0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_ACKMSK0` reader - "]
pub type H_ACKMSK0_R = crate::BitReader;
#[doc = "Field `H_ACKMSK0` writer - "]
pub type H_ACKMSK0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_NYETMSK0` reader - "]
pub type H_NYETMSK0_R = crate::BitReader;
#[doc = "Field `H_NYETMSK0` writer - "]
pub type H_NYETMSK0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_XACTERRMSK0` reader - "]
pub type H_XACTERRMSK0_R = crate::BitReader;
#[doc = "Field `H_XACTERRMSK0` writer - "]
pub type H_XACTERRMSK0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_BBLERRMSK0` reader - "]
pub type H_BBLERRMSK0_R = crate::BitReader;
#[doc = "Field `H_BBLERRMSK0` writer - "]
pub type H_BBLERRMSK0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_FRMOVRUNMSK0` reader - "]
pub type H_FRMOVRUNMSK0_R = crate::BitReader;
#[doc = "Field `H_FRMOVRUNMSK0` writer - "]
pub type H_FRMOVRUNMSK0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_DATATGLERRMSK0` reader - "]
pub type H_DATATGLERRMSK0_R = crate::BitReader;
#[doc = "Field `H_DATATGLERRMSK0` writer - "]
pub type H_DATATGLERRMSK0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_BNAINTRMSK0` reader - "]
pub type H_BNAINTRMSK0_R = crate::BitReader;
#[doc = "Field `H_BNAINTRMSK0` writer - "]
pub type H_BNAINTRMSK0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_DESC_LST_ROLLINTRMSK0` reader - "]
pub type H_DESC_LST_ROLLINTRMSK0_R = crate::BitReader;
#[doc = "Field `H_DESC_LST_ROLLINTRMSK0` writer - "]
pub type H_DESC_LST_ROLLINTRMSK0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn h_xfercomplmsk0(&self) -> H_XFERCOMPLMSK0_R {
        H_XFERCOMPLMSK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn h_chhltdmsk0(&self) -> H_CHHLTDMSK0_R {
        H_CHHLTDMSK0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn h_ahberrmsk0(&self) -> H_AHBERRMSK0_R {
        H_AHBERRMSK0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn h_stallmsk0(&self) -> H_STALLMSK0_R {
        H_STALLMSK0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn h_nakmsk0(&self) -> H_NAKMSK0_R {
        H_NAKMSK0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn h_ackmsk0(&self) -> H_ACKMSK0_R {
        H_ACKMSK0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn h_nyetmsk0(&self) -> H_NYETMSK0_R {
        H_NYETMSK0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn h_xacterrmsk0(&self) -> H_XACTERRMSK0_R {
        H_XACTERRMSK0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn h_bblerrmsk0(&self) -> H_BBLERRMSK0_R {
        H_BBLERRMSK0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn h_frmovrunmsk0(&self) -> H_FRMOVRUNMSK0_R {
        H_FRMOVRUNMSK0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn h_datatglerrmsk0(&self) -> H_DATATGLERRMSK0_R {
        H_DATATGLERRMSK0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn h_bnaintrmsk0(&self) -> H_BNAINTRMSK0_R {
        H_BNAINTRMSK0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn h_desc_lst_rollintrmsk0(&self) -> H_DESC_LST_ROLLINTRMSK0_R {
        H_DESC_LST_ROLLINTRMSK0_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINTMSK0")
            .field(
                "h_xfercomplmsk0",
                &format_args!("{}", self.h_xfercomplmsk0().bit()),
            )
            .field(
                "h_chhltdmsk0",
                &format_args!("{}", self.h_chhltdmsk0().bit()),
            )
            .field(
                "h_ahberrmsk0",
                &format_args!("{}", self.h_ahberrmsk0().bit()),
            )
            .field("h_stallmsk0", &format_args!("{}", self.h_stallmsk0().bit()))
            .field("h_nakmsk0", &format_args!("{}", self.h_nakmsk0().bit()))
            .field("h_ackmsk0", &format_args!("{}", self.h_ackmsk0().bit()))
            .field("h_nyetmsk0", &format_args!("{}", self.h_nyetmsk0().bit()))
            .field(
                "h_xacterrmsk0",
                &format_args!("{}", self.h_xacterrmsk0().bit()),
            )
            .field(
                "h_bblerrmsk0",
                &format_args!("{}", self.h_bblerrmsk0().bit()),
            )
            .field(
                "h_frmovrunmsk0",
                &format_args!("{}", self.h_frmovrunmsk0().bit()),
            )
            .field(
                "h_datatglerrmsk0",
                &format_args!("{}", self.h_datatglerrmsk0().bit()),
            )
            .field(
                "h_bnaintrmsk0",
                &format_args!("{}", self.h_bnaintrmsk0().bit()),
            )
            .field(
                "h_desc_lst_rollintrmsk0",
                &format_args!("{}", self.h_desc_lst_rollintrmsk0().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HCINTMSK0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn h_xfercomplmsk0(&mut self) -> H_XFERCOMPLMSK0_W<HCINTMSK0_SPEC> {
        H_XFERCOMPLMSK0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn h_chhltdmsk0(&mut self) -> H_CHHLTDMSK0_W<HCINTMSK0_SPEC> {
        H_CHHLTDMSK0_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn h_ahberrmsk0(&mut self) -> H_AHBERRMSK0_W<HCINTMSK0_SPEC> {
        H_AHBERRMSK0_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn h_stallmsk0(&mut self) -> H_STALLMSK0_W<HCINTMSK0_SPEC> {
        H_STALLMSK0_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn h_nakmsk0(&mut self) -> H_NAKMSK0_W<HCINTMSK0_SPEC> {
        H_NAKMSK0_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn h_ackmsk0(&mut self) -> H_ACKMSK0_W<HCINTMSK0_SPEC> {
        H_ACKMSK0_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn h_nyetmsk0(&mut self) -> H_NYETMSK0_W<HCINTMSK0_SPEC> {
        H_NYETMSK0_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn h_xacterrmsk0(&mut self) -> H_XACTERRMSK0_W<HCINTMSK0_SPEC> {
        H_XACTERRMSK0_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn h_bblerrmsk0(&mut self) -> H_BBLERRMSK0_W<HCINTMSK0_SPEC> {
        H_BBLERRMSK0_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn h_frmovrunmsk0(&mut self) -> H_FRMOVRUNMSK0_W<HCINTMSK0_SPEC> {
        H_FRMOVRUNMSK0_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn h_datatglerrmsk0(&mut self) -> H_DATATGLERRMSK0_W<HCINTMSK0_SPEC> {
        H_DATATGLERRMSK0_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn h_bnaintrmsk0(&mut self) -> H_BNAINTRMSK0_W<HCINTMSK0_SPEC> {
        H_BNAINTRMSK0_W::new(self, 11)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn h_desc_lst_rollintrmsk0(&mut self) -> H_DESC_LST_ROLLINTRMSK0_W<HCINTMSK0_SPEC> {
        H_DESC_LST_ROLLINTRMSK0_W::new(self, 13)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCINTMSK0_SPEC;
impl crate::RegisterSpec for HCINTMSK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcintmsk0::R`](R) reader structure"]
impl crate::Readable for HCINTMSK0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcintmsk0::W`](W) writer structure"]
impl crate::Writable for HCINTMSK0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCINTMSK0 to value 0"]
impl crate::Resettable for HCINTMSK0_SPEC {
    const RESET_VALUE: u32 = 0;
}
