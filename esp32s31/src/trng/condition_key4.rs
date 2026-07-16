#[doc = "Register `CONDITION_KEY4` reader"]
pub type R = crate::R<CONDITION_KEY4_SPEC>;
#[doc = "Register `CONDITION_KEY4` writer"]
pub type W = crate::W<CONDITION_KEY4_SPEC>;
#[doc = "Field `CONDITION_KEY4` reader - condition key"]
pub type CONDITION_KEY4_R = crate::FieldReader<u32>;
#[doc = "Field `CONDITION_KEY4` writer - condition key"]
pub type CONDITION_KEY4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - condition key"]
    #[inline(always)]
    pub fn condition_key4(&self) -> CONDITION_KEY4_R {
        CONDITION_KEY4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONDITION_KEY4")
            .field("condition_key4", &self.condition_key4())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - condition key"]
    #[inline(always)]
    pub fn condition_key4(&mut self) -> CONDITION_KEY4_W<'_, CONDITION_KEY4_SPEC> {
        CONDITION_KEY4_W::new(self, 0)
    }
}
#[doc = "condition key reg\n\nYou can [`read`](crate::Reg::read) this register and get [`condition_key4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`condition_key4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONDITION_KEY4_SPEC;
impl crate::RegisterSpec for CONDITION_KEY4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`condition_key4::R`](R) reader structure"]
impl crate::Readable for CONDITION_KEY4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`condition_key4::W`](W) writer structure"]
impl crate::Writable for CONDITION_KEY4_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONDITION_KEY4 to value 0"]
impl crate::Resettable for CONDITION_KEY4_SPEC {}
