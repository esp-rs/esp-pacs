#[doc = "Register `PSRAM_D_PIN0` reader"]
pub type R = crate::R<PSRAM_D_PIN0_SPEC>;
#[doc = "Register `PSRAM_D_PIN0` writer"]
pub type W = crate::W<PSRAM_D_PIN0_SPEC>;
#[doc = "Field `DLI` reader - psram d dli"]
pub type DLI_R = crate::FieldReader;
#[doc = "Field `DLI` writer - psram d dli"]
pub type DLI_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DLC` reader - psram d dlc"]
pub type DLC_R = crate::FieldReader;
#[doc = "Field `DLC` writer - psram d dlc"]
pub type DLC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HYS` reader - psram d sl"]
pub type HYS_R = crate::BitReader;
#[doc = "Field `HYS` writer - psram d sl"]
pub type HYS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE` reader - Reserved"]
pub type IE_R = crate::BitReader;
#[doc = "Field `IE` writer - Reserved"]
pub type IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPU` reader - psram d wpu"]
pub type WPU_R = crate::BitReader;
#[doc = "Field `WPU` writer - psram d wpu"]
pub type WPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPD` reader - psram d wpd"]
pub type WPD_R = crate::BitReader;
#[doc = "Field `WPD` writer - psram d wpd"]
pub type WPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRV` reader - psram d drv"]
pub type DRV_R = crate::FieldReader;
#[doc = "Field `DRV` writer - psram d drv"]
pub type DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HOLD` reader - psram d hold"]
pub type HOLD_R = crate::BitReader;
#[doc = "Field `HOLD` writer - psram d hold"]
pub type HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - psram d dli"]
    #[inline(always)]
    pub fn dli(&self) -> DLI_R {
        DLI_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - psram d dlc"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - psram d sl"]
    #[inline(always)]
    pub fn hys(&self) -> HYS_R {
        HYS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - psram d wpu"]
    #[inline(always)]
    pub fn wpu(&self) -> WPU_R {
        WPU_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - psram d wpd"]
    #[inline(always)]
    pub fn wpd(&self) -> WPD_R {
        WPD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - psram d drv"]
    #[inline(always)]
    pub fn drv(&self) -> DRV_R {
        DRV_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - psram d hold"]
    #[inline(always)]
    pub fn hold(&self) -> HOLD_R {
        HOLD_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSRAM_D_PIN0")
            .field("dli", &self.dli())
            .field("dlc", &self.dlc())
            .field("hys", &self.hys())
            .field("ie", &self.ie())
            .field("wpu", &self.wpu())
            .field("wpd", &self.wpd())
            .field("drv", &self.drv())
            .field("hold", &self.hold())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - psram d dli"]
    #[inline(always)]
    pub fn dli(&mut self) -> DLI_W<'_, PSRAM_D_PIN0_SPEC> {
        DLI_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - psram d dlc"]
    #[inline(always)]
    pub fn dlc(&mut self) -> DLC_W<'_, PSRAM_D_PIN0_SPEC> {
        DLC_W::new(self, 4)
    }
    #[doc = "Bit 8 - psram d sl"]
    #[inline(always)]
    pub fn hys(&mut self) -> HYS_W<'_, PSRAM_D_PIN0_SPEC> {
        HYS_W::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W<'_, PSRAM_D_PIN0_SPEC> {
        IE_W::new(self, 9)
    }
    #[doc = "Bit 10 - psram d wpu"]
    #[inline(always)]
    pub fn wpu(&mut self) -> WPU_W<'_, PSRAM_D_PIN0_SPEC> {
        WPU_W::new(self, 10)
    }
    #[doc = "Bit 11 - psram d wpd"]
    #[inline(always)]
    pub fn wpd(&mut self) -> WPD_W<'_, PSRAM_D_PIN0_SPEC> {
        WPD_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - psram d drv"]
    #[inline(always)]
    pub fn drv(&mut self) -> DRV_W<'_, PSRAM_D_PIN0_SPEC> {
        DRV_W::new(self, 12)
    }
    #[doc = "Bit 15 - psram d hold"]
    #[inline(always)]
    pub fn hold(&mut self) -> HOLD_W<'_, PSRAM_D_PIN0_SPEC> {
        HOLD_W::new(self, 15)
    }
}
#[doc = "IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_d_pin0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_d_pin0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSRAM_D_PIN0_SPEC;
impl crate::RegisterSpec for PSRAM_D_PIN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_d_pin0::R`](R) reader structure"]
impl crate::Readable for PSRAM_D_PIN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psram_d_pin0::W`](W) writer structure"]
impl crate::Writable for PSRAM_D_PIN0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_D_PIN0 to value 0x4000"]
impl crate::Resettable for PSRAM_D_PIN0_SPEC {
    const RESET_VALUE: u32 = 0x4000;
}
