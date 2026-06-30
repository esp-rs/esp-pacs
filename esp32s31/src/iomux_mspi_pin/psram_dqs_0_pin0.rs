#[doc = "Register `PSRAM_DQS_0_PIN0` reader"]
pub type R = crate::R<PSRAM_DQS_0_PIN0_SPEC>;
#[doc = "Register `PSRAM_DQS_0_PIN0` writer"]
pub type W = crate::W<PSRAM_DQS_0_PIN0_SPEC>;
#[doc = "Field `REG_PSRAM_DQS_0_XPD` reader - psram xpd dqs0"]
pub type REG_PSRAM_DQS_0_XPD_R = crate::BitReader;
#[doc = "Field `REG_PSRAM_DQS_0_XPD` writer - psram xpd dqs0"]
pub type REG_PSRAM_DQS_0_XPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PSRAM_DQS_0_PHASE` reader - psram dqs0 phase"]
pub type REG_PSRAM_DQS_0_PHASE_R = crate::FieldReader;
#[doc = "Field `REG_PSRAM_DQS_0_PHASE` writer - psram dqs0 phase"]
pub type REG_PSRAM_DQS_0_PHASE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REG_PSRAM_DQS_0_DLI` reader - psram dqs0 dli"]
pub type REG_PSRAM_DQS_0_DLI_R = crate::FieldReader;
#[doc = "Field `REG_PSRAM_DQS_0_DLI` writer - psram dqs0 dli"]
pub type REG_PSRAM_DQS_0_DLI_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REG_PSRAM_DQS_0_DELAY_90` reader - psram dqs0 delay 90"]
pub type REG_PSRAM_DQS_0_DELAY_90_R = crate::FieldReader;
#[doc = "Field `REG_PSRAM_DQS_0_DELAY_90` writer - psram dqs0 delay 90"]
pub type REG_PSRAM_DQS_0_DELAY_90_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REG_PSRAM_DQS_0_HYS` reader - psram dqs0 sl"]
pub type REG_PSRAM_DQS_0_HYS_R = crate::BitReader;
#[doc = "Field `REG_PSRAM_DQS_0_HYS` writer - psram dqs0 sl"]
pub type REG_PSRAM_DQS_0_HYS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PSRAM_DQS_0_IE` reader - Reserved"]
pub type REG_PSRAM_DQS_0_IE_R = crate::BitReader;
#[doc = "Field `REG_PSRAM_DQS_0_IE` writer - Reserved"]
pub type REG_PSRAM_DQS_0_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PSRAM_DQS_0_WPU` reader - psram dqs0 wpu"]
pub type REG_PSRAM_DQS_0_WPU_R = crate::BitReader;
#[doc = "Field `REG_PSRAM_DQS_0_WPU` writer - psram dqs0 wpu"]
pub type REG_PSRAM_DQS_0_WPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PSRAM_DQS_0_WPD` reader - psram dqs0 wpd"]
pub type REG_PSRAM_DQS_0_WPD_R = crate::BitReader;
#[doc = "Field `REG_PSRAM_DQS_0_WPD` writer - psram dqs0 wpd"]
pub type REG_PSRAM_DQS_0_WPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_PSRAM_DQS_0_DRV` reader - psram dqs0 drv"]
pub type REG_PSRAM_DQS_0_DRV_R = crate::FieldReader;
#[doc = "Field `REG_PSRAM_DQS_0_DRV` writer - psram dqs0 drv"]
pub type REG_PSRAM_DQS_0_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REG_PSRAM_DQS_0_DELAY_270` reader - psram dqs0 delay 270"]
pub type REG_PSRAM_DQS_0_DELAY_270_R = crate::FieldReader;
#[doc = "Field `REG_PSRAM_DQS_0_DELAY_270` writer - psram dqs0 delay 270"]
pub type REG_PSRAM_DQS_0_DELAY_270_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REG_PSRAM_DQS_HOLD` reader - psram dqs hold"]
pub type REG_PSRAM_DQS_HOLD_R = crate::BitReader;
#[doc = "Field `REG_PSRAM_DQS_HOLD` writer - psram dqs hold"]
pub type REG_PSRAM_DQS_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - psram xpd dqs0"]
    #[inline(always)]
    pub fn reg_psram_dqs_0_xpd(&self) -> REG_PSRAM_DQS_0_XPD_R {
        REG_PSRAM_DQS_0_XPD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - psram dqs0 phase"]
    #[inline(always)]
    pub fn reg_psram_dqs_0_phase(&self) -> REG_PSRAM_DQS_0_PHASE_R {
        REG_PSRAM_DQS_0_PHASE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:6 - psram dqs0 dli"]
    #[inline(always)]
    pub fn reg_psram_dqs_0_dli(&self) -> REG_PSRAM_DQS_0_DLI_R {
        REG_PSRAM_DQS_0_DLI_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 7:10 - psram dqs0 delay 90"]
    #[inline(always)]
    pub fn reg_psram_dqs_0_delay_90(&self) -> REG_PSRAM_DQS_0_DELAY_90_R {
        REG_PSRAM_DQS_0_DELAY_90_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - psram dqs0 sl"]
    #[inline(always)]
    pub fn reg_psram_dqs_0_hys(&self) -> REG_PSRAM_DQS_0_HYS_R {
        REG_PSRAM_DQS_0_HYS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reg_psram_dqs_0_ie(&self) -> REG_PSRAM_DQS_0_IE_R {
        REG_PSRAM_DQS_0_IE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - psram dqs0 wpu"]
    #[inline(always)]
    pub fn reg_psram_dqs_0_wpu(&self) -> REG_PSRAM_DQS_0_WPU_R {
        REG_PSRAM_DQS_0_WPU_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - psram dqs0 wpd"]
    #[inline(always)]
    pub fn reg_psram_dqs_0_wpd(&self) -> REG_PSRAM_DQS_0_WPD_R {
        REG_PSRAM_DQS_0_WPD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:17 - psram dqs0 drv"]
    #[inline(always)]
    pub fn reg_psram_dqs_0_drv(&self) -> REG_PSRAM_DQS_0_DRV_R {
        REG_PSRAM_DQS_0_DRV_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:21 - psram dqs0 delay 270"]
    #[inline(always)]
    pub fn reg_psram_dqs_0_delay_270(&self) -> REG_PSRAM_DQS_0_DELAY_270_R {
        REG_PSRAM_DQS_0_DELAY_270_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - psram dqs hold"]
    #[inline(always)]
    pub fn reg_psram_dqs_hold(&self) -> REG_PSRAM_DQS_HOLD_R {
        REG_PSRAM_DQS_HOLD_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSRAM_DQS_0_PIN0")
            .field("reg_psram_dqs_0_xpd", &self.reg_psram_dqs_0_xpd())
            .field("reg_psram_dqs_0_phase", &self.reg_psram_dqs_0_phase())
            .field("reg_psram_dqs_0_dli", &self.reg_psram_dqs_0_dli())
            .field("reg_psram_dqs_0_delay_90", &self.reg_psram_dqs_0_delay_90())
            .field("reg_psram_dqs_0_hys", &self.reg_psram_dqs_0_hys())
            .field("reg_psram_dqs_0_ie", &self.reg_psram_dqs_0_ie())
            .field("reg_psram_dqs_0_wpu", &self.reg_psram_dqs_0_wpu())
            .field("reg_psram_dqs_0_wpd", &self.reg_psram_dqs_0_wpd())
            .field("reg_psram_dqs_0_drv", &self.reg_psram_dqs_0_drv())
            .field(
                "reg_psram_dqs_0_delay_270",
                &self.reg_psram_dqs_0_delay_270(),
            )
            .field("reg_psram_dqs_hold", &self.reg_psram_dqs_hold())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - psram xpd dqs0"]
    #[inline(always)]
    pub fn reg_psram_dqs_0_xpd(&mut self) -> REG_PSRAM_DQS_0_XPD_W<'_, PSRAM_DQS_0_PIN0_SPEC> {
        REG_PSRAM_DQS_0_XPD_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - psram dqs0 phase"]
    #[inline(always)]
    pub fn reg_psram_dqs_0_phase(&mut self) -> REG_PSRAM_DQS_0_PHASE_W<'_, PSRAM_DQS_0_PIN0_SPEC> {
        REG_PSRAM_DQS_0_PHASE_W::new(self, 1)
    }
    #[doc = "Bits 3:6 - psram dqs0 dli"]
    #[inline(always)]
    pub fn reg_psram_dqs_0_dli(&mut self) -> REG_PSRAM_DQS_0_DLI_W<'_, PSRAM_DQS_0_PIN0_SPEC> {
        REG_PSRAM_DQS_0_DLI_W::new(self, 3)
    }
    #[doc = "Bits 7:10 - psram dqs0 delay 90"]
    #[inline(always)]
    pub fn reg_psram_dqs_0_delay_90(
        &mut self,
    ) -> REG_PSRAM_DQS_0_DELAY_90_W<'_, PSRAM_DQS_0_PIN0_SPEC> {
        REG_PSRAM_DQS_0_DELAY_90_W::new(self, 7)
    }
    #[doc = "Bit 11 - psram dqs0 sl"]
    #[inline(always)]
    pub fn reg_psram_dqs_0_hys(&mut self) -> REG_PSRAM_DQS_0_HYS_W<'_, PSRAM_DQS_0_PIN0_SPEC> {
        REG_PSRAM_DQS_0_HYS_W::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reg_psram_dqs_0_ie(&mut self) -> REG_PSRAM_DQS_0_IE_W<'_, PSRAM_DQS_0_PIN0_SPEC> {
        REG_PSRAM_DQS_0_IE_W::new(self, 12)
    }
    #[doc = "Bit 13 - psram dqs0 wpu"]
    #[inline(always)]
    pub fn reg_psram_dqs_0_wpu(&mut self) -> REG_PSRAM_DQS_0_WPU_W<'_, PSRAM_DQS_0_PIN0_SPEC> {
        REG_PSRAM_DQS_0_WPU_W::new(self, 13)
    }
    #[doc = "Bit 14 - psram dqs0 wpd"]
    #[inline(always)]
    pub fn reg_psram_dqs_0_wpd(&mut self) -> REG_PSRAM_DQS_0_WPD_W<'_, PSRAM_DQS_0_PIN0_SPEC> {
        REG_PSRAM_DQS_0_WPD_W::new(self, 14)
    }
    #[doc = "Bits 15:17 - psram dqs0 drv"]
    #[inline(always)]
    pub fn reg_psram_dqs_0_drv(&mut self) -> REG_PSRAM_DQS_0_DRV_W<'_, PSRAM_DQS_0_PIN0_SPEC> {
        REG_PSRAM_DQS_0_DRV_W::new(self, 15)
    }
    #[doc = "Bits 18:21 - psram dqs0 delay 270"]
    #[inline(always)]
    pub fn reg_psram_dqs_0_delay_270(
        &mut self,
    ) -> REG_PSRAM_DQS_0_DELAY_270_W<'_, PSRAM_DQS_0_PIN0_SPEC> {
        REG_PSRAM_DQS_0_DELAY_270_W::new(self, 18)
    }
    #[doc = "Bit 22 - psram dqs hold"]
    #[inline(always)]
    pub fn reg_psram_dqs_hold(&mut self) -> REG_PSRAM_DQS_HOLD_W<'_, PSRAM_DQS_0_PIN0_SPEC> {
        REG_PSRAM_DQS_HOLD_W::new(self, 22)
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
