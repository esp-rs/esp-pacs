#[doc = "Register `CPU_RESET` reader"]
pub struct R(crate::R<CPU_RESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_RESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_RESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_RESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_RESET` writer"]
pub struct W(crate::W<CPU_RESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_RESET_SPEC>;
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
impl From<crate::W<CPU_RESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_RESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_WDT_CPU_RESET_LENGTH` reader - need_des"]
pub type RTC_WDT_CPU_RESET_LENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_WDT_CPU_RESET_LENGTH` writer - need_des"]
pub type RTC_WDT_CPU_RESET_LENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_RESET_SPEC, u8, u8, 3, O>;
#[doc = "Field `RTC_WDT_CPU_RESET_EN` reader - need_des"]
pub type RTC_WDT_CPU_RESET_EN_R = crate::BitReader<bool>;
#[doc = "Field `RTC_WDT_CPU_RESET_EN` writer - need_des"]
pub type RTC_WDT_CPU_RESET_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CPU_RESET_SPEC, bool, O>;
#[doc = "Field `CPU_STALL_WAIT` reader - need_des"]
pub type CPU_STALL_WAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPU_STALL_WAIT` writer - need_des"]
pub type CPU_STALL_WAIT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CPU_RESET_SPEC, u8, u8, 5, O>;
#[doc = "Field `CPU_STALL_EN` reader - need_des"]
pub type CPU_STALL_EN_R = crate::BitReader<bool>;
#[doc = "Field `CPU_STALL_EN` writer - need_des"]
pub type CPU_STALL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPU_RESET_SPEC, bool, O>;
impl R {
    #[doc = "Bits 22:24 - need_des"]
    #[inline(always)]
    pub fn rtc_wdt_cpu_reset_length(&self) -> RTC_WDT_CPU_RESET_LENGTH_R {
        RTC_WDT_CPU_RESET_LENGTH_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn rtc_wdt_cpu_reset_en(&self) -> RTC_WDT_CPU_RESET_EN_R {
        RTC_WDT_CPU_RESET_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - need_des"]
    #[inline(always)]
    pub fn cpu_stall_wait(&self) -> CPU_STALL_WAIT_R {
        CPU_STALL_WAIT_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn cpu_stall_en(&self) -> CPU_STALL_EN_R {
        CPU_STALL_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 22:24 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_wdt_cpu_reset_length(&mut self) -> RTC_WDT_CPU_RESET_LENGTH_W<22> {
        RTC_WDT_CPU_RESET_LENGTH_W::new(self)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_wdt_cpu_reset_en(&mut self) -> RTC_WDT_CPU_RESET_EN_W<25> {
        RTC_WDT_CPU_RESET_EN_W::new(self)
    }
    #[doc = "Bits 26:30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_stall_wait(&mut self) -> CPU_STALL_WAIT_W<26> {
        CPU_STALL_WAIT_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_stall_en(&mut self) -> CPU_STALL_EN_W<31> {
        CPU_STALL_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_reset](index.html) module"]
pub struct CPU_RESET_SPEC;
impl crate::RegisterSpec for CPU_RESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_reset::R](R) reader structure"]
impl crate::Readable for CPU_RESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_reset::W](W) writer structure"]
impl crate::Writable for CPU_RESET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPU_RESET to value 0x0440_0000"]
impl crate::Resettable for CPU_RESET_SPEC {
    const RESET_VALUE: Self::Ux = 0x0440_0000;
}
