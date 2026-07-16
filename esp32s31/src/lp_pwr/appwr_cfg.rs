#[doc = "Register `APPWR_CFG` reader"]
pub type R = crate::R<APPWR_CFG_SPEC>;
#[doc = "Register `APPWR_CFG` writer"]
pub type W = crate::W<APPWR_CFG_SPEC>;
#[doc = "Field `STALL_WAIT_TIMER` reader - the wait timer config for ap sys"]
pub type STALL_WAIT_TIMER_R = crate::FieldReader;
#[doc = "Field `STALL_WAIT_TIMER` writer - the wait timer config for ap sys"]
pub type STALL_WAIT_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BACKUP_EN` reader - the backup enable config register for ap sys"]
pub type BACKUP_EN_R = crate::BitReader;
#[doc = "Field `BACKUP_EN` writer - the backup enable config register for ap sys"]
pub type BACKUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESTORE_MODE` reader - restore mode config register for apsys"]
pub type RESTORE_MODE_R = crate::FieldReader;
#[doc = "Field `RESTORE_MODE` writer - restore mode config register for apsys"]
pub type RESTORE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BACKUP_MODE` reader - backup mode config register for apsys"]
pub type BACKUP_MODE_R = crate::FieldReader;
#[doc = "Field `BACKUP_MODE` writer - backup mode config register for apsys"]
pub type BACKUP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `HPCPU_STALL_EN` reader - 1: enable hpcpu stall 0: disable hpcpu stall"]
pub type HPCPU_STALL_EN_R = crate::BitReader;
#[doc = "Field `HPCPU_STALL_EN` writer - 1: enable hpcpu stall 0: disable hpcpu stall"]
pub type HPCPU_STALL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - the wait timer config for ap sys"]
    #[inline(always)]
    pub fn stall_wait_timer(&self) -> STALL_WAIT_TIMER_R {
        STALL_WAIT_TIMER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - the backup enable config register for ap sys"]
    #[inline(always)]
    pub fn backup_en(&self) -> BACKUP_EN_R {
        BACKUP_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:13 - restore mode config register for apsys"]
    #[inline(always)]
    pub fn restore_mode(&self) -> RESTORE_MODE_R {
        RESTORE_MODE_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bits 14:18 - backup mode config register for apsys"]
    #[inline(always)]
    pub fn backup_mode(&self) -> BACKUP_MODE_R {
        BACKUP_MODE_R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bit 19 - 1: enable hpcpu stall 0: disable hpcpu stall"]
    #[inline(always)]
    pub fn hpcpu_stall_en(&self) -> HPCPU_STALL_EN_R {
        HPCPU_STALL_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APPWR_CFG")
            .field("stall_wait_timer", &self.stall_wait_timer())
            .field("backup_en", &self.backup_en())
            .field("restore_mode", &self.restore_mode())
            .field("backup_mode", &self.backup_mode())
            .field("hpcpu_stall_en", &self.hpcpu_stall_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - the wait timer config for ap sys"]
    #[inline(always)]
    pub fn stall_wait_timer(&mut self) -> STALL_WAIT_TIMER_W<'_, APPWR_CFG_SPEC> {
        STALL_WAIT_TIMER_W::new(self, 0)
    }
    #[doc = "Bit 8 - the backup enable config register for ap sys"]
    #[inline(always)]
    pub fn backup_en(&mut self) -> BACKUP_EN_W<'_, APPWR_CFG_SPEC> {
        BACKUP_EN_W::new(self, 8)
    }
    #[doc = "Bits 9:13 - restore mode config register for apsys"]
    #[inline(always)]
    pub fn restore_mode(&mut self) -> RESTORE_MODE_W<'_, APPWR_CFG_SPEC> {
        RESTORE_MODE_W::new(self, 9)
    }
    #[doc = "Bits 14:18 - backup mode config register for apsys"]
    #[inline(always)]
    pub fn backup_mode(&mut self) -> BACKUP_MODE_W<'_, APPWR_CFG_SPEC> {
        BACKUP_MODE_W::new(self, 14)
    }
    #[doc = "Bit 19 - 1: enable hpcpu stall 0: disable hpcpu stall"]
    #[inline(always)]
    pub fn hpcpu_stall_en(&mut self) -> HPCPU_STALL_EN_W<'_, APPWR_CFG_SPEC> {
        HPCPU_STALL_EN_W::new(self, 19)
    }
}
#[doc = "config register for apsys pwr\n\nYou can [`read`](crate::Reg::read) this register and get [`appwr_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appwr_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APPWR_CFG_SPEC;
impl crate::RegisterSpec for APPWR_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appwr_cfg::R`](R) reader structure"]
impl crate::Readable for APPWR_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`appwr_cfg::W`](W) writer structure"]
impl crate::Writable for APPWR_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APPWR_CFG to value 0x0008_0000"]
impl crate::Resettable for APPWR_CFG_SPEC {
    const RESET_VALUE: u32 = 0x0008_0000;
}
