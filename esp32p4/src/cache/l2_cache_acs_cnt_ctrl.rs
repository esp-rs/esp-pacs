#[doc = "Register `L2_CACHE_ACS_CNT_CTRL` reader"]
pub type R = crate::R<L2_CACHE_ACS_CNT_CTRL_SPEC>;
#[doc = "Register `L2_CACHE_ACS_CNT_CTRL` writer"]
pub type W = crate::W<L2_CACHE_ACS_CNT_CTRL_SPEC>;
#[doc = "Field `L2_IBUS0_CNT_ENA` reader - The bit is used to enable ibus0 counter in L2-Cache."]
pub type L2_IBUS0_CNT_ENA_R = crate::BitReader;
#[doc = "Field `L2_IBUS0_CNT_ENA` writer - The bit is used to enable ibus0 counter in L2-Cache."]
pub type L2_IBUS0_CNT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_IBUS1_CNT_ENA` reader - The bit is used to enable ibus1 counter in L2-Cache."]
pub type L2_IBUS1_CNT_ENA_R = crate::BitReader;
#[doc = "Field `L2_IBUS1_CNT_ENA` writer - The bit is used to enable ibus1 counter in L2-Cache."]
pub type L2_IBUS1_CNT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_IBUS2_CNT_ENA` reader - Reserved"]
pub type L2_IBUS2_CNT_ENA_R = crate::BitReader;
#[doc = "Field `L2_IBUS3_CNT_ENA` reader - Reserved"]
pub type L2_IBUS3_CNT_ENA_R = crate::BitReader;
#[doc = "Field `L2_DBUS0_CNT_ENA` reader - The bit is used to enable dbus0 counter in L2-Cache."]
pub type L2_DBUS0_CNT_ENA_R = crate::BitReader;
#[doc = "Field `L2_DBUS0_CNT_ENA` writer - The bit is used to enable dbus0 counter in L2-Cache."]
pub type L2_DBUS0_CNT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_DBUS1_CNT_ENA` reader - The bit is used to enable dbus1 counter in L2-Cache."]
pub type L2_DBUS1_CNT_ENA_R = crate::BitReader;
#[doc = "Field `L2_DBUS1_CNT_ENA` writer - The bit is used to enable dbus1 counter in L2-Cache."]
pub type L2_DBUS1_CNT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_DBUS2_CNT_ENA` reader - Reserved"]
pub type L2_DBUS2_CNT_ENA_R = crate::BitReader;
#[doc = "Field `L2_DBUS3_CNT_ENA` reader - Reserved"]
pub type L2_DBUS3_CNT_ENA_R = crate::BitReader;
#[doc = "Field `L2_IBUS0_CNT_CLR` writer - The bit is used to clear ibus0 counter in L2-Cache."]
pub type L2_IBUS0_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_IBUS1_CNT_CLR` writer - The bit is used to clear ibus1 counter in L2-Cache."]
pub type L2_IBUS1_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_IBUS2_CNT_CLR` reader - Reserved"]
pub type L2_IBUS2_CNT_CLR_R = crate::BitReader;
#[doc = "Field `L2_IBUS3_CNT_CLR` reader - Reserved"]
pub type L2_IBUS3_CNT_CLR_R = crate::BitReader;
#[doc = "Field `L2_DBUS0_CNT_CLR` writer - The bit is used to clear dbus0 counter in L2-Cache."]
pub type L2_DBUS0_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_DBUS1_CNT_CLR` writer - The bit is used to clear dbus1 counter in L2-Cache."]
pub type L2_DBUS1_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_DBUS2_CNT_CLR` reader - Reserved"]
pub type L2_DBUS2_CNT_CLR_R = crate::BitReader;
#[doc = "Field `L2_DBUS3_CNT_CLR` reader - Reserved"]
pub type L2_DBUS3_CNT_CLR_R = crate::BitReader;
impl R {
    #[doc = "Bit 8 - The bit is used to enable ibus0 counter in L2-Cache."]
    #[inline(always)]
    pub fn l2_ibus0_cnt_ena(&self) -> L2_IBUS0_CNT_ENA_R {
        L2_IBUS0_CNT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The bit is used to enable ibus1 counter in L2-Cache."]
    #[inline(always)]
    pub fn l2_ibus1_cnt_ena(&self) -> L2_IBUS1_CNT_ENA_R {
        L2_IBUS1_CNT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn l2_ibus2_cnt_ena(&self) -> L2_IBUS2_CNT_ENA_R {
        L2_IBUS2_CNT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    pub fn l2_ibus3_cnt_ena(&self) -> L2_IBUS3_CNT_ENA_R {
        L2_IBUS3_CNT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The bit is used to enable dbus0 counter in L2-Cache."]
    #[inline(always)]
    pub fn l2_dbus0_cnt_ena(&self) -> L2_DBUS0_CNT_ENA_R {
        L2_DBUS0_CNT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The bit is used to enable dbus1 counter in L2-Cache."]
    #[inline(always)]
    pub fn l2_dbus1_cnt_ena(&self) -> L2_DBUS1_CNT_ENA_R {
        L2_DBUS1_CNT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn l2_dbus2_cnt_ena(&self) -> L2_DBUS2_CNT_ENA_R {
        L2_DBUS2_CNT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn l2_dbus3_cnt_ena(&self) -> L2_DBUS3_CNT_ENA_R {
        L2_DBUS3_CNT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 26 - Reserved"]
    #[inline(always)]
    pub fn l2_ibus2_cnt_clr(&self) -> L2_IBUS2_CNT_CLR_R {
        L2_IBUS2_CNT_CLR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reserved"]
    #[inline(always)]
    pub fn l2_ibus3_cnt_clr(&self) -> L2_IBUS3_CNT_CLR_R {
        L2_IBUS3_CNT_CLR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    pub fn l2_dbus2_cnt_clr(&self) -> L2_DBUS2_CNT_CLR_R {
        L2_DBUS2_CNT_CLR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reserved"]
    #[inline(always)]
    pub fn l2_dbus3_cnt_clr(&self) -> L2_DBUS3_CNT_CLR_R {
        L2_DBUS3_CNT_CLR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_ACS_CNT_CTRL")
            .field(
                "l2_ibus0_cnt_ena",
                &format_args!("{}", self.l2_ibus0_cnt_ena().bit()),
            )
            .field(
                "l2_ibus1_cnt_ena",
                &format_args!("{}", self.l2_ibus1_cnt_ena().bit()),
            )
            .field(
                "l2_ibus2_cnt_ena",
                &format_args!("{}", self.l2_ibus2_cnt_ena().bit()),
            )
            .field(
                "l2_ibus3_cnt_ena",
                &format_args!("{}", self.l2_ibus3_cnt_ena().bit()),
            )
            .field(
                "l2_dbus0_cnt_ena",
                &format_args!("{}", self.l2_dbus0_cnt_ena().bit()),
            )
            .field(
                "l2_dbus1_cnt_ena",
                &format_args!("{}", self.l2_dbus1_cnt_ena().bit()),
            )
            .field(
                "l2_dbus2_cnt_ena",
                &format_args!("{}", self.l2_dbus2_cnt_ena().bit()),
            )
            .field(
                "l2_dbus3_cnt_ena",
                &format_args!("{}", self.l2_dbus3_cnt_ena().bit()),
            )
            .field(
                "l2_ibus2_cnt_clr",
                &format_args!("{}", self.l2_ibus2_cnt_clr().bit()),
            )
            .field(
                "l2_ibus3_cnt_clr",
                &format_args!("{}", self.l2_ibus3_cnt_clr().bit()),
            )
            .field(
                "l2_dbus2_cnt_clr",
                &format_args!("{}", self.l2_dbus2_cnt_clr().bit()),
            )
            .field(
                "l2_dbus3_cnt_clr",
                &format_args!("{}", self.l2_dbus3_cnt_clr().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_ACS_CNT_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 8 - The bit is used to enable ibus0 counter in L2-Cache."]
    #[inline(always)]
    #[must_use]
    pub fn l2_ibus0_cnt_ena(&mut self) -> L2_IBUS0_CNT_ENA_W<L2_CACHE_ACS_CNT_CTRL_SPEC> {
        L2_IBUS0_CNT_ENA_W::new(self, 8)
    }
    #[doc = "Bit 9 - The bit is used to enable ibus1 counter in L2-Cache."]
    #[inline(always)]
    #[must_use]
    pub fn l2_ibus1_cnt_ena(&mut self) -> L2_IBUS1_CNT_ENA_W<L2_CACHE_ACS_CNT_CTRL_SPEC> {
        L2_IBUS1_CNT_ENA_W::new(self, 9)
    }
    #[doc = "Bit 12 - The bit is used to enable dbus0 counter in L2-Cache."]
    #[inline(always)]
    #[must_use]
    pub fn l2_dbus0_cnt_ena(&mut self) -> L2_DBUS0_CNT_ENA_W<L2_CACHE_ACS_CNT_CTRL_SPEC> {
        L2_DBUS0_CNT_ENA_W::new(self, 12)
    }
    #[doc = "Bit 13 - The bit is used to enable dbus1 counter in L2-Cache."]
    #[inline(always)]
    #[must_use]
    pub fn l2_dbus1_cnt_ena(&mut self) -> L2_DBUS1_CNT_ENA_W<L2_CACHE_ACS_CNT_CTRL_SPEC> {
        L2_DBUS1_CNT_ENA_W::new(self, 13)
    }
    #[doc = "Bit 24 - The bit is used to clear ibus0 counter in L2-Cache."]
    #[inline(always)]
    #[must_use]
    pub fn l2_ibus0_cnt_clr(&mut self) -> L2_IBUS0_CNT_CLR_W<L2_CACHE_ACS_CNT_CTRL_SPEC> {
        L2_IBUS0_CNT_CLR_W::new(self, 24)
    }
    #[doc = "Bit 25 - The bit is used to clear ibus1 counter in L2-Cache."]
    #[inline(always)]
    #[must_use]
    pub fn l2_ibus1_cnt_clr(&mut self) -> L2_IBUS1_CNT_CLR_W<L2_CACHE_ACS_CNT_CTRL_SPEC> {
        L2_IBUS1_CNT_CLR_W::new(self, 25)
    }
    #[doc = "Bit 28 - The bit is used to clear dbus0 counter in L2-Cache."]
    #[inline(always)]
    #[must_use]
    pub fn l2_dbus0_cnt_clr(&mut self) -> L2_DBUS0_CNT_CLR_W<L2_CACHE_ACS_CNT_CTRL_SPEC> {
        L2_DBUS0_CNT_CLR_W::new(self, 28)
    }
    #[doc = "Bit 29 - The bit is used to clear dbus1 counter in L2-Cache."]
    #[inline(always)]
    #[must_use]
    pub fn l2_dbus1_cnt_clr(&mut self) -> L2_DBUS1_CNT_CLR_W<L2_CACHE_ACS_CNT_CTRL_SPEC> {
        L2_DBUS1_CNT_CLR_W::new(self, 29)
    }
}
#[doc = "Cache Access Counter enable and clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_acs_cnt_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_acs_cnt_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_ACS_CNT_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_ACS_CNT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_acs_cnt_ctrl::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_ACS_CNT_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_acs_cnt_ctrl::W`](W) writer structure"]
impl crate::Writable for L2_CACHE_ACS_CNT_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L2_CACHE_ACS_CNT_CTRL to value 0"]
impl crate::Resettable for L2_CACHE_ACS_CNT_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
