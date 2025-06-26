#[doc = "Register `MODEXP_MODE` reader"]
pub type R = crate::R<MODEXP_MODE_SPEC>;
#[doc = "Register `MODEXP_MODE` writer"]
pub type W = crate::W<MODEXP_MODE_SPEC>;
#[doc = "Field `MODEXP_MODE` reader - This register contains the mode of modular exponentiation."]
pub type MODEXP_MODE_R = crate::FieldReader;
#[doc = "Field `MODEXP_MODE` writer - This register contains the mode of modular exponentiation."]
pub type MODEXP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - This register contains the mode of modular exponentiation."]
    #[inline(always)]
    pub fn modexp_mode(&self) -> MODEXP_MODE_R {
        MODEXP_MODE_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODEXP_MODE")
            .field("modexp_mode", &self.modexp_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - This register contains the mode of modular exponentiation."]
    #[inline(always)]
    pub fn modexp_mode(&mut self) -> MODEXP_MODE_W<MODEXP_MODE_SPEC> {
        MODEXP_MODE_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`modexp_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modexp_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MODEXP_MODE_SPEC;
impl crate::RegisterSpec for MODEXP_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modexp_mode::R`](R) reader structure"]
impl crate::Readable for MODEXP_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`modexp_mode::W`](W) writer structure"]
impl crate::Writable for MODEXP_MODE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODEXP_MODE to value 0"]
impl crate::Resettable for MODEXP_MODE_SPEC {}
