#[doc = "Register `FILTER_COMPARATOR_CONTROL` reader"]
pub type R = crate::R<FILTER_COMPARATOR_CONTROL_SPEC>;
#[doc = "Register `FILTER_COMPARATOR_CONTROL` writer"]
pub type W = crate::W<FILTER_COMPARATOR_CONTROL_SPEC>;
#[doc = "Field `P_INPUT` reader - Configures the input of the primary comparator for matching: \\\\0: iaddr \\\\1: tval\\\\"]
pub type P_INPUT_R = crate::BitReader;
#[doc = "Field `P_INPUT` writer - Configures the input of the primary comparator for matching: \\\\0: iaddr \\\\1: tval\\\\"]
pub type P_INPUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P_FUNCTION` reader - Configures the function for the primary comparator. \\\\0: Equal, \\\\1: Not equal, \\\\2: Less than, \\\\3: Less than or equal, \\\\4: Greater than, \\\\5: Greater than or equal, \\\\Other: Always match"]
pub type P_FUNCTION_R = crate::FieldReader;
#[doc = "Field `P_FUNCTION` writer - Configures the function for the primary comparator. \\\\0: Equal, \\\\1: Not equal, \\\\2: Less than, \\\\3: Less than or equal, \\\\4: Greater than, \\\\5: Greater than or equal, \\\\Other: Always match"]
pub type P_FUNCTION_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `P_NOTIFY` reader - Configure whether to explicitly report an instruction address matched against the primary comparator. \\\\0:Not report \\\\1:Report"]
pub type P_NOTIFY_R = crate::BitReader;
#[doc = "Field `P_NOTIFY` writer - Configure whether to explicitly report an instruction address matched against the primary comparator. \\\\0:Not report \\\\1:Report"]
pub type P_NOTIFY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_INPUT` reader - Configures the input of the secondary comparator for matching: \\\\0: iaddr \\\\1: tval\\\\"]
pub type S_INPUT_R = crate::BitReader;
#[doc = "Field `S_INPUT` writer - Configures the input of the secondary comparator for matching: \\\\0: iaddr \\\\1: tval\\\\"]
pub type S_INPUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_FUNCTION` reader - Configures the function for the secondary comparator. \\\\0: Equal, \\\\1: Not equal, \\\\2: Less than, \\\\3: Less than or equal, \\\\4: Greater than, \\\\5: Greater than or equal, \\\\Other: Always match"]
pub type S_FUNCTION_R = crate::FieldReader;
#[doc = "Field `S_FUNCTION` writer - Configures the function for the secondary comparator. \\\\0: Equal, \\\\1: Not equal, \\\\2: Less than, \\\\3: Less than or equal, \\\\4: Greater than, \\\\5: Greater than or equal, \\\\Other: Always match"]
pub type S_FUNCTION_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `S_NOTIFY` reader - Generate a trace packet explicitly reporting the address that cause the secondary match"]
pub type S_NOTIFY_R = crate::BitReader;
#[doc = "Field `S_NOTIFY` writer - Generate a trace packet explicitly reporting the address that cause the secondary match"]
pub type S_NOTIFY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCH_MODE` reader - Configures the comparator match mode: \\\\0: Only the primary comparator matches \\\\1: Both primary and secondary comparator matches(P\\&\\&S) \\\\ 2:Neither primary or secondary comparator matches !(P\\&\\&S) \\\\3: Start filtering when the primary comparator matches and stop filtering when the secondary comparator matches\\\\"]
pub type MATCH_MODE_R = crate::FieldReader;
#[doc = "Field `MATCH_MODE` writer - Configures the comparator match mode: \\\\0: Only the primary comparator matches \\\\1: Both primary and secondary comparator matches(P\\&\\&S) \\\\ 2:Neither primary or secondary comparator matches !(P\\&\\&S) \\\\3: Start filtering when the primary comparator matches and stop filtering when the secondary comparator matches\\\\"]
pub type MATCH_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Configures the input of the primary comparator for matching: \\\\0: iaddr \\\\1: tval\\\\"]
    #[inline(always)]
    pub fn p_input(&self) -> P_INPUT_R {
        P_INPUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:4 - Configures the function for the primary comparator. \\\\0: Equal, \\\\1: Not equal, \\\\2: Less than, \\\\3: Less than or equal, \\\\4: Greater than, \\\\5: Greater than or equal, \\\\Other: Always match"]
    #[inline(always)]
    pub fn p_function(&self) -> P_FUNCTION_R {
        P_FUNCTION_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Configure whether to explicitly report an instruction address matched against the primary comparator. \\\\0:Not report \\\\1:Report"]
    #[inline(always)]
    pub fn p_notify(&self) -> P_NOTIFY_R {
        P_NOTIFY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Configures the input of the secondary comparator for matching: \\\\0: iaddr \\\\1: tval\\\\"]
    #[inline(always)]
    pub fn s_input(&self) -> S_INPUT_R {
        S_INPUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:12 - Configures the function for the secondary comparator. \\\\0: Equal, \\\\1: Not equal, \\\\2: Less than, \\\\3: Less than or equal, \\\\4: Greater than, \\\\5: Greater than or equal, \\\\Other: Always match"]
    #[inline(always)]
    pub fn s_function(&self) -> S_FUNCTION_R {
        S_FUNCTION_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 13 - Generate a trace packet explicitly reporting the address that cause the secondary match"]
    #[inline(always)]
    pub fn s_notify(&self) -> S_NOTIFY_R {
        S_NOTIFY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Configures the comparator match mode: \\\\0: Only the primary comparator matches \\\\1: Both primary and secondary comparator matches(P\\&\\&S) \\\\ 2:Neither primary or secondary comparator matches !(P\\&\\&S) \\\\3: Start filtering when the primary comparator matches and stop filtering when the secondary comparator matches\\\\"]
    #[inline(always)]
    pub fn match_mode(&self) -> MATCH_MODE_R {
        MATCH_MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FILTER_COMPARATOR_CONTROL")
            .field("p_input", &self.p_input())
            .field("p_function", &self.p_function())
            .field("p_notify", &self.p_notify())
            .field("s_input", &self.s_input())
            .field("s_function", &self.s_function())
            .field("s_notify", &self.s_notify())
            .field("match_mode", &self.match_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures the input of the primary comparator for matching: \\\\0: iaddr \\\\1: tval\\\\"]
    #[inline(always)]
    pub fn p_input(&mut self) -> P_INPUT_W<FILTER_COMPARATOR_CONTROL_SPEC> {
        P_INPUT_W::new(self, 0)
    }
    #[doc = "Bits 2:4 - Configures the function for the primary comparator. \\\\0: Equal, \\\\1: Not equal, \\\\2: Less than, \\\\3: Less than or equal, \\\\4: Greater than, \\\\5: Greater than or equal, \\\\Other: Always match"]
    #[inline(always)]
    pub fn p_function(&mut self) -> P_FUNCTION_W<FILTER_COMPARATOR_CONTROL_SPEC> {
        P_FUNCTION_W::new(self, 2)
    }
    #[doc = "Bit 5 - Configure whether to explicitly report an instruction address matched against the primary comparator. \\\\0:Not report \\\\1:Report"]
    #[inline(always)]
    pub fn p_notify(&mut self) -> P_NOTIFY_W<FILTER_COMPARATOR_CONTROL_SPEC> {
        P_NOTIFY_W::new(self, 5)
    }
    #[doc = "Bit 8 - Configures the input of the secondary comparator for matching: \\\\0: iaddr \\\\1: tval\\\\"]
    #[inline(always)]
    pub fn s_input(&mut self) -> S_INPUT_W<FILTER_COMPARATOR_CONTROL_SPEC> {
        S_INPUT_W::new(self, 8)
    }
    #[doc = "Bits 10:12 - Configures the function for the secondary comparator. \\\\0: Equal, \\\\1: Not equal, \\\\2: Less than, \\\\3: Less than or equal, \\\\4: Greater than, \\\\5: Greater than or equal, \\\\Other: Always match"]
    #[inline(always)]
    pub fn s_function(&mut self) -> S_FUNCTION_W<FILTER_COMPARATOR_CONTROL_SPEC> {
        S_FUNCTION_W::new(self, 10)
    }
    #[doc = "Bit 13 - Generate a trace packet explicitly reporting the address that cause the secondary match"]
    #[inline(always)]
    pub fn s_notify(&mut self) -> S_NOTIFY_W<FILTER_COMPARATOR_CONTROL_SPEC> {
        S_NOTIFY_W::new(self, 13)
    }
    #[doc = "Bits 16:17 - Configures the comparator match mode: \\\\0: Only the primary comparator matches \\\\1: Both primary and secondary comparator matches(P\\&\\&S) \\\\ 2:Neither primary or secondary comparator matches !(P\\&\\&S) \\\\3: Start filtering when the primary comparator matches and stop filtering when the secondary comparator matches\\\\"]
    #[inline(always)]
    pub fn match_mode(&mut self) -> MATCH_MODE_W<FILTER_COMPARATOR_CONTROL_SPEC> {
        MATCH_MODE_W::new(self, 16)
    }
}
#[doc = "filter comparator match control register\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_comparator_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_comparator_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FILTER_COMPARATOR_CONTROL_SPEC;
impl crate::RegisterSpec for FILTER_COMPARATOR_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter_comparator_control::R`](R) reader structure"]
impl crate::Readable for FILTER_COMPARATOR_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`filter_comparator_control::W`](W) writer structure"]
impl crate::Writable for FILTER_COMPARATOR_CONTROL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FILTER_COMPARATOR_CONTROL to value 0"]
impl crate::Resettable for FILTER_COMPARATOR_CONTROL_SPEC {}
