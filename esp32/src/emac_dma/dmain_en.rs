#[doc = "Register `DMAIN_EN` reader"]
pub type R = crate::R<DMAIN_EN_SPEC>;
#[doc = "Register `DMAIN_EN` writer"]
pub type W = crate::W<DMAIN_EN_SPEC>;
#[doc = "Field `DMAIN_TIE` reader - When this bit is set with Normal Interrupt Summary Enable (Bit\\[16\\]) the Transmit Interrupt is enabled. When this bit is reset the Transmit Interrupt is disabled."]
pub type DMAIN_TIE_R = crate::BitReader;
#[doc = "Field `DMAIN_TIE` writer - When this bit is set with Normal Interrupt Summary Enable (Bit\\[16\\]) the Transmit Interrupt is enabled. When this bit is reset the Transmit Interrupt is disabled."]
pub type DMAIN_TIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_TSE` reader - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Transmission Stopped Interrupt is enabled. When this bit is reset the Transmission Stopped Interrupt is disabled."]
pub type DMAIN_TSE_R = crate::BitReader;
#[doc = "Field `DMAIN_TSE` writer - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Transmission Stopped Interrupt is enabled. When this bit is reset the Transmission Stopped Interrupt is disabled."]
pub type DMAIN_TSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_TBUE` reader - When this bit is set with Normal Interrupt Summary Enable (Bit 16) the Transmit Buffer Unavailable Interrupt is enabled. When this bit is reset the Transmit Buffer Unavailable Interrupt is Disabled."]
pub type DMAIN_TBUE_R = crate::BitReader;
#[doc = "Field `DMAIN_TBUE` writer - When this bit is set with Normal Interrupt Summary Enable (Bit 16) the Transmit Buffer Unavailable Interrupt is enabled. When this bit is reset the Transmit Buffer Unavailable Interrupt is Disabled."]
pub type DMAIN_TBUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_TJTE` reader - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Transmit Jabber Timeout Interrupt is enabled. When this bit is reset the Transmit Jabber Timeout Interrupt is disabled."]
pub type DMAIN_TJTE_R = crate::BitReader;
#[doc = "Field `DMAIN_TJTE` writer - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Transmit Jabber Timeout Interrupt is enabled. When this bit is reset the Transmit Jabber Timeout Interrupt is disabled."]
pub type DMAIN_TJTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_OIE` reader - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Overflow Interrupt is enabled. When this bit is reset the Overflow Interrupt is disabled."]
pub type DMAIN_OIE_R = crate::BitReader;
#[doc = "Field `DMAIN_OIE` writer - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Overflow Interrupt is enabled. When this bit is reset the Overflow Interrupt is disabled."]
pub type DMAIN_OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_UIE` reader - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Transmit Underflow Interrupt is enabled. When this bit is reset the Underflow Interrupt is disabled."]
pub type DMAIN_UIE_R = crate::BitReader;
#[doc = "Field `DMAIN_UIE` writer - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Transmit Underflow Interrupt is enabled. When this bit is reset the Underflow Interrupt is disabled."]
pub type DMAIN_UIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_RIE` reader - When this bit is set with Normal Interrupt Summary Enable (Bit\\[16\\]) the Receive Interrupt is enabled. When this bit is reset the Receive Interrupt is disabled."]
pub type DMAIN_RIE_R = crate::BitReader;
#[doc = "Field `DMAIN_RIE` writer - When this bit is set with Normal Interrupt Summary Enable (Bit\\[16\\]) the Receive Interrupt is enabled. When this bit is reset the Receive Interrupt is disabled."]
pub type DMAIN_RIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_RBUE` reader - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Buffer Unavailable Interrupt is enabled. When this bit is reset the Receive Buffer Unavailable Interrupt is disabled."]
pub type DMAIN_RBUE_R = crate::BitReader;
#[doc = "Field `DMAIN_RBUE` writer - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Buffer Unavailable Interrupt is enabled. When this bit is reset the Receive Buffer Unavailable Interrupt is disabled."]
pub type DMAIN_RBUE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_RSE` reader - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Stopped Interrupt is enabled. When this bit is reset the Receive Stopped Interrupt is disabled."]
pub type DMAIN_RSE_R = crate::BitReader;
#[doc = "Field `DMAIN_RSE` writer - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Stopped Interrupt is enabled. When this bit is reset the Receive Stopped Interrupt is disabled."]
pub type DMAIN_RSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_RWTE` reader - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Watchdog Timeout Interrupt is enabled. When this bit is reset the Receive Watchdog Timeout Interrupt is disabled."]
pub type DMAIN_RWTE_R = crate::BitReader;
#[doc = "Field `DMAIN_RWTE` writer - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Watchdog Timeout Interrupt is enabled. When this bit is reset the Receive Watchdog Timeout Interrupt is disabled."]
pub type DMAIN_RWTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_ETIE` reader - When this bit is set with an Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Early Transmit Interrupt is enabled. When this bit is reset the Early Transmit Interrupt is disabled."]
pub type DMAIN_ETIE_R = crate::BitReader;
#[doc = "Field `DMAIN_ETIE` writer - When this bit is set with an Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Early Transmit Interrupt is enabled. When this bit is reset the Early Transmit Interrupt is disabled."]
pub type DMAIN_ETIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_FBEE` reader - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Fatal Bus Error Interrupt is enabled. When this bit is reset the Fatal Bus Error Enable Interrupt is disabled."]
pub type DMAIN_FBEE_R = crate::BitReader;
#[doc = "Field `DMAIN_FBEE` writer - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Fatal Bus Error Interrupt is enabled. When this bit is reset the Fatal Bus Error Enable Interrupt is disabled."]
pub type DMAIN_FBEE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_ERIE` reader - When this bit is set with Normal Interrupt Summary Enable (Bit\\[16\\]) the Early Receive Interrupt is enabled. When this bit is reset the Early Receive Interrupt is disabled."]
pub type DMAIN_ERIE_R = crate::BitReader;
#[doc = "Field `DMAIN_ERIE` writer - When this bit is set with Normal Interrupt Summary Enable (Bit\\[16\\]) the Early Receive Interrupt is enabled. When this bit is reset the Early Receive Interrupt is disabled."]
pub type DMAIN_ERIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_AISE` reader - When this bit is set abnormal interrupt summary is enabled. When this bit is reset the abnormal interrupt summary is disabled. This bit enables the following interrupts in Status Register: Bit\\[1\\]: Transmit Process Stopped. Bit\\[3\\]: Transmit Jabber Timeout. Bit\\[4\\]: Receive Overflow. Bit\\[5\\]: Transmit Underflow. Bit\\[7\\]: Receive Buffer Unavailable. Bit\\[8\\]: Receive Process Stopped. Bit\\[9\\]: Receive Watchdog Timeout. Bit\\[10\\]: Early Transmit Interrupt. Bit\\[13\\]: Fatal Bus Error."]
pub type DMAIN_AISE_R = crate::BitReader;
#[doc = "Field `DMAIN_AISE` writer - When this bit is set abnormal interrupt summary is enabled. When this bit is reset the abnormal interrupt summary is disabled. This bit enables the following interrupts in Status Register: Bit\\[1\\]: Transmit Process Stopped. Bit\\[3\\]: Transmit Jabber Timeout. Bit\\[4\\]: Receive Overflow. Bit\\[5\\]: Transmit Underflow. Bit\\[7\\]: Receive Buffer Unavailable. Bit\\[8\\]: Receive Process Stopped. Bit\\[9\\]: Receive Watchdog Timeout. Bit\\[10\\]: Early Transmit Interrupt. Bit\\[13\\]: Fatal Bus Error."]
pub type DMAIN_AISE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAIN_NISE` reader - When this bit is set normal interrupt summary is enabled. When this bit is reset normal interrupt summary is disabled. This bit enables the following interrupts in Status Register: Bit\\[0\\]: Transmit Interrupt. Bit\\[2\\]: Transmit Buffer Unavailable. Bit\\[6\\]: Receive Interrupt. Bit\\[14\\]: Early Receive Interrupt."]
pub type DMAIN_NISE_R = crate::BitReader;
#[doc = "Field `DMAIN_NISE` writer - When this bit is set normal interrupt summary is enabled. When this bit is reset normal interrupt summary is disabled. This bit enables the following interrupts in Status Register: Bit\\[0\\]: Transmit Interrupt. Bit\\[2\\]: Transmit Buffer Unavailable. Bit\\[6\\]: Receive Interrupt. Bit\\[14\\]: Early Receive Interrupt."]
pub type DMAIN_NISE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When this bit is set with Normal Interrupt Summary Enable (Bit\\[16\\]) the Transmit Interrupt is enabled. When this bit is reset the Transmit Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_tie(&self) -> DMAIN_TIE_R {
        DMAIN_TIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Transmission Stopped Interrupt is enabled. When this bit is reset the Transmission Stopped Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_tse(&self) -> DMAIN_TSE_R {
        DMAIN_TSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When this bit is set with Normal Interrupt Summary Enable (Bit 16) the Transmit Buffer Unavailable Interrupt is enabled. When this bit is reset the Transmit Buffer Unavailable Interrupt is Disabled."]
    #[inline(always)]
    pub fn dmain_tbue(&self) -> DMAIN_TBUE_R {
        DMAIN_TBUE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Transmit Jabber Timeout Interrupt is enabled. When this bit is reset the Transmit Jabber Timeout Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_tjte(&self) -> DMAIN_TJTE_R {
        DMAIN_TJTE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Overflow Interrupt is enabled. When this bit is reset the Overflow Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_oie(&self) -> DMAIN_OIE_R {
        DMAIN_OIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Transmit Underflow Interrupt is enabled. When this bit is reset the Underflow Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_uie(&self) -> DMAIN_UIE_R {
        DMAIN_UIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When this bit is set with Normal Interrupt Summary Enable (Bit\\[16\\]) the Receive Interrupt is enabled. When this bit is reset the Receive Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_rie(&self) -> DMAIN_RIE_R {
        DMAIN_RIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Buffer Unavailable Interrupt is enabled. When this bit is reset the Receive Buffer Unavailable Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_rbue(&self) -> DMAIN_RBUE_R {
        DMAIN_RBUE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Stopped Interrupt is enabled. When this bit is reset the Receive Stopped Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_rse(&self) -> DMAIN_RSE_R {
        DMAIN_RSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Watchdog Timeout Interrupt is enabled. When this bit is reset the Receive Watchdog Timeout Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_rwte(&self) -> DMAIN_RWTE_R {
        DMAIN_RWTE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When this bit is set with an Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Early Transmit Interrupt is enabled. When this bit is reset the Early Transmit Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_etie(&self) -> DMAIN_ETIE_R {
        DMAIN_ETIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Fatal Bus Error Interrupt is enabled. When this bit is reset the Fatal Bus Error Enable Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_fbee(&self) -> DMAIN_FBEE_R {
        DMAIN_FBEE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - When this bit is set with Normal Interrupt Summary Enable (Bit\\[16\\]) the Early Receive Interrupt is enabled. When this bit is reset the Early Receive Interrupt is disabled."]
    #[inline(always)]
    pub fn dmain_erie(&self) -> DMAIN_ERIE_R {
        DMAIN_ERIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - When this bit is set abnormal interrupt summary is enabled. When this bit is reset the abnormal interrupt summary is disabled. This bit enables the following interrupts in Status Register: Bit\\[1\\]: Transmit Process Stopped. Bit\\[3\\]: Transmit Jabber Timeout. Bit\\[4\\]: Receive Overflow. Bit\\[5\\]: Transmit Underflow. Bit\\[7\\]: Receive Buffer Unavailable. Bit\\[8\\]: Receive Process Stopped. Bit\\[9\\]: Receive Watchdog Timeout. Bit\\[10\\]: Early Transmit Interrupt. Bit\\[13\\]: Fatal Bus Error."]
    #[inline(always)]
    pub fn dmain_aise(&self) -> DMAIN_AISE_R {
        DMAIN_AISE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - When this bit is set normal interrupt summary is enabled. When this bit is reset normal interrupt summary is disabled. This bit enables the following interrupts in Status Register: Bit\\[0\\]: Transmit Interrupt. Bit\\[2\\]: Transmit Buffer Unavailable. Bit\\[6\\]: Receive Interrupt. Bit\\[14\\]: Early Receive Interrupt."]
    #[inline(always)]
    pub fn dmain_nise(&self) -> DMAIN_NISE_R {
        DMAIN_NISE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAIN_EN")
            .field("dmain_tie", &self.dmain_tie())
            .field("dmain_tse", &self.dmain_tse())
            .field("dmain_tbue", &self.dmain_tbue())
            .field("dmain_tjte", &self.dmain_tjte())
            .field("dmain_oie", &self.dmain_oie())
            .field("dmain_uie", &self.dmain_uie())
            .field("dmain_rie", &self.dmain_rie())
            .field("dmain_rbue", &self.dmain_rbue())
            .field("dmain_rse", &self.dmain_rse())
            .field("dmain_rwte", &self.dmain_rwte())
            .field("dmain_etie", &self.dmain_etie())
            .field("dmain_fbee", &self.dmain_fbee())
            .field("dmain_erie", &self.dmain_erie())
            .field("dmain_aise", &self.dmain_aise())
            .field("dmain_nise", &self.dmain_nise())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - When this bit is set with Normal Interrupt Summary Enable (Bit\\[16\\]) the Transmit Interrupt is enabled. When this bit is reset the Transmit Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn dmain_tie(&mut self) -> DMAIN_TIE_W<DMAIN_EN_SPEC> {
        DMAIN_TIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Transmission Stopped Interrupt is enabled. When this bit is reset the Transmission Stopped Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn dmain_tse(&mut self) -> DMAIN_TSE_W<DMAIN_EN_SPEC> {
        DMAIN_TSE_W::new(self, 1)
    }
    #[doc = "Bit 2 - When this bit is set with Normal Interrupt Summary Enable (Bit 16) the Transmit Buffer Unavailable Interrupt is enabled. When this bit is reset the Transmit Buffer Unavailable Interrupt is Disabled."]
    #[inline(always)]
    #[must_use]
    pub fn dmain_tbue(&mut self) -> DMAIN_TBUE_W<DMAIN_EN_SPEC> {
        DMAIN_TBUE_W::new(self, 2)
    }
    #[doc = "Bit 3 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Transmit Jabber Timeout Interrupt is enabled. When this bit is reset the Transmit Jabber Timeout Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn dmain_tjte(&mut self) -> DMAIN_TJTE_W<DMAIN_EN_SPEC> {
        DMAIN_TJTE_W::new(self, 3)
    }
    #[doc = "Bit 4 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Overflow Interrupt is enabled. When this bit is reset the Overflow Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn dmain_oie(&mut self) -> DMAIN_OIE_W<DMAIN_EN_SPEC> {
        DMAIN_OIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Transmit Underflow Interrupt is enabled. When this bit is reset the Underflow Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn dmain_uie(&mut self) -> DMAIN_UIE_W<DMAIN_EN_SPEC> {
        DMAIN_UIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - When this bit is set with Normal Interrupt Summary Enable (Bit\\[16\\]) the Receive Interrupt is enabled. When this bit is reset the Receive Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn dmain_rie(&mut self) -> DMAIN_RIE_W<DMAIN_EN_SPEC> {
        DMAIN_RIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Buffer Unavailable Interrupt is enabled. When this bit is reset the Receive Buffer Unavailable Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn dmain_rbue(&mut self) -> DMAIN_RBUE_W<DMAIN_EN_SPEC> {
        DMAIN_RBUE_W::new(self, 7)
    }
    #[doc = "Bit 8 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Stopped Interrupt is enabled. When this bit is reset the Receive Stopped Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn dmain_rse(&mut self) -> DMAIN_RSE_W<DMAIN_EN_SPEC> {
        DMAIN_RSE_W::new(self, 8)
    }
    #[doc = "Bit 9 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Receive Watchdog Timeout Interrupt is enabled. When this bit is reset the Receive Watchdog Timeout Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn dmain_rwte(&mut self) -> DMAIN_RWTE_W<DMAIN_EN_SPEC> {
        DMAIN_RWTE_W::new(self, 9)
    }
    #[doc = "Bit 10 - When this bit is set with an Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Early Transmit Interrupt is enabled. When this bit is reset the Early Transmit Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn dmain_etie(&mut self) -> DMAIN_ETIE_W<DMAIN_EN_SPEC> {
        DMAIN_ETIE_W::new(self, 10)
    }
    #[doc = "Bit 13 - When this bit is set with Abnormal Interrupt Summary Enable (Bit\\[15\\]) the Fatal Bus Error Interrupt is enabled. When this bit is reset the Fatal Bus Error Enable Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn dmain_fbee(&mut self) -> DMAIN_FBEE_W<DMAIN_EN_SPEC> {
        DMAIN_FBEE_W::new(self, 13)
    }
    #[doc = "Bit 14 - When this bit is set with Normal Interrupt Summary Enable (Bit\\[16\\]) the Early Receive Interrupt is enabled. When this bit is reset the Early Receive Interrupt is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn dmain_erie(&mut self) -> DMAIN_ERIE_W<DMAIN_EN_SPEC> {
        DMAIN_ERIE_W::new(self, 14)
    }
    #[doc = "Bit 15 - When this bit is set abnormal interrupt summary is enabled. When this bit is reset the abnormal interrupt summary is disabled. This bit enables the following interrupts in Status Register: Bit\\[1\\]: Transmit Process Stopped. Bit\\[3\\]: Transmit Jabber Timeout. Bit\\[4\\]: Receive Overflow. Bit\\[5\\]: Transmit Underflow. Bit\\[7\\]: Receive Buffer Unavailable. Bit\\[8\\]: Receive Process Stopped. Bit\\[9\\]: Receive Watchdog Timeout. Bit\\[10\\]: Early Transmit Interrupt. Bit\\[13\\]: Fatal Bus Error."]
    #[inline(always)]
    #[must_use]
    pub fn dmain_aise(&mut self) -> DMAIN_AISE_W<DMAIN_EN_SPEC> {
        DMAIN_AISE_W::new(self, 15)
    }
    #[doc = "Bit 16 - When this bit is set normal interrupt summary is enabled. When this bit is reset normal interrupt summary is disabled. This bit enables the following interrupts in Status Register: Bit\\[0\\]: Transmit Interrupt. Bit\\[2\\]: Transmit Buffer Unavailable. Bit\\[6\\]: Receive Interrupt. Bit\\[14\\]: Early Receive Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dmain_nise(&mut self) -> DMAIN_NISE_W<DMAIN_EN_SPEC> {
        DMAIN_NISE_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmain_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmain_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAIN_EN_SPEC;
impl crate::RegisterSpec for DMAIN_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmain_en::R`](R) reader structure"]
impl crate::Readable for DMAIN_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmain_en::W`](W) writer structure"]
impl crate::Writable for DMAIN_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAIN_EN to value 0"]
impl crate::Resettable for DMAIN_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
