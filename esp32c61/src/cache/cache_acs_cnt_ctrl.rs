#[doc = "Register `CACHE_ACS_CNT_CTRL` reader"]
pub type R = crate::R<CACHE_ACS_CNT_CTRL_SPEC>;
#[doc = "Register `CACHE_ACS_CNT_CTRL` writer"]
pub type W = crate::W<CACHE_ACS_CNT_CTRL_SPEC>;
#[doc = "Field `IBUS0_CNT_ENA` reader - The bit is used to enable ibus0 counter in L1-ICache0."]
pub type IBUS0_CNT_ENA_R = crate::BitReader;
#[doc = "Field `IBUS1_CNT_ENA` reader - The bit is used to enable ibus1 counter in L1-ICache1."]
pub type IBUS1_CNT_ENA_R = crate::BitReader;
#[doc = "Field `IBUS2_CNT_ENA` reader - Reserved"]
pub type IBUS2_CNT_ENA_R = crate::BitReader;
#[doc = "Field `IBUS3_CNT_ENA` reader - Reserved"]
pub type IBUS3_CNT_ENA_R = crate::BitReader;
#[doc = "Field `BUS0_CNT_ENA` reader - The bit is used to enable dbus0 counter in L1-DCache."]
pub type BUS0_CNT_ENA_R = crate::BitReader;
#[doc = "Field `BUS0_CNT_ENA` writer - The bit is used to enable dbus0 counter in L1-DCache."]
pub type BUS0_CNT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS1_CNT_ENA` reader - The bit is used to enable dbus1 counter in L1-DCache."]
pub type BUS1_CNT_ENA_R = crate::BitReader;
#[doc = "Field `BUS1_CNT_ENA` writer - The bit is used to enable dbus1 counter in L1-DCache."]
pub type BUS1_CNT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBUS2_CNT_ENA` reader - Reserved"]
pub type DBUS2_CNT_ENA_R = crate::BitReader;
#[doc = "Field `DBUS3_CNT_ENA` reader - Reserved"]
pub type DBUS3_CNT_ENA_R = crate::BitReader;
#[doc = "Field `IBUS0_CNT_CLR` writer - The bit is used to clear ibus0 counter in L1-ICache0."]
pub type IBUS0_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBUS1_CNT_CLR` writer - The bit is used to clear ibus1 counter in L1-ICache1."]
pub type IBUS1_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBUS2_CNT_CLR` writer - Reserved"]
pub type IBUS2_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBUS3_CNT_CLR` writer - Reserved"]
pub type IBUS3_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS0_CNT_CLR` writer - The bit is used to clear dbus0 counter in L1-DCache."]
pub type BUS0_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS1_CNT_CLR` writer - The bit is used to clear dbus1 counter in L1-DCache."]
pub type BUS1_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBUS2_CNT_CLR` reader - Reserved"]
pub type DBUS2_CNT_CLR_R = crate::BitReader;
#[doc = "Field `DBUS3_CNT_CLR` reader - Reserved"]
pub type DBUS3_CNT_CLR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable ibus0 counter in L1-ICache0."]
    #[inline(always)]
    pub fn ibus0_cnt_ena(&self) -> IBUS0_CNT_ENA_R {
        IBUS0_CNT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable ibus1 counter in L1-ICache1."]
    #[inline(always)]
    pub fn ibus1_cnt_ena(&self) -> IBUS1_CNT_ENA_R {
        IBUS1_CNT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn ibus2_cnt_ena(&self) -> IBUS2_CNT_ENA_R {
        IBUS2_CNT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn ibus3_cnt_ena(&self) -> IBUS3_CNT_ENA_R {
        IBUS3_CNT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to enable dbus0 counter in L1-DCache."]
    #[inline(always)]
    pub fn bus0_cnt_ena(&self) -> BUS0_CNT_ENA_R {
        BUS0_CNT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit is used to enable dbus1 counter in L1-DCache."]
    #[inline(always)]
    pub fn bus1_cnt_ena(&self) -> BUS1_CNT_ENA_R {
        BUS1_CNT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn dbus2_cnt_ena(&self) -> DBUS2_CNT_ENA_R {
        DBUS2_CNT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn dbus3_cnt_ena(&self) -> DBUS3_CNT_ENA_R {
        DBUS3_CNT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    pub fn dbus2_cnt_clr(&self) -> DBUS2_CNT_CLR_R {
        DBUS2_CNT_CLR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn dbus3_cnt_clr(&self) -> DBUS3_CNT_CLR_R {
        DBUS3_CNT_CLR_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_ACS_CNT_CTRL")
            .field("ibus0_cnt_ena", &self.ibus0_cnt_ena())
            .field("ibus1_cnt_ena", &self.ibus1_cnt_ena())
            .field("ibus2_cnt_ena", &self.ibus2_cnt_ena())
            .field("ibus3_cnt_ena", &self.ibus3_cnt_ena())
            .field("bus0_cnt_ena", &self.bus0_cnt_ena())
            .field("bus1_cnt_ena", &self.bus1_cnt_ena())
            .field("dbus2_cnt_ena", &self.dbus2_cnt_ena())
            .field("dbus3_cnt_ena", &self.dbus3_cnt_ena())
            .field("dbus2_cnt_clr", &self.dbus2_cnt_clr())
            .field("dbus3_cnt_clr", &self.dbus3_cnt_clr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - The bit is used to enable dbus0 counter in L1-DCache."]
    #[inline(always)]
    pub fn bus0_cnt_ena(&mut self) -> BUS0_CNT_ENA_W<CACHE_ACS_CNT_CTRL_SPEC> {
        BUS0_CNT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - The bit is used to enable dbus1 counter in L1-DCache."]
    #[inline(always)]
    pub fn bus1_cnt_ena(&mut self) -> BUS1_CNT_ENA_W<CACHE_ACS_CNT_CTRL_SPEC> {
        BUS1_CNT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 16 - The bit is used to clear ibus0 counter in L1-ICache0."]
    #[inline(always)]
    pub fn ibus0_cnt_clr(&mut self) -> IBUS0_CNT_CLR_W<CACHE_ACS_CNT_CTRL_SPEC> {
        IBUS0_CNT_CLR_W::new(self, 16)
    }
    #[doc = "Bit 17 - The bit is used to clear ibus1 counter in L1-ICache1."]
    #[inline(always)]
    pub fn ibus1_cnt_clr(&mut self) -> IBUS1_CNT_CLR_W<CACHE_ACS_CNT_CTRL_SPEC> {
        IBUS1_CNT_CLR_W::new(self, 17)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn ibus2_cnt_clr(&mut self) -> IBUS2_CNT_CLR_W<CACHE_ACS_CNT_CTRL_SPEC> {
        IBUS2_CNT_CLR_W::new(self, 18)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn ibus3_cnt_clr(&mut self) -> IBUS3_CNT_CLR_W<CACHE_ACS_CNT_CTRL_SPEC> {
        IBUS3_CNT_CLR_W::new(self, 19)
    }
    #[doc = "Bit 20 - The bit is used to clear dbus0 counter in L1-DCache."]
    #[inline(always)]
    pub fn bus0_cnt_clr(&mut self) -> BUS0_CNT_CLR_W<CACHE_ACS_CNT_CTRL_SPEC> {
        BUS0_CNT_CLR_W::new(self, 20)
    }
    #[doc = "Bit 21 - The bit is used to clear dbus1 counter in L1-DCache."]
    #[inline(always)]
    pub fn bus1_cnt_clr(&mut self) -> BUS1_CNT_CLR_W<CACHE_ACS_CNT_CTRL_SPEC> {
        BUS1_CNT_CLR_W::new(self, 21)
    }
}
#[doc = "Cache Access Counter enable and clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_acs_cnt_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_acs_cnt_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_ACS_CNT_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_ACS_CNT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_acs_cnt_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_ACS_CNT_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_acs_cnt_ctrl::W`](W) writer structure"]
impl crate::Writable for CACHE_ACS_CNT_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_ACS_CNT_CTRL to value 0"]
impl crate::Resettable for CACHE_ACS_CNT_CTRL_SPEC {}
