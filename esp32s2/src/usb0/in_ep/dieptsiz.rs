#[doc = "Register `DIEPTSIZ` reader"]
pub type R = crate::R<DIEPTSIZ_SPEC>;
#[doc = "Register `DIEPTSIZ` writer"]
pub type W = crate::W<DIEPTSIZ_SPEC>;
#[doc = "Field `XFERSIZE` reader - "]
pub type XFERSIZE_R = crate::FieldReader<u32>;
#[doc = "Field `XFERSIZE` writer - "]
pub type XFERSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32, crate::Safe>;
#[doc = "Field `PKTCNT` reader - "]
pub type PKTCNT_R = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - "]
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16, crate::Safe>;
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
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTSIZ")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn xfersize(&mut self) -> XFERSIZE_W<DIEPTSIZ_SPEC> {
        XFERSIZE_W::new(self, 0)
    }
    #[doc = "Bits 19:28"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W<DIEPTSIZ_SPEC> {
        PKTCNT_W::new(self, 19)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTSIZ_SPEC;
impl crate::RegisterSpec for DIEPTSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptsiz::R`](R) reader structure"]
impl crate::Readable for DIEPTSIZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dieptsiz::W`](W) writer structure"]
impl crate::Writable for DIEPTSIZ_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPTSIZ to value 0"]
impl crate::Resettable for DIEPTSIZ_SPEC {
    const RESET_VALUE: u32 = 0;
}
