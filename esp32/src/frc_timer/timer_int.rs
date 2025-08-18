#[doc = "Register `TIMER_INT` reader"]
pub type R = crate::R<TIMER_INT_SPEC>;
#[doc = "Register `TIMER_INT` writer"]
pub type W = crate::W<TIMER_INT_SPEC>;
#[doc = "Field `CLR` reader - "]
pub type CLR_R = crate::BitReader;
#[doc = "Field `CLR` writer - "]
pub type CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clr(&self) -> CLR_R {
        CLR_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_INT")
            .field("clr", &self.clr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clr(&mut self) -> CLR_W<'_, TIMER_INT_SPEC> {
        CLR_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_INT_SPEC;
impl crate::RegisterSpec for TIMER_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_int::R`](R) reader structure"]
impl crate::Readable for TIMER_INT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer_int::W`](W) writer structure"]
impl crate::Writable for TIMER_INT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMER_INT to value 0"]
impl crate::Resettable for TIMER_INT_SPEC {}
