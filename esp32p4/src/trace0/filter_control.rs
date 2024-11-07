#[doc = "Register `FILTER_CONTROL` reader"]
pub type R = crate::R<FILTER_CONTROL_SPEC>;
#[doc = "Register `FILTER_CONTROL` writer"]
pub type W = crate::W<FILTER_CONTROL_SPEC>;
#[doc = "Field `FILTER_EN` reader - Configure whether or not enable filter unit. \\\\1: enable filter.\\\\ 0: always match"]
pub type FILTER_EN_R = crate::BitReader;
#[doc = "Field `FILTER_EN` writer - Configure whether or not enable filter unit. \\\\1: enable filter.\\\\ 0: always match"]
pub type FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCH_COMP` reader - when set, the comparator must be high in order for the filter to match"]
pub type MATCH_COMP_R = crate::BitReader;
#[doc = "Field `MATCH_COMP` writer - when set, the comparator must be high in order for the filter to match"]
pub type MATCH_COMP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCH_PRIVILEGE` reader - when set, match privilege levels specified by \\hyperref\\[fielddesc:TRACEMATCHCHOICEPRIVILEGE\\]{TRACE_MATCH_CHOICE_PRIVILEGE}."]
pub type MATCH_PRIVILEGE_R = crate::BitReader;
#[doc = "Field `MATCH_PRIVILEGE` writer - when set, match privilege levels specified by \\hyperref\\[fielddesc:TRACEMATCHCHOICEPRIVILEGE\\]{TRACE_MATCH_CHOICE_PRIVILEGE}."]
pub type MATCH_PRIVILEGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCH_ECAUSE` reader - when set, start matching from exception cause codes specified by \\hyperref\\[fielddesc:TRACEMATCHCHOICEECAUSE\\]{TRACE_MATCH_CHOICE_ECAUSE}, and stop matching upon return from the 1st matching exception."]
pub type MATCH_ECAUSE_R = crate::BitReader;
#[doc = "Field `MATCH_ECAUSE` writer - when set, start matching from exception cause codes specified by \\hyperref\\[fielddesc:TRACEMATCHCHOICEECAUSE\\]{TRACE_MATCH_CHOICE_ECAUSE}, and stop matching upon return from the 1st matching exception."]
pub type MATCH_ECAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCH_INTERRUPT` reader - when set, start matching from a trap with the interrupt level codes specified by \\hyperref\\[fielddesc:TRACEMATCHVALUEINTERRUPT\\]{TRACE_MATCH_VALUE_INTERRUPT}, and stop matching upon return from the 1st matching trap."]
pub type MATCH_INTERRUPT_R = crate::BitReader;
#[doc = "Field `MATCH_INTERRUPT` writer - when set, start matching from a trap with the interrupt level codes specified by \\hyperref\\[fielddesc:TRACEMATCHVALUEINTERRUPT\\]{TRACE_MATCH_VALUE_INTERRUPT}, and stop matching upon return from the 1st matching trap."]
pub type MATCH_INTERRUPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configure whether or not enable filter unit. \\\\1: enable filter.\\\\ 0: always match"]
    #[inline(always)]
    pub fn filter_en(&self) -> FILTER_EN_R {
        FILTER_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - when set, the comparator must be high in order for the filter to match"]
    #[inline(always)]
    pub fn match_comp(&self) -> MATCH_COMP_R {
        MATCH_COMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - when set, match privilege levels specified by \\hyperref\\[fielddesc:TRACEMATCHCHOICEPRIVILEGE\\]{TRACE_MATCH_CHOICE_PRIVILEGE}."]
    #[inline(always)]
    pub fn match_privilege(&self) -> MATCH_PRIVILEGE_R {
        MATCH_PRIVILEGE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - when set, start matching from exception cause codes specified by \\hyperref\\[fielddesc:TRACEMATCHCHOICEECAUSE\\]{TRACE_MATCH_CHOICE_ECAUSE}, and stop matching upon return from the 1st matching exception."]
    #[inline(always)]
    pub fn match_ecause(&self) -> MATCH_ECAUSE_R {
        MATCH_ECAUSE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - when set, start matching from a trap with the interrupt level codes specified by \\hyperref\\[fielddesc:TRACEMATCHVALUEINTERRUPT\\]{TRACE_MATCH_VALUE_INTERRUPT}, and stop matching upon return from the 1st matching trap."]
    #[inline(always)]
    pub fn match_interrupt(&self) -> MATCH_INTERRUPT_R {
        MATCH_INTERRUPT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FILTER_CONTROL")
            .field("filter_en", &self.filter_en())
            .field("match_comp", &self.match_comp())
            .field("match_privilege", &self.match_privilege())
            .field("match_ecause", &self.match_ecause())
            .field("match_interrupt", &self.match_interrupt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configure whether or not enable filter unit. \\\\1: enable filter.\\\\ 0: always match"]
    #[inline(always)]
    pub fn filter_en(&mut self) -> FILTER_EN_W<FILTER_CONTROL_SPEC> {
        FILTER_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - when set, the comparator must be high in order for the filter to match"]
    #[inline(always)]
    pub fn match_comp(&mut self) -> MATCH_COMP_W<FILTER_CONTROL_SPEC> {
        MATCH_COMP_W::new(self, 1)
    }
    #[doc = "Bit 2 - when set, match privilege levels specified by \\hyperref\\[fielddesc:TRACEMATCHCHOICEPRIVILEGE\\]{TRACE_MATCH_CHOICE_PRIVILEGE}."]
    #[inline(always)]
    pub fn match_privilege(&mut self) -> MATCH_PRIVILEGE_W<FILTER_CONTROL_SPEC> {
        MATCH_PRIVILEGE_W::new(self, 2)
    }
    #[doc = "Bit 3 - when set, start matching from exception cause codes specified by \\hyperref\\[fielddesc:TRACEMATCHCHOICEECAUSE\\]{TRACE_MATCH_CHOICE_ECAUSE}, and stop matching upon return from the 1st matching exception."]
    #[inline(always)]
    pub fn match_ecause(&mut self) -> MATCH_ECAUSE_W<FILTER_CONTROL_SPEC> {
        MATCH_ECAUSE_W::new(self, 3)
    }
    #[doc = "Bit 4 - when set, start matching from a trap with the interrupt level codes specified by \\hyperref\\[fielddesc:TRACEMATCHVALUEINTERRUPT\\]{TRACE_MATCH_VALUE_INTERRUPT}, and stop matching upon return from the 1st matching trap."]
    #[inline(always)]
    pub fn match_interrupt(&mut self) -> MATCH_INTERRUPT_W<FILTER_CONTROL_SPEC> {
        MATCH_INTERRUPT_W::new(self, 4)
    }
}
#[doc = "filter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FILTER_CONTROL_SPEC;
impl crate::RegisterSpec for FILTER_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter_control::R`](R) reader structure"]
impl crate::Readable for FILTER_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`filter_control::W`](W) writer structure"]
impl crate::Writable for FILTER_CONTROL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FILTER_CONTROL to value 0"]
impl crate::Resettable for FILTER_CONTROL_SPEC {
    const RESET_VALUE: u32 = 0;
}
