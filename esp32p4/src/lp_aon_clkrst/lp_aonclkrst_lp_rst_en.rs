#[doc = "Register `LP_AONCLKRST_LP_RST_EN` reader"]
pub type R = crate::R<LP_AONCLKRST_LP_RST_EN_SPEC>;
#[doc = "Register `LP_AONCLKRST_LP_RST_EN` writer"]
pub type W = crate::W<LP_AONCLKRST_LP_RST_EN_SPEC>;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_HUK` reader - need_des"]
pub type LP_AONCLKRST_RST_EN_LP_HUK_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_HUK` writer - need_des"]
pub type LP_AONCLKRST_RST_EN_LP_HUK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_ANAPERI` reader - need_des"]
pub type LP_AONCLKRST_RST_EN_LP_ANAPERI_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_ANAPERI` writer - need_des"]
pub type LP_AONCLKRST_RST_EN_LP_ANAPERI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_WDT` reader - need_des"]
pub type LP_AONCLKRST_RST_EN_LP_WDT_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_WDT` writer - need_des"]
pub type LP_AONCLKRST_RST_EN_LP_WDT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_TIMER` reader - need_des"]
pub type LP_AONCLKRST_RST_EN_LP_TIMER_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_TIMER` writer - need_des"]
pub type LP_AONCLKRST_RST_EN_LP_TIMER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_RTC` reader - need_des"]
pub type LP_AONCLKRST_RST_EN_LP_RTC_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_RTC` writer - need_des"]
pub type LP_AONCLKRST_RST_EN_LP_RTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_MAILBOX` reader - need_des"]
pub type LP_AONCLKRST_RST_EN_LP_MAILBOX_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_MAILBOX` writer - need_des"]
pub type LP_AONCLKRST_RST_EN_LP_MAILBOX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_AONEFUSEREG` reader - need_des"]
pub type LP_AONCLKRST_RST_EN_LP_AONEFUSEREG_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_AONEFUSEREG` writer - need_des"]
pub type LP_AONCLKRST_RST_EN_LP_AONEFUSEREG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_RAM` reader - need_des"]
pub type LP_AONCLKRST_RST_EN_LP_RAM_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_RST_EN_LP_RAM` writer - need_des"]
pub type LP_AONCLKRST_RST_EN_LP_RAM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_huk(&self) -> LP_AONCLKRST_RST_EN_LP_HUK_R {
        LP_AONCLKRST_RST_EN_LP_HUK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_anaperi(&self) -> LP_AONCLKRST_RST_EN_LP_ANAPERI_R {
        LP_AONCLKRST_RST_EN_LP_ANAPERI_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_wdt(&self) -> LP_AONCLKRST_RST_EN_LP_WDT_R {
        LP_AONCLKRST_RST_EN_LP_WDT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_timer(&self) -> LP_AONCLKRST_RST_EN_LP_TIMER_R {
        LP_AONCLKRST_RST_EN_LP_TIMER_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_rtc(&self) -> LP_AONCLKRST_RST_EN_LP_RTC_R {
        LP_AONCLKRST_RST_EN_LP_RTC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_mailbox(&self) -> LP_AONCLKRST_RST_EN_LP_MAILBOX_R {
        LP_AONCLKRST_RST_EN_LP_MAILBOX_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_aonefusereg(&self) -> LP_AONCLKRST_RST_EN_LP_AONEFUSEREG_R {
        LP_AONCLKRST_RST_EN_LP_AONEFUSEREG_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_ram(&self) -> LP_AONCLKRST_RST_EN_LP_RAM_R {
        LP_AONCLKRST_RST_EN_LP_RAM_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_LP_RST_EN")
            .field(
                "lp_aonclkrst_rst_en_lp_huk",
                &self.lp_aonclkrst_rst_en_lp_huk(),
            )
            .field(
                "lp_aonclkrst_rst_en_lp_anaperi",
                &self.lp_aonclkrst_rst_en_lp_anaperi(),
            )
            .field(
                "lp_aonclkrst_rst_en_lp_wdt",
                &self.lp_aonclkrst_rst_en_lp_wdt(),
            )
            .field(
                "lp_aonclkrst_rst_en_lp_timer",
                &self.lp_aonclkrst_rst_en_lp_timer(),
            )
            .field(
                "lp_aonclkrst_rst_en_lp_rtc",
                &self.lp_aonclkrst_rst_en_lp_rtc(),
            )
            .field(
                "lp_aonclkrst_rst_en_lp_mailbox",
                &self.lp_aonclkrst_rst_en_lp_mailbox(),
            )
            .field(
                "lp_aonclkrst_rst_en_lp_aonefusereg",
                &self.lp_aonclkrst_rst_en_lp_aonefusereg(),
            )
            .field(
                "lp_aonclkrst_rst_en_lp_ram",
                &self.lp_aonclkrst_rst_en_lp_ram(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_huk(
        &mut self,
    ) -> LP_AONCLKRST_RST_EN_LP_HUK_W<LP_AONCLKRST_LP_RST_EN_SPEC> {
        LP_AONCLKRST_RST_EN_LP_HUK_W::new(self, 24)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_anaperi(
        &mut self,
    ) -> LP_AONCLKRST_RST_EN_LP_ANAPERI_W<LP_AONCLKRST_LP_RST_EN_SPEC> {
        LP_AONCLKRST_RST_EN_LP_ANAPERI_W::new(self, 25)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_wdt(
        &mut self,
    ) -> LP_AONCLKRST_RST_EN_LP_WDT_W<LP_AONCLKRST_LP_RST_EN_SPEC> {
        LP_AONCLKRST_RST_EN_LP_WDT_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_timer(
        &mut self,
    ) -> LP_AONCLKRST_RST_EN_LP_TIMER_W<LP_AONCLKRST_LP_RST_EN_SPEC> {
        LP_AONCLKRST_RST_EN_LP_TIMER_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_rtc(
        &mut self,
    ) -> LP_AONCLKRST_RST_EN_LP_RTC_W<LP_AONCLKRST_LP_RST_EN_SPEC> {
        LP_AONCLKRST_RST_EN_LP_RTC_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_mailbox(
        &mut self,
    ) -> LP_AONCLKRST_RST_EN_LP_MAILBOX_W<LP_AONCLKRST_LP_RST_EN_SPEC> {
        LP_AONCLKRST_RST_EN_LP_MAILBOX_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_aonefusereg(
        &mut self,
    ) -> LP_AONCLKRST_RST_EN_LP_AONEFUSEREG_W<LP_AONCLKRST_LP_RST_EN_SPEC> {
        LP_AONCLKRST_RST_EN_LP_AONEFUSEREG_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_rst_en_lp_ram(
        &mut self,
    ) -> LP_AONCLKRST_RST_EN_LP_RAM_W<LP_AONCLKRST_LP_RST_EN_SPEC> {
        LP_AONCLKRST_RST_EN_LP_RAM_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_lp_rst_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_lp_rst_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_LP_RST_EN_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_LP_RST_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_lp_rst_en::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_LP_RST_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_lp_rst_en::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_LP_RST_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_AONCLKRST_LP_RST_EN to value 0"]
impl crate::Resettable for LP_AONCLKRST_LP_RST_EN_SPEC {}
