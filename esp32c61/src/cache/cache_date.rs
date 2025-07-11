#[doc = "Register `CACHE_DATE` reader"]
pub type R = crate::R<CACHE_DATE_SPEC>;
#[doc = "Register `CACHE_DATE` writer"]
pub type W = crate::W<CACHE_DATE_SPEC>;
#[doc = "Field `CACHE_DATE` reader - version control register. Note that this default value stored is the latest date when the hardware logic was updated."]
pub type CACHE_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `CACHE_DATE` writer - version control register. Note that this default value stored is the latest date when the hardware logic was updated."]
pub type CACHE_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - version control register. Note that this default value stored is the latest date when the hardware logic was updated."]
    #[inline(always)]
    pub fn cache_date(&self) -> CACHE_DATE_R {
        CACHE_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_DATE")
            .field("cache_date", &self.cache_date())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:27 - version control register. Note that this default value stored is the latest date when the hardware logic was updated."]
    #[inline(always)]
    pub fn cache_date(&mut self) -> CACHE_DATE_W<CACHE_DATE_SPEC> {
        CACHE_DATE_W::new(self, 0)
    }
}
#[doc = "Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_DATE_SPEC;
impl crate::RegisterSpec for CACHE_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_date::R`](R) reader structure"]
impl crate::Readable for CACHE_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_date::W`](W) writer structure"]
impl crate::Writable for CACHE_DATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_DATE to value 0x0231_2220"]
impl crate::Resettable for CACHE_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0231_2220;
}
