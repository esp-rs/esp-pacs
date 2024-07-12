#[doc = "Register `DOEPMSK` reader"]
pub type R = crate::R<DOEPMSK_SPEC>;
#[doc = "Register `DOEPMSK` writer"]
pub type W = crate::W<DOEPMSK_SPEC>;
#[doc = "Field `XFERCOMPLMSK` reader - "]
pub type XFERCOMPLMSK_R = crate::BitReader;
#[doc = "Field `XFERCOMPLMSK` writer - "]
pub type XFERCOMPLMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISBLDMSK` reader - "]
pub type EPDISBLDMSK_R = crate::BitReader;
#[doc = "Field `EPDISBLDMSK` writer - "]
pub type EPDISBLDMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERMSK` reader - "]
pub type AHBERMSK_R = crate::BitReader;
#[doc = "Field `AHBERMSK` writer - "]
pub type AHBERMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETUPMSK` reader - "]
pub type SETUPMSK_R = crate::BitReader;
#[doc = "Field `SETUPMSK` writer - "]
pub type SETUPMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTTKNEPDISMSK` reader - "]
pub type OUTTKNEPDISMSK_R = crate::BitReader;
#[doc = "Field `OUTTKNEPDISMSK` writer - "]
pub type OUTTKNEPDISMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STSPHSERCVDMSK` reader - "]
pub type STSPHSERCVDMSK_R = crate::BitReader;
#[doc = "Field `STSPHSERCVDMSK` writer - "]
pub type STSPHSERCVDMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BACK2BACKSETUP` reader - "]
pub type BACK2BACKSETUP_R = crate::BitReader;
#[doc = "Field `BACK2BACKSETUP` writer - "]
pub type BACK2BACKSETUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTPKTERRMSK` reader - "]
pub type OUTPKTERRMSK_R = crate::BitReader;
#[doc = "Field `OUTPKTERRMSK` writer - "]
pub type OUTPKTERRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BNAOUTINTRMSK` reader - "]
pub type BNAOUTINTRMSK_R = crate::BitReader;
#[doc = "Field `BNAOUTINTRMSK` writer - "]
pub type BNAOUTINTRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBLEERRMSK` reader - "]
pub type BBLEERRMSK_R = crate::BitReader;
#[doc = "Field `BBLEERRMSK` writer - "]
pub type BBLEERRMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKMSK` reader - "]
pub type NAKMSK_R = crate::BitReader;
#[doc = "Field `NAKMSK` writer - "]
pub type NAKMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYETMSK` reader - "]
pub type NYETMSK_R = crate::BitReader;
#[doc = "Field `NYETMSK` writer - "]
pub type NYETMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn xfercomplmsk(&self) -> XFERCOMPLMSK_R {
        XFERCOMPLMSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn epdisbldmsk(&self) -> EPDISBLDMSK_R {
        EPDISBLDMSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ahbermsk(&self) -> AHBERMSK_R {
        AHBERMSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn setupmsk(&self) -> SETUPMSK_R {
        SETUPMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn outtknepdismsk(&self) -> OUTTKNEPDISMSK_R {
        OUTTKNEPDISMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn stsphsercvdmsk(&self) -> STSPHSERCVDMSK_R {
        STSPHSERCVDMSK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn back2backsetup(&self) -> BACK2BACKSETUP_R {
        BACK2BACKSETUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn outpkterrmsk(&self) -> OUTPKTERRMSK_R {
        OUTPKTERRMSK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bnaoutintrmsk(&self) -> BNAOUTINTRMSK_R {
        BNAOUTINTRMSK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bbleerrmsk(&self) -> BBLEERRMSK_R {
        BBLEERRMSK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn nakmsk(&self) -> NAKMSK_R {
        NAKMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn nyetmsk(&self) -> NYETMSK_R {
        NYETMSK_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPMSK")
            .field("xfercomplmsk", &self.xfercomplmsk())
            .field("epdisbldmsk", &self.epdisbldmsk())
            .field("ahbermsk", &self.ahbermsk())
            .field("setupmsk", &self.setupmsk())
            .field("outtknepdismsk", &self.outtknepdismsk())
            .field("stsphsercvdmsk", &self.stsphsercvdmsk())
            .field("back2backsetup", &self.back2backsetup())
            .field("outpkterrmsk", &self.outpkterrmsk())
            .field("bnaoutintrmsk", &self.bnaoutintrmsk())
            .field("bbleerrmsk", &self.bbleerrmsk())
            .field("nakmsk", &self.nakmsk())
            .field("nyetmsk", &self.nyetmsk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn xfercomplmsk(&mut self) -> XFERCOMPLMSK_W<DOEPMSK_SPEC> {
        XFERCOMPLMSK_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn epdisbldmsk(&mut self) -> EPDISBLDMSK_W<DOEPMSK_SPEC> {
        EPDISBLDMSK_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ahbermsk(&mut self) -> AHBERMSK_W<DOEPMSK_SPEC> {
        AHBERMSK_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn setupmsk(&mut self) -> SETUPMSK_W<DOEPMSK_SPEC> {
        SETUPMSK_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn outtknepdismsk(&mut self) -> OUTTKNEPDISMSK_W<DOEPMSK_SPEC> {
        OUTTKNEPDISMSK_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn stsphsercvdmsk(&mut self) -> STSPHSERCVDMSK_W<DOEPMSK_SPEC> {
        STSPHSERCVDMSK_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn back2backsetup(&mut self) -> BACK2BACKSETUP_W<DOEPMSK_SPEC> {
        BACK2BACKSETUP_W::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn outpkterrmsk(&mut self) -> OUTPKTERRMSK_W<DOEPMSK_SPEC> {
        OUTPKTERRMSK_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn bnaoutintrmsk(&mut self) -> BNAOUTINTRMSK_W<DOEPMSK_SPEC> {
        BNAOUTINTRMSK_W::new(self, 9)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn bbleerrmsk(&mut self) -> BBLEERRMSK_W<DOEPMSK_SPEC> {
        BBLEERRMSK_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn nakmsk(&mut self) -> NAKMSK_W<DOEPMSK_SPEC> {
        NAKMSK_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn nyetmsk(&mut self) -> NYETMSK_W<DOEPMSK_SPEC> {
        NYETMSK_W::new(self, 14)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`doepmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPMSK_SPEC;
impl crate::RegisterSpec for DOEPMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepmsk::R`](R) reader structure"]
impl crate::Readable for DOEPMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doepmsk::W`](W) writer structure"]
impl crate::Writable for DOEPMSK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPMSK to value 0"]
impl crate::Resettable for DOEPMSK_SPEC {
    const RESET_VALUE: u32 = 0;
}
