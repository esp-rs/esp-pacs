#[doc = "Register `FSM` reader"]
pub type R = crate::R<FSM_SPEC>;
#[doc = "Register `FSM` writer"]
pub type W = crate::W<FSM_SPEC>;
#[doc = "Field `SAMPLE_NUM` reader - sample number"]
pub type SAMPLE_NUM_R = crate::FieldReader;
#[doc = "Field `SAMPLE_NUM` writer - sample number"]
pub type SAMPLE_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAMPLE_CYCLE` reader - sample cycles"]
pub type SAMPLE_CYCLE_R = crate::FieldReader;
#[doc = "Field `SAMPLE_CYCLE` writer - sample cycles"]
pub type SAMPLE_CYCLE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 16:23 - sample number"]
    #[inline(always)]
    pub fn sample_num(&self) -> SAMPLE_NUM_R {
        SAMPLE_NUM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - sample cycles"]
    #[inline(always)]
    pub fn sample_cycle(&self) -> SAMPLE_CYCLE_R {
        SAMPLE_CYCLE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM")
            .field("sample_num", &self.sample_num())
            .field("sample_cycle", &self.sample_cycle())
            .finish()
    }
}
impl W {
    #[doc = "Bits 16:23 - sample number"]
    #[inline(always)]
    pub fn sample_num(&mut self) -> SAMPLE_NUM_W<'_, FSM_SPEC> {
        SAMPLE_NUM_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - sample cycles"]
    #[inline(always)]
    pub fn sample_cycle(&mut self) -> SAMPLE_CYCLE_W<'_, FSM_SPEC> {
        SAMPLE_CYCLE_W::new(self, 24)
    }
}
#[doc = "digital adc control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSM_SPEC;
impl crate::RegisterSpec for FSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm::R`](R) reader structure"]
impl crate::Readable for FSM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fsm::W`](W) writer structure"]
impl crate::Writable for FSM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FSM to value 0x0200_0000"]
impl crate::Resettable for FSM_SPEC {
    const RESET_VALUE: u32 = 0x0200_0000;
}
