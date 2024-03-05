#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `APB_SARADC_THRES1_LOW_INT_ENA` reader - saradc thres1 low interrupt enable"]
pub type APB_SARADC_THRES1_LOW_INT_ENA_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES1_LOW_INT_ENA` writer - saradc thres1 low interrupt enable"]
pub type APB_SARADC_THRES1_LOW_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_SARADC_THRES0_LOW_INT_ENA` reader - saradc thres0 low interrupt enable"]
pub type APB_SARADC_THRES0_LOW_INT_ENA_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES0_LOW_INT_ENA` writer - saradc thres0 low interrupt enable"]
pub type APB_SARADC_THRES0_LOW_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_SARADC_THRES1_HIGH_INT_ENA` reader - saradc thres1 high interrupt enable"]
pub type APB_SARADC_THRES1_HIGH_INT_ENA_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES1_HIGH_INT_ENA` writer - saradc thres1 high interrupt enable"]
pub type APB_SARADC_THRES1_HIGH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_SARADC_THRES0_HIGH_INT_ENA` reader - saradc thres0 high interrupt enable"]
pub type APB_SARADC_THRES0_HIGH_INT_ENA_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES0_HIGH_INT_ENA` writer - saradc thres0 high interrupt enable"]
pub type APB_SARADC_THRES0_HIGH_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_SARADC2_DONE_INT_ENA` reader - saradc2 done interrupt enable"]
pub type APB_SARADC2_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `APB_SARADC2_DONE_INT_ENA` writer - saradc2 done interrupt enable"]
pub type APB_SARADC2_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APB_SARADC1_DONE_INT_ENA` reader - saradc1 done interrupt enable"]
pub type APB_SARADC1_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `APB_SARADC1_DONE_INT_ENA` writer - saradc1 done interrupt enable"]
pub type APB_SARADC1_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 26 - saradc thres1 low interrupt enable"]
    #[inline(always)]
    pub fn apb_saradc_thres1_low_int_ena(&self) -> APB_SARADC_THRES1_LOW_INT_ENA_R {
        APB_SARADC_THRES1_LOW_INT_ENA_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - saradc thres0 low interrupt enable"]
    #[inline(always)]
    pub fn apb_saradc_thres0_low_int_ena(&self) -> APB_SARADC_THRES0_LOW_INT_ENA_R {
        APB_SARADC_THRES0_LOW_INT_ENA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - saradc thres1 high interrupt enable"]
    #[inline(always)]
    pub fn apb_saradc_thres1_high_int_ena(&self) -> APB_SARADC_THRES1_HIGH_INT_ENA_R {
        APB_SARADC_THRES1_HIGH_INT_ENA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - saradc thres0 high interrupt enable"]
    #[inline(always)]
    pub fn apb_saradc_thres0_high_int_ena(&self) -> APB_SARADC_THRES0_HIGH_INT_ENA_R {
        APB_SARADC_THRES0_HIGH_INT_ENA_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - saradc2 done interrupt enable"]
    #[inline(always)]
    pub fn apb_saradc2_done_int_ena(&self) -> APB_SARADC2_DONE_INT_ENA_R {
        APB_SARADC2_DONE_INT_ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - saradc1 done interrupt enable"]
    #[inline(always)]
    pub fn apb_saradc1_done_int_ena(&self) -> APB_SARADC1_DONE_INT_ENA_R {
        APB_SARADC1_DONE_INT_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "apb_saradc_thres1_low_int_ena",
                &format_args!("{}", self.apb_saradc_thres1_low_int_ena().bit()),
            )
            .field(
                "apb_saradc_thres0_low_int_ena",
                &format_args!("{}", self.apb_saradc_thres0_low_int_ena().bit()),
            )
            .field(
                "apb_saradc_thres1_high_int_ena",
                &format_args!("{}", self.apb_saradc_thres1_high_int_ena().bit()),
            )
            .field(
                "apb_saradc_thres0_high_int_ena",
                &format_args!("{}", self.apb_saradc_thres0_high_int_ena().bit()),
            )
            .field(
                "apb_saradc2_done_int_ena",
                &format_args!("{}", self.apb_saradc2_done_int_ena().bit()),
            )
            .field(
                "apb_saradc1_done_int_ena",
                &format_args!("{}", self.apb_saradc1_done_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 26 - saradc thres1 low interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_thres1_low_int_ena(
        &mut self,
    ) -> APB_SARADC_THRES1_LOW_INT_ENA_W<INT_ENA_SPEC> {
        APB_SARADC_THRES1_LOW_INT_ENA_W::new(self, 26)
    }
    #[doc = "Bit 27 - saradc thres0 low interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_thres0_low_int_ena(
        &mut self,
    ) -> APB_SARADC_THRES0_LOW_INT_ENA_W<INT_ENA_SPEC> {
        APB_SARADC_THRES0_LOW_INT_ENA_W::new(self, 27)
    }
    #[doc = "Bit 28 - saradc thres1 high interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_thres1_high_int_ena(
        &mut self,
    ) -> APB_SARADC_THRES1_HIGH_INT_ENA_W<INT_ENA_SPEC> {
        APB_SARADC_THRES1_HIGH_INT_ENA_W::new(self, 28)
    }
    #[doc = "Bit 29 - saradc thres0 high interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_thres0_high_int_ena(
        &mut self,
    ) -> APB_SARADC_THRES0_HIGH_INT_ENA_W<INT_ENA_SPEC> {
        APB_SARADC_THRES0_HIGH_INT_ENA_W::new(self, 29)
    }
    #[doc = "Bit 30 - saradc2 done interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc2_done_int_ena(&mut self) -> APB_SARADC2_DONE_INT_ENA_W<INT_ENA_SPEC> {
        APB_SARADC2_DONE_INT_ENA_W::new(self, 30)
    }
    #[doc = "Bit 31 - saradc1 done interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc1_done_int_ena(&mut self) -> APB_SARADC1_DONE_INT_ENA_W<INT_ENA_SPEC> {
        APB_SARADC1_DONE_INT_ENA_W::new(self, 31)
    }
}
#[doc = "digital saradc int register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
