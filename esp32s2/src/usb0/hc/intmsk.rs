#[doc = "Register `INTMSK` reader"]
pub type R = crate::R<INTMSK_SPEC>;
#[doc = "Register `INTMSK` writer"]
pub type W = crate::W<INTMSK_SPEC>;
#[doc = "Field `XFERCOMPLMSK` reader - "]
pub type XFERCOMPLMSK_R = crate::BitReader;
#[doc = "Field `XFERCOMPLMSK` writer - "]
pub type XFERCOMPLMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHHLTDMSK` reader - "]
pub type CHHLTDMSK_R = crate::BitReader;
#[doc = "Field `CHHLTDMSK` writer - "]
pub type CHHLTDMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERRMSK` reader - "]
pub type AHBERRMSK_R = crate::BitReader;
#[doc = "Field `AHBERRMSK` writer - "]
pub type AHBERRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLMSK` reader - "]
pub type STALLMSK_R = crate::BitReader;
#[doc = "Field `STALLMSK` writer - "]
pub type STALLMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKMSK` reader - "]
pub type NAKMSK_R = crate::BitReader;
#[doc = "Field `NAKMSK` writer - "]
pub type NAKMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKMSK` reader - "]
pub type ACKMSK_R = crate::BitReader;
#[doc = "Field `ACKMSK` writer - "]
pub type ACKMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYETMSK` reader - "]
pub type NYETMSK_R = crate::BitReader;
#[doc = "Field `NYETMSK` writer - "]
pub type NYETMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XACTERRMSK` reader - "]
pub type XACTERRMSK_R = crate::BitReader;
#[doc = "Field `XACTERRMSK` writer - "]
pub type XACTERRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBLERRMSK` reader - "]
pub type BBLERRMSK_R = crate::BitReader;
#[doc = "Field `BBLERRMSK` writer - "]
pub type BBLERRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMOVRUNMSK` reader - "]
pub type FRMOVRUNMSK_R = crate::BitReader;
#[doc = "Field `FRMOVRUNMSK` writer - "]
pub type FRMOVRUNMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATATGLERRMSK` reader - "]
pub type DATATGLERRMSK_R = crate::BitReader;
#[doc = "Field `DATATGLERRMSK` writer - "]
pub type DATATGLERRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNAINTRMSK` reader - "]
pub type BNAINTRMSK_R = crate::BitReader;
#[doc = "Field `BNAINTRMSK` writer - "]
pub type BNAINTRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DESC_LST_ROLLINTRMSK` reader - "]
pub type DESC_LST_ROLLINTRMSK_R = crate::BitReader;
#[doc = "Field `DESC_LST_ROLLINTRMSK` writer - "]
pub type DESC_LST_ROLLINTRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn xfercomplmsk(&self) -> XFERCOMPLMSK_R {
        XFERCOMPLMSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn chhltdmsk(&self) -> CHHLTDMSK_R {
        CHHLTDMSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ahberrmsk(&self) -> AHBERRMSK_R {
        AHBERRMSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn stallmsk(&self) -> STALLMSK_R {
        STALLMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn nakmsk(&self) -> NAKMSK_R {
        NAKMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ackmsk(&self) -> ACKMSK_R {
        ACKMSK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn nyetmsk(&self) -> NYETMSK_R {
        NYETMSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn xacterrmsk(&self) -> XACTERRMSK_R {
        XACTERRMSK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn bblerrmsk(&self) -> BBLERRMSK_R {
        BBLERRMSK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn frmovrunmsk(&self) -> FRMOVRUNMSK_R {
        FRMOVRUNMSK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn datatglerrmsk(&self) -> DATATGLERRMSK_R {
        DATATGLERRMSK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn bnaintrmsk(&self) -> BNAINTRMSK_R {
        BNAINTRMSK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn desc_lst_rollintrmsk(&self) -> DESC_LST_ROLLINTRMSK_R {
        DESC_LST_ROLLINTRMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTMSK")
            .field("xfercomplmsk", &self.xfercomplmsk())
            .field("chhltdmsk", &self.chhltdmsk())
            .field("ahberrmsk", &self.ahberrmsk())
            .field("stallmsk", &self.stallmsk())
            .field("nakmsk", &self.nakmsk())
            .field("ackmsk", &self.ackmsk())
            .field("nyetmsk", &self.nyetmsk())
            .field("xacterrmsk", &self.xacterrmsk())
            .field("bblerrmsk", &self.bblerrmsk())
            .field("frmovrunmsk", &self.frmovrunmsk())
            .field("datatglerrmsk", &self.datatglerrmsk())
            .field("bnaintrmsk", &self.bnaintrmsk())
            .field("desc_lst_rollintrmsk", &self.desc_lst_rollintrmsk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn xfercomplmsk(&mut self) -> XFERCOMPLMSK_W<'_, INTMSK_SPEC> {
        XFERCOMPLMSK_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn chhltdmsk(&mut self) -> CHHLTDMSK_W<'_, INTMSK_SPEC> {
        CHHLTDMSK_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ahberrmsk(&mut self) -> AHBERRMSK_W<'_, INTMSK_SPEC> {
        AHBERRMSK_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn stallmsk(&mut self) -> STALLMSK_W<'_, INTMSK_SPEC> {
        STALLMSK_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn nakmsk(&mut self) -> NAKMSK_W<'_, INTMSK_SPEC> {
        NAKMSK_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ackmsk(&mut self) -> ACKMSK_W<'_, INTMSK_SPEC> {
        ACKMSK_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn nyetmsk(&mut self) -> NYETMSK_W<'_, INTMSK_SPEC> {
        NYETMSK_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn xacterrmsk(&mut self) -> XACTERRMSK_W<'_, INTMSK_SPEC> {
        XACTERRMSK_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn bblerrmsk(&mut self) -> BBLERRMSK_W<'_, INTMSK_SPEC> {
        BBLERRMSK_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn frmovrunmsk(&mut self) -> FRMOVRUNMSK_W<'_, INTMSK_SPEC> {
        FRMOVRUNMSK_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn datatglerrmsk(&mut self) -> DATATGLERRMSK_W<'_, INTMSK_SPEC> {
        DATATGLERRMSK_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn bnaintrmsk(&mut self) -> BNAINTRMSK_W<'_, INTMSK_SPEC> {
        BNAINTRMSK_W::new(self, 11)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn desc_lst_rollintrmsk(&mut self) -> DESC_LST_ROLLINTRMSK_W<'_, INTMSK_SPEC> {
        DESC_LST_ROLLINTRMSK_W::new(self, 13)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`intmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTMSK_SPEC;
impl crate::RegisterSpec for INTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intmsk::R`](R) reader structure"]
impl crate::Readable for INTMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intmsk::W`](W) writer structure"]
impl crate::Writable for INTMSK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTMSK to value 0"]
impl crate::Resettable for INTMSK_SPEC {}
