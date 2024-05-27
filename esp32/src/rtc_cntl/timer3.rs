#[doc = "Register `TIMER3` reader"]
pub type R = crate::R<TIMER3_SPEC>;
#[doc = "Register `TIMER3` writer"]
pub type W = crate::W<TIMER3_SPEC>;
#[doc = "Field `WIFI_WAIT_TIMER` reader - "]
pub type WIFI_WAIT_TIMER_R = crate::FieldReader<u16>;
#[doc = "Field `WIFI_WAIT_TIMER` writer - "]
pub type WIFI_WAIT_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `WIFI_POWERUP_TIMER` reader - "]
pub type WIFI_POWERUP_TIMER_R = crate::FieldReader;
#[doc = "Field `WIFI_POWERUP_TIMER` writer - "]
pub type WIFI_POWERUP_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ROM_RAM_WAIT_TIMER` reader - "]
pub type ROM_RAM_WAIT_TIMER_R = crate::FieldReader<u16>;
#[doc = "Field `ROM_RAM_WAIT_TIMER` writer - "]
pub type ROM_RAM_WAIT_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `ROM_RAM_POWERUP_TIMER` reader - "]
pub type ROM_RAM_POWERUP_TIMER_R = crate::FieldReader;
#[doc = "Field `ROM_RAM_POWERUP_TIMER` writer - "]
pub type ROM_RAM_POWERUP_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn wifi_wait_timer(&self) -> WIFI_WAIT_TIMER_R {
        WIFI_WAIT_TIMER_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15"]
    #[inline(always)]
    pub fn wifi_powerup_timer(&self) -> WIFI_POWERUP_TIMER_R {
        WIFI_POWERUP_TIMER_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    pub fn rom_ram_wait_timer(&self) -> ROM_RAM_WAIT_TIMER_R {
        ROM_RAM_WAIT_TIMER_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    pub fn rom_ram_powerup_timer(&self) -> ROM_RAM_POWERUP_TIMER_R {
        ROM_RAM_POWERUP_TIMER_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER3")
            .field("wifi_wait_timer", &self.wifi_wait_timer())
            .field("wifi_powerup_timer", &self.wifi_powerup_timer())
            .field("rom_ram_wait_timer", &self.rom_ram_wait_timer())
            .field("rom_ram_powerup_timer", &self.rom_ram_powerup_timer())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_wait_timer(&mut self) -> WIFI_WAIT_TIMER_W<TIMER3_SPEC> {
        WIFI_WAIT_TIMER_W::new(self, 0)
    }
    #[doc = "Bits 9:15"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_powerup_timer(&mut self) -> WIFI_POWERUP_TIMER_W<TIMER3_SPEC> {
        WIFI_POWERUP_TIMER_W::new(self, 9)
    }
    #[doc = "Bits 16:24"]
    #[inline(always)]
    #[must_use]
    pub fn rom_ram_wait_timer(&mut self) -> ROM_RAM_WAIT_TIMER_W<TIMER3_SPEC> {
        ROM_RAM_WAIT_TIMER_W::new(self, 16)
    }
    #[doc = "Bits 25:31"]
    #[inline(always)]
    #[must_use]
    pub fn rom_ram_powerup_timer(&mut self) -> ROM_RAM_POWERUP_TIMER_W<TIMER3_SPEC> {
        ROM_RAM_POWERUP_TIMER_W::new(self, 25)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER3_SPEC;
impl crate::RegisterSpec for TIMER3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer3::R`](R) reader structure"]
impl crate::Readable for TIMER3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer3::W`](W) writer structure"]
impl crate::Writable for TIMER3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER3 to value 0x1416_0a08"]
impl crate::Resettable for TIMER3_SPEC {
    const RESET_VALUE: u32 = 0x1416_0a08;
}
