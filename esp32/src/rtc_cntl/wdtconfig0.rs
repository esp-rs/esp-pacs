#[doc = "Register `WDTCONFIG0` reader"]
pub struct R(crate::R<WDTCONFIG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCONFIG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCONFIG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCONFIG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCONFIG0` writer"]
pub struct W(crate::W<WDTCONFIG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCONFIG0_SPEC>;
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
impl From<crate::W<WDTCONFIG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCONFIG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_PAUSE_IN_SLP` reader - pause WDT in sleep"]
pub type WDT_PAUSE_IN_SLP_R = crate::BitReader;
#[doc = "Field `WDT_PAUSE_IN_SLP` writer - pause WDT in sleep"]
pub type WDT_PAUSE_IN_SLP_W<'a, const O: u8> = crate::BitWriter<'a, WDTCONFIG0_SPEC, O>;
#[doc = "Field `WDT_APPCPU_RESET_EN` reader - enable WDT reset APP CPU"]
pub type WDT_APPCPU_RESET_EN_R = crate::BitReader;
#[doc = "Field `WDT_APPCPU_RESET_EN` writer - enable WDT reset APP CPU"]
pub type WDT_APPCPU_RESET_EN_W<'a, const O: u8> = crate::BitWriter<'a, WDTCONFIG0_SPEC, O>;
#[doc = "Field `WDT_PROCPU_RESET_EN` reader - enable WDT reset PRO CPU"]
pub type WDT_PROCPU_RESET_EN_R = crate::BitReader;
#[doc = "Field `WDT_PROCPU_RESET_EN` writer - enable WDT reset PRO CPU"]
pub type WDT_PROCPU_RESET_EN_W<'a, const O: u8> = crate::BitWriter<'a, WDTCONFIG0_SPEC, O>;
#[doc = "Field `WDT_FLASHBOOT_MOD_EN` reader - enable WDT in flash boot"]
pub type WDT_FLASHBOOT_MOD_EN_R = crate::BitReader;
#[doc = "Field `WDT_FLASHBOOT_MOD_EN` writer - enable WDT in flash boot"]
pub type WDT_FLASHBOOT_MOD_EN_W<'a, const O: u8> = crate::BitWriter<'a, WDTCONFIG0_SPEC, O>;
#[doc = "Field `WDT_SYS_RESET_LENGTH` reader - system reset counter length"]
pub type WDT_SYS_RESET_LENGTH_R = crate::FieldReader;
#[doc = "Field `WDT_SYS_RESET_LENGTH` writer - system reset counter length"]
pub type WDT_SYS_RESET_LENGTH_W<'a, const O: u8> = crate::FieldWriter<'a, WDTCONFIG0_SPEC, 3, O>;
#[doc = "Field `WDT_CPU_RESET_LENGTH` reader - CPU reset counter length"]
pub type WDT_CPU_RESET_LENGTH_R = crate::FieldReader;
#[doc = "Field `WDT_CPU_RESET_LENGTH` writer - CPU reset counter length"]
pub type WDT_CPU_RESET_LENGTH_W<'a, const O: u8> = crate::FieldWriter<'a, WDTCONFIG0_SPEC, 3, O>;
#[doc = "Field `WDT_LEVEL_INT_EN` reader - N/A"]
pub type WDT_LEVEL_INT_EN_R = crate::BitReader;
#[doc = "Field `WDT_LEVEL_INT_EN` writer - N/A"]
pub type WDT_LEVEL_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, WDTCONFIG0_SPEC, O>;
#[doc = "Field `WDT_EDGE_INT_EN` reader - N/A"]
pub type WDT_EDGE_INT_EN_R = crate::BitReader;
#[doc = "Field `WDT_EDGE_INT_EN` writer - N/A"]
pub type WDT_EDGE_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, WDTCONFIG0_SPEC, O>;
#[doc = "Field `WDT_STG3` reader - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
pub type WDT_STG3_R = crate::FieldReader;
#[doc = "Field `WDT_STG3` writer - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
pub type WDT_STG3_W<'a, const O: u8> = crate::FieldWriter<'a, WDTCONFIG0_SPEC, 3, O>;
#[doc = "Field `WDT_STG2` reader - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
pub type WDT_STG2_R = crate::FieldReader;
#[doc = "Field `WDT_STG2` writer - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
pub type WDT_STG2_W<'a, const O: u8> = crate::FieldWriter<'a, WDTCONFIG0_SPEC, 3, O>;
#[doc = "Field `WDT_STG1` reader - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
pub type WDT_STG1_R = crate::FieldReader;
#[doc = "Field `WDT_STG1` writer - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
pub type WDT_STG1_W<'a, const O: u8> = crate::FieldWriter<'a, WDTCONFIG0_SPEC, 3, O>;
#[doc = "Field `WDT_STG0` reader - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
pub type WDT_STG0_R = crate::FieldReader;
#[doc = "Field `WDT_STG0` writer - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
pub type WDT_STG0_W<'a, const O: u8> = crate::FieldWriter<'a, WDTCONFIG0_SPEC, 3, O>;
#[doc = "Field `WDT_EN` reader - enable RTC WDT"]
pub type WDT_EN_R = crate::BitReader;
#[doc = "Field `WDT_EN` writer - enable RTC WDT"]
pub type WDT_EN_W<'a, const O: u8> = crate::BitWriter<'a, WDTCONFIG0_SPEC, O>;
impl R {
    #[doc = "Bit 7 - pause WDT in sleep"]
    #[inline(always)]
    pub fn wdt_pause_in_slp(&self) -> WDT_PAUSE_IN_SLP_R {
        WDT_PAUSE_IN_SLP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - enable WDT reset APP CPU"]
    #[inline(always)]
    pub fn wdt_appcpu_reset_en(&self) -> WDT_APPCPU_RESET_EN_R {
        WDT_APPCPU_RESET_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - enable WDT reset PRO CPU"]
    #[inline(always)]
    pub fn wdt_procpu_reset_en(&self) -> WDT_PROCPU_RESET_EN_R {
        WDT_PROCPU_RESET_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - enable WDT in flash boot"]
    #[inline(always)]
    pub fn wdt_flashboot_mod_en(&self) -> WDT_FLASHBOOT_MOD_EN_R {
        WDT_FLASHBOOT_MOD_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - system reset counter length"]
    #[inline(always)]
    pub fn wdt_sys_reset_length(&self) -> WDT_SYS_RESET_LENGTH_R {
        WDT_SYS_RESET_LENGTH_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:16 - CPU reset counter length"]
    #[inline(always)]
    pub fn wdt_cpu_reset_length(&self) -> WDT_CPU_RESET_LENGTH_R {
        WDT_CPU_RESET_LENGTH_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    pub fn wdt_level_int_en(&self) -> WDT_LEVEL_INT_EN_R {
        WDT_LEVEL_INT_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn wdt_edge_int_en(&self) -> WDT_EDGE_INT_EN_R {
        WDT_EDGE_INT_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
    #[inline(always)]
    pub fn wdt_stg3(&self) -> WDT_STG3_R {
        WDT_STG3_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
    #[inline(always)]
    pub fn wdt_stg2(&self) -> WDT_STG2_R {
        WDT_STG2_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27 - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
    #[inline(always)]
    pub fn wdt_stg1(&self) -> WDT_STG1_R {
        WDT_STG1_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:30 - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
    #[inline(always)]
    pub fn wdt_stg0(&self) -> WDT_STG0_R {
        WDT_STG0_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - enable RTC WDT"]
    #[inline(always)]
    pub fn wdt_en(&self) -> WDT_EN_R {
        WDT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTCONFIG0")
            .field(
                "wdt_pause_in_slp",
                &format_args!("{}", self.wdt_pause_in_slp().bit()),
            )
            .field(
                "wdt_appcpu_reset_en",
                &format_args!("{}", self.wdt_appcpu_reset_en().bit()),
            )
            .field(
                "wdt_procpu_reset_en",
                &format_args!("{}", self.wdt_procpu_reset_en().bit()),
            )
            .field(
                "wdt_flashboot_mod_en",
                &format_args!("{}", self.wdt_flashboot_mod_en().bit()),
            )
            .field(
                "wdt_sys_reset_length",
                &format_args!("{}", self.wdt_sys_reset_length().bits()),
            )
            .field(
                "wdt_cpu_reset_length",
                &format_args!("{}", self.wdt_cpu_reset_length().bits()),
            )
            .field(
                "wdt_level_int_en",
                &format_args!("{}", self.wdt_level_int_en().bit()),
            )
            .field(
                "wdt_edge_int_en",
                &format_args!("{}", self.wdt_edge_int_en().bit()),
            )
            .field("wdt_stg3", &format_args!("{}", self.wdt_stg3().bits()))
            .field("wdt_stg2", &format_args!("{}", self.wdt_stg2().bits()))
            .field("wdt_stg1", &format_args!("{}", self.wdt_stg1().bits()))
            .field("wdt_stg0", &format_args!("{}", self.wdt_stg0().bits()))
            .field("wdt_en", &format_args!("{}", self.wdt_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WDTCONFIG0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 7 - pause WDT in sleep"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_pause_in_slp(&mut self) -> WDT_PAUSE_IN_SLP_W<7> {
        WDT_PAUSE_IN_SLP_W::new(self)
    }
    #[doc = "Bit 8 - enable WDT reset APP CPU"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_appcpu_reset_en(&mut self) -> WDT_APPCPU_RESET_EN_W<8> {
        WDT_APPCPU_RESET_EN_W::new(self)
    }
    #[doc = "Bit 9 - enable WDT reset PRO CPU"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_procpu_reset_en(&mut self) -> WDT_PROCPU_RESET_EN_W<9> {
        WDT_PROCPU_RESET_EN_W::new(self)
    }
    #[doc = "Bit 10 - enable WDT in flash boot"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_flashboot_mod_en(&mut self) -> WDT_FLASHBOOT_MOD_EN_W<10> {
        WDT_FLASHBOOT_MOD_EN_W::new(self)
    }
    #[doc = "Bits 11:13 - system reset counter length"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_sys_reset_length(&mut self) -> WDT_SYS_RESET_LENGTH_W<11> {
        WDT_SYS_RESET_LENGTH_W::new(self)
    }
    #[doc = "Bits 14:16 - CPU reset counter length"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_cpu_reset_length(&mut self) -> WDT_CPU_RESET_LENGTH_W<14> {
        WDT_CPU_RESET_LENGTH_W::new(self)
    }
    #[doc = "Bit 17 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_level_int_en(&mut self) -> WDT_LEVEL_INT_EN_W<17> {
        WDT_LEVEL_INT_EN_W::new(self)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_edge_int_en(&mut self) -> WDT_EDGE_INT_EN_W<18> {
        WDT_EDGE_INT_EN_W::new(self)
    }
    #[doc = "Bits 19:21 - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg3(&mut self) -> WDT_STG3_W<19> {
        WDT_STG3_W::new(self)
    }
    #[doc = "Bits 22:24 - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg2(&mut self) -> WDT_STG2_W<22> {
        WDT_STG2_W::new(self)
    }
    #[doc = "Bits 25:27 - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg1(&mut self) -> WDT_STG1_W<25> {
        WDT_STG1_W::new(self)
    }
    #[doc = "Bits 28:30 - 1: interrupt stage en 2: CPU reset stage en 3: system reset stage en 4: RTC reset stage en"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg0(&mut self) -> WDT_STG0_W<28> {
        WDT_STG0_W::new(self)
    }
    #[doc = "Bit 31 - enable RTC WDT"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_en(&mut self) -> WDT_EN_W<31> {
        WDT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtconfig0](index.html) module"]
pub struct WDTCONFIG0_SPEC;
impl crate::RegisterSpec for WDTCONFIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtconfig0::R](R) reader structure"]
impl crate::Readable for WDTCONFIG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtconfig0::W](W) writer structure"]
impl crate::Writable for WDTCONFIG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTCONFIG0 to value 0x4c80"]
impl crate::Resettable for WDTCONFIG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x4c80;
}
