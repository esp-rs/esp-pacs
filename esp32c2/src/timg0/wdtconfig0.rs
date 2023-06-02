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
#[doc = "Field `WDT_APPCPU_RESET_EN` reader - WDT reset CPU enable."]
pub type WDT_APPCPU_RESET_EN_R = crate::BitReader;
#[doc = "Field `WDT_APPCPU_RESET_EN` writer - WDT reset CPU enable."]
pub type WDT_APPCPU_RESET_EN_W<'a, const O: u8> = crate::BitWriter<'a, WDTCONFIG0_SPEC, O>;
#[doc = "Field `WDT_PROCPU_RESET_EN` reader - WDT reset CPU enable."]
pub type WDT_PROCPU_RESET_EN_R = crate::BitReader;
#[doc = "Field `WDT_PROCPU_RESET_EN` writer - WDT reset CPU enable."]
pub type WDT_PROCPU_RESET_EN_W<'a, const O: u8> = crate::BitWriter<'a, WDTCONFIG0_SPEC, O>;
#[doc = "Field `WDT_FLASHBOOT_MOD_EN` reader - When set, Flash boot protection is enabled."]
pub type WDT_FLASHBOOT_MOD_EN_R = crate::BitReader;
#[doc = "Field `WDT_FLASHBOOT_MOD_EN` writer - When set, Flash boot protection is enabled."]
pub type WDT_FLASHBOOT_MOD_EN_W<'a, const O: u8> = crate::BitWriter<'a, WDTCONFIG0_SPEC, O>;
#[doc = "Field `WDT_SYS_RESET_LENGTH` reader - System reset signal length selection. 0: 100 ns, 1: 200 ns, 2: 300 ns, 3: 400 ns, 4: 500 ns, 5: 800 ns, 6: 1.6 us, 7: 3.2 us."]
pub type WDT_SYS_RESET_LENGTH_R = crate::FieldReader;
#[doc = "Field `WDT_SYS_RESET_LENGTH` writer - System reset signal length selection. 0: 100 ns, 1: 200 ns, 2: 300 ns, 3: 400 ns, 4: 500 ns, 5: 800 ns, 6: 1.6 us, 7: 3.2 us."]
pub type WDT_SYS_RESET_LENGTH_W<'a, const O: u8> = crate::FieldWriter<'a, WDTCONFIG0_SPEC, 3, O>;
#[doc = "Field `WDT_CPU_RESET_LENGTH` reader - CPU reset signal length selection. 0: 100 ns, 1: 200 ns, 2: 300 ns, 3: 400 ns, 4: 500 ns, 5: 800 ns, 6: 1.6 us, 7: 3.2 us."]
pub type WDT_CPU_RESET_LENGTH_R = crate::FieldReader;
#[doc = "Field `WDT_CPU_RESET_LENGTH` writer - CPU reset signal length selection. 0: 100 ns, 1: 200 ns, 2: 300 ns, 3: 400 ns, 4: 500 ns, 5: 800 ns, 6: 1.6 us, 7: 3.2 us."]
pub type WDT_CPU_RESET_LENGTH_W<'a, const O: u8> = crate::FieldWriter<'a, WDTCONFIG0_SPEC, 3, O>;
#[doc = "Field `WDT_USE_XTAL` reader - choose WDT clock:0-apb_clk; 1-xtal_clk."]
pub type WDT_USE_XTAL_R = crate::BitReader;
#[doc = "Field `WDT_USE_XTAL` writer - choose WDT clock:0-apb_clk; 1-xtal_clk."]
pub type WDT_USE_XTAL_W<'a, const O: u8> = crate::BitWriter<'a, WDTCONFIG0_SPEC, O>;
#[doc = "Field `WDT_CONF_UPDATE_EN` writer - update the WDT configuration registers"]
pub type WDT_CONF_UPDATE_EN_W<'a, const O: u8> = crate::BitWriter<'a, WDTCONFIG0_SPEC, O>;
#[doc = "Field `WDT_STG3` reader - Stage 3 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system."]
pub type WDT_STG3_R = crate::FieldReader;
#[doc = "Field `WDT_STG3` writer - Stage 3 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system."]
pub type WDT_STG3_W<'a, const O: u8> = crate::FieldWriter<'a, WDTCONFIG0_SPEC, 2, O>;
#[doc = "Field `WDT_STG2` reader - Stage 2 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system."]
pub type WDT_STG2_R = crate::FieldReader;
#[doc = "Field `WDT_STG2` writer - Stage 2 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system."]
pub type WDT_STG2_W<'a, const O: u8> = crate::FieldWriter<'a, WDTCONFIG0_SPEC, 2, O>;
#[doc = "Field `WDT_STG1` reader - Stage 1 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system."]
pub type WDT_STG1_R = crate::FieldReader;
#[doc = "Field `WDT_STG1` writer - Stage 1 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system."]
pub type WDT_STG1_W<'a, const O: u8> = crate::FieldWriter<'a, WDTCONFIG0_SPEC, 2, O>;
#[doc = "Field `WDT_STG0` reader - Stage 0 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system."]
pub type WDT_STG0_R = crate::FieldReader;
#[doc = "Field `WDT_STG0` writer - Stage 0 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system."]
pub type WDT_STG0_W<'a, const O: u8> = crate::FieldWriter<'a, WDTCONFIG0_SPEC, 2, O>;
#[doc = "Field `WDT_EN` reader - When set, MWDT is enabled."]
pub type WDT_EN_R = crate::BitReader;
#[doc = "Field `WDT_EN` writer - When set, MWDT is enabled."]
pub type WDT_EN_W<'a, const O: u8> = crate::BitWriter<'a, WDTCONFIG0_SPEC, O>;
impl R {
    #[doc = "Bit 12 - WDT reset CPU enable."]
    #[inline(always)]
    pub fn wdt_appcpu_reset_en(&self) -> WDT_APPCPU_RESET_EN_R {
        WDT_APPCPU_RESET_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - WDT reset CPU enable."]
    #[inline(always)]
    pub fn wdt_procpu_reset_en(&self) -> WDT_PROCPU_RESET_EN_R {
        WDT_PROCPU_RESET_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - When set, Flash boot protection is enabled."]
    #[inline(always)]
    pub fn wdt_flashboot_mod_en(&self) -> WDT_FLASHBOOT_MOD_EN_R {
        WDT_FLASHBOOT_MOD_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:17 - System reset signal length selection. 0: 100 ns, 1: 200 ns, 2: 300 ns, 3: 400 ns, 4: 500 ns, 5: 800 ns, 6: 1.6 us, 7: 3.2 us."]
    #[inline(always)]
    pub fn wdt_sys_reset_length(&self) -> WDT_SYS_RESET_LENGTH_R {
        WDT_SYS_RESET_LENGTH_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - CPU reset signal length selection. 0: 100 ns, 1: 200 ns, 2: 300 ns, 3: 400 ns, 4: 500 ns, 5: 800 ns, 6: 1.6 us, 7: 3.2 us."]
    #[inline(always)]
    pub fn wdt_cpu_reset_length(&self) -> WDT_CPU_RESET_LENGTH_R {
        WDT_CPU_RESET_LENGTH_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 21 - choose WDT clock:0-apb_clk; 1-xtal_clk."]
    #[inline(always)]
    pub fn wdt_use_xtal(&self) -> WDT_USE_XTAL_R {
        WDT_USE_XTAL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 23:24 - Stage 3 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system."]
    #[inline(always)]
    pub fn wdt_stg3(&self) -> WDT_STG3_R {
        WDT_STG3_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:26 - Stage 2 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system."]
    #[inline(always)]
    pub fn wdt_stg2(&self) -> WDT_STG2_R {
        WDT_STG2_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:28 - Stage 1 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system."]
    #[inline(always)]
    pub fn wdt_stg1(&self) -> WDT_STG1_R {
        WDT_STG1_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 29:30 - Stage 0 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system."]
    #[inline(always)]
    pub fn wdt_stg0(&self) -> WDT_STG0_R {
        WDT_STG0_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - When set, MWDT is enabled."]
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
                "wdt_use_xtal",
                &format_args!("{}", self.wdt_use_xtal().bit()),
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
    #[doc = "Bit 12 - WDT reset CPU enable."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_appcpu_reset_en(&mut self) -> WDT_APPCPU_RESET_EN_W<12> {
        WDT_APPCPU_RESET_EN_W::new(self)
    }
    #[doc = "Bit 13 - WDT reset CPU enable."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_procpu_reset_en(&mut self) -> WDT_PROCPU_RESET_EN_W<13> {
        WDT_PROCPU_RESET_EN_W::new(self)
    }
    #[doc = "Bit 14 - When set, Flash boot protection is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_flashboot_mod_en(&mut self) -> WDT_FLASHBOOT_MOD_EN_W<14> {
        WDT_FLASHBOOT_MOD_EN_W::new(self)
    }
    #[doc = "Bits 15:17 - System reset signal length selection. 0: 100 ns, 1: 200 ns, 2: 300 ns, 3: 400 ns, 4: 500 ns, 5: 800 ns, 6: 1.6 us, 7: 3.2 us."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_sys_reset_length(&mut self) -> WDT_SYS_RESET_LENGTH_W<15> {
        WDT_SYS_RESET_LENGTH_W::new(self)
    }
    #[doc = "Bits 18:20 - CPU reset signal length selection. 0: 100 ns, 1: 200 ns, 2: 300 ns, 3: 400 ns, 4: 500 ns, 5: 800 ns, 6: 1.6 us, 7: 3.2 us."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_cpu_reset_length(&mut self) -> WDT_CPU_RESET_LENGTH_W<18> {
        WDT_CPU_RESET_LENGTH_W::new(self)
    }
    #[doc = "Bit 21 - choose WDT clock:0-apb_clk; 1-xtal_clk."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_use_xtal(&mut self) -> WDT_USE_XTAL_W<21> {
        WDT_USE_XTAL_W::new(self)
    }
    #[doc = "Bit 22 - update the WDT configuration registers"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_conf_update_en(&mut self) -> WDT_CONF_UPDATE_EN_W<22> {
        WDT_CONF_UPDATE_EN_W::new(self)
    }
    #[doc = "Bits 23:24 - Stage 3 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg3(&mut self) -> WDT_STG3_W<23> {
        WDT_STG3_W::new(self)
    }
    #[doc = "Bits 25:26 - Stage 2 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg2(&mut self) -> WDT_STG2_W<25> {
        WDT_STG2_W::new(self)
    }
    #[doc = "Bits 27:28 - Stage 1 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg1(&mut self) -> WDT_STG1_W<27> {
        WDT_STG1_W::new(self)
    }
    #[doc = "Bits 29:30 - Stage 0 configuration. 0: off, 1: interrupt, 2: reset CPU, 3: reset system."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg0(&mut self) -> WDT_STG0_W<29> {
        WDT_STG0_W::new(self)
    }
    #[doc = "Bit 31 - When set, MWDT is enabled."]
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
#[doc = "Watchdog timer configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtconfig0](index.html) module"]
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
#[doc = "`reset()` method sets WDTCONFIG0 to value 0x0004_c000"]
impl crate::Resettable for WDTCONFIG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0004_c000;
}
