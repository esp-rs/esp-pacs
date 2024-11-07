#[doc = "Register `LACTCONFIG` reader"]
pub type R = crate::R<LACTCONFIG_SPEC>;
#[doc = "Register `LACTCONFIG` writer"]
pub type W = crate::W<LACTCONFIG_SPEC>;
#[doc = "Field `RTC_ONLY` reader - "]
pub type RTC_ONLY_R = crate::BitReader;
#[doc = "Field `RTC_ONLY` writer - "]
pub type RTC_ONLY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPST_EN` reader - "]
pub type CPST_EN_R = crate::BitReader;
#[doc = "Field `CPST_EN` writer - "]
pub type CPST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LAC_EN` reader - "]
pub type LAC_EN_R = crate::BitReader;
#[doc = "Field `LAC_EN` writer - "]
pub type LAC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALARM_EN` reader - "]
pub type ALARM_EN_R = crate::BitReader;
#[doc = "Field `ALARM_EN` writer - "]
pub type ALARM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEVEL_INT_EN` reader - "]
pub type LEVEL_INT_EN_R = crate::BitReader;
#[doc = "Field `LEVEL_INT_EN` writer - "]
pub type LEVEL_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDGE_INT_EN` reader - "]
pub type EDGE_INT_EN_R = crate::BitReader;
#[doc = "Field `EDGE_INT_EN` writer - "]
pub type EDGE_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVIDER` reader - "]
pub type DIVIDER_R = crate::FieldReader<u16>;
#[doc = "Field `DIVIDER` writer - "]
pub type DIVIDER_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `AUTORELOAD` reader - "]
pub type AUTORELOAD_R = crate::BitReader;
#[doc = "Field `AUTORELOAD` writer - "]
pub type AUTORELOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCREASE` reader - "]
pub type INCREASE_R = crate::BitReader;
#[doc = "Field `INCREASE` writer - "]
pub type INCREASE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - "]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - "]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rtc_only(&self) -> RTC_ONLY_R {
        RTC_ONLY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cpst_en(&self) -> CPST_EN_R {
        CPST_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn lac_en(&self) -> LAC_EN_R {
        LAC_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn alarm_en(&self) -> ALARM_EN_R {
        ALARM_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn level_int_en(&self) -> LEVEL_INT_EN_R {
        LEVEL_INT_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn edge_int_en(&self) -> EDGE_INT_EN_R {
        EDGE_INT_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:28"]
    #[inline(always)]
    pub fn divider(&self) -> DIVIDER_R {
        DIVIDER_R::new(((self.bits >> 13) & 0xffff) as u16)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn autoreload(&self) -> AUTORELOAD_R {
        AUTORELOAD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn increase(&self) -> INCREASE_R {
        INCREASE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LACTCONFIG")
            .field("rtc_only", &self.rtc_only())
            .field("cpst_en", &self.cpst_en())
            .field("lac_en", &self.lac_en())
            .field("alarm_en", &self.alarm_en())
            .field("level_int_en", &self.level_int_en())
            .field("edge_int_en", &self.edge_int_en())
            .field("divider", &self.divider())
            .field("autoreload", &self.autoreload())
            .field("increase", &self.increase())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rtc_only(&mut self) -> RTC_ONLY_W<LACTCONFIG_SPEC> {
        RTC_ONLY_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cpst_en(&mut self) -> CPST_EN_W<LACTCONFIG_SPEC> {
        CPST_EN_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn lac_en(&mut self) -> LAC_EN_W<LACTCONFIG_SPEC> {
        LAC_EN_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn alarm_en(&mut self) -> ALARM_EN_W<LACTCONFIG_SPEC> {
        ALARM_EN_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn level_int_en(&mut self) -> LEVEL_INT_EN_W<LACTCONFIG_SPEC> {
        LEVEL_INT_EN_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn edge_int_en(&mut self) -> EDGE_INT_EN_W<LACTCONFIG_SPEC> {
        EDGE_INT_EN_W::new(self, 12)
    }
    #[doc = "Bits 13:28"]
    #[inline(always)]
    pub fn divider(&mut self) -> DIVIDER_W<LACTCONFIG_SPEC> {
        DIVIDER_W::new(self, 13)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn autoreload(&mut self) -> AUTORELOAD_W<LACTCONFIG_SPEC> {
        AUTORELOAD_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn increase(&mut self) -> INCREASE_W<LACTCONFIG_SPEC> {
        INCREASE_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<LACTCONFIG_SPEC> {
        EN_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`lactconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lactconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LACTCONFIG_SPEC;
impl crate::RegisterSpec for LACTCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lactconfig::R`](R) reader structure"]
impl crate::Readable for LACTCONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lactconfig::W`](W) writer structure"]
impl crate::Writable for LACTCONFIG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LACTCONFIG to value 0x6000_2300"]
impl crate::Resettable for LACTCONFIG_SPEC {
    const RESET_VALUE: u32 = 0x6000_2300;
}
