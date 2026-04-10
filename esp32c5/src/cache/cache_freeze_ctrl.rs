#[doc = "Register `CACHE_FREEZE_CTRL` reader"]
pub type R = crate::R<CACHE_FREEZE_CTRL_SPEC>;
#[doc = "Register `CACHE_FREEZE_CTRL` writer"]
pub type W = crate::W<CACHE_FREEZE_CTRL_SPEC>;
#[doc = "Field `ICACHE2_FREEZE_EN` reader - Reserved"]
pub type ICACHE2_FREEZE_EN_R = crate::BitReader;
#[doc = "Field `ICACHE2_FREEZE_EN` writer - Reserved"]
pub type ICACHE2_FREEZE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE2_FREEZE_MODE` reader - Reserved"]
pub type ICACHE2_FREEZE_MODE_R = crate::BitReader;
#[doc = "Field `ICACHE2_FREEZE_MODE` writer - Reserved"]
pub type ICACHE2_FREEZE_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_FREEZE_EN` reader - The bit is used to enable freeze operation on Cache. It can be cleared by software."]
pub type CACHE_FREEZE_EN_R = crate::BitReader;
#[doc = "Field `CACHE_FREEZE_EN` writer - The bit is used to enable freeze operation on Cache. It can be cleared by software."]
pub type CACHE_FREEZE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_FREEZE_MODE` reader - The bit is used to configure mode of freeze operation Cache. 0: a miss-access will not stuck. 1: a miss-access will stuck."]
pub type CACHE_FREEZE_MODE_R = crate::BitReader;
#[doc = "Field `CACHE_FREEZE_MODE` writer - The bit is used to configure mode of freeze operation Cache. 0: a miss-access will not stuck. 1: a miss-access will stuck."]
pub type CACHE_FREEZE_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_FREEZE_DONE` reader - The bit is used to indicate whether freeze operation on Cache is finished or not. 0: not finished. 1: finished."]
pub type CACHE_FREEZE_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn icache2_freeze_en(&self) -> ICACHE2_FREEZE_EN_R {
        ICACHE2_FREEZE_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn icache2_freeze_mode(&self) -> ICACHE2_FREEZE_MODE_R {
        ICACHE2_FREEZE_MODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - The bit is used to enable freeze operation on Cache. It can be cleared by software."]
    #[inline(always)]
    pub fn cache_freeze_en(&self) -> CACHE_FREEZE_EN_R {
        CACHE_FREEZE_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The bit is used to configure mode of freeze operation Cache. 0: a miss-access will not stuck. 1: a miss-access will stuck."]
    #[inline(always)]
    pub fn cache_freeze_mode(&self) -> CACHE_FREEZE_MODE_R {
        CACHE_FREEZE_MODE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The bit is used to indicate whether freeze operation on Cache is finished or not. 0: not finished. 1: finished."]
    #[inline(always)]
    pub fn cache_freeze_done(&self) -> CACHE_FREEZE_DONE_R {
        CACHE_FREEZE_DONE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_FREEZE_CTRL")
            .field("icache2_freeze_en", &self.icache2_freeze_en())
            .field("icache2_freeze_mode", &self.icache2_freeze_mode())
            .field("cache_freeze_en", &self.cache_freeze_en())
            .field("cache_freeze_mode", &self.cache_freeze_mode())
            .field("cache_freeze_done", &self.cache_freeze_done())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn icache2_freeze_en(&mut self) -> ICACHE2_FREEZE_EN_W<'_, CACHE_FREEZE_CTRL_SPEC> {
        ICACHE2_FREEZE_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn icache2_freeze_mode(&mut self) -> ICACHE2_FREEZE_MODE_W<'_, CACHE_FREEZE_CTRL_SPEC> {
        ICACHE2_FREEZE_MODE_W::new(self, 9)
    }
    #[doc = "Bit 16 - The bit is used to enable freeze operation on Cache. It can be cleared by software."]
    #[inline(always)]
    pub fn cache_freeze_en(&mut self) -> CACHE_FREEZE_EN_W<'_, CACHE_FREEZE_CTRL_SPEC> {
        CACHE_FREEZE_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - The bit is used to configure mode of freeze operation Cache. 0: a miss-access will not stuck. 1: a miss-access will stuck."]
    #[inline(always)]
    pub fn cache_freeze_mode(&mut self) -> CACHE_FREEZE_MODE_W<'_, CACHE_FREEZE_CTRL_SPEC> {
        CACHE_FREEZE_MODE_W::new(self, 17)
    }
}
#[doc = "Cache Freeze control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_freeze_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_freeze_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_FREEZE_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_FREEZE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_freeze_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_FREEZE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_freeze_ctrl::W`](W) writer structure"]
impl crate::Writable for CACHE_FREEZE_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_FREEZE_CTRL to value 0"]
impl crate::Resettable for CACHE_FREEZE_CTRL_SPEC {}
