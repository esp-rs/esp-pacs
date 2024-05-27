///Register `LACTCONFIG` reader
pub type R = crate::R<LACTCONFIG_SPEC>;
///Register `LACTCONFIG` writer
pub type W = crate::W<LACTCONFIG_SPEC>;
///Field `USE_REFTICK` reader - Reserved.
pub type USE_REFTICK_R = crate::BitReader;
///Field `USE_REFTICK` writer - Reserved.
pub type USE_REFTICK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTC_ONLY` reader - Reserved.
pub type RTC_ONLY_R = crate::BitReader;
///Field `RTC_ONLY` writer - Reserved.
pub type RTC_ONLY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CPST_EN` reader - Reserved.
pub type CPST_EN_R = crate::BitReader;
///Field `CPST_EN` writer - Reserved.
pub type CPST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LAC_EN` reader - Reserved.
pub type LAC_EN_R = crate::BitReader;
///Field `LAC_EN` writer - Reserved.
pub type LAC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALARM_EN` reader - Reserved.
pub type ALARM_EN_R = crate::BitReader;
///Field `ALARM_EN` writer - Reserved.
pub type ALARM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LEVEL_INT_EN` reader - Reserved.
pub type LEVEL_INT_EN_R = crate::BitReader;
///Field `LEVEL_INT_EN` writer - Reserved.
pub type LEVEL_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EDGE_INT_EN` reader - Reserved.
pub type EDGE_INT_EN_R = crate::BitReader;
///Field `EDGE_INT_EN` writer - Reserved.
pub type EDGE_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIVIDER` reader - Reserved.
pub type DIVIDER_R = crate::FieldReader<u16>;
///Field `DIVIDER` writer - Reserved.
pub type DIVIDER_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `AUTORELOAD` reader - Reserved.
pub type AUTORELOAD_R = crate::BitReader;
///Field `AUTORELOAD` writer - Reserved.
pub type AUTORELOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INCREASE` reader - Reserved.
pub type INCREASE_R = crate::BitReader;
///Field `INCREASE` writer - Reserved.
pub type INCREASE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EN` reader - Reserved.
pub type EN_R = crate::BitReader;
///Field `EN` writer - Reserved.
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 6 - Reserved.
    #[inline(always)]
    pub fn use_reftick(&self) -> USE_REFTICK_R {
        USE_REFTICK_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Reserved.
    #[inline(always)]
    pub fn rtc_only(&self) -> RTC_ONLY_R {
        RTC_ONLY_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Reserved.
    #[inline(always)]
    pub fn cpst_en(&self) -> CPST_EN_R {
        CPST_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Reserved.
    #[inline(always)]
    pub fn lac_en(&self) -> LAC_EN_R {
        LAC_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Reserved.
    #[inline(always)]
    pub fn alarm_en(&self) -> ALARM_EN_R {
        ALARM_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Reserved.
    #[inline(always)]
    pub fn level_int_en(&self) -> LEVEL_INT_EN_R {
        LEVEL_INT_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Reserved.
    #[inline(always)]
    pub fn edge_int_en(&self) -> EDGE_INT_EN_R {
        EDGE_INT_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:28 - Reserved.
    #[inline(always)]
    pub fn divider(&self) -> DIVIDER_R {
        DIVIDER_R::new(((self.bits >> 13) & 0xffff) as u16)
    }
    ///Bit 29 - Reserved.
    #[inline(always)]
    pub fn autoreload(&self) -> AUTORELOAD_R {
        AUTORELOAD_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Reserved.
    #[inline(always)]
    pub fn increase(&self) -> INCREASE_R {
        INCREASE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Reserved.
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LACTCONFIG")
            .field("use_reftick", &self.use_reftick())
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
    ///Bit 6 - Reserved.
    #[inline(always)]
    #[must_use]
    pub fn use_reftick(&mut self) -> USE_REFTICK_W<LACTCONFIG_SPEC> {
        USE_REFTICK_W::new(self, 6)
    }
    ///Bit 7 - Reserved.
    #[inline(always)]
    #[must_use]
    pub fn rtc_only(&mut self) -> RTC_ONLY_W<LACTCONFIG_SPEC> {
        RTC_ONLY_W::new(self, 7)
    }
    ///Bit 8 - Reserved.
    #[inline(always)]
    #[must_use]
    pub fn cpst_en(&mut self) -> CPST_EN_W<LACTCONFIG_SPEC> {
        CPST_EN_W::new(self, 8)
    }
    ///Bit 9 - Reserved.
    #[inline(always)]
    #[must_use]
    pub fn lac_en(&mut self) -> LAC_EN_W<LACTCONFIG_SPEC> {
        LAC_EN_W::new(self, 9)
    }
    ///Bit 10 - Reserved.
    #[inline(always)]
    #[must_use]
    pub fn alarm_en(&mut self) -> ALARM_EN_W<LACTCONFIG_SPEC> {
        ALARM_EN_W::new(self, 10)
    }
    ///Bit 11 - Reserved.
    #[inline(always)]
    #[must_use]
    pub fn level_int_en(&mut self) -> LEVEL_INT_EN_W<LACTCONFIG_SPEC> {
        LEVEL_INT_EN_W::new(self, 11)
    }
    ///Bit 12 - Reserved.
    #[inline(always)]
    #[must_use]
    pub fn edge_int_en(&mut self) -> EDGE_INT_EN_W<LACTCONFIG_SPEC> {
        EDGE_INT_EN_W::new(self, 12)
    }
    ///Bits 13:28 - Reserved.
    #[inline(always)]
    #[must_use]
    pub fn divider(&mut self) -> DIVIDER_W<LACTCONFIG_SPEC> {
        DIVIDER_W::new(self, 13)
    }
    ///Bit 29 - Reserved.
    #[inline(always)]
    #[must_use]
    pub fn autoreload(&mut self) -> AUTORELOAD_W<LACTCONFIG_SPEC> {
        AUTORELOAD_W::new(self, 29)
    }
    ///Bit 30 - Reserved.
    #[inline(always)]
    #[must_use]
    pub fn increase(&mut self) -> INCREASE_W<LACTCONFIG_SPEC> {
        INCREASE_W::new(self, 30)
    }
    ///Bit 31 - Reserved.
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<LACTCONFIG_SPEC> {
        EN_W::new(self, 31)
    }
}
/**LACT configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`lactconfig::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lactconfig::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LACTCONFIG_SPEC;
impl crate::RegisterSpec for LACTCONFIG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lactconfig::R`](R) reader structure
impl crate::Readable for LACTCONFIG_SPEC {}
///`write(|w| ..)` method takes [`lactconfig::W`](W) writer structure
impl crate::Writable for LACTCONFIG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LACTCONFIG to value 0x6000_2300
impl crate::Resettable for LACTCONFIG_SPEC {
    const RESET_VALUE: u32 = 0x6000_2300;
}
