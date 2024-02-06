#[doc = "Register `FILTER_COMPARATOR_CONTROL` reader"]
pub type R = crate::R<FILTER_COMPARATOR_CONTROL_SPEC>;
#[doc = "Register `FILTER_COMPARATOR_CONTROL` writer"]
pub type W = crate::W<FILTER_COMPARATOR_CONTROL_SPEC>;
#[doc = "Field `P_INPUT` reader - Determines which input to compare against the primary comparator, \\\\0: iaddr, \\\\1: tval."]
pub type P_INPUT_R = crate::BitReader;
#[doc = "Field `P_INPUT` writer - Determines which input to compare against the primary comparator, \\\\0: iaddr, \\\\1: tval."]
pub type P_INPUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P_FUNCTION` reader - Select the primary comparator function. \\\\0: equal, \\\\1: not equal, \\\\2: less than, \\\\3: less than or equal, \\\\4: greater than, \\\\5: greater than or equal, \\\\other: always match"]
pub type P_FUNCTION_R = crate::FieldReader;
#[doc = "Field `P_FUNCTION` writer - Select the primary comparator function. \\\\0: equal, \\\\1: not equal, \\\\2: less than, \\\\3: less than or equal, \\\\4: greater than, \\\\5: greater than or equal, \\\\other: always match"]
pub type P_FUNCTION_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `P_NOTIFY` reader - Generate a trace packet explicitly reporting the address that cause the primary match"]
pub type P_NOTIFY_R = crate::BitReader;
#[doc = "Field `P_NOTIFY` writer - Generate a trace packet explicitly reporting the address that cause the primary match"]
pub type P_NOTIFY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_INPUT` reader - Determines which input to compare against the secondary comparator, \\\\0: iaddr, \\\\1: tval."]
pub type S_INPUT_R = crate::BitReader;
#[doc = "Field `S_INPUT` writer - Determines which input to compare against the secondary comparator, \\\\0: iaddr, \\\\1: tval."]
pub type S_INPUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S_FUNCTION` reader - Select the secondary comparator function. \\\\0: equal, \\\\1: not equal, \\\\2: less than, \\\\3: less than or equal, \\\\4: greater than, \\\\5: greater than or equal, \\\\other: always match"]
pub type S_FUNCTION_R = crate::FieldReader;
#[doc = "Field `S_FUNCTION` writer - Select the secondary comparator function. \\\\0: equal, \\\\1: not equal, \\\\2: less than, \\\\3: less than or equal, \\\\4: greater than, \\\\5: greater than or equal, \\\\other: always match"]
pub type S_FUNCTION_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `S_NOTIFY` reader - Generate a trace packet explicitly reporting the address that cause the secondary match"]
pub type S_NOTIFY_R = crate::BitReader;
#[doc = "Field `S_NOTIFY` writer - Generate a trace packet explicitly reporting the address that cause the secondary match"]
pub type S_NOTIFY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCH_MODE` reader - 0: only primary matches, \\\\1: primary and secondary comparator both matches(P\\&amp;\\&amp;S),\\\\ 2:either primary or secondary comparator matches !(P\\&amp;\\&amp;S), \\\\3: set when primary matches and continue to match until after secondary comparator matches"]
pub type MATCH_MODE_R = crate::FieldReader;
#[doc = "Field `MATCH_MODE` writer - 0: only primary matches, \\\\1: primary and secondary comparator both matches(P\\&amp;\\&amp;S),\\\\ 2:either primary or secondary comparator matches !(P\\&amp;\\&amp;S), \\\\3: set when primary matches and continue to match until after secondary comparator matches"]
pub type MATCH_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Determines which input to compare against the primary comparator, \\\\0: iaddr, \\\\1: tval."]
    #[inline(always)]
    pub fn p_input(&self) -> P_INPUT_R {
        P_INPUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:4 - Select the primary comparator function. \\\\0: equal, \\\\1: not equal, \\\\2: less than, \\\\3: less than or equal, \\\\4: greater than, \\\\5: greater than or equal, \\\\other: always match"]
    #[inline(always)]
    pub fn p_function(&self) -> P_FUNCTION_R {
        P_FUNCTION_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Generate a trace packet explicitly reporting the address that cause the primary match"]
    #[inline(always)]
    pub fn p_notify(&self) -> P_NOTIFY_R {
        P_NOTIFY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Determines which input to compare against the secondary comparator, \\\\0: iaddr, \\\\1: tval."]
    #[inline(always)]
    pub fn s_input(&self) -> S_INPUT_R {
        S_INPUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:12 - Select the secondary comparator function. \\\\0: equal, \\\\1: not equal, \\\\2: less than, \\\\3: less than or equal, \\\\4: greater than, \\\\5: greater than or equal, \\\\other: always match"]
    #[inline(always)]
    pub fn s_function(&self) -> S_FUNCTION_R {
        S_FUNCTION_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 13 - Generate a trace packet explicitly reporting the address that cause the secondary match"]
    #[inline(always)]
    pub fn s_notify(&self) -> S_NOTIFY_R {
        S_NOTIFY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 0: only primary matches, \\\\1: primary and secondary comparator both matches(P\\&amp;\\&amp;S),\\\\ 2:either primary or secondary comparator matches !(P\\&amp;\\&amp;S), \\\\3: set when primary matches and continue to match until after secondary comparator matches"]
    #[inline(always)]
    pub fn match_mode(&self) -> MATCH_MODE_R {
        MATCH_MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FILTER_COMPARATOR_CONTROL")
            .field("p_input", &format_args!("{}", self.p_input().bit()))
            .field("p_function", &format_args!("{}", self.p_function().bits()))
            .field("p_notify", &format_args!("{}", self.p_notify().bit()))
            .field("s_input", &format_args!("{}", self.s_input().bit()))
            .field("s_function", &format_args!("{}", self.s_function().bits()))
            .field("s_notify", &format_args!("{}", self.s_notify().bit()))
            .field("match_mode", &format_args!("{}", self.match_mode().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FILTER_COMPARATOR_CONTROL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Determines which input to compare against the primary comparator, \\\\0: iaddr, \\\\1: tval."]
    #[inline(always)]
    #[must_use]
    pub fn p_input(&mut self) -> P_INPUT_W<FILTER_COMPARATOR_CONTROL_SPEC> {
        P_INPUT_W::new(self, 0)
    }
    #[doc = "Bits 2:4 - Select the primary comparator function. \\\\0: equal, \\\\1: not equal, \\\\2: less than, \\\\3: less than or equal, \\\\4: greater than, \\\\5: greater than or equal, \\\\other: always match"]
    #[inline(always)]
    #[must_use]
    pub fn p_function(&mut self) -> P_FUNCTION_W<FILTER_COMPARATOR_CONTROL_SPEC> {
        P_FUNCTION_W::new(self, 2)
    }
    #[doc = "Bit 5 - Generate a trace packet explicitly reporting the address that cause the primary match"]
    #[inline(always)]
    #[must_use]
    pub fn p_notify(&mut self) -> P_NOTIFY_W<FILTER_COMPARATOR_CONTROL_SPEC> {
        P_NOTIFY_W::new(self, 5)
    }
    #[doc = "Bit 8 - Determines which input to compare against the secondary comparator, \\\\0: iaddr, \\\\1: tval."]
    #[inline(always)]
    #[must_use]
    pub fn s_input(&mut self) -> S_INPUT_W<FILTER_COMPARATOR_CONTROL_SPEC> {
        S_INPUT_W::new(self, 8)
    }
    #[doc = "Bits 10:12 - Select the secondary comparator function. \\\\0: equal, \\\\1: not equal, \\\\2: less than, \\\\3: less than or equal, \\\\4: greater than, \\\\5: greater than or equal, \\\\other: always match"]
    #[inline(always)]
    #[must_use]
    pub fn s_function(&mut self) -> S_FUNCTION_W<FILTER_COMPARATOR_CONTROL_SPEC> {
        S_FUNCTION_W::new(self, 10)
    }
    #[doc = "Bit 13 - Generate a trace packet explicitly reporting the address that cause the secondary match"]
    #[inline(always)]
    #[must_use]
    pub fn s_notify(&mut self) -> S_NOTIFY_W<FILTER_COMPARATOR_CONTROL_SPEC> {
        S_NOTIFY_W::new(self, 13)
    }
    #[doc = "Bits 16:17 - 0: only primary matches, \\\\1: primary and secondary comparator both matches(P\\&amp;\\&amp;S),\\\\ 2:either primary or secondary comparator matches !(P\\&amp;\\&amp;S), \\\\3: set when primary matches and continue to match until after secondary comparator matches"]
    #[inline(always)]
    #[must_use]
    pub fn match_mode(&mut self) -> MATCH_MODE_W<FILTER_COMPARATOR_CONTROL_SPEC> {
        MATCH_MODE_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "filter comparator match control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filter_comparator_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filter_comparator_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FILTER_COMPARATOR_CONTROL_SPEC;
impl crate::RegisterSpec for FILTER_COMPARATOR_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter_comparator_control::R`](R) reader structure"]
impl crate::Readable for FILTER_COMPARATOR_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`filter_comparator_control::W`](W) writer structure"]
impl crate::Writable for FILTER_COMPARATOR_CONTROL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FILTER_COMPARATOR_CONTROL to value 0"]
impl crate::Resettable for FILTER_COMPARATOR_CONTROL_SPEC {
    const RESET_VALUE: u32 = 0;
}
