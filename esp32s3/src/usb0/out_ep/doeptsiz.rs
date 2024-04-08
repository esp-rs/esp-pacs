#[doc = "Register `DOEPTSIZ` reader"]
pub type R = crate::R<DOEPTSIZ_SPEC>;
#[doc = "Register `DOEPTSIZ` writer"]
pub type W = crate::W<DOEPTSIZ_SPEC>;
#[doc = "Field `XFERSIZE` reader - "]
pub type XFERSIZE_R = crate::FieldReader<u32>;
#[doc = "Field `XFERSIZE` writer - "]
pub type XFERSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32, crate::Safe>;
#[doc = "Field `PKTCNT` reader - "]
pub type PKTCNT_R = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - "]
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16, crate::Safe>;
#[doc = "Field `SUPCNT` reader - "]
pub type SUPCNT_R = crate::FieldReader;
#[doc = "Field `SUPCNT` writer - "]
pub type SUPCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
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
    pub fn supcnt(&self) -> SUPCNT_R {
        SUPCNT_R::new(((self.bits >> 29) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPTSIZ")
            .field("xfersize", &format_args!("{}", self.xfersize().bits()))
            .field("pktcnt", &format_args!("{}", self.pktcnt().bits()))
            .field("supcnt", &format_args!("{}", self.supcnt().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DOEPTSIZ_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    #[must_use]
    pub fn xfersize(&mut self) -> XFERSIZE_W<DOEPTSIZ_SPEC> {
        XFERSIZE_W::new(self, 0)
    }
    #[doc = "Bits 19:28"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<DOEPTSIZ_SPEC> {
        PKTCNT_W::new(self, 19)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    #[must_use]
    pub fn supcnt(&mut self) -> SUPCNT_W<DOEPTSIZ_SPEC> {
        SUPCNT_W::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPTSIZ_SPEC;
impl crate::RegisterSpec for DOEPTSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doeptsiz::R`](R) reader structure"]
impl crate::Readable for DOEPTSIZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doeptsiz::W`](W) writer structure"]
impl crate::Writable for DOEPTSIZ_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPTSIZ to value 0"]
impl crate::Resettable for DOEPTSIZ_SPEC {
    const RESET_VALUE: u32 = 0;
}
