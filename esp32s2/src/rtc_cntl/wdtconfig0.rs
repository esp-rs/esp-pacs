#[doc = "Register `WDTCONFIG0` reader"]
pub type R = crate::R<WDTCONFIG0_SPEC>;
#[doc = "Register `WDTCONFIG0` writer"]
pub type W = crate::W<WDTCONFIG0_SPEC>;
#[doc = "Field `WDT_CHIP_RESET_WIDTH` reader - chip reset siginal pulse width"]
pub type WDT_CHIP_RESET_WIDTH_R = crate::FieldReader;
#[doc = "Field `WDT_CHIP_RESET_WIDTH` writer - chip reset siginal pulse width"]
pub type WDT_CHIP_RESET_WIDTH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WDT_CHIP_RESET_EN` reader - wdt reset whole chip enable"]
pub type WDT_CHIP_RESET_EN_R = crate::BitReader;
#[doc = "Field `WDT_CHIP_RESET_EN` writer - wdt reset whole chip enable"]
pub type WDT_CHIP_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_PAUSE_IN_SLP` reader - Set this bit to pause the watchdog in sleep."]
pub type WDT_PAUSE_IN_SLP_R = crate::BitReader;
#[doc = "Field `WDT_PAUSE_IN_SLP` writer - Set this bit to pause the watchdog in sleep."]
pub type WDT_PAUSE_IN_SLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_APPCPU_RESET_EN` reader - enable WDT reset APP CPU"]
pub type WDT_APPCPU_RESET_EN_R = crate::BitReader;
#[doc = "Field `WDT_APPCPU_RESET_EN` writer - enable WDT reset APP CPU"]
pub type WDT_APPCPU_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_PROCPU_RESET_EN` reader - Set this bit to allow the watchdog to be able to reset CPU."]
pub type WDT_PROCPU_RESET_EN_R = crate::BitReader;
#[doc = "Field `WDT_PROCPU_RESET_EN` writer - Set this bit to allow the watchdog to be able to reset CPU."]
pub type WDT_PROCPU_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_FLASHBOOT_MOD_EN` reader - Set this bit to enable watchdog when the chip boots from flash."]
pub type WDT_FLASHBOOT_MOD_EN_R = crate::BitReader;
#[doc = "Field `WDT_FLASHBOOT_MOD_EN` writer - Set this bit to enable watchdog when the chip boots from flash."]
pub type WDT_FLASHBOOT_MOD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_SYS_RESET_LENGTH` reader - Sets the length of the system reset counter."]
pub type WDT_SYS_RESET_LENGTH_R = crate::FieldReader;
#[doc = "Field `WDT_SYS_RESET_LENGTH` writer - Sets the length of the system reset counter."]
pub type WDT_SYS_RESET_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDT_CPU_RESET_LENGTH` reader - Sets the length of the CPU reset counter."]
pub type WDT_CPU_RESET_LENGTH_R = crate::FieldReader;
#[doc = "Field `WDT_CPU_RESET_LENGTH` writer - Sets the length of the CPU reset counter."]
pub type WDT_CPU_RESET_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDT_STG3` reader - 1: enable at the interrupt stage 2: enable at the CPU stage 3: enable at the system stage 4: enable at the system and RTC stage."]
pub type WDT_STG3_R = crate::FieldReader;
#[doc = "Field `WDT_STG3` writer - 1: enable at the interrupt stage 2: enable at the CPU stage 3: enable at the system stage 4: enable at the system and RTC stage."]
pub type WDT_STG3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDT_STG2` reader - 1: enable at the interrupt stage 2: enable at the CPU stage 3: enable at the system stage 4: enable at the system and RTC stage."]
pub type WDT_STG2_R = crate::FieldReader;
#[doc = "Field `WDT_STG2` writer - 1: enable at the interrupt stage 2: enable at the CPU stage 3: enable at the system stage 4: enable at the system and RTC stage."]
pub type WDT_STG2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDT_STG1` reader - 1: enable at the interrupt stage 2: enable at the CPU stage 3: enable at the system stage 4: enable at the system and RTC stage."]
pub type WDT_STG1_R = crate::FieldReader;
#[doc = "Field `WDT_STG1` writer - 1: enable at the interrupt stage 2: enable at the CPU stage 3: enable at the system stage 4: enable at the system and RTC stage."]
pub type WDT_STG1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDT_STG0` reader - 1: enable at the interrupt stage 2: enable at the CPU stage 3: enable at the system stage 4: enable at the system and RTC stage."]
pub type WDT_STG0_R = crate::FieldReader;
#[doc = "Field `WDT_STG0` writer - 1: enable at the interrupt stage 2: enable at the CPU stage 3: enable at the system stage 4: enable at the system and RTC stage."]
pub type WDT_STG0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WDT_EN` reader - Set this bit to enable the RTC watchdog."]
pub type WDT_EN_R = crate::BitReader;
#[doc = "Field `WDT_EN` writer - Set this bit to enable the RTC watchdog."]
pub type WDT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - chip reset siginal pulse width"]
    #[inline(always)]
    pub fn wdt_chip_reset_width(&self) -> WDT_CHIP_RESET_WIDTH_R {
        WDT_CHIP_RESET_WIDTH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - wdt reset whole chip enable"]
    #[inline(always)]
    pub fn wdt_chip_reset_en(&self) -> WDT_CHIP_RESET_EN_R {
        WDT_CHIP_RESET_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set this bit to pause the watchdog in sleep."]
    #[inline(always)]
    pub fn wdt_pause_in_slp(&self) -> WDT_PAUSE_IN_SLP_R {
        WDT_PAUSE_IN_SLP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - enable WDT reset APP CPU"]
    #[inline(always)]
    pub fn wdt_appcpu_reset_en(&self) -> WDT_APPCPU_RESET_EN_R {
        WDT_APPCPU_RESET_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set this bit to allow the watchdog to be able to reset CPU."]
    #[inline(always)]
    pub fn wdt_procpu_reset_en(&self) -> WDT_PROCPU_RESET_EN_R {
        WDT_PROCPU_RESET_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set this bit to enable watchdog when the chip boots from flash."]
    #[inline(always)]
    pub fn wdt_flashboot_mod_en(&self) -> WDT_FLASHBOOT_MOD_EN_R {
        WDT_FLASHBOOT_MOD_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Sets the length of the system reset counter."]
    #[inline(always)]
    pub fn wdt_sys_reset_length(&self) -> WDT_SYS_RESET_LENGTH_R {
        WDT_SYS_RESET_LENGTH_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Sets the length of the CPU reset counter."]
    #[inline(always)]
    pub fn wdt_cpu_reset_length(&self) -> WDT_CPU_RESET_LENGTH_R {
        WDT_CPU_RESET_LENGTH_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - 1: enable at the interrupt stage 2: enable at the CPU stage 3: enable at the system stage 4: enable at the system and RTC stage."]
    #[inline(always)]
    pub fn wdt_stg3(&self) -> WDT_STG3_R {
        WDT_STG3_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - 1: enable at the interrupt stage 2: enable at the CPU stage 3: enable at the system stage 4: enable at the system and RTC stage."]
    #[inline(always)]
    pub fn wdt_stg2(&self) -> WDT_STG2_R {
        WDT_STG2_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27 - 1: enable at the interrupt stage 2: enable at the CPU stage 3: enable at the system stage 4: enable at the system and RTC stage."]
    #[inline(always)]
    pub fn wdt_stg1(&self) -> WDT_STG1_R {
        WDT_STG1_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:30 - 1: enable at the interrupt stage 2: enable at the CPU stage 3: enable at the system stage 4: enable at the system and RTC stage."]
    #[inline(always)]
    pub fn wdt_stg0(&self) -> WDT_STG0_R {
        WDT_STG0_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Set this bit to enable the RTC watchdog."]
    #[inline(always)]
    pub fn wdt_en(&self) -> WDT_EN_R {
        WDT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTCONFIG0")
            .field("wdt_chip_reset_width", &self.wdt_chip_reset_width())
            .field("wdt_chip_reset_en", &self.wdt_chip_reset_en())
            .field("wdt_pause_in_slp", &self.wdt_pause_in_slp())
            .field("wdt_appcpu_reset_en", &self.wdt_appcpu_reset_en())
            .field("wdt_procpu_reset_en", &self.wdt_procpu_reset_en())
            .field("wdt_flashboot_mod_en", &self.wdt_flashboot_mod_en())
            .field("wdt_sys_reset_length", &self.wdt_sys_reset_length())
            .field("wdt_cpu_reset_length", &self.wdt_cpu_reset_length())
            .field("wdt_stg3", &self.wdt_stg3())
            .field("wdt_stg2", &self.wdt_stg2())
            .field("wdt_stg1", &self.wdt_stg1())
            .field("wdt_stg0", &self.wdt_stg0())
            .field("wdt_en", &self.wdt_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - chip reset siginal pulse width"]
    #[inline(always)]
    pub fn wdt_chip_reset_width(&mut self) -> WDT_CHIP_RESET_WIDTH_W<'_, WDTCONFIG0_SPEC> {
        WDT_CHIP_RESET_WIDTH_W::new(self, 0)
    }
    #[doc = "Bit 8 - wdt reset whole chip enable"]
    #[inline(always)]
    pub fn wdt_chip_reset_en(&mut self) -> WDT_CHIP_RESET_EN_W<'_, WDTCONFIG0_SPEC> {
        WDT_CHIP_RESET_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to pause the watchdog in sleep."]
    #[inline(always)]
    pub fn wdt_pause_in_slp(&mut self) -> WDT_PAUSE_IN_SLP_W<'_, WDTCONFIG0_SPEC> {
        WDT_PAUSE_IN_SLP_W::new(self, 9)
    }
    #[doc = "Bit 10 - enable WDT reset APP CPU"]
    #[inline(always)]
    pub fn wdt_appcpu_reset_en(&mut self) -> WDT_APPCPU_RESET_EN_W<'_, WDTCONFIG0_SPEC> {
        WDT_APPCPU_RESET_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Set this bit to allow the watchdog to be able to reset CPU."]
    #[inline(always)]
    pub fn wdt_procpu_reset_en(&mut self) -> WDT_PROCPU_RESET_EN_W<'_, WDTCONFIG0_SPEC> {
        WDT_PROCPU_RESET_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Set this bit to enable watchdog when the chip boots from flash."]
    #[inline(always)]
    pub fn wdt_flashboot_mod_en(&mut self) -> WDT_FLASHBOOT_MOD_EN_W<'_, WDTCONFIG0_SPEC> {
        WDT_FLASHBOOT_MOD_EN_W::new(self, 12)
    }
    #[doc = "Bits 13:15 - Sets the length of the system reset counter."]
    #[inline(always)]
    pub fn wdt_sys_reset_length(&mut self) -> WDT_SYS_RESET_LENGTH_W<'_, WDTCONFIG0_SPEC> {
        WDT_SYS_RESET_LENGTH_W::new(self, 13)
    }
    #[doc = "Bits 16:18 - Sets the length of the CPU reset counter."]
    #[inline(always)]
    pub fn wdt_cpu_reset_length(&mut self) -> WDT_CPU_RESET_LENGTH_W<'_, WDTCONFIG0_SPEC> {
        WDT_CPU_RESET_LENGTH_W::new(self, 16)
    }
    #[doc = "Bits 19:21 - 1: enable at the interrupt stage 2: enable at the CPU stage 3: enable at the system stage 4: enable at the system and RTC stage."]
    #[inline(always)]
    pub fn wdt_stg3(&mut self) -> WDT_STG3_W<'_, WDTCONFIG0_SPEC> {
        WDT_STG3_W::new(self, 19)
    }
    #[doc = "Bits 22:24 - 1: enable at the interrupt stage 2: enable at the CPU stage 3: enable at the system stage 4: enable at the system and RTC stage."]
    #[inline(always)]
    pub fn wdt_stg2(&mut self) -> WDT_STG2_W<'_, WDTCONFIG0_SPEC> {
        WDT_STG2_W::new(self, 22)
    }
    #[doc = "Bits 25:27 - 1: enable at the interrupt stage 2: enable at the CPU stage 3: enable at the system stage 4: enable at the system and RTC stage."]
    #[inline(always)]
    pub fn wdt_stg1(&mut self) -> WDT_STG1_W<'_, WDTCONFIG0_SPEC> {
        WDT_STG1_W::new(self, 25)
    }
    #[doc = "Bits 28:30 - 1: enable at the interrupt stage 2: enable at the CPU stage 3: enable at the system stage 4: enable at the system and RTC stage."]
    #[inline(always)]
    pub fn wdt_stg0(&mut self) -> WDT_STG0_W<'_, WDTCONFIG0_SPEC> {
        WDT_STG0_W::new(self, 28)
    }
    #[doc = "Bit 31 - Set this bit to enable the RTC watchdog."]
    #[inline(always)]
    pub fn wdt_en(&mut self) -> WDT_EN_W<'_, WDTCONFIG0_SPEC> {
        WDT_EN_W::new(self, 31)
    }
}
#[doc = "RTC watchdog configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtconfig0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtconfig0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDTCONFIG0_SPEC;
impl crate::RegisterSpec for WDTCONFIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtconfig0::R`](R) reader structure"]
impl crate::Readable for WDTCONFIG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdtconfig0::W`](W) writer structure"]
impl crate::Writable for WDTCONFIG0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDTCONFIG0 to value 0x0001_3214"]
impl crate::Resettable for WDTCONFIG0_SPEC {
    const RESET_VALUE: u32 = 0x0001_3214;
}
