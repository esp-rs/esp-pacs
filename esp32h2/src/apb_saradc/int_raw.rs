#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `APB_SARADC_TSENS_INT_RAW` reader - saradc tsens interrupt raw"]
pub type APB_SARADC_TSENS_INT_RAW_R = crate::BitReader;
#[doc = "Field `APB_SARADC_TSENS_INT_RAW` writer - saradc tsens interrupt raw"]
pub type APB_SARADC_TSENS_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `APB_SARADC_THRES1_LOW_INT_RAW` reader - saradc thres1 low interrupt raw"]
pub type APB_SARADC_THRES1_LOW_INT_RAW_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES1_LOW_INT_RAW` writer - saradc thres1 low interrupt raw"]
pub type APB_SARADC_THRES1_LOW_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `APB_SARADC_THRES0_LOW_INT_RAW` reader - saradc thres0 low interrupt raw"]
pub type APB_SARADC_THRES0_LOW_INT_RAW_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES0_LOW_INT_RAW` writer - saradc thres0 low interrupt raw"]
pub type APB_SARADC_THRES0_LOW_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `APB_SARADC_THRES1_HIGH_INT_RAW` reader - saradc thres1 high interrupt raw"]
pub type APB_SARADC_THRES1_HIGH_INT_RAW_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES1_HIGH_INT_RAW` writer - saradc thres1 high interrupt raw"]
pub type APB_SARADC_THRES1_HIGH_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `APB_SARADC_THRES0_HIGH_INT_RAW` reader - saradc thres0 high interrupt raw"]
pub type APB_SARADC_THRES0_HIGH_INT_RAW_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES0_HIGH_INT_RAW` writer - saradc thres0 high interrupt raw"]
pub type APB_SARADC_THRES0_HIGH_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `APB_SARADC2_DONE_INT_RAW` reader - saradc2 done interrupt raw"]
pub type APB_SARADC2_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `APB_SARADC2_DONE_INT_RAW` writer - saradc2 done interrupt raw"]
pub type APB_SARADC2_DONE_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `APB_SARADC1_DONE_INT_RAW` reader - saradc1 done interrupt raw"]
pub type APB_SARADC1_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `APB_SARADC1_DONE_INT_RAW` writer - saradc1 done interrupt raw"]
pub type APB_SARADC1_DONE_INT_RAW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 25 - saradc tsens interrupt raw"]
    #[inline(always)]
    pub fn apb_saradc_tsens_int_raw(&self) -> APB_SARADC_TSENS_INT_RAW_R {
        APB_SARADC_TSENS_INT_RAW_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - saradc thres1 low interrupt raw"]
    #[inline(always)]
    pub fn apb_saradc_thres1_low_int_raw(&self) -> APB_SARADC_THRES1_LOW_INT_RAW_R {
        APB_SARADC_THRES1_LOW_INT_RAW_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - saradc thres0 low interrupt raw"]
    #[inline(always)]
    pub fn apb_saradc_thres0_low_int_raw(&self) -> APB_SARADC_THRES0_LOW_INT_RAW_R {
        APB_SARADC_THRES0_LOW_INT_RAW_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - saradc thres1 high interrupt raw"]
    #[inline(always)]
    pub fn apb_saradc_thres1_high_int_raw(&self) -> APB_SARADC_THRES1_HIGH_INT_RAW_R {
        APB_SARADC_THRES1_HIGH_INT_RAW_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - saradc thres0 high interrupt raw"]
    #[inline(always)]
    pub fn apb_saradc_thres0_high_int_raw(&self) -> APB_SARADC_THRES0_HIGH_INT_RAW_R {
        APB_SARADC_THRES0_HIGH_INT_RAW_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - saradc2 done interrupt raw"]
    #[inline(always)]
    pub fn apb_saradc2_done_int_raw(&self) -> APB_SARADC2_DONE_INT_RAW_R {
        APB_SARADC2_DONE_INT_RAW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - saradc1 done interrupt raw"]
    #[inline(always)]
    pub fn apb_saradc1_done_int_raw(&self) -> APB_SARADC1_DONE_INT_RAW_R {
        APB_SARADC1_DONE_INT_RAW_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "apb_saradc_tsens_int_raw",
                &format_args!("{}", self.apb_saradc_tsens_int_raw().bit()),
            )
            .field(
                "apb_saradc_thres1_low_int_raw",
                &format_args!("{}", self.apb_saradc_thres1_low_int_raw().bit()),
            )
            .field(
                "apb_saradc_thres0_low_int_raw",
                &format_args!("{}", self.apb_saradc_thres0_low_int_raw().bit()),
            )
            .field(
                "apb_saradc_thres1_high_int_raw",
                &format_args!("{}", self.apb_saradc_thres1_high_int_raw().bit()),
            )
            .field(
                "apb_saradc_thres0_high_int_raw",
                &format_args!("{}", self.apb_saradc_thres0_high_int_raw().bit()),
            )
            .field(
                "apb_saradc2_done_int_raw",
                &format_args!("{}", self.apb_saradc2_done_int_raw().bit()),
            )
            .field(
                "apb_saradc1_done_int_raw",
                &format_args!("{}", self.apb_saradc1_done_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 25 - saradc tsens interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_tsens_int_raw(&mut self) -> APB_SARADC_TSENS_INT_RAW_W<INT_RAW_SPEC, 25> {
        APB_SARADC_TSENS_INT_RAW_W::new(self)
    }
    #[doc = "Bit 26 - saradc thres1 low interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_thres1_low_int_raw(
        &mut self,
    ) -> APB_SARADC_THRES1_LOW_INT_RAW_W<INT_RAW_SPEC, 26> {
        APB_SARADC_THRES1_LOW_INT_RAW_W::new(self)
    }
    #[doc = "Bit 27 - saradc thres0 low interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_thres0_low_int_raw(
        &mut self,
    ) -> APB_SARADC_THRES0_LOW_INT_RAW_W<INT_RAW_SPEC, 27> {
        APB_SARADC_THRES0_LOW_INT_RAW_W::new(self)
    }
    #[doc = "Bit 28 - saradc thres1 high interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_thres1_high_int_raw(
        &mut self,
    ) -> APB_SARADC_THRES1_HIGH_INT_RAW_W<INT_RAW_SPEC, 28> {
        APB_SARADC_THRES1_HIGH_INT_RAW_W::new(self)
    }
    #[doc = "Bit 29 - saradc thres0 high interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_thres0_high_int_raw(
        &mut self,
    ) -> APB_SARADC_THRES0_HIGH_INT_RAW_W<INT_RAW_SPEC, 29> {
        APB_SARADC_THRES0_HIGH_INT_RAW_W::new(self)
    }
    #[doc = "Bit 30 - saradc2 done interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc2_done_int_raw(&mut self) -> APB_SARADC2_DONE_INT_RAW_W<INT_RAW_SPEC, 30> {
        APB_SARADC2_DONE_INT_RAW_W::new(self)
    }
    #[doc = "Bit 31 - saradc1 done interrupt raw"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc1_done_int_raw(&mut self) -> APB_SARADC1_DONE_INT_RAW_W<INT_RAW_SPEC, 31> {
        APB_SARADC1_DONE_INT_RAW_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "digital saradc int register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
