#[doc = "Register `MASSEGE%s` reader"]
pub type R = crate::R<MASSEGE_SPEC>;
#[doc = "Register `MASSEGE%s` writer"]
pub type W = crate::W<MASSEGE_SPEC>;
#[doc = "Field `MASSEGE` reader - "]
pub type MASSEGE_R = crate::FieldReader<u32>;
#[doc = "Field `MASSEGE` writer - "]
pub type MASSEGE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn massege(&self) -> MASSEGE_R {
        MASSEGE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MASSEGE")
            .field("massege", &self.massege())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn massege(&mut self) -> MASSEGE_W<'_, MASSEGE_SPEC> {
        MASSEGE_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`massege::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`massege::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MASSEGE_SPEC;
impl crate::RegisterSpec for MASSEGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`massege::R`](R) reader structure"]
impl crate::Readable for MASSEGE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`massege::W`](W) writer structure"]
impl crate::Writable for MASSEGE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MASSEGE%s to value 0"]
impl crate::Resettable for MASSEGE_SPEC {}
