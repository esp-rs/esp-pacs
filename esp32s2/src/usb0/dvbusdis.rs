#[doc = "Register `DVBUSDIS` reader"]
pub type R = crate::R<DVBUSDIS_SPEC>;
#[doc = "Register `DVBUSDIS` writer"]
pub type W = crate::W<DVBUSDIS_SPEC>;
#[doc = "Field `DVBUSDIS` reader - "]
pub type DVBUSDIS_R = crate::FieldReader<u16>;
#[doc = "Field `DVBUSDIS` writer - "]
pub type DVBUSDIS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn dvbusdis(&self) -> DVBUSDIS_R {
        DVBUSDIS_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DVBUSDIS")
            .field("dvbusdis", &self.dvbusdis())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn dvbusdis(&mut self) -> DVBUSDIS_W<DVBUSDIS_SPEC> {
        DVBUSDIS_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`dvbusdis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvbusdis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DVBUSDIS_SPEC;
impl crate::RegisterSpec for DVBUSDIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dvbusdis::R`](R) reader structure"]
impl crate::Readable for DVBUSDIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dvbusdis::W`](W) writer structure"]
impl crate::Writable for DVBUSDIS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DVBUSDIS to value 0x17d7"]
impl crate::Resettable for DVBUSDIS_SPEC {
    const RESET_VALUE: u32 = 0x17d7;
}
