#[doc = "Register `SIGMADELTA%s` reader"]
pub type R = crate::R<SIGMADELTA_SPEC>;
#[doc = "Register `SIGMADELTA%s` writer"]
pub type W = crate::W<SIGMADELTA_SPEC>;
#[doc = "Field `IN` reader - "]
pub type IN_R = crate::FieldReader;
#[doc = "Field `IN` writer - "]
pub type IN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRESCALE` reader - "]
pub type PRESCALE_R = crate::FieldReader;
#[doc = "Field `PRESCALE` writer - "]
pub type PRESCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn in_(&self) -> IN_R {
        IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SIGMADELTA")
            .field("in_", &self.in_())
            .field("prescale", &self.prescale())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn in_(&mut self) -> IN_W<SIGMADELTA_SPEC> {
        IN_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn prescale(&mut self) -> PRESCALE_W<SIGMADELTA_SPEC> {
        PRESCALE_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sigmadelta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sigmadelta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIGMADELTA_SPEC;
impl crate::RegisterSpec for SIGMADELTA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sigmadelta::R`](R) reader structure"]
impl crate::Readable for SIGMADELTA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sigmadelta::W`](W) writer structure"]
impl crate::Writable for SIGMADELTA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIGMADELTA%s to value 0xff00"]
impl crate::Resettable for SIGMADELTA_SPEC {
    const RESET_VALUE: u32 = 0xff00;
}
