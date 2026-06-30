#[doc = "Register `CONDITION_KEY2` reader"]
pub type R = crate::R<CONDITION_KEY2_SPEC>;
#[doc = "Register `CONDITION_KEY2` writer"]
pub type W = crate::W<CONDITION_KEY2_SPEC>;
#[doc = "Field `CONDITION_KEY2` reader - condition key"]
pub type CONDITION_KEY2_R = crate::FieldReader<u32>;
#[doc = "Field `CONDITION_KEY2` writer - condition key"]
pub type CONDITION_KEY2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - condition key"]
    #[inline(always)]
    pub fn condition_key2(&self) -> CONDITION_KEY2_R {
        CONDITION_KEY2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONDITION_KEY2")
            .field("condition_key2", &self.condition_key2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - condition key"]
    #[inline(always)]
    pub fn condition_key2(&mut self) -> CONDITION_KEY2_W<'_, CONDITION_KEY2_SPEC> {
        CONDITION_KEY2_W::new(self, 0)
    }
}
#[doc = "condition key reg\n\nYou can [`read`](crate::Reg::read) this register and get [`condition_key2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`condition_key2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONDITION_KEY2_SPEC;
impl crate::RegisterSpec for CONDITION_KEY2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`condition_key2::R`](R) reader structure"]
impl crate::Readable for CONDITION_KEY2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`condition_key2::W`](W) writer structure"]
impl crate::Writable for CONDITION_KEY2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONDITION_KEY2 to value 0"]
impl crate::Resettable for CONDITION_KEY2_SPEC {}
