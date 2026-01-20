#[doc = "Register `BLEFINETIMECNT` reader"]
pub type R = crate::R<BLEFINETIMECNT_SPEC>;
#[doc = "Register `BLEFINETIMECNT` writer"]
pub type W = crate::W<BLEFINETIMECNT_SPEC>;
#[doc = "Field `FINECNT` reader - Fine time counter value"]
pub type FINECNT_R = crate::FieldReader<u16>;
#[doc = "Field `FINECNT` writer - Fine time counter value"]
pub type FINECNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Fine time counter value"]
    #[inline(always)]
    pub fn finecnt(&self) -> FINECNT_R {
        FINECNT_R::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLEFINETIMECNT")
            .field("finecnt", &self.finecnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - Fine time counter value"]
    #[inline(always)]
    pub fn finecnt(&mut self) -> FINECNT_W<'_, BLEFINETIMECNT_SPEC> {
        FINECNT_W::new(self, 0)
    }
}
#[doc = "Fine time reference counter\n\nYou can [`read`](crate::Reg::read) this register and get [`blefinetimecnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blefinetimecnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEFINETIMECNT_SPEC;
impl crate::RegisterSpec for BLEFINETIMECNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blefinetimecnt::R`](R) reader structure"]
impl crate::Readable for BLEFINETIMECNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blefinetimecnt::W`](W) writer structure"]
impl crate::Writable for BLEFINETIMECNT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEFINETIMECNT to value 0"]
impl crate::Resettable for BLEFINETIMECNT_SPEC {}
