#[doc = "Register `BLEWLNBDEV` reader"]
pub type R = crate::R<BLEWLNBDEV_SPEC>;
#[doc = "Register `BLEWLNBDEV` writer"]
pub type W = crate::W<BLEWLNBDEV_SPEC>;
#[doc = "Field `NBPUBDEV` reader - Number of public devices in the white list"]
pub type NBPUBDEV_R = crate::FieldReader;
#[doc = "Field `NBPUBDEV` writer - Number of public devices in the white list"]
pub type NBPUBDEV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NBPRIVDEV` reader - Number of private devices in the white list"]
pub type NBPRIVDEV_R = crate::FieldReader;
#[doc = "Field `NBPRIVDEV` writer - Number of private devices in the white list"]
pub type NBPRIVDEV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Number of public devices in the white list"]
    #[inline(always)]
    pub fn nbpubdev(&self) -> NBPUBDEV_R {
        NBPUBDEV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Number of private devices in the white list"]
    #[inline(always)]
    pub fn nbprivdev(&self) -> NBPRIVDEV_R {
        NBPRIVDEV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLEWLNBDEV")
            .field("nbpubdev", &self.nbpubdev())
            .field("nbprivdev", &self.nbprivdev())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of public devices in the white list"]
    #[inline(always)]
    pub fn nbpubdev(&mut self) -> NBPUBDEV_W<'_, BLEWLNBDEV_SPEC> {
        NBPUBDEV_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Number of private devices in the white list"]
    #[inline(always)]
    pub fn nbprivdev(&mut self) -> NBPRIVDEV_W<'_, BLEWLNBDEV_SPEC> {
        NBPRIVDEV_W::new(self, 8)
    }
}
#[doc = "Number of devices in whitelist\n\nYou can [`read`](crate::Reg::read) this register and get [`blewlnbdev::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blewlnbdev::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEWLNBDEV_SPEC;
impl crate::RegisterSpec for BLEWLNBDEV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blewlnbdev::R`](R) reader structure"]
impl crate::Readable for BLEWLNBDEV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blewlnbdev::W`](W) writer structure"]
impl crate::Writable for BLEWLNBDEV_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEWLNBDEV to value 0"]
impl crate::Resettable for BLEWLNBDEV_SPEC {}
