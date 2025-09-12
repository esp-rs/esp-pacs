#[doc = "Register `PRO_TRACE_0` reader"]
pub type R = crate::R<PRO_TRACE_0_SPEC>;
#[doc = "Register `PRO_TRACE_0` writer"]
pub type W = crate::W<PRO_TRACE_0_SPEC>;
#[doc = "Field `PRO_TRACE_LOCK` reader - Lock register. Setting to 1 locks trace function permission control registers."]
pub type PRO_TRACE_LOCK_R = crate::BitReader;
#[doc = "Field `PRO_TRACE_LOCK` writer - Lock register. Setting to 1 locks trace function permission control registers."]
pub type PRO_TRACE_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks trace function permission control registers."]
    #[inline(always)]
    pub fn pro_trace_lock(&self) -> PRO_TRACE_LOCK_R {
        PRO_TRACE_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_TRACE_0")
            .field("pro_trace_lock", &self.pro_trace_lock())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks trace function permission control registers."]
    #[inline(always)]
    pub fn pro_trace_lock(&mut self) -> PRO_TRACE_LOCK_W<'_, PRO_TRACE_0_SPEC> {
        PRO_TRACE_LOCK_W::new(self, 0)
    }
}
#[doc = "Trace memory permission control register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_trace_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_trace_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_TRACE_0_SPEC;
impl crate::RegisterSpec for PRO_TRACE_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_trace_0::R`](R) reader structure"]
impl crate::Readable for PRO_TRACE_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_trace_0::W`](W) writer structure"]
impl crate::Writable for PRO_TRACE_0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRO_TRACE_0 to value 0"]
impl crate::Resettable for PRO_TRACE_0_SPEC {}
