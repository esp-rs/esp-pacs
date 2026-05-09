#[doc = "Register `HFLBADDR` reader"]
pub type R = crate::R<HFLBADDR_SPEC>;
#[doc = "Register `HFLBADDR` writer"]
pub type W = crate::W<HFLBADDR_SPEC>;
#[doc = "Field `HFLBADDR` reader - "]
pub type HFLBADDR_R = crate::FieldReader<u32>;
#[doc = "Field `HFLBADDR` writer - "]
pub type HFLBADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hflbaddr(&self) -> HFLBADDR_R {
        HFLBADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFLBADDR")
            .field("hflbaddr", &self.hflbaddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hflbaddr(&mut self) -> HFLBADDR_W<'_, HFLBADDR_SPEC> {
        HFLBADDR_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`hflbaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hflbaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFLBADDR_SPEC;
impl crate::RegisterSpec for HFLBADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hflbaddr::R`](R) reader structure"]
impl crate::Readable for HFLBADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hflbaddr::W`](W) writer structure"]
impl crate::Writable for HFLBADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HFLBADDR to value 0"]
impl crate::Resettable for HFLBADDR_SPEC {}
