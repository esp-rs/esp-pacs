#[doc = "Register `DISCHARGE` reader"]
pub type R = crate::R<DISCHARGE_SPEC>;
#[doc = "Register `DISCHARGE` writer"]
pub type W = crate::W<DISCHARGE_SPEC>;
#[doc = "Field `VAL` reader - "]
pub type VAL_R = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - "]
pub type VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DISCHARGE")
            .field("val", &self.val())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W<'_, DISCHARGE_SPEC> {
        VAL_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`discharge::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`discharge::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DISCHARGE_SPEC;
impl crate::RegisterSpec for DISCHARGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`discharge::R`](R) reader structure"]
impl crate::Readable for DISCHARGE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`discharge::W`](W) writer structure"]
impl crate::Writable for DISCHARGE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DISCHARGE to value 0"]
impl crate::Resettable for DISCHARGE_SPEC {}
