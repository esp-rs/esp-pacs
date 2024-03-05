#[doc = "Register `WIFI_CLK_EN` reader"]
pub type R = crate::R<WIFI_CLK_EN_SPEC>;
#[doc = "Register `WIFI_CLK_EN` writer"]
pub type W = crate::W<WIFI_CLK_EN_SPEC>;
#[doc = "Field `WIFI_CLK_EN` reader - reg_wifi_clk_en"]
pub type WIFI_CLK_EN_R = crate::FieldReader<u32>;
#[doc = "Field `WIFI_CLK_EN` writer - reg_wifi_clk_en"]
pub type WIFI_CLK_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reg_wifi_clk_en"]
    #[inline(always)]
    pub fn wifi_clk_en(&self) -> WIFI_CLK_EN_R {
        WIFI_CLK_EN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIFI_CLK_EN")
            .field(
                "wifi_clk_en",
                &format_args!("{}", self.wifi_clk_en().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WIFI_CLK_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - reg_wifi_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_clk_en(&mut self) -> WIFI_CLK_EN_W<WIFI_CLK_EN_SPEC> {
        WIFI_CLK_EN_W::new(self, 0)
    }
}
#[doc = "APB_CTRL_WIFI_CLK_EN_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_clk_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_clk_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIFI_CLK_EN_SPEC;
impl crate::RegisterSpec for WIFI_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_clk_en::R`](R) reader structure"]
impl crate::Readable for WIFI_CLK_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wifi_clk_en::W`](W) writer structure"]
impl crate::Writable for WIFI_CLK_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIFI_CLK_EN to value 0xfffc_e030"]
impl crate::Resettable for WIFI_CLK_EN_SPEC {
    const RESET_VALUE: u32 = 0xfffc_e030;
}
