#[doc = "Register `TBTT_START` reader"]
pub type R = crate::R<TBTT_START_SPEC>;
#[doc = "Register `TBTT_START` writer"]
pub type W = crate::W<TBTT_START_SPEC>;
#[doc = "Field `START_TIME` reader - "]
pub type START_TIME_R = crate::FieldReader<u32>;
#[doc = "Field `START_TIME` writer - "]
pub type START_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn start_time(&self) -> START_TIME_R {
        START_TIME_R::new(self.bits & 0x03ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TBTT_START")
            .field("start_time", &self.start_time())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn start_time(&mut self) -> START_TIME_W<'_, TBTT_START_SPEC> {
        START_TIME_W::new(self, 0)
    }
}
#[doc = "TBTT start time, loaded into an interface with TSF_CTRL.LOAD_TBTT_START\n\nYou can [`read`](crate::Reg::read) this register and get [`tbtt_start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbtt_start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBTT_START_SPEC;
impl crate::RegisterSpec for TBTT_START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbtt_start::R`](R) reader structure"]
impl crate::Readable for TBTT_START_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tbtt_start::W`](W) writer structure"]
impl crate::Writable for TBTT_START_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TBTT_START to value 0"]
impl crate::Resettable for TBTT_START_SPEC {}
