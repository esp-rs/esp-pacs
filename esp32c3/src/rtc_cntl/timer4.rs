#[doc = "Register `TIMER4` reader"]
pub struct R(crate::R<TIMER4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER4` writer"]
pub struct W(crate::W<TIMER4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER4_SPEC>;
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
impl From<crate::W<TIMER4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU_TOP_WAIT_TIMER` reader - cpu top power domain wakeup time"]
pub type CPU_TOP_WAIT_TIMER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CPU_TOP_WAIT_TIMER` writer - cpu top power domain wakeup time"]
pub type CPU_TOP_WAIT_TIMER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER4_SPEC, u16, u16, 9, O>;
#[doc = "Field `CPU_TOP_POWERUP_TIMER` reader - cpu top power domain power on time"]
pub type CPU_TOP_POWERUP_TIMER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPU_TOP_POWERUP_TIMER` writer - cpu top power domain power on time"]
pub type CPU_TOP_POWERUP_TIMER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER4_SPEC, u8, u8, 7, O>;
#[doc = "Field `DG_WRAP_WAIT_TIMER` reader - digital wrap power domain wakeup time"]
pub type DG_WRAP_WAIT_TIMER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DG_WRAP_WAIT_TIMER` writer - digital wrap power domain wakeup time"]
pub type DG_WRAP_WAIT_TIMER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER4_SPEC, u16, u16, 9, O>;
#[doc = "Field `DG_WRAP_POWERUP_TIMER` reader - digital wrap power domain power on time"]
pub type DG_WRAP_POWERUP_TIMER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DG_WRAP_POWERUP_TIMER` writer - digital wrap power domain power on time"]
pub type DG_WRAP_POWERUP_TIMER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER4_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:8 - cpu top power domain wakeup time"]
    #[inline(always)]
    pub fn cpu_top_wait_timer(&self) -> CPU_TOP_WAIT_TIMER_R {
        CPU_TOP_WAIT_TIMER_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - cpu top power domain power on time"]
    #[inline(always)]
    pub fn cpu_top_powerup_timer(&self) -> CPU_TOP_POWERUP_TIMER_R {
        CPU_TOP_POWERUP_TIMER_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:24 - digital wrap power domain wakeup time"]
    #[inline(always)]
    pub fn dg_wrap_wait_timer(&self) -> DG_WRAP_WAIT_TIMER_R {
        DG_WRAP_WAIT_TIMER_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31 - digital wrap power domain power on time"]
    #[inline(always)]
    pub fn dg_wrap_powerup_timer(&self) -> DG_WRAP_POWERUP_TIMER_R {
        DG_WRAP_POWERUP_TIMER_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - cpu top power domain wakeup time"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_top_wait_timer(&mut self) -> CPU_TOP_WAIT_TIMER_W<0> {
        CPU_TOP_WAIT_TIMER_W::new(self)
    }
    #[doc = "Bits 9:15 - cpu top power domain power on time"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_top_powerup_timer(&mut self) -> CPU_TOP_POWERUP_TIMER_W<9> {
        CPU_TOP_POWERUP_TIMER_W::new(self)
    }
    #[doc = "Bits 16:24 - digital wrap power domain wakeup time"]
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_wait_timer(&mut self) -> DG_WRAP_WAIT_TIMER_W<16> {
        DG_WRAP_WAIT_TIMER_W::new(self)
    }
    #[doc = "Bits 25:31 - digital wrap power domain power on time"]
    #[inline(always)]
    #[must_use]
    pub fn dg_wrap_powerup_timer(&mut self) -> DG_WRAP_POWERUP_TIMER_W<25> {
        DG_WRAP_POWERUP_TIMER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer4](index.html) module"]
pub struct TIMER4_SPEC;
impl crate::RegisterSpec for TIMER4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer4::R](R) reader structure"]
impl crate::Readable for TIMER4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer4::W](W) writer structure"]
impl crate::Writable for TIMER4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER4 to value 0x1020_0a08"]
impl crate::Resettable for TIMER4_SPEC {
    const RESET_VALUE: Self::Ux = 0x1020_0a08;
}
