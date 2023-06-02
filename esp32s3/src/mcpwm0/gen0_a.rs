#[doc = "Register `GEN0_A` reader"]
pub struct R(crate::R<GEN0_A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GEN0_A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GEN0_A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GEN0_A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GEN0_A` writer"]
pub struct W(crate::W<GEN0_A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GEN0_A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GEN0_A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GEN0_A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UTEZ` reader - Action on PWM0A triggered by event TEZ when timer increasing"]
pub type UTEZ_R = crate::FieldReader;
#[doc = "Field `UTEZ` writer - Action on PWM0A triggered by event TEZ when timer increasing"]
pub type UTEZ_W<'a, const O: u8> = crate::FieldWriter<'a, GEN0_A_SPEC, 2, O>;
#[doc = "Field `UTEP` reader - Action on PWM0A triggered by event TEP when timer increasing"]
pub type UTEP_R = crate::FieldReader;
#[doc = "Field `UTEP` writer - Action on PWM0A triggered by event TEP when timer increasing"]
pub type UTEP_W<'a, const O: u8> = crate::FieldWriter<'a, GEN0_A_SPEC, 2, O>;
#[doc = "Field `UTEA` reader - Action on PWM0A triggered by event TEA when timer increasing"]
pub type UTEA_R = crate::FieldReader;
#[doc = "Field `UTEA` writer - Action on PWM0A triggered by event TEA when timer increasing"]
pub type UTEA_W<'a, const O: u8> = crate::FieldWriter<'a, GEN0_A_SPEC, 2, O>;
#[doc = "Field `UTEB` reader - Action on PWM0A triggered by event TEB when timer increasing"]
pub type UTEB_R = crate::FieldReader;
#[doc = "Field `UTEB` writer - Action on PWM0A triggered by event TEB when timer increasing"]
pub type UTEB_W<'a, const O: u8> = crate::FieldWriter<'a, GEN0_A_SPEC, 2, O>;
#[doc = "Field `UT0` reader - Action on PWM0A triggered by event_t0 when timer increasing"]
pub type UT0_R = crate::FieldReader;
#[doc = "Field `UT0` writer - Action on PWM0A triggered by event_t0 when timer increasing"]
pub type UT0_W<'a, const O: u8> = crate::FieldWriter<'a, GEN0_A_SPEC, 2, O>;
#[doc = "Field `UT1` reader - Action on PWM0A triggered by event_t1 when timer increasing"]
pub type UT1_R = crate::FieldReader;
#[doc = "Field `UT1` writer - Action on PWM0A triggered by event_t1 when timer increasing"]
pub type UT1_W<'a, const O: u8> = crate::FieldWriter<'a, GEN0_A_SPEC, 2, O>;
#[doc = "Field `DTEZ` reader - Action on PWM0A triggered by event TEZ when timer decreasing"]
pub type DTEZ_R = crate::FieldReader;
#[doc = "Field `DTEZ` writer - Action on PWM0A triggered by event TEZ when timer decreasing"]
pub type DTEZ_W<'a, const O: u8> = crate::FieldWriter<'a, GEN0_A_SPEC, 2, O>;
#[doc = "Field `DTEP` reader - Action on PWM0A triggered by event TEP when timer decreasing"]
pub type DTEP_R = crate::FieldReader;
#[doc = "Field `DTEP` writer - Action on PWM0A triggered by event TEP when timer decreasing"]
pub type DTEP_W<'a, const O: u8> = crate::FieldWriter<'a, GEN0_A_SPEC, 2, O>;
#[doc = "Field `DTEA` reader - Action on PWM0A triggered by event TEA when timer decreasing"]
pub type DTEA_R = crate::FieldReader;
#[doc = "Field `DTEA` writer - Action on PWM0A triggered by event TEA when timer decreasing"]
pub type DTEA_W<'a, const O: u8> = crate::FieldWriter<'a, GEN0_A_SPEC, 2, O>;
#[doc = "Field `DTEB` reader - Action on PWM0A triggered by event TEB when timer decreasing"]
pub type DTEB_R = crate::FieldReader;
#[doc = "Field `DTEB` writer - Action on PWM0A triggered by event TEB when timer decreasing"]
pub type DTEB_W<'a, const O: u8> = crate::FieldWriter<'a, GEN0_A_SPEC, 2, O>;
#[doc = "Field `DT0` reader - Action on PWM0A triggered by event_t0 when timer decreasing"]
pub type DT0_R = crate::FieldReader;
#[doc = "Field `DT0` writer - Action on PWM0A triggered by event_t0 when timer decreasing"]
pub type DT0_W<'a, const O: u8> = crate::FieldWriter<'a, GEN0_A_SPEC, 2, O>;
#[doc = "Field `DT1` reader - Action on PWM0A triggered by event_t1 when timer decreasing. 0: no change, 1: low, 2: high, 3: toggle"]
pub type DT1_R = crate::FieldReader;
#[doc = "Field `DT1` writer - Action on PWM0A triggered by event_t1 when timer decreasing. 0: no change, 1: low, 2: high, 3: toggle"]
pub type DT1_W<'a, const O: u8> = crate::FieldWriter<'a, GEN0_A_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Action on PWM0A triggered by event TEZ when timer increasing"]
    #[inline(always)]
    pub fn utez(&self) -> UTEZ_R {
        UTEZ_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Action on PWM0A triggered by event TEP when timer increasing"]
    #[inline(always)]
    pub fn utep(&self) -> UTEP_R {
        UTEP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Action on PWM0A triggered by event TEA when timer increasing"]
    #[inline(always)]
    pub fn utea(&self) -> UTEA_R {
        UTEA_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Action on PWM0A triggered by event TEB when timer increasing"]
    #[inline(always)]
    pub fn uteb(&self) -> UTEB_R {
        UTEB_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Action on PWM0A triggered by event_t0 when timer increasing"]
    #[inline(always)]
    pub fn ut0(&self) -> UT0_R {
        UT0_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Action on PWM0A triggered by event_t1 when timer increasing"]
    #[inline(always)]
    pub fn ut1(&self) -> UT1_R {
        UT1_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Action on PWM0A triggered by event TEZ when timer decreasing"]
    #[inline(always)]
    pub fn dtez(&self) -> DTEZ_R {
        DTEZ_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Action on PWM0A triggered by event TEP when timer decreasing"]
    #[inline(always)]
    pub fn dtep(&self) -> DTEP_R {
        DTEP_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Action on PWM0A triggered by event TEA when timer decreasing"]
    #[inline(always)]
    pub fn dtea(&self) -> DTEA_R {
        DTEA_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Action on PWM0A triggered by event TEB when timer decreasing"]
    #[inline(always)]
    pub fn dteb(&self) -> DTEB_R {
        DTEB_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Action on PWM0A triggered by event_t0 when timer decreasing"]
    #[inline(always)]
    pub fn dt0(&self) -> DT0_R {
        DT0_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Action on PWM0A triggered by event_t1 when timer decreasing. 0: no change, 1: low, 2: high, 3: toggle"]
    #[inline(always)]
    pub fn dt1(&self) -> DT1_R {
        DT1_R::new(((self.bits >> 22) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN0_A")
            .field("utez", &format_args!("{}", self.utez().bits()))
            .field("utep", &format_args!("{}", self.utep().bits()))
            .field("utea", &format_args!("{}", self.utea().bits()))
            .field("uteb", &format_args!("{}", self.uteb().bits()))
            .field("ut0", &format_args!("{}", self.ut0().bits()))
            .field("ut1", &format_args!("{}", self.ut1().bits()))
            .field("dtez", &format_args!("{}", self.dtez().bits()))
            .field("dtep", &format_args!("{}", self.dtep().bits()))
            .field("dtea", &format_args!("{}", self.dtea().bits()))
            .field("dteb", &format_args!("{}", self.dteb().bits()))
            .field("dt0", &format_args!("{}", self.dt0().bits()))
            .field("dt1", &format_args!("{}", self.dt1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GEN0_A_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - Action on PWM0A triggered by event TEZ when timer increasing"]
    #[inline(always)]
    #[must_use]
    pub fn utez(&mut self) -> UTEZ_W<0> {
        UTEZ_W::new(self)
    }
    #[doc = "Bits 2:3 - Action on PWM0A triggered by event TEP when timer increasing"]
    #[inline(always)]
    #[must_use]
    pub fn utep(&mut self) -> UTEP_W<2> {
        UTEP_W::new(self)
    }
    #[doc = "Bits 4:5 - Action on PWM0A triggered by event TEA when timer increasing"]
    #[inline(always)]
    #[must_use]
    pub fn utea(&mut self) -> UTEA_W<4> {
        UTEA_W::new(self)
    }
    #[doc = "Bits 6:7 - Action on PWM0A triggered by event TEB when timer increasing"]
    #[inline(always)]
    #[must_use]
    pub fn uteb(&mut self) -> UTEB_W<6> {
        UTEB_W::new(self)
    }
    #[doc = "Bits 8:9 - Action on PWM0A triggered by event_t0 when timer increasing"]
    #[inline(always)]
    #[must_use]
    pub fn ut0(&mut self) -> UT0_W<8> {
        UT0_W::new(self)
    }
    #[doc = "Bits 10:11 - Action on PWM0A triggered by event_t1 when timer increasing"]
    #[inline(always)]
    #[must_use]
    pub fn ut1(&mut self) -> UT1_W<10> {
        UT1_W::new(self)
    }
    #[doc = "Bits 12:13 - Action on PWM0A triggered by event TEZ when timer decreasing"]
    #[inline(always)]
    #[must_use]
    pub fn dtez(&mut self) -> DTEZ_W<12> {
        DTEZ_W::new(self)
    }
    #[doc = "Bits 14:15 - Action on PWM0A triggered by event TEP when timer decreasing"]
    #[inline(always)]
    #[must_use]
    pub fn dtep(&mut self) -> DTEP_W<14> {
        DTEP_W::new(self)
    }
    #[doc = "Bits 16:17 - Action on PWM0A triggered by event TEA when timer decreasing"]
    #[inline(always)]
    #[must_use]
    pub fn dtea(&mut self) -> DTEA_W<16> {
        DTEA_W::new(self)
    }
    #[doc = "Bits 18:19 - Action on PWM0A triggered by event TEB when timer decreasing"]
    #[inline(always)]
    #[must_use]
    pub fn dteb(&mut self) -> DTEB_W<18> {
        DTEB_W::new(self)
    }
    #[doc = "Bits 20:21 - Action on PWM0A triggered by event_t0 when timer decreasing"]
    #[inline(always)]
    #[must_use]
    pub fn dt0(&mut self) -> DT0_W<20> {
        DT0_W::new(self)
    }
    #[doc = "Bits 22:23 - Action on PWM0A triggered by event_t1 when timer decreasing. 0: no change, 1: low, 2: high, 3: toggle"]
    #[inline(always)]
    #[must_use]
    pub fn dt1(&mut self) -> DT1_W<22> {
        DT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Actions triggered by events on PWM0A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gen0_a](index.html) module"]
pub struct GEN0_A_SPEC;
impl crate::RegisterSpec for GEN0_A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gen0_a::R](R) reader structure"]
impl crate::Readable for GEN0_A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gen0_a::W](W) writer structure"]
impl crate::Writable for GEN0_A_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GEN0_A to value 0"]
impl crate::Resettable for GEN0_A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
