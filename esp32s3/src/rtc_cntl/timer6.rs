#[doc = "Register `TIMER6` reader"]
pub struct R(crate::R<TIMER6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER6` writer"]
pub struct W(crate::W<TIMER6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER6_SPEC>;
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
impl From<crate::W<TIMER6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU_TOP_WAIT_TIMER` reader - No public"]
pub type CPU_TOP_WAIT_TIMER_R = crate::FieldReader<u16>;
#[doc = "Field `CPU_TOP_WAIT_TIMER` writer - No public"]
pub type CPU_TOP_WAIT_TIMER_W<'a, const O: u8> = crate::FieldWriter<'a, TIMER6_SPEC, 9, O, u16>;
#[doc = "Field `CPU_TOP_POWERUP_TIMER` reader - No public"]
pub type CPU_TOP_POWERUP_TIMER_R = crate::FieldReader;
#[doc = "Field `CPU_TOP_POWERUP_TIMER` writer - No public"]
pub type CPU_TOP_POWERUP_TIMER_W<'a, const O: u8> = crate::FieldWriter<'a, TIMER6_SPEC, 7, O>;
#[doc = "Field `DG_PERI_WAIT_TIMER` reader - No public"]
pub type DG_PERI_WAIT_TIMER_R = crate::FieldReader<u16>;
#[doc = "Field `DG_PERI_WAIT_TIMER` writer - No public"]
pub type DG_PERI_WAIT_TIMER_W<'a, const O: u8> = crate::FieldWriter<'a, TIMER6_SPEC, 9, O, u16>;
#[doc = "Field `DG_PERI_POWERUP_TIMER` reader - No public"]
pub type DG_PERI_POWERUP_TIMER_R = crate::FieldReader;
#[doc = "Field `DG_PERI_POWERUP_TIMER` writer - No public"]
pub type DG_PERI_POWERUP_TIMER_W<'a, const O: u8> = crate::FieldWriter<'a, TIMER6_SPEC, 7, O>;
impl R {
    #[doc = "Bits 0:8 - No public"]
    #[inline(always)]
    pub fn cpu_top_wait_timer(&self) -> CPU_TOP_WAIT_TIMER_R {
        CPU_TOP_WAIT_TIMER_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - No public"]
    #[inline(always)]
    pub fn cpu_top_powerup_timer(&self) -> CPU_TOP_POWERUP_TIMER_R {
        CPU_TOP_POWERUP_TIMER_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:24 - No public"]
    #[inline(always)]
    pub fn dg_peri_wait_timer(&self) -> DG_PERI_WAIT_TIMER_R {
        DG_PERI_WAIT_TIMER_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31 - No public"]
    #[inline(always)]
    pub fn dg_peri_powerup_timer(&self) -> DG_PERI_POWERUP_TIMER_R {
        DG_PERI_POWERUP_TIMER_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER6")
            .field(
                "cpu_top_wait_timer",
                &format_args!("{}", self.cpu_top_wait_timer().bits()),
            )
            .field(
                "cpu_top_powerup_timer",
                &format_args!("{}", self.cpu_top_powerup_timer().bits()),
            )
            .field(
                "dg_peri_wait_timer",
                &format_args!("{}", self.dg_peri_wait_timer().bits()),
            )
            .field(
                "dg_peri_powerup_timer",
                &format_args!("{}", self.dg_peri_powerup_timer().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:8 - No public"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_top_wait_timer(&mut self) -> CPU_TOP_WAIT_TIMER_W<0> {
        CPU_TOP_WAIT_TIMER_W::new(self)
    }
    #[doc = "Bits 9:15 - No public"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_top_powerup_timer(&mut self) -> CPU_TOP_POWERUP_TIMER_W<9> {
        CPU_TOP_POWERUP_TIMER_W::new(self)
    }
    #[doc = "Bits 16:24 - No public"]
    #[inline(always)]
    #[must_use]
    pub fn dg_peri_wait_timer(&mut self) -> DG_PERI_WAIT_TIMER_W<16> {
        DG_PERI_WAIT_TIMER_W::new(self)
    }
    #[doc = "Bits 25:31 - No public"]
    #[inline(always)]
    #[must_use]
    pub fn dg_peri_powerup_timer(&mut self) -> DG_PERI_POWERUP_TIMER_W<25> {
        DG_PERI_POWERUP_TIMER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No public\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer6](index.html) module"]
pub struct TIMER6_SPEC;
impl crate::RegisterSpec for TIMER6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer6::R](R) reader structure"]
impl crate::Readable for TIMER6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer6::W](W) writer structure"]
impl crate::Writable for TIMER6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER6 to value 0x1020_0a08"]
impl crate::Resettable for TIMER6_SPEC {
    const RESET_VALUE: Self::Ux = 0x1020_0a08;
}
