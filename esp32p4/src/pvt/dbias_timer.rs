#[doc = "Register `DBIAS_TIMER` reader"]
pub type R = crate::R<DBIAS_TIMER_SPEC>;
#[doc = "Register `DBIAS_TIMER` writer"]
pub type W = crate::W<DBIAS_TIMER_SPEC>;
#[doc = "Field `TIMER_TARGET` reader - needs field desc"]
pub type TIMER_TARGET_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER_TARGET` writer - needs field desc"]
pub type TIMER_TARGET_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TIMER_EN` reader - needs field desc"]
pub type TIMER_EN_R = crate::BitReader;
#[doc = "Field `TIMER_EN` writer - needs field desc"]
pub type TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 15:30 - needs field desc"]
    #[inline(always)]
    pub fn timer_target(&self) -> TIMER_TARGET_R {
        TIMER_TARGET_R::new(((self.bits >> 15) & 0xffff) as u16)
    }
    #[doc = "Bit 31 - needs field desc"]
    #[inline(always)]
    pub fn timer_en(&self) -> TIMER_EN_R {
        TIMER_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBIAS_TIMER")
            .field(
                "timer_target",
                &format_args!("{}", self.timer_target().bits()),
            )
            .field("timer_en", &format_args!("{}", self.timer_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DBIAS_TIMER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 15:30 - needs field desc"]
    #[inline(always)]
    #[must_use]
    pub fn timer_target(&mut self) -> TIMER_TARGET_W<DBIAS_TIMER_SPEC> {
        TIMER_TARGET_W::new(self, 15)
    }
    #[doc = "Bit 31 - needs field desc"]
    #[inline(always)]
    #[must_use]
    pub fn timer_en(&mut self) -> TIMER_EN_W<DBIAS_TIMER_SPEC> {
        TIMER_EN_W::new(self, 31)
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
#[doc = "needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbias_timer::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbias_timer::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBIAS_TIMER_SPEC;
impl crate::RegisterSpec for DBIAS_TIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbias_timer::R`](R) reader structure"]
impl crate::Readable for DBIAS_TIMER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbias_timer::W`](W) writer structure"]
impl crate::Writable for DBIAS_TIMER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBIAS_TIMER to value 0x7fff_8000"]
impl crate::Resettable for DBIAS_TIMER_SPEC {
    const RESET_VALUE: Self::Ux = 0x7fff_8000;
}
