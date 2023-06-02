#[doc = "Register `PWC` reader"]
pub struct R(crate::R<PWC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWC` writer"]
pub struct W(crate::W<PWC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWC_SPEC>;
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
impl From<crate::W<PWC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FASTMEM_FORCE_NOISO` reader - Fast RTC memory force no ISO"]
pub type FASTMEM_FORCE_NOISO_R = crate::BitReader;
#[doc = "Field `FASTMEM_FORCE_NOISO` writer - Fast RTC memory force no ISO"]
pub type FASTMEM_FORCE_NOISO_W<'a, const O: u8> = crate::BitWriter<'a, PWC_SPEC, O>;
#[doc = "Field `FASTMEM_FORCE_ISO` reader - Fast RTC memory force ISO"]
pub type FASTMEM_FORCE_ISO_R = crate::BitReader;
#[doc = "Field `FASTMEM_FORCE_ISO` writer - Fast RTC memory force ISO"]
pub type FASTMEM_FORCE_ISO_W<'a, const O: u8> = crate::BitWriter<'a, PWC_SPEC, O>;
#[doc = "Field `SLOWMEM_FORCE_NOISO` reader - RTC memory force no ISO"]
pub type SLOWMEM_FORCE_NOISO_R = crate::BitReader;
#[doc = "Field `SLOWMEM_FORCE_NOISO` writer - RTC memory force no ISO"]
pub type SLOWMEM_FORCE_NOISO_W<'a, const O: u8> = crate::BitWriter<'a, PWC_SPEC, O>;
#[doc = "Field `SLOWMEM_FORCE_ISO` reader - RTC memory force ISO"]
pub type SLOWMEM_FORCE_ISO_R = crate::BitReader;
#[doc = "Field `SLOWMEM_FORCE_ISO` writer - RTC memory force ISO"]
pub type SLOWMEM_FORCE_ISO_W<'a, const O: u8> = crate::BitWriter<'a, PWC_SPEC, O>;
#[doc = "Field `FORCE_ISO` reader - rtc_peri force ISO"]
pub type FORCE_ISO_R = crate::BitReader;
#[doc = "Field `FORCE_ISO` writer - rtc_peri force ISO"]
pub type FORCE_ISO_W<'a, const O: u8> = crate::BitWriter<'a, PWC_SPEC, O>;
#[doc = "Field `FORCE_NOISO` reader - rtc_peri force no ISO"]
pub type FORCE_NOISO_R = crate::BitReader;
#[doc = "Field `FORCE_NOISO` writer - rtc_peri force no ISO"]
pub type FORCE_NOISO_W<'a, const O: u8> = crate::BitWriter<'a, PWC_SPEC, O>;
#[doc = "Field `FASTMEM_FOLW_CPU` reader - 1: Fast RTC memory PD following CPU, 0: fast RTC memory PD following RTC state machine"]
pub type FASTMEM_FOLW_CPU_R = crate::BitReader;
#[doc = "Field `FASTMEM_FOLW_CPU` writer - 1: Fast RTC memory PD following CPU, 0: fast RTC memory PD following RTC state machine"]
pub type FASTMEM_FOLW_CPU_W<'a, const O: u8> = crate::BitWriter<'a, PWC_SPEC, O>;
#[doc = "Field `FASTMEM_FORCE_LPD` reader - Fast RTC memory force PD"]
pub type FASTMEM_FORCE_LPD_R = crate::BitReader;
#[doc = "Field `FASTMEM_FORCE_LPD` writer - Fast RTC memory force PD"]
pub type FASTMEM_FORCE_LPD_W<'a, const O: u8> = crate::BitWriter<'a, PWC_SPEC, O>;
#[doc = "Field `FASTMEM_FORCE_LPU` reader - Fast RTC memory force no PD"]
pub type FASTMEM_FORCE_LPU_R = crate::BitReader;
#[doc = "Field `FASTMEM_FORCE_LPU` writer - Fast RTC memory force no PD"]
pub type FASTMEM_FORCE_LPU_W<'a, const O: u8> = crate::BitWriter<'a, PWC_SPEC, O>;
#[doc = "Field `SLOWMEM_FOLW_CPU` reader - 1: RTC memory PD following CPU, 0: RTC memory PD following RTC state machine"]
pub type SLOWMEM_FOLW_CPU_R = crate::BitReader;
#[doc = "Field `SLOWMEM_FOLW_CPU` writer - 1: RTC memory PD following CPU, 0: RTC memory PD following RTC state machine"]
pub type SLOWMEM_FOLW_CPU_W<'a, const O: u8> = crate::BitWriter<'a, PWC_SPEC, O>;
#[doc = "Field `SLOWMEM_FORCE_LPD` reader - RTC memory force PD"]
pub type SLOWMEM_FORCE_LPD_R = crate::BitReader;
#[doc = "Field `SLOWMEM_FORCE_LPD` writer - RTC memory force PD"]
pub type SLOWMEM_FORCE_LPD_W<'a, const O: u8> = crate::BitWriter<'a, PWC_SPEC, O>;
#[doc = "Field `SLOWMEM_FORCE_LPU` reader - RTC memory force no PD"]
pub type SLOWMEM_FORCE_LPU_R = crate::BitReader;
#[doc = "Field `SLOWMEM_FORCE_LPU` writer - RTC memory force no PD"]
pub type SLOWMEM_FORCE_LPU_W<'a, const O: u8> = crate::BitWriter<'a, PWC_SPEC, O>;
#[doc = "Field `FORCE_PD` reader - rtc_peri force power down"]
pub type FORCE_PD_R = crate::BitReader;
#[doc = "Field `FORCE_PD` writer - rtc_peri force power down"]
pub type FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, PWC_SPEC, O>;
#[doc = "Field `FORCE_PU` reader - rtc_peri force power up"]
pub type FORCE_PU_R = crate::BitReader;
#[doc = "Field `FORCE_PU` writer - rtc_peri force power up"]
pub type FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, PWC_SPEC, O>;
#[doc = "Field `PD_EN` reader - enable power down rtc_peri in sleep"]
pub type PD_EN_R = crate::BitReader;
#[doc = "Field `PD_EN` writer - enable power down rtc_peri in sleep"]
pub type PD_EN_W<'a, const O: u8> = crate::BitWriter<'a, PWC_SPEC, O>;
#[doc = "Field `PAD_FORCE_HOLD` reader - rtc pad force hold"]
pub type PAD_FORCE_HOLD_R = crate::BitReader;
#[doc = "Field `PAD_FORCE_HOLD` writer - rtc pad force hold"]
pub type PAD_FORCE_HOLD_W<'a, const O: u8> = crate::BitWriter<'a, PWC_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Fast RTC memory force no ISO"]
    #[inline(always)]
    pub fn fastmem_force_noiso(&self) -> FASTMEM_FORCE_NOISO_R {
        FASTMEM_FORCE_NOISO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fast RTC memory force ISO"]
    #[inline(always)]
    pub fn fastmem_force_iso(&self) -> FASTMEM_FORCE_ISO_R {
        FASTMEM_FORCE_ISO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC memory force no ISO"]
    #[inline(always)]
    pub fn slowmem_force_noiso(&self) -> SLOWMEM_FORCE_NOISO_R {
        SLOWMEM_FORCE_NOISO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC memory force ISO"]
    #[inline(always)]
    pub fn slowmem_force_iso(&self) -> SLOWMEM_FORCE_ISO_R {
        SLOWMEM_FORCE_ISO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - rtc_peri force ISO"]
    #[inline(always)]
    pub fn force_iso(&self) -> FORCE_ISO_R {
        FORCE_ISO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - rtc_peri force no ISO"]
    #[inline(always)]
    pub fn force_noiso(&self) -> FORCE_NOISO_R {
        FORCE_NOISO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Fast RTC memory PD following CPU, 0: fast RTC memory PD following RTC state machine"]
    #[inline(always)]
    pub fn fastmem_folw_cpu(&self) -> FASTMEM_FOLW_CPU_R {
        FASTMEM_FOLW_CPU_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fast RTC memory force PD"]
    #[inline(always)]
    pub fn fastmem_force_lpd(&self) -> FASTMEM_FORCE_LPD_R {
        FASTMEM_FORCE_LPD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Fast RTC memory force no PD"]
    #[inline(always)]
    pub fn fastmem_force_lpu(&self) -> FASTMEM_FORCE_LPU_R {
        FASTMEM_FORCE_LPU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: RTC memory PD following CPU, 0: RTC memory PD following RTC state machine"]
    #[inline(always)]
    pub fn slowmem_folw_cpu(&self) -> SLOWMEM_FOLW_CPU_R {
        SLOWMEM_FOLW_CPU_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC memory force PD"]
    #[inline(always)]
    pub fn slowmem_force_lpd(&self) -> SLOWMEM_FORCE_LPD_R {
        SLOWMEM_FORCE_LPD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - RTC memory force no PD"]
    #[inline(always)]
    pub fn slowmem_force_lpu(&self) -> SLOWMEM_FORCE_LPU_R {
        SLOWMEM_FORCE_LPU_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 18 - rtc_peri force power down"]
    #[inline(always)]
    pub fn force_pd(&self) -> FORCE_PD_R {
        FORCE_PD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - rtc_peri force power up"]
    #[inline(always)]
    pub fn force_pu(&self) -> FORCE_PU_R {
        FORCE_PU_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - enable power down rtc_peri in sleep"]
    #[inline(always)]
    pub fn pd_en(&self) -> PD_EN_R {
        PD_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - rtc pad force hold"]
    #[inline(always)]
    pub fn pad_force_hold(&self) -> PAD_FORCE_HOLD_R {
        PAD_FORCE_HOLD_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWC")
            .field(
                "fastmem_force_noiso",
                &format_args!("{}", self.fastmem_force_noiso().bit()),
            )
            .field(
                "fastmem_force_iso",
                &format_args!("{}", self.fastmem_force_iso().bit()),
            )
            .field(
                "slowmem_force_noiso",
                &format_args!("{}", self.slowmem_force_noiso().bit()),
            )
            .field(
                "slowmem_force_iso",
                &format_args!("{}", self.slowmem_force_iso().bit()),
            )
            .field("force_iso", &format_args!("{}", self.force_iso().bit()))
            .field("force_noiso", &format_args!("{}", self.force_noiso().bit()))
            .field(
                "fastmem_folw_cpu",
                &format_args!("{}", self.fastmem_folw_cpu().bit()),
            )
            .field(
                "fastmem_force_lpd",
                &format_args!("{}", self.fastmem_force_lpd().bit()),
            )
            .field(
                "fastmem_force_lpu",
                &format_args!("{}", self.fastmem_force_lpu().bit()),
            )
            .field(
                "slowmem_folw_cpu",
                &format_args!("{}", self.slowmem_folw_cpu().bit()),
            )
            .field(
                "slowmem_force_lpd",
                &format_args!("{}", self.slowmem_force_lpd().bit()),
            )
            .field(
                "slowmem_force_lpu",
                &format_args!("{}", self.slowmem_force_lpu().bit()),
            )
            .field("force_pd", &format_args!("{}", self.force_pd().bit()))
            .field("force_pu", &format_args!("{}", self.force_pu().bit()))
            .field("pd_en", &format_args!("{}", self.pd_en().bit()))
            .field(
                "pad_force_hold",
                &format_args!("{}", self.pad_force_hold().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PWC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Fast RTC memory force no ISO"]
    #[inline(always)]
    #[must_use]
    pub fn fastmem_force_noiso(&mut self) -> FASTMEM_FORCE_NOISO_W<0> {
        FASTMEM_FORCE_NOISO_W::new(self)
    }
    #[doc = "Bit 1 - Fast RTC memory force ISO"]
    #[inline(always)]
    #[must_use]
    pub fn fastmem_force_iso(&mut self) -> FASTMEM_FORCE_ISO_W<1> {
        FASTMEM_FORCE_ISO_W::new(self)
    }
    #[doc = "Bit 2 - RTC memory force no ISO"]
    #[inline(always)]
    #[must_use]
    pub fn slowmem_force_noiso(&mut self) -> SLOWMEM_FORCE_NOISO_W<2> {
        SLOWMEM_FORCE_NOISO_W::new(self)
    }
    #[doc = "Bit 3 - RTC memory force ISO"]
    #[inline(always)]
    #[must_use]
    pub fn slowmem_force_iso(&mut self) -> SLOWMEM_FORCE_ISO_W<3> {
        SLOWMEM_FORCE_ISO_W::new(self)
    }
    #[doc = "Bit 4 - rtc_peri force ISO"]
    #[inline(always)]
    #[must_use]
    pub fn force_iso(&mut self) -> FORCE_ISO_W<4> {
        FORCE_ISO_W::new(self)
    }
    #[doc = "Bit 5 - rtc_peri force no ISO"]
    #[inline(always)]
    #[must_use]
    pub fn force_noiso(&mut self) -> FORCE_NOISO_W<5> {
        FORCE_NOISO_W::new(self)
    }
    #[doc = "Bit 6 - 1: Fast RTC memory PD following CPU, 0: fast RTC memory PD following RTC state machine"]
    #[inline(always)]
    #[must_use]
    pub fn fastmem_folw_cpu(&mut self) -> FASTMEM_FOLW_CPU_W<6> {
        FASTMEM_FOLW_CPU_W::new(self)
    }
    #[doc = "Bit 7 - Fast RTC memory force PD"]
    #[inline(always)]
    #[must_use]
    pub fn fastmem_force_lpd(&mut self) -> FASTMEM_FORCE_LPD_W<7> {
        FASTMEM_FORCE_LPD_W::new(self)
    }
    #[doc = "Bit 8 - Fast RTC memory force no PD"]
    #[inline(always)]
    #[must_use]
    pub fn fastmem_force_lpu(&mut self) -> FASTMEM_FORCE_LPU_W<8> {
        FASTMEM_FORCE_LPU_W::new(self)
    }
    #[doc = "Bit 9 - 1: RTC memory PD following CPU, 0: RTC memory PD following RTC state machine"]
    #[inline(always)]
    #[must_use]
    pub fn slowmem_folw_cpu(&mut self) -> SLOWMEM_FOLW_CPU_W<9> {
        SLOWMEM_FOLW_CPU_W::new(self)
    }
    #[doc = "Bit 10 - RTC memory force PD"]
    #[inline(always)]
    #[must_use]
    pub fn slowmem_force_lpd(&mut self) -> SLOWMEM_FORCE_LPD_W<10> {
        SLOWMEM_FORCE_LPD_W::new(self)
    }
    #[doc = "Bit 11 - RTC memory force no PD"]
    #[inline(always)]
    #[must_use]
    pub fn slowmem_force_lpu(&mut self) -> SLOWMEM_FORCE_LPU_W<11> {
        SLOWMEM_FORCE_LPU_W::new(self)
    }
    #[doc = "Bit 18 - rtc_peri force power down"]
    #[inline(always)]
    #[must_use]
    pub fn force_pd(&mut self) -> FORCE_PD_W<18> {
        FORCE_PD_W::new(self)
    }
    #[doc = "Bit 19 - rtc_peri force power up"]
    #[inline(always)]
    #[must_use]
    pub fn force_pu(&mut self) -> FORCE_PU_W<19> {
        FORCE_PU_W::new(self)
    }
    #[doc = "Bit 20 - enable power down rtc_peri in sleep"]
    #[inline(always)]
    #[must_use]
    pub fn pd_en(&mut self) -> PD_EN_W<20> {
        PD_EN_W::new(self)
    }
    #[doc = "Bit 21 - rtc pad force hold"]
    #[inline(always)]
    #[must_use]
    pub fn pad_force_hold(&mut self) -> PAD_FORCE_HOLD_W<21> {
        PAD_FORCE_HOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure rtc power\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwc](index.html) module"]
pub struct PWC_SPEC;
impl crate::RegisterSpec for PWC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwc::R](R) reader structure"]
impl crate::Readable for PWC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwc::W](W) writer structure"]
impl crate::Writable for PWC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWC to value 0x0925"]
impl crate::Resettable for PWC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0925;
}
