#[doc = "Register `PSRAM_DQS_0_PIN0` reader"]
pub type R = crate::R<PSRAM_DQS_0_PIN0_SPEC>;
#[doc = "Register `PSRAM_DQS_0_PIN0` writer"]
pub type W = crate::W<PSRAM_DQS_0_PIN0_SPEC>;
#[doc = "Field `XPD` reader - psram xpd dqs0"]
pub type XPD_R = crate::BitReader;
#[doc = "Field `XPD` writer - psram xpd dqs0"]
pub type XPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHASE` reader - psram dqs0 phase"]
pub type PHASE_R = crate::FieldReader;
#[doc = "Field `PHASE` writer - psram dqs0 phase"]
pub type PHASE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DLI` reader - psram dqs0 dli"]
pub type DLI_R = crate::FieldReader;
#[doc = "Field `DLI` writer - psram dqs0 dli"]
pub type DLI_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DELAY_90` reader - psram dqs0 delay 90"]
pub type DELAY_90_R = crate::FieldReader;
#[doc = "Field `DELAY_90` writer - psram dqs0 delay 90"]
pub type DELAY_90_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HYS` reader - psram dqs0 sl"]
pub type HYS_R = crate::BitReader;
#[doc = "Field `HYS` writer - psram dqs0 sl"]
pub type HYS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE` reader - Reserved"]
pub type IE_R = crate::BitReader;
#[doc = "Field `IE` writer - Reserved"]
pub type IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPU` reader - psram dqs0 wpu"]
pub type WPU_R = crate::BitReader;
#[doc = "Field `WPU` writer - psram dqs0 wpu"]
pub type WPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPD` reader - psram dqs0 wpd"]
pub type WPD_R = crate::BitReader;
#[doc = "Field `WPD` writer - psram dqs0 wpd"]
pub type WPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRV` reader - psram dqs0 drv"]
pub type DRV_R = crate::FieldReader;
#[doc = "Field `DRV` writer - psram dqs0 drv"]
pub type DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DELAY_270` reader - psram dqs0 delay 270"]
pub type DELAY_270_R = crate::FieldReader;
#[doc = "Field `DELAY_270` writer - psram dqs0 delay 270"]
pub type DELAY_270_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HOLD` reader - psram dqs hold"]
pub type HOLD_R = crate::BitReader;
#[doc = "Field `HOLD` writer - psram dqs hold"]
pub type HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - psram xpd dqs0"]
    #[inline(always)]
    pub fn xpd(&self) -> XPD_R {
        XPD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - psram dqs0 phase"]
    #[inline(always)]
    pub fn phase(&self) -> PHASE_R {
        PHASE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:6 - psram dqs0 dli"]
    #[inline(always)]
    pub fn dli(&self) -> DLI_R {
        DLI_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:10 - psram dqs0 delay 90"]
    #[inline(always)]
    pub fn delay_90(&self) -> DELAY_90_R {
        DELAY_90_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - psram dqs0 sl"]
    #[inline(always)]
    pub fn hys(&self) -> HYS_R {
        HYS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - psram dqs0 wpu"]
    #[inline(always)]
    pub fn wpu(&self) -> WPU_R {
        WPU_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - psram dqs0 wpd"]
    #[inline(always)]
    pub fn wpd(&self) -> WPD_R {
        WPD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:17 - psram dqs0 drv"]
    #[inline(always)]
    pub fn drv(&self) -> DRV_R {
        DRV_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:21 - psram dqs0 delay 270"]
    #[inline(always)]
    pub fn delay_270(&self) -> DELAY_270_R {
        DELAY_270_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - psram dqs hold"]
    #[inline(always)]
    pub fn hold(&self) -> HOLD_R {
        HOLD_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSRAM_DQS_0_PIN0")
            .field("xpd", &self.xpd())
            .field("phase", &self.phase())
            .field("dli", &self.dli())
            .field("delay_90", &self.delay_90())
            .field("hys", &self.hys())
            .field("ie", &self.ie())
            .field("wpu", &self.wpu())
            .field("wpd", &self.wpd())
            .field("drv", &self.drv())
            .field("delay_270", &self.delay_270())
            .field("hold", &self.hold())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - psram xpd dqs0"]
    #[inline(always)]
    pub fn xpd(&mut self) -> XPD_W<'_, PSRAM_DQS_0_PIN0_SPEC> {
        XPD_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - psram dqs0 phase"]
    #[inline(always)]
    pub fn phase(&mut self) -> PHASE_W<'_, PSRAM_DQS_0_PIN0_SPEC> {
        PHASE_W::new(self, 1)
    }
    #[doc = "Bits 3:6 - psram dqs0 dli"]
    #[inline(always)]
    pub fn dli(&mut self) -> DLI_W<'_, PSRAM_DQS_0_PIN0_SPEC> {
        DLI_W::new(self, 3)
    }
    #[doc = "Bits 7:10 - psram dqs0 delay 90"]
    #[inline(always)]
    pub fn delay_90(&mut self) -> DELAY_90_W<'_, PSRAM_DQS_0_PIN0_SPEC> {
        DELAY_90_W::new(self, 7)
    }
    #[doc = "Bit 11 - psram dqs0 sl"]
    #[inline(always)]
    pub fn hys(&mut self) -> HYS_W<'_, PSRAM_DQS_0_PIN0_SPEC> {
        HYS_W::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W<'_, PSRAM_DQS_0_PIN0_SPEC> {
        IE_W::new(self, 12)
    }
    #[doc = "Bit 13 - psram dqs0 wpu"]
    #[inline(always)]
    pub fn wpu(&mut self) -> WPU_W<'_, PSRAM_DQS_0_PIN0_SPEC> {
        WPU_W::new(self, 13)
    }
    #[doc = "Bit 14 - psram dqs0 wpd"]
    #[inline(always)]
    pub fn wpd(&mut self) -> WPD_W<'_, PSRAM_DQS_0_PIN0_SPEC> {
        WPD_W::new(self, 14)
    }
    #[doc = "Bits 15:17 - psram dqs0 drv"]
    #[inline(always)]
    pub fn drv(&mut self) -> DRV_W<'_, PSRAM_DQS_0_PIN0_SPEC> {
        DRV_W::new(self, 15)
    }
    #[doc = "Bits 18:21 - psram dqs0 delay 270"]
    #[inline(always)]
    pub fn delay_270(&mut self) -> DELAY_270_W<'_, PSRAM_DQS_0_PIN0_SPEC> {
        DELAY_270_W::new(self, 18)
    }
    #[doc = "Bit 22 - psram dqs hold"]
    #[inline(always)]
    pub fn hold(&mut self) -> HOLD_W<'_, PSRAM_DQS_0_PIN0_SPEC> {
        HOLD_W::new(self, 22)
    }
}
#[doc = "IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_dqs_0_pin0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_dqs_0_pin0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSRAM_DQS_0_PIN0_SPEC;
impl crate::RegisterSpec for PSRAM_DQS_0_PIN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_dqs_0_pin0::R`](R) reader structure"]
impl crate::Readable for PSRAM_DQS_0_PIN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psram_dqs_0_pin0::W`](W) writer structure"]
impl crate::Writable for PSRAM_DQS_0_PIN0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_DQS_0_PIN0 to value 0x0002_0000"]
impl crate::Resettable for PSRAM_DQS_0_PIN0_SPEC {
    const RESET_VALUE: u32 = 0x0002_0000;
}
