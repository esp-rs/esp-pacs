#[doc = "Register `TRIGGER` writer"]
pub type W = crate::W<TRIGGER_SPEC>;
#[doc = "Field `START` writer - Start tigger typical rma. \\\\"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOAD` writer - Rma load done. \\\\"]
pub type LOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAIN` writer - Rma gain done. \\\\"]
pub type GAIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USC` writer - Rma usc done. \\\\"]
pub type USC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TRIGGER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Start tigger typical rma. \\\\"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<'_, TRIGGER_SPEC> {
        START_W::new(self, 0)
    }
    #[doc = "Bit 1 - Rma load done. \\\\"]
    #[inline(always)]
    pub fn load(&mut self) -> LOAD_W<'_, TRIGGER_SPEC> {
        LOAD_W::new(self, 1)
    }
    #[doc = "Bit 2 - Rma gain done. \\\\"]
    #[inline(always)]
    pub fn gain(&mut self) -> GAIN_W<'_, TRIGGER_SPEC> {
        GAIN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Rma usc done. \\\\"]
    #[inline(always)]
    pub fn usc(&mut self) -> USC_W<'_, TRIGGER_SPEC> {
        USC_W::new(self, 3)
    }
}
#[doc = "Starts the RMA module.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigger::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRIGGER_SPEC;
impl crate::RegisterSpec for TRIGGER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`trigger::W`](W) writer structure"]
impl crate::Writable for TRIGGER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRIGGER to value 0"]
impl crate::Resettable for TRIGGER_SPEC {}
