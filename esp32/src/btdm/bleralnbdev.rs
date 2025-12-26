#[doc = "Register `BLERALNBDEV` reader"]
pub type R = crate::R<BLERALNBDEV_SPEC>;
#[doc = "Register `BLERALNBDEV` writer"]
pub type W = crate::W<BLERALNBDEV_SPEC>;
#[doc = "Field `NBRALDEV` reader - Number of devices in Resolve Address List"]
pub type NBRALDEV_R = crate::FieldReader;
#[doc = "Field `NBRALDEV` writer - Number of devices in Resolve Address List"]
pub type NBRALDEV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Number of devices in Resolve Address List"]
    #[inline(always)]
    pub fn nbraldev(&self) -> NBRALDEV_R {
        NBRALDEV_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLERALNBDEV")
            .field("nbraldev", &self.nbraldev())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of devices in Resolve Address List"]
    #[inline(always)]
    pub fn nbraldev(&mut self) -> NBRALDEV_W<'_, BLERALNBDEV_SPEC> {
        NBRALDEV_W::new(self, 0)
    }
}
#[doc = "Number of devices in Resolve Address List\n\nYou can [`read`](crate::Reg::read) this register and get [`bleralnbdev::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleralnbdev::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLERALNBDEV_SPEC;
impl crate::RegisterSpec for BLERALNBDEV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bleralnbdev::R`](R) reader structure"]
impl crate::Readable for BLERALNBDEV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bleralnbdev::W`](W) writer structure"]
impl crate::Writable for BLERALNBDEV_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLERALNBDEV to value 0"]
impl crate::Resettable for BLERALNBDEV_SPEC {}
