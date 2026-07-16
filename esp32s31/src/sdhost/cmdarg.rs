#[doc = "Register `CMDARG` reader"]
pub type R = crate::R<CMDARG_SPEC>;
#[doc = "Register `CMDARG` writer"]
pub type W = crate::W<CMDARG_SPEC>;
#[doc = "Field `CMDARG` reader - Value indicates command argument to be passed to the card."]
pub type CMDARG_R = crate::FieldReader<u32>;
#[doc = "Field `CMDARG` writer - Value indicates command argument to be passed to the card."]
pub type CMDARG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Value indicates command argument to be passed to the card."]
    #[inline(always)]
    pub fn cmdarg(&self) -> CMDARG_R {
        CMDARG_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMDARG")
            .field("cmdarg", &self.cmdarg())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Value indicates command argument to be passed to the card."]
    #[inline(always)]
    pub fn cmdarg(&mut self) -> CMDARG_W<'_, CMDARG_SPEC> {
        CMDARG_W::new(self, 0)
    }
}
#[doc = "Command argument data register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdarg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdarg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMDARG_SPEC;
impl crate::RegisterSpec for CMDARG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdarg::R`](R) reader structure"]
impl crate::Readable for CMDARG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmdarg::W`](W) writer structure"]
impl crate::Writable for CMDARG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMDARG to value 0"]
impl crate::Resettable for CMDARG_SPEC {}
