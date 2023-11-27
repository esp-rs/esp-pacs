#[doc = "Register `LP_INT_ENA` reader"]
pub type R = crate::R<LP_INT_ENA_SPEC>;
#[doc = "Register `LP_INT_ENA` writer"]
pub type W = crate::W<LP_INT_ENA_SPEC>;
#[doc = "Field `MAIN_TIMER_OVERFLOW_LP_INT_ENA` reader - need_des"]
pub type MAIN_TIMER_OVERFLOW_LP_INT_ENA_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_OVERFLOW_LP_INT_ENA` writer - need_des"]
pub type MAIN_TIMER_OVERFLOW_LP_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_TIMER_LP_INT_ENA` reader - need_des"]
pub type MAIN_TIMER_LP_INT_ENA_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_LP_INT_ENA` writer - need_des"]
pub type MAIN_TIMER_LP_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn main_timer_overflow_lp_int_ena(&self) -> MAIN_TIMER_OVERFLOW_LP_INT_ENA_R {
        MAIN_TIMER_OVERFLOW_LP_INT_ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn main_timer_lp_int_ena(&self) -> MAIN_TIMER_LP_INT_ENA_R {
        MAIN_TIMER_LP_INT_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_INT_ENA")
            .field(
                "main_timer_overflow_lp_int_ena",
                &format_args!("{}", self.main_timer_overflow_lp_int_ena().bit()),
            )
            .field(
                "main_timer_lp_int_ena",
                &format_args!("{}", self.main_timer_lp_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_overflow_lp_int_ena(
        &mut self,
    ) -> MAIN_TIMER_OVERFLOW_LP_INT_ENA_W<LP_INT_ENA_SPEC> {
        MAIN_TIMER_OVERFLOW_LP_INT_ENA_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_lp_int_ena(&mut self) -> MAIN_TIMER_LP_INT_ENA_W<LP_INT_ENA_SPEC> {
        MAIN_TIMER_LP_INT_ENA_W::new(self, 31)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_INT_ENA_SPEC;
impl crate::RegisterSpec for LP_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_int_ena::R`](R) reader structure"]
impl crate::Readable for LP_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_int_ena::W`](W) writer structure"]
impl crate::Writable for LP_INT_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_INT_ENA to value 0"]
impl crate::Resettable for LP_INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
