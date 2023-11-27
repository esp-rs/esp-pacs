#[doc = "Register `TIMESTAMP_CFG` reader"]
pub type R = crate::R<TIMESTAMP_CFG_SPEC>;
#[doc = "Register `TIMESTAMP_CFG` writer"]
pub type W = crate::W<TIMESTAMP_CFG_SPEC>;
#[doc = "Field `TS_ENABLE` reader - enable the timestamp collection function."]
pub type TS_ENABLE_R = crate::BitReader;
#[doc = "Field `TS_ENABLE` writer - enable the timestamp collection function."]
pub type TS_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable the timestamp collection function."]
    #[inline(always)]
    pub fn ts_enable(&self) -> TS_ENABLE_R {
        TS_ENABLE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMESTAMP_CFG")
            .field("ts_enable", &format_args!("{}", self.ts_enable().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMESTAMP_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - enable the timestamp collection function."]
    #[inline(always)]
    #[must_use]
    pub fn ts_enable(&mut self) -> TS_ENABLE_W<TIMESTAMP_CFG_SPEC> {
        TS_ENABLE_W::new(self, 0)
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
#[doc = "Timestamp configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timestamp_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timestamp_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMESTAMP_CFG_SPEC;
impl crate::RegisterSpec for TIMESTAMP_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timestamp_cfg::R`](R) reader structure"]
impl crate::Readable for TIMESTAMP_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timestamp_cfg::W`](W) writer structure"]
impl crate::Writable for TIMESTAMP_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMESTAMP_CFG to value 0"]
impl crate::Resettable for TIMESTAMP_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
