#[doc = "Register `IDEXT` reader"]
pub type R = crate::R<IDEXT_SPEC>;
#[doc = "Register `IDEXT` writer"]
pub type W = crate::W<IDEXT_SPEC>;
#[doc = "Field `IDEXT` reader - NA"]
pub type IDEXT_R = crate::FieldReader<u32>;
#[doc = "Field `IDEXT` writer - NA"]
pub type IDEXT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn idext(&self) -> IDEXT_R {
        IDEXT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDEXT")
            .field("idext", &self.idext())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn idext(&mut self) -> IDEXT_W<'_, IDEXT_SPEC> {
        IDEXT_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`idext::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idext::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDEXT_SPEC;
impl crate::RegisterSpec for IDEXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idext::R`](R) reader structure"]
impl crate::Readable for IDEXT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`idext::W`](W) writer structure"]
impl crate::Writable for IDEXT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDEXT to value 0"]
impl crate::Resettable for IDEXT_SPEC {}
