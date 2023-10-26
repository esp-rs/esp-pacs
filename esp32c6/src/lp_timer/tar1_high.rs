#[doc = "Register `TAR1_HIGH` reader"]
pub type R = crate::R<TAR1_HIGH_SPEC>;
#[doc = "Register `TAR1_HIGH` writer"]
pub type W = crate::W<TAR1_HIGH_SPEC>;
#[doc = "Field `MAIN_TIMER_TAR_HIGH1` reader - need_des"]
pub type MAIN_TIMER_TAR_HIGH1_R = crate::FieldReader<u16>;
#[doc = "Field `MAIN_TIMER_TAR_HIGH1` writer - need_des"]
pub type MAIN_TIMER_TAR_HIGH1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
#[doc = "Field `MAIN_TIMER_TAR_EN1` writer - need_des"]
pub type MAIN_TIMER_TAR_EN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn main_timer_tar_high1(&self) -> MAIN_TIMER_TAR_HIGH1_R {
        MAIN_TIMER_TAR_HIGH1_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAR1_HIGH")
            .field(
                "main_timer_tar_high1",
                &format_args!("{}", self.main_timer_tar_high1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TAR1_HIGH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_tar_high1(&mut self) -> MAIN_TIMER_TAR_HIGH1_W<TAR1_HIGH_SPEC, 0> {
        MAIN_TIMER_TAR_HIGH1_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_tar_en1(&mut self) -> MAIN_TIMER_TAR_EN1_W<TAR1_HIGH_SPEC, 31> {
        MAIN_TIMER_TAR_EN1_W::new(self)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tar1_high::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tar1_high::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAR1_HIGH_SPEC;
impl crate::RegisterSpec for TAR1_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tar1_high::R`](R) reader structure"]
impl crate::Readable for TAR1_HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tar1_high::W`](W) writer structure"]
impl crate::Writable for TAR1_HIGH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAR1_HIGH to value 0"]
impl crate::Resettable for TAR1_HIGH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
