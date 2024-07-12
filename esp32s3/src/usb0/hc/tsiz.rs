#[doc = "Register `TSIZ` reader"]
pub type R = crate::R<TSIZ_SPEC>;
#[doc = "Register `TSIZ` writer"]
pub type W = crate::W<TSIZ_SPEC>;
#[doc = "Field `XFERSIZE` reader - "]
pub type XFERSIZE_R = crate::FieldReader<u32>;
#[doc = "Field `XFERSIZE` writer - "]
pub type XFERSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PKTCNT` reader - "]
pub type PKTCNT_R = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - "]
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PID` reader - "]
pub type PID_R = crate::FieldReader;
#[doc = "Field `PID` writer - "]
pub type PID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DOPNG` reader - "]
pub type DOPNG_R = crate::BitReader;
#[doc = "Field `DOPNG` writer - "]
pub type DOPNG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn xfersize(&self) -> XFERSIZE_R {
        XFERSIZE_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dopng(&self) -> DOPNG_R {
        DOPNG_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSIZ")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .field("pid", &self.pid())
            .field("dopng", &self.dopng())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    #[must_use]
    pub fn xfersize(&mut self) -> XFERSIZE_W<TSIZ_SPEC> {
        XFERSIZE_W::new(self, 0)
    }
    #[doc = "Bits 19:28"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<TSIZ_SPEC> {
        PKTCNT_W::new(self, 19)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    #[must_use]
    pub fn pid(&mut self) -> PID_W<TSIZ_SPEC> {
        PID_W::new(self, 29)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn dopng(&mut self) -> DOPNG_W<TSIZ_SPEC> {
        DOPNG_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSIZ_SPEC;
impl crate::RegisterSpec for TSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsiz::R`](R) reader structure"]
impl crate::Readable for TSIZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsiz::W`](W) writer structure"]
impl crate::Writable for TSIZ_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSIZ to value 0"]
impl crate::Resettable for TSIZ_SPEC {
    const RESET_VALUE: u32 = 0;
}
