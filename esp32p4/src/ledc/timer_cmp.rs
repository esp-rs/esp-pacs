#[doc = "Register `TIMER%s_CMP` reader"]
pub type R = crate::R<TIMER_CMP_SPEC>;
#[doc = "Register `TIMER%s_CMP` writer"]
pub type W = crate::W<TIMER_CMP_SPEC>;
#[doc = "Field `TIMER_CMP` reader - Configures the comparison value for LEDC timer%s."]
pub type TIMER_CMP_R = crate::FieldReader<u32>;
#[doc = "Field `TIMER_CMP` writer - Configures the comparison value for LEDC timer%s."]
pub type TIMER_CMP_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Configures the comparison value for LEDC timer%s."]
    #[inline(always)]
    pub fn timer_cmp(&self) -> TIMER_CMP_R {
        TIMER_CMP_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_CMP")
            .field("timer_cmp", &self.timer_cmp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - Configures the comparison value for LEDC timer%s."]
    #[inline(always)]
    #[must_use]
    pub fn timer_cmp(&mut self) -> TIMER_CMP_W<TIMER_CMP_SPEC> {
        TIMER_CMP_W::new(self, 0)
    }
}
#[doc = "Ledc timer%s compare value register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_cmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_cmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_CMP_SPEC;
impl crate::RegisterSpec for TIMER_CMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_cmp::R`](R) reader structure"]
impl crate::Readable for TIMER_CMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer_cmp::W`](W) writer structure"]
impl crate::Writable for TIMER_CMP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER%s_CMP to value 0"]
impl crate::Resettable for TIMER_CMP_SPEC {
    const RESET_VALUE: u32 = 0;
}
