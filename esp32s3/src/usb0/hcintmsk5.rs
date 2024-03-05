#[doc = "Register `HCINTMSK5` reader"]
pub type R = crate::R<HCINTMSK5_SPEC>;
#[doc = "Register `HCINTMSK5` writer"]
pub type W = crate::W<HCINTMSK5_SPEC>;
#[doc = "Field `H_XFERCOMPLMSK5` reader - "]
pub type H_XFERCOMPLMSK5_R = crate::BitReader;
#[doc = "Field `H_XFERCOMPLMSK5` writer - "]
pub type H_XFERCOMPLMSK5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_CHHLTDMSK5` reader - "]
pub type H_CHHLTDMSK5_R = crate::BitReader;
#[doc = "Field `H_CHHLTDMSK5` writer - "]
pub type H_CHHLTDMSK5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_AHBERRMSK5` reader - "]
pub type H_AHBERRMSK5_R = crate::BitReader;
#[doc = "Field `H_AHBERRMSK5` writer - "]
pub type H_AHBERRMSK5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_STALLMSK5` reader - "]
pub type H_STALLMSK5_R = crate::BitReader;
#[doc = "Field `H_STALLMSK5` writer - "]
pub type H_STALLMSK5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_NAKMSK5` reader - "]
pub type H_NAKMSK5_R = crate::BitReader;
#[doc = "Field `H_NAKMSK5` writer - "]
pub type H_NAKMSK5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_ACKMSK5` reader - "]
pub type H_ACKMSK5_R = crate::BitReader;
#[doc = "Field `H_ACKMSK5` writer - "]
pub type H_ACKMSK5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_NYETMSK5` reader - "]
pub type H_NYETMSK5_R = crate::BitReader;
#[doc = "Field `H_NYETMSK5` writer - "]
pub type H_NYETMSK5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_XACTERRMSK5` reader - "]
pub type H_XACTERRMSK5_R = crate::BitReader;
#[doc = "Field `H_XACTERRMSK5` writer - "]
pub type H_XACTERRMSK5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_BBLERRMSK5` reader - "]
pub type H_BBLERRMSK5_R = crate::BitReader;
#[doc = "Field `H_BBLERRMSK5` writer - "]
pub type H_BBLERRMSK5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_FRMOVRUNMSK5` reader - "]
pub type H_FRMOVRUNMSK5_R = crate::BitReader;
#[doc = "Field `H_FRMOVRUNMSK5` writer - "]
pub type H_FRMOVRUNMSK5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_DATATGLERRMSK5` reader - "]
pub type H_DATATGLERRMSK5_R = crate::BitReader;
#[doc = "Field `H_DATATGLERRMSK5` writer - "]
pub type H_DATATGLERRMSK5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_BNAINTRMSK5` reader - "]
pub type H_BNAINTRMSK5_R = crate::BitReader;
#[doc = "Field `H_BNAINTRMSK5` writer - "]
pub type H_BNAINTRMSK5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H_DESC_LST_ROLLINTRMSK5` reader - "]
pub type H_DESC_LST_ROLLINTRMSK5_R = crate::BitReader;
#[doc = "Field `H_DESC_LST_ROLLINTRMSK5` writer - "]
pub type H_DESC_LST_ROLLINTRMSK5_W<'a, REG> = crate::BitWriter<'a, REG>;
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
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn h_xfercomplmsk5(&mut self) -> H_XFERCOMPLMSK5_W<HCINTMSK5_SPEC> {
        H_XFERCOMPLMSK5_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn h_chhltdmsk5(&mut self) -> H_CHHLTDMSK5_W<HCINTMSK5_SPEC> {
        H_CHHLTDMSK5_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn h_ahberrmsk5(&mut self) -> H_AHBERRMSK5_W<HCINTMSK5_SPEC> {
        H_AHBERRMSK5_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn h_stallmsk5(&mut self) -> H_STALLMSK5_W<HCINTMSK5_SPEC> {
        H_STALLMSK5_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn h_nakmsk5(&mut self) -> H_NAKMSK5_W<HCINTMSK5_SPEC> {
        H_NAKMSK5_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn h_ackmsk5(&mut self) -> H_ACKMSK5_W<HCINTMSK5_SPEC> {
        H_ACKMSK5_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn h_nyetmsk5(&mut self) -> H_NYETMSK5_W<HCINTMSK5_SPEC> {
        H_NYETMSK5_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn h_xacterrmsk5(&mut self) -> H_XACTERRMSK5_W<HCINTMSK5_SPEC> {
        H_XACTERRMSK5_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn h_bblerrmsk5(&mut self) -> H_BBLERRMSK5_W<HCINTMSK5_SPEC> {
        H_BBLERRMSK5_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn h_frmovrunmsk5(&mut self) -> H_FRMOVRUNMSK5_W<HCINTMSK5_SPEC> {
        H_FRMOVRUNMSK5_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn h_datatglerrmsk5(&mut self) -> H_DATATGLERRMSK5_W<HCINTMSK5_SPEC> {
        H_DATATGLERRMSK5_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn h_bnaintrmsk5(&mut self) -> H_BNAINTRMSK5_W<HCINTMSK5_SPEC> {
        H_BNAINTRMSK5_W::new(self, 11)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn h_desc_lst_rollintrmsk5(&mut self) -> H_DESC_LST_ROLLINTRMSK5_W<HCINTMSK5_SPEC> {
        H_DESC_LST_ROLLINTRMSK5_W::new(self, 13)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCINTMSK5_SPEC;
impl crate::RegisterSpec for HCINTMSK5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcintmsk5::R`](R) reader structure"]
impl crate::Readable for HCINTMSK5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hcintmsk5::W`](W) writer structure"]
impl crate::Writable for HCINTMSK5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCINTMSK5 to value 0"]
impl crate::Resettable for HCINTMSK5_SPEC {
    const RESET_VALUE: u32 = 0;
}
