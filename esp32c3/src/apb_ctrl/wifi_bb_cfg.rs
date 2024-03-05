#[doc = "Register `WIFI_BB_CFG` reader"]
pub type R = crate::R<WIFI_BB_CFG_SPEC>;
#[doc = "Register `WIFI_BB_CFG` writer"]
pub type W = crate::W<WIFI_BB_CFG_SPEC>;
#[doc = "Field `WIFI_BB_CFG` reader - reg_wifi_bb_cfg"]
pub type WIFI_BB_CFG_R = crate::FieldReader<u32>;
#[doc = "Field `WIFI_BB_CFG` writer - reg_wifi_bb_cfg"]
pub type WIFI_BB_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reg_wifi_bb_cfg"]
    #[inline(always)]
    pub fn wifi_bb_cfg(&self) -> WIFI_BB_CFG_R {
        WIFI_BB_CFG_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIFI_BB_CFG")
            .field(
                "wifi_bb_cfg",
                &format_args!("{}", self.wifi_bb_cfg().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WIFI_BB_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - reg_wifi_bb_cfg"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_bb_cfg(&mut self) -> WIFI_BB_CFG_W<WIFI_BB_CFG_SPEC> {
        WIFI_BB_CFG_W::new(self, 0)
    }
}
#[doc = "APB_CTRL_WIFI_BB_CFG_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_bb_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_bb_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIFI_BB_CFG_SPEC;
impl crate::RegisterSpec for WIFI_BB_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_bb_cfg::R`](R) reader structure"]
impl crate::Readable for WIFI_BB_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wifi_bb_cfg::W`](W) writer structure"]
impl crate::Writable for WIFI_BB_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIFI_BB_CFG to value 0"]
impl crate::Resettable for WIFI_BB_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
