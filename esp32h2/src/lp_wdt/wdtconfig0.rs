#[doc = "Register `WDTCONFIG0` reader"]
pub type R = crate::R<WDTCONFIG0_SPEC>;
#[doc = "Register `WDTCONFIG0` writer"]
pub type W = crate::W<WDTCONFIG0_SPEC>;
#[doc = "Field `WDT_PAUSE_IN_SLP` reader - need_des"]
pub type WDT_PAUSE_IN_SLP_R = crate::BitReader;
#[doc = "Field `WDT_PAUSE_IN_SLP` writer - need_des"]
pub type WDT_PAUSE_IN_SLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_APPCPU_RESET_EN` reader - need_des"]
pub type WDT_APPCPU_RESET_EN_R = crate::BitReader;
#[doc = "Field `WDT_APPCPU_RESET_EN` writer - need_des"]
pub type WDT_APPCPU_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_PROCPU_RESET_EN` reader - need_des"]
pub type WDT_PROCPU_RESET_EN_R = crate::BitReader;
#[doc = "Field `WDT_PROCPU_RESET_EN` writer - need_des"]
pub type WDT_PROCPU_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_FLASHBOOT_MOD_EN` reader - need_des"]
pub type WDT_FLASHBOOT_MOD_EN_R = crate::BitReader;
#[doc = "Field `WDT_FLASHBOOT_MOD_EN` writer - need_des"]
pub type WDT_FLASHBOOT_MOD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_SYS_RESET_LENGTH` reader - need_des"]
pub type WDT_SYS_RESET_LENGTH_R = crate::FieldReader;
#[doc = "Field `WDT_SYS_RESET_LENGTH` writer - need_des"]
pub type WDT_SYS_RESET_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDT_CPU_RESET_LENGTH` reader - need_des"]
pub type WDT_CPU_RESET_LENGTH_R = crate::FieldReader;
#[doc = "Field `WDT_CPU_RESET_LENGTH` writer - need_des"]
pub type WDT_CPU_RESET_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDT_STG3` reader - need_des"]
pub type WDT_STG3_R = crate::FieldReader;
#[doc = "Field `WDT_STG3` writer - need_des"]
pub type WDT_STG3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDT_STG2` reader - need_des"]
pub type WDT_STG2_R = crate::FieldReader;
#[doc = "Field `WDT_STG2` writer - need_des"]
pub type WDT_STG2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDT_STG1` reader - need_des"]
pub type WDT_STG1_R = crate::FieldReader;
#[doc = "Field `WDT_STG1` writer - need_des"]
pub type WDT_STG1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDT_STG0` reader - need_des"]
pub type WDT_STG0_R = crate::FieldReader;
#[doc = "Field `WDT_STG0` writer - need_des"]
pub type WDT_STG0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDT_EN` reader - need_des"]
pub type WDT_EN_R = crate::BitReader;
#[doc = "Field `WDT_EN` writer - need_des"]
pub type WDT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 9 - need_des"]
    #[inline(always)]
    pub fn wdt_pause_in_slp(&self) -> WDT_PAUSE_IN_SLP_R {
        WDT_PAUSE_IN_SLP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn wdt_appcpu_reset_en(&self) -> WDT_APPCPU_RESET_EN_R {
        WDT_APPCPU_RESET_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn wdt_procpu_reset_en(&self) -> WDT_PROCPU_RESET_EN_R {
        WDT_PROCPU_RESET_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - need_des"]
    #[inline(always)]
    pub fn wdt_flashboot_mod_en(&self) -> WDT_FLASHBOOT_MOD_EN_R {
        WDT_FLASHBOOT_MOD_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - need_des"]
    #[inline(always)]
    pub fn wdt_sys_reset_length(&self) -> WDT_SYS_RESET_LENGTH_R {
        WDT_SYS_RESET_LENGTH_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn wdt_cpu_reset_length(&self) -> WDT_CPU_RESET_LENGTH_R {
        WDT_CPU_RESET_LENGTH_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - need_des"]
    #[inline(always)]
    pub fn wdt_stg3(&self) -> WDT_STG3_R {
        WDT_STG3_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - need_des"]
    #[inline(always)]
    pub fn wdt_stg2(&self) -> WDT_STG2_R {
        WDT_STG2_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27 - need_des"]
    #[inline(always)]
    pub fn wdt_stg1(&self) -> WDT_STG1_R {
        WDT_STG1_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:30 - need_des"]
    #[inline(always)]
    pub fn wdt_stg0(&self) -> WDT_STG0_R {
        WDT_STG0_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - need_des"]
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
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 9 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_pause_in_slp(&mut self) -> WDT_PAUSE_IN_SLP_W<WDTCONFIG0_SPEC> {
        WDT_PAUSE_IN_SLP_W::new(self, 9)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_appcpu_reset_en(&mut self) -> WDT_APPCPU_RESET_EN_W<WDTCONFIG0_SPEC> {
        WDT_APPCPU_RESET_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_procpu_reset_en(&mut self) -> WDT_PROCPU_RESET_EN_W<WDTCONFIG0_SPEC> {
        WDT_PROCPU_RESET_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_flashboot_mod_en(&mut self) -> WDT_FLASHBOOT_MOD_EN_W<WDTCONFIG0_SPEC> {
        WDT_FLASHBOOT_MOD_EN_W::new(self, 12)
    }
    #[doc = "Bits 13:15 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_sys_reset_length(&mut self) -> WDT_SYS_RESET_LENGTH_W<WDTCONFIG0_SPEC> {
        WDT_SYS_RESET_LENGTH_W::new(self, 13)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_cpu_reset_length(&mut self) -> WDT_CPU_RESET_LENGTH_W<WDTCONFIG0_SPEC> {
        WDT_CPU_RESET_LENGTH_W::new(self, 16)
    }
    #[doc = "Bits 19:21 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg3(&mut self) -> WDT_STG3_W<WDTCONFIG0_SPEC> {
        WDT_STG3_W::new(self, 19)
    }
    #[doc = "Bits 22:24 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg2(&mut self) -> WDT_STG2_W<WDTCONFIG0_SPEC> {
        WDT_STG2_W::new(self, 22)
    }
    #[doc = "Bits 25:27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg1(&mut self) -> WDT_STG1_W<WDTCONFIG0_SPEC> {
        WDT_STG1_W::new(self, 25)
    }
    #[doc = "Bits 28:30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg0(&mut self) -> WDT_STG0_W<WDTCONFIG0_SPEC> {
        WDT_STG0_W::new(self, 28)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_en(&mut self) -> WDT_EN_W<WDTCONFIG0_SPEC> {
        WDT_EN_W::new(self, 31)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtconfig0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtconfig0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDTCONFIG0_SPEC;
impl crate::RegisterSpec for WDTCONFIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtconfig0::R`](R) reader structure"]
impl crate::Readable for WDTCONFIG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdtconfig0::W`](W) writer structure"]
impl crate::Writable for WDTCONFIG0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDTCONFIG0 to value 0x0001_3200"]
impl crate::Resettable for WDTCONFIG0_SPEC {
    const RESET_VALUE: u32 = 0x0001_3200;
}
