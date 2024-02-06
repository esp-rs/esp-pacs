#[doc = "Register `PIN1` reader"]
pub type R = crate::R<PIN1_SPEC>;
#[doc = "Register `PIN1` writer"]
pub type W = crate::W<PIN1_SPEC>;
#[doc = "Field `LP_GPIO1_SYNC_BYPASS` reader - need des"]
pub type LP_GPIO1_SYNC_BYPASS_R = crate::FieldReader;
#[doc = "Field `LP_GPIO1_SYNC_BYPASS` writer - need des"]
pub type LP_GPIO1_SYNC_BYPASS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_GPIO1_PAD_DRIVER` reader - need des"]
pub type LP_GPIO1_PAD_DRIVER_R = crate::BitReader;
#[doc = "Field `LP_GPIO1_PAD_DRIVER` writer - need des"]
pub type LP_GPIO1_PAD_DRIVER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_GPIO1_EDGE_WAKEUP_CLR` writer - need des"]
pub type LP_GPIO1_EDGE_WAKEUP_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_GPIO1_INT_TYPE` reader - need des"]
pub type LP_GPIO1_INT_TYPE_R = crate::FieldReader;
#[doc = "Field `LP_GPIO1_INT_TYPE` writer - need des"]
pub type LP_GPIO1_INT_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LP_GPIO1_WAKEUP_ENABLE` reader - need des"]
pub type LP_GPIO1_WAKEUP_ENABLE_R = crate::BitReader;
#[doc = "Field `LP_GPIO1_WAKEUP_ENABLE` writer - need des"]
pub type LP_GPIO1_WAKEUP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_GPIO1_FILTER_EN` reader - need des"]
pub type LP_GPIO1_FILTER_EN_R = crate::BitReader;
#[doc = "Field `LP_GPIO1_FILTER_EN` writer - need des"]
pub type LP_GPIO1_FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - need des"]
    #[inline(always)]
    pub fn lp_gpio1_sync_bypass(&self) -> LP_GPIO1_SYNC_BYPASS_R {
        LP_GPIO1_SYNC_BYPASS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - need des"]
    #[inline(always)]
    pub fn lp_gpio1_pad_driver(&self) -> LP_GPIO1_PAD_DRIVER_R {
        LP_GPIO1_PAD_DRIVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 7:9 - need des"]
    #[inline(always)]
    pub fn lp_gpio1_int_type(&self) -> LP_GPIO1_INT_TYPE_R {
        LP_GPIO1_INT_TYPE_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - need des"]
    #[inline(always)]
    pub fn lp_gpio1_wakeup_enable(&self) -> LP_GPIO1_WAKEUP_ENABLE_R {
        LP_GPIO1_WAKEUP_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn lp_gpio1_filter_en(&self) -> LP_GPIO1_FILTER_EN_R {
        LP_GPIO1_FILTER_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIN1")
            .field(
                "lp_gpio1_sync_bypass",
                &format_args!("{}", self.lp_gpio1_sync_bypass().bits()),
            )
            .field(
                "lp_gpio1_pad_driver",
                &format_args!("{}", self.lp_gpio1_pad_driver().bit()),
            )
            .field(
                "lp_gpio1_int_type",
                &format_args!("{}", self.lp_gpio1_int_type().bits()),
            )
            .field(
                "lp_gpio1_wakeup_enable",
                &format_args!("{}", self.lp_gpio1_wakeup_enable().bit()),
            )
            .field(
                "lp_gpio1_filter_en",
                &format_args!("{}", self.lp_gpio1_filter_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PIN1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio1_sync_bypass(&mut self) -> LP_GPIO1_SYNC_BYPASS_W<PIN1_SPEC> {
        LP_GPIO1_SYNC_BYPASS_W::new(self, 0)
    }
    #[doc = "Bit 2 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio1_pad_driver(&mut self) -> LP_GPIO1_PAD_DRIVER_W<PIN1_SPEC> {
        LP_GPIO1_PAD_DRIVER_W::new(self, 2)
    }
    #[doc = "Bit 3 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio1_edge_wakeup_clr(&mut self) -> LP_GPIO1_EDGE_WAKEUP_CLR_W<PIN1_SPEC> {
        LP_GPIO1_EDGE_WAKEUP_CLR_W::new(self, 3)
    }
    #[doc = "Bits 7:9 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio1_int_type(&mut self) -> LP_GPIO1_INT_TYPE_W<PIN1_SPEC> {
        LP_GPIO1_INT_TYPE_W::new(self, 7)
    }
    #[doc = "Bit 10 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio1_wakeup_enable(&mut self) -> LP_GPIO1_WAKEUP_ENABLE_W<PIN1_SPEC> {
        LP_GPIO1_WAKEUP_ENABLE_W::new(self, 10)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio1_filter_en(&mut self) -> LP_GPIO1_FILTER_EN_W<PIN1_SPEC> {
        LP_GPIO1_FILTER_EN_W::new(self, 11)
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
#[doc = "need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIN1_SPEC;
impl crate::RegisterSpec for PIN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pin1::R`](R) reader structure"]
impl crate::Readable for PIN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pin1::W`](W) writer structure"]
impl crate::Writable for PIN1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PIN1 to value 0"]
impl crate::Resettable for PIN1_SPEC {
    const RESET_VALUE: u32 = 0;
}
