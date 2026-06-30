#[doc = "Register `PSRAM_DQ6_PIN0` reader"]
pub type R = crate::R<PSRAM_DQ6_PIN0_SPEC>;
#[doc = "Register `PSRAM_DQ6_PIN0` writer"]
pub type W = crate::W<PSRAM_DQ6_PIN0_SPEC>;
#[doc = "Field `REG_PSRAM_DQ6_DLI` reader - psram dq6 dli"]
pub type REG_PSRAM_DQ6_DLI_R = crate::FieldReader;
#[doc = "Field `REG_PSRAM_DQ6_DLI` writer - psram dq6 dli"]
pub type REG_PSRAM_DQ6_DLI_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REG_PSRAM_DQ6_DLC` reader - psram dq6 dlc"]
pub type REG_PSRAM_DQ6_DLC_R = crate::FieldReader;
#[doc = "Field `REG_PSRAM_DQ6_DLC` writer - psram dq6 dlc"]
pub type REG_PSRAM_DQ6_DLC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REG_PSRAM_DQ6_HYS` reader - psram dq6 sl"]
pub type REG_PSRAM_DQ6_HYS_R = crate::BitReader;
#[doc = "Field `REG_PSRAM_DQ6_HYS` writer - psram dq6 sl"]
pub type REG_PSRAM_DQ6_HYS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PSRAM_DQ6_IE` reader - Reserved"]
pub type REG_PSRAM_DQ6_IE_R = crate::BitReader;
#[doc = "Field `REG_PSRAM_DQ6_IE` writer - Reserved"]
pub type REG_PSRAM_DQ6_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PSRAM_DQ6_WPU` reader - psram dq6 wpu"]
pub type REG_PSRAM_DQ6_WPU_R = crate::BitReader;
#[doc = "Field `REG_PSRAM_DQ6_WPU` writer - psram dq6 wpu"]
pub type REG_PSRAM_DQ6_WPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PSRAM_DQ6_WPD` reader - psram dq6 wpd"]
pub type REG_PSRAM_DQ6_WPD_R = crate::BitReader;
#[doc = "Field `REG_PSRAM_DQ6_WPD` writer - psram dq6 wpd"]
pub type REG_PSRAM_DQ6_WPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PSRAM_DQ6_DRV` reader - psram dq6 drv"]
pub type REG_PSRAM_DQ6_DRV_R = crate::FieldReader;
#[doc = "Field `REG_PSRAM_DQ6_DRV` writer - psram dq6 drv"]
pub type REG_PSRAM_DQ6_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_PSRAM_DQ6_HOLD` reader - psram dq6 hold"]
pub type REG_PSRAM_DQ6_HOLD_R = crate::BitReader;
#[doc = "Field `REG_PSRAM_DQ6_HOLD` writer - psram dq6 hold"]
pub type REG_PSRAM_DQ6_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - psram dq6 dli"]
    #[inline(always)]
    pub fn reg_psram_dq6_dli(&self) -> REG_PSRAM_DQ6_DLI_R {
        REG_PSRAM_DQ6_DLI_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - psram dq6 dlc"]
    #[inline(always)]
    pub fn reg_psram_dq6_dlc(&self) -> REG_PSRAM_DQ6_DLC_R {
        REG_PSRAM_DQ6_DLC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - psram dq6 sl"]
    #[inline(always)]
    pub fn reg_psram_dq6_hys(&self) -> REG_PSRAM_DQ6_HYS_R {
        REG_PSRAM_DQ6_HYS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn reg_psram_dq6_ie(&self) -> REG_PSRAM_DQ6_IE_R {
        REG_PSRAM_DQ6_IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - psram dq6 wpu"]
    #[inline(always)]
    pub fn reg_psram_dq6_wpu(&self) -> REG_PSRAM_DQ6_WPU_R {
        REG_PSRAM_DQ6_WPU_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - psram dq6 wpd"]
    #[inline(always)]
    pub fn reg_psram_dq6_wpd(&self) -> REG_PSRAM_DQ6_WPD_R {
        REG_PSRAM_DQ6_WPD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - psram dq6 drv"]
    #[inline(always)]
    pub fn reg_psram_dq6_drv(&self) -> REG_PSRAM_DQ6_DRV_R {
        REG_PSRAM_DQ6_DRV_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - psram dq6 hold"]
    #[inline(always)]
    pub fn reg_psram_dq6_hold(&self) -> REG_PSRAM_DQ6_HOLD_R {
        REG_PSRAM_DQ6_HOLD_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSRAM_DQ6_PIN0")
            .field("reg_psram_dq6_dli", &self.reg_psram_dq6_dli())
            .field("reg_psram_dq6_dlc", &self.reg_psram_dq6_dlc())
            .field("reg_psram_dq6_hys", &self.reg_psram_dq6_hys())
            .field("reg_psram_dq6_ie", &self.reg_psram_dq6_ie())
            .field("reg_psram_dq6_wpu", &self.reg_psram_dq6_wpu())
            .field("reg_psram_dq6_wpd", &self.reg_psram_dq6_wpd())
            .field("reg_psram_dq6_drv", &self.reg_psram_dq6_drv())
            .field("reg_psram_dq6_hold", &self.reg_psram_dq6_hold())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - psram dq6 dli"]
    #[inline(always)]
    pub fn reg_psram_dq6_dli(&mut self) -> REG_PSRAM_DQ6_DLI_W<'_, PSRAM_DQ6_PIN0_SPEC> {
        REG_PSRAM_DQ6_DLI_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - psram dq6 dlc"]
    #[inline(always)]
    pub fn reg_psram_dq6_dlc(&mut self) -> REG_PSRAM_DQ6_DLC_W<'_, PSRAM_DQ6_PIN0_SPEC> {
        REG_PSRAM_DQ6_DLC_W::new(self, 4)
    }
    #[doc = "Bit 8 - psram dq6 sl"]
    #[inline(always)]
    pub fn reg_psram_dq6_hys(&mut self) -> REG_PSRAM_DQ6_HYS_W<'_, PSRAM_DQ6_PIN0_SPEC> {
        REG_PSRAM_DQ6_HYS_W::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn reg_psram_dq6_ie(&mut self) -> REG_PSRAM_DQ6_IE_W<'_, PSRAM_DQ6_PIN0_SPEC> {
        REG_PSRAM_DQ6_IE_W::new(self, 9)
    }
    #[doc = "Bit 10 - psram dq6 wpu"]
    #[inline(always)]
    pub fn reg_psram_dq6_wpu(&mut self) -> REG_PSRAM_DQ6_WPU_W<'_, PSRAM_DQ6_PIN0_SPEC> {
        REG_PSRAM_DQ6_WPU_W::new(self, 10)
    }
    #[doc = "Bit 11 - psram dq6 wpd"]
    #[inline(always)]
    pub fn reg_psram_dq6_wpd(&mut self) -> REG_PSRAM_DQ6_WPD_W<'_, PSRAM_DQ6_PIN0_SPEC> {
        REG_PSRAM_DQ6_WPD_W::new(self, 11)
    }
    #[doc = "Bits 12:14 - psram dq6 drv"]
    #[inline(always)]
    pub fn reg_psram_dq6_drv(&mut self) -> REG_PSRAM_DQ6_DRV_W<'_, PSRAM_DQ6_PIN0_SPEC> {
        REG_PSRAM_DQ6_DRV_W::new(self, 12)
    }
    #[doc = "Bit 15 - psram dq6 hold"]
    #[inline(always)]
    pub fn reg_psram_dq6_hold(&mut self) -> REG_PSRAM_DQ6_HOLD_W<'_, PSRAM_DQ6_PIN0_SPEC> {
        REG_PSRAM_DQ6_HOLD_W::new(self, 15)
    }
}
#[doc = "IOMUX_MSPI_PIN_PSRAM_D_PIN0_REG\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_dq6_pin0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_dq6_pin0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSRAM_DQ6_PIN0_SPEC;
impl crate::RegisterSpec for PSRAM_DQ6_PIN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_dq6_pin0::R`](R) reader structure"]
impl crate::Readable for PSRAM_DQ6_PIN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psram_dq6_pin0::W`](W) writer structure"]
impl crate::Writable for PSRAM_DQ6_PIN0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_DQ6_PIN0 to value 0x4000"]
impl crate::Resettable for PSRAM_DQ6_PIN0_SPEC {
    const RESET_VALUE: u32 = 0x4000;
}
