#[doc = "Register `DOEPTSIZ` reader"]
pub type R = crate::R<DOEPTSIZ_SPEC>;
#[doc = "Register `DOEPTSIZ` writer"]
pub type W = crate::W<DOEPTSIZ_SPEC>;
#[doc = "Field `XFERSIZE` reader - "]
pub type XFERSIZE_R = crate::FieldReader;
#[doc = "Field `XFERSIZE` writer - "]
pub type XFERSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
#[doc = "Field `PKTCNT` reader - "]
pub type PKTCNT_R = crate::BitReader;
#[doc = "Field `PKTCNT` writer - "]
pub type PKTCNT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUPCNT` reader - "]
pub type SUPCNT_R = crate::FieldReader;
#[doc = "Field `SUPCNT` writer - "]
pub type SUPCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn xfersize(&self) -> XFERSIZE_R {
        XFERSIZE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 1) != 0)
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
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .field("supcnt", &self.supcnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn xfersize(&mut self) -> XFERSIZE_W<'_, DOEPTSIZ_SPEC> {
        XFERSIZE_W::new(self, 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W<'_, DOEPTSIZ_SPEC> {
        PKTCNT_W::new(self, 19)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn supcnt(&mut self) -> SUPCNT_W<'_, DOEPTSIZ_SPEC> {
        SUPCNT_W::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPTSIZ_SPEC;
impl crate::RegisterSpec for DOEPTSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doeptsiz::R`](R) reader structure"]
impl crate::Readable for DOEPTSIZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doeptsiz::W`](W) writer structure"]
impl crate::Writable for DOEPTSIZ_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOEPTSIZ to value 0"]
impl crate::Resettable for DOEPTSIZ_SPEC {}
