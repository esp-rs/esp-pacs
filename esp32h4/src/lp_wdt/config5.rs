#[doc = "Register `CONFIG5` reader"]
pub type R = crate::R<CONFIG5_SPEC>;
#[doc = "Register `CONFIG5` writer"]
pub type W = crate::W<CONFIG5_SPEC>;
#[doc = "Field `WDT_CHIP_RESET_TARGET` reader - need_des"]
pub type WDT_CHIP_RESET_TARGET_R = crate::FieldReader;
#[doc = "Field `WDT_CHIP_RESET_TARGET` writer - need_des"]
pub type WDT_CHIP_RESET_TARGET_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WDT_CHIP_RESET_EN` reader - need_des"]
pub type WDT_CHIP_RESET_EN_R = crate::BitReader;
#[doc = "Field `WDT_CHIP_RESET_EN` writer - need_des"]
pub type WDT_CHIP_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_CHIP_RESET_KEY` reader - need_des"]
pub type WDT_CHIP_RESET_KEY_R = crate::FieldReader<u16>;
#[doc = "Field `WDT_CHIP_RESET_KEY` writer - need_des"]
pub type WDT_CHIP_RESET_KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn wdt_chip_reset_target(&self) -> WDT_CHIP_RESET_TARGET_R {
        WDT_CHIP_RESET_TARGET_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn wdt_chip_reset_en(&self) -> WDT_CHIP_RESET_EN_R {
        WDT_CHIP_RESET_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:17 - need_des"]
    #[inline(always)]
    pub fn wdt_chip_reset_key(&self) -> WDT_CHIP_RESET_KEY_R {
        WDT_CHIP_RESET_KEY_R::new(((self.bits >> 9) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG5")
            .field("wdt_chip_reset_target", &self.wdt_chip_reset_target())
            .field("wdt_chip_reset_en", &self.wdt_chip_reset_en())
            .field("wdt_chip_reset_key", &self.wdt_chip_reset_key())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn wdt_chip_reset_target(&mut self) -> WDT_CHIP_RESET_TARGET_W<'_, CONFIG5_SPEC> {
        WDT_CHIP_RESET_TARGET_W::new(self, 0)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn wdt_chip_reset_en(&mut self) -> WDT_CHIP_RESET_EN_W<'_, CONFIG5_SPEC> {
        WDT_CHIP_RESET_EN_W::new(self, 8)
    }
    #[doc = "Bits 9:17 - need_des"]
    #[inline(always)]
    pub fn wdt_chip_reset_key(&mut self) -> WDT_CHIP_RESET_KEY_W<'_, CONFIG5_SPEC> {
        WDT_CHIP_RESET_KEY_W::new(self, 9)
    }
}
#[doc = "Configure the RWDT timeout of stage3\n\nYou can [`read`](crate::Reg::read) this register and get [`config5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG5_SPEC;
impl crate::RegisterSpec for CONFIG5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config5::R`](R) reader structure"]
impl crate::Readable for CONFIG5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config5::W`](W) writer structure"]
impl crate::Writable for CONFIG5_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG5 to value 0xff"]
impl crate::Resettable for CONFIG5_SPEC {
    const RESET_VALUE: u32 = 0xff;
}
