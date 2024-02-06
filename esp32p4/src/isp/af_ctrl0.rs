#[doc = "Register `AF_CTRL0` reader"]
pub type R = crate::R<AF_CTRL0_SPEC>;
#[doc = "Register `AF_CTRL0` writer"]
pub type W = crate::W<AF_CTRL0_SPEC>;
#[doc = "Field `AF_AUTO_UPDATE` reader - this bit configures auto_update enable. when set to 1, will update sum and lum each frame"]
pub type AF_AUTO_UPDATE_R = crate::BitReader;
#[doc = "Field `AF_AUTO_UPDATE` writer - this bit configures auto_update enable. when set to 1, will update sum and lum each frame"]
pub type AF_AUTO_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AF_MANUAL_UPDATE` writer - write 1 to this bit will update the sum and lum once"]
pub type AF_MANUAL_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AF_ENV_THRESHOLD` reader - this field configures env threshold. when both sum and lum changes larger than this value, consider environment changes and need to trigger a new autofocus. 4Bit fractional"]
pub type AF_ENV_THRESHOLD_R = crate::FieldReader;
#[doc = "Field `AF_ENV_THRESHOLD` writer - this field configures env threshold. when both sum and lum changes larger than this value, consider environment changes and need to trigger a new autofocus. 4Bit fractional"]
pub type AF_ENV_THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AF_ENV_PERIOD` reader - this field configures environment changes detection period (frame). When set to 0, disable this function"]
pub type AF_ENV_PERIOD_R = crate::FieldReader;
#[doc = "Field `AF_ENV_PERIOD` writer - this field configures environment changes detection period (frame). When set to 0, disable this function"]
pub type AF_ENV_PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - this bit configures auto_update enable. when set to 1, will update sum and lum each frame"]
    #[inline(always)]
    pub fn af_auto_update(&self) -> AF_AUTO_UPDATE_R {
        AF_AUTO_UPDATE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - this field configures env threshold. when both sum and lum changes larger than this value, consider environment changes and need to trigger a new autofocus. 4Bit fractional"]
    #[inline(always)]
    pub fn af_env_threshold(&self) -> AF_ENV_THRESHOLD_R {
        AF_ENV_THRESHOLD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - this field configures environment changes detection period (frame). When set to 0, disable this function"]
    #[inline(always)]
    pub fn af_env_period(&self) -> AF_ENV_PERIOD_R {
        AF_ENV_PERIOD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF_CTRL0")
            .field(
                "af_auto_update",
                &format_args!("{}", self.af_auto_update().bit()),
            )
            .field(
                "af_env_threshold",
                &format_args!("{}", self.af_env_threshold().bits()),
            )
            .field(
                "af_env_period",
                &format_args!("{}", self.af_env_period().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AF_CTRL0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - this bit configures auto_update enable. when set to 1, will update sum and lum each frame"]
    #[inline(always)]
    #[must_use]
    pub fn af_auto_update(&mut self) -> AF_AUTO_UPDATE_W<AF_CTRL0_SPEC> {
        AF_AUTO_UPDATE_W::new(self, 0)
    }
    #[doc = "Bit 4 - write 1 to this bit will update the sum and lum once"]
    #[inline(always)]
    #[must_use]
    pub fn af_manual_update(&mut self) -> AF_MANUAL_UPDATE_W<AF_CTRL0_SPEC> {
        AF_MANUAL_UPDATE_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - this field configures env threshold. when both sum and lum changes larger than this value, consider environment changes and need to trigger a new autofocus. 4Bit fractional"]
    #[inline(always)]
    #[must_use]
    pub fn af_env_threshold(&mut self) -> AF_ENV_THRESHOLD_W<AF_CTRL0_SPEC> {
        AF_ENV_THRESHOLD_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures environment changes detection period (frame). When set to 0, disable this function"]
    #[inline(always)]
    #[must_use]
    pub fn af_env_period(&mut self) -> AF_ENV_PERIOD_W<AF_CTRL0_SPEC> {
        AF_ENV_PERIOD_W::new(self, 16)
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
#[doc = "af control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`af_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AF_CTRL0_SPEC;
impl crate::RegisterSpec for AF_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af_ctrl0::R`](R) reader structure"]
impl crate::Readable for AF_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`af_ctrl0::W`](W) writer structure"]
impl crate::Writable for AF_CTRL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AF_CTRL0 to value 0"]
impl crate::Resettable for AF_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0;
}
