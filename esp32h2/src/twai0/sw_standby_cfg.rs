#[doc = "Register `SW_STANDBY_CFG` reader"]
pub type R = crate::R<SW_STANDBY_CFG_SPEC>;
#[doc = "Register `SW_STANDBY_CFG` writer"]
pub type W = crate::W<SW_STANDBY_CFG_SPEC>;
#[doc = "Field `SW_STANDBY_EN` reader - Enable standby pin."]
pub type SW_STANDBY_EN_R = crate::BitReader;
#[doc = "Field `SW_STANDBY_EN` writer - Enable standby pin."]
pub type SW_STANDBY_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SW_STANDBY_CLR` reader - Clear standby pin."]
pub type SW_STANDBY_CLR_R = crate::BitReader;
#[doc = "Field `SW_STANDBY_CLR` writer - Clear standby pin."]
pub type SW_STANDBY_CLR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Enable standby pin."]
    #[inline(always)]
    pub fn sw_standby_en(&self) -> SW_STANDBY_EN_R {
        SW_STANDBY_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear standby pin."]
    #[inline(always)]
    pub fn sw_standby_clr(&self) -> SW_STANDBY_CLR_R {
        SW_STANDBY_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SW_STANDBY_CFG")
            .field(
                "sw_standby_en",
                &format_args!("{}", self.sw_standby_en().bit()),
            )
            .field(
                "sw_standby_clr",
                &format_args!("{}", self.sw_standby_clr().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SW_STANDBY_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Enable standby pin."]
    #[inline(always)]
    #[must_use]
    pub fn sw_standby_en(&mut self) -> SW_STANDBY_EN_W<SW_STANDBY_CFG_SPEC, 0> {
        SW_STANDBY_EN_W::new(self)
    }
    #[doc = "Bit 1 - Clear standby pin."]
    #[inline(always)]
    #[must_use]
    pub fn sw_standby_clr(&mut self) -> SW_STANDBY_CLR_W<SW_STANDBY_CFG_SPEC, 1> {
        SW_STANDBY_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Software configure standby pin directly.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sw_standby_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sw_standby_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SW_STANDBY_CFG_SPEC;
impl crate::RegisterSpec for SW_STANDBY_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_standby_cfg::R`](R) reader structure"]
impl crate::Readable for SW_STANDBY_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sw_standby_cfg::W`](W) writer structure"]
impl crate::Writable for SW_STANDBY_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SW_STANDBY_CFG to value 0x02"]
impl crate::Resettable for SW_STANDBY_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
