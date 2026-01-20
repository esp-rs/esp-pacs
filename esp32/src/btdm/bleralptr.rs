#[doc = "Register `BLERALPTR` reader"]
pub type R = crate::R<BLERALPTR_SPEC>;
#[doc = "Register `BLERALPTR` writer"]
pub type W = crate::W<BLERALPTR_SPEC>;
#[doc = "Field `RALPTR` reader - Pointer to Resolve Address List"]
pub type RALPTR_R = crate::FieldReader<u16>;
#[doc = "Field `RALPTR` writer - Pointer to Resolve Address List"]
pub type RALPTR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Pointer to Resolve Address List"]
    #[inline(always)]
    pub fn ralptr(&self) -> RALPTR_R {
        RALPTR_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLERALPTR")
            .field("ralptr", &self.ralptr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Pointer to Resolve Address List"]
    #[inline(always)]
    pub fn ralptr(&mut self) -> RALPTR_W<'_, BLERALPTR_SPEC> {
        RALPTR_W::new(self, 0)
    }
}
#[doc = "Resolve Address List pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`bleralptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bleralptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLERALPTR_SPEC;
impl crate::RegisterSpec for BLERALPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bleralptr::R`](R) reader structure"]
impl crate::Readable for BLERALPTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bleralptr::W`](W) writer structure"]
impl crate::Writable for BLERALPTR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLERALPTR to value 0"]
impl crate::Resettable for BLERALPTR_SPEC {}
