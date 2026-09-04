#[doc = "Register `OPERATOR_TIMERSEL` reader"]
pub type R = crate::R<OPERATOR_TIMERSEL_SPEC>;
#[doc = "Register `OPERATOR_TIMERSEL` writer"]
pub type W = crate::W<OPERATOR_TIMERSEL_SPEC>;
#[doc = "Field `OPERATOR_TIMERSEL(0-2)` reader - Configures which PWM timer will be the timing reference for PWM operator%s.\\\\0: Timer0\\\\1: Timer1\\\\2: Timer2\\\\3: Invalid, will select timer2"]
pub type OPERATOR_TIMERSEL_R = crate::FieldReader;
#[doc = "Field `OPERATOR_TIMERSEL(0-2)` writer - Configures which PWM timer will be the timing reference for PWM operator%s.\\\\0: Timer0\\\\1: Timer1\\\\2: Timer2\\\\3: Invalid, will select timer2"]
pub type OPERATOR_TIMERSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Configures which PWM timer will be the timing reference for PWM operator(0-2).\\\\0: Timer0\\\\1: Timer1\\\\2: Timer2\\\\3: Invalid, will select timer2"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OPERATOR0_TIMERSEL` field.</div>"]
    #[inline(always)]
    pub fn operator_timersel(&self, n: u8) -> OPERATOR_TIMERSEL_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        OPERATOR_TIMERSEL_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Configures which PWM timer will be the timing reference for PWM operator(0-2).\\\\0: Timer0\\\\1: Timer1\\\\2: Timer2\\\\3: Invalid, will select timer2"]
    #[inline(always)]
    pub fn operator_timersel_iter(&self) -> impl Iterator<Item = OPERATOR_TIMERSEL_R> + '_ {
        (0..3).map(move |n| OPERATOR_TIMERSEL_R::new(((self.bits >> (n * 2)) & 3) as u8))
    }
    #[doc = "Bits 0:1 - Configures which PWM timer will be the timing reference for PWM operator0.\\\\0: Timer0\\\\1: Timer1\\\\2: Timer2\\\\3: Invalid, will select timer2"]
    #[inline(always)]
    pub fn operator0_timersel(&self) -> OPERATOR_TIMERSEL_R {
        OPERATOR_TIMERSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Configures which PWM timer will be the timing reference for PWM operator1.\\\\0: Timer0\\\\1: Timer1\\\\2: Timer2\\\\3: Invalid, will select timer2"]
    #[inline(always)]
    pub fn operator1_timersel(&self) -> OPERATOR_TIMERSEL_R {
        OPERATOR_TIMERSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Configures which PWM timer will be the timing reference for PWM operator2.\\\\0: Timer0\\\\1: Timer1\\\\2: Timer2\\\\3: Invalid, will select timer2"]
    #[inline(always)]
    pub fn operator2_timersel(&self) -> OPERATOR_TIMERSEL_R {
        OPERATOR_TIMERSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPERATOR_TIMERSEL")
            .field("operator0_timersel", &self.operator0_timersel())
            .field("operator1_timersel", &self.operator1_timersel())
            .field("operator2_timersel", &self.operator2_timersel())
            .finish()
    }
}
impl W {
    #[doc = "Configures which PWM timer will be the timing reference for PWM operator(0-2).\\\\0: Timer0\\\\1: Timer1\\\\2: Timer2\\\\3: Invalid, will select timer2"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OPERATOR0_TIMERSEL` field.</div>"]
    #[inline(always)]
    pub fn operator_timersel(&mut self, n: u8) -> OPERATOR_TIMERSEL_W<'_, OPERATOR_TIMERSEL_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        OPERATOR_TIMERSEL_W::new(self, n * 2)
    }
    #[doc = "Bits 0:1 - Configures which PWM timer will be the timing reference for PWM operator0.\\\\0: Timer0\\\\1: Timer1\\\\2: Timer2\\\\3: Invalid, will select timer2"]
    #[inline(always)]
    pub fn operator0_timersel(&mut self) -> OPERATOR_TIMERSEL_W<'_, OPERATOR_TIMERSEL_SPEC> {
        OPERATOR_TIMERSEL_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Configures which PWM timer will be the timing reference for PWM operator1.\\\\0: Timer0\\\\1: Timer1\\\\2: Timer2\\\\3: Invalid, will select timer2"]
    #[inline(always)]
    pub fn operator1_timersel(&mut self) -> OPERATOR_TIMERSEL_W<'_, OPERATOR_TIMERSEL_SPEC> {
        OPERATOR_TIMERSEL_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Configures which PWM timer will be the timing reference for PWM operator2.\\\\0: Timer0\\\\1: Timer1\\\\2: Timer2\\\\3: Invalid, will select timer2"]
    #[inline(always)]
    pub fn operator2_timersel(&mut self) -> OPERATOR_TIMERSEL_W<'_, OPERATOR_TIMERSEL_SPEC> {
        OPERATOR_TIMERSEL_W::new(self, 4)
    }
}
#[doc = "PWM operator's timer select register\n\nYou can [`read`](crate::Reg::read) this register and get [`operator_timersel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`operator_timersel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPERATOR_TIMERSEL_SPEC;
impl crate::RegisterSpec for OPERATOR_TIMERSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`operator_timersel::R`](R) reader structure"]
impl crate::Readable for OPERATOR_TIMERSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`operator_timersel::W`](W) writer structure"]
impl crate::Writable for OPERATOR_TIMERSEL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPERATOR_TIMERSEL to value 0"]
impl crate::Resettable for OPERATOR_TIMERSEL_SPEC {}
