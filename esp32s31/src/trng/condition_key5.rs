#[doc = "Register `CONDITION_KEY5` reader"]
pub type R = crate::R<CONDITION_KEY5_SPEC>;
#[doc = "Register `CONDITION_KEY5` writer"]
pub type W = crate::W<CONDITION_KEY5_SPEC>;
#[doc = "Field `CONDITION_KEY5` reader - condition key"]
pub type CONDITION_KEY5_R = crate::FieldReader<u32>;
#[doc = "Field `CONDITION_KEY5` writer - condition key"]
pub type CONDITION_KEY5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - condition key"]
    #[inline(always)]
    pub fn condition_key5(&self) -> CONDITION_KEY5_R {
        CONDITION_KEY5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONDITION_KEY5")
            .field("condition_key5", &self.condition_key5())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - condition key"]
    #[inline(always)]
    pub fn condition_key5(&mut self) -> CONDITION_KEY5_W<'_, CONDITION_KEY5_SPEC> {
        CONDITION_KEY5_W::new(self, 0)
    }
}
#[doc = "condition key reg\n\nYou can [`read`](crate::Reg::read) this register and get [`condition_key5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`condition_key5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONDITION_KEY5_SPEC;
impl crate::RegisterSpec for CONDITION_KEY5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`condition_key5::R`](R) reader structure"]
impl crate::Readable for CONDITION_KEY5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`condition_key5::W`](W) writer structure"]
impl crate::Writable for CONDITION_KEY5_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONDITION_KEY5 to value 0"]
impl crate::Resettable for CONDITION_KEY5_SPEC {}
