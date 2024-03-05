#[doc = "Register `WIFI_RST_EN` reader"]
pub type R = crate::R<WIFI_RST_EN_SPEC>;
#[doc = "Register `WIFI_RST_EN` writer"]
pub type W = crate::W<WIFI_RST_EN_SPEC>;
#[doc = "Field `WIFI_RST` reader - reg_wifi_rst"]
pub type WIFI_RST_R = crate::FieldReader<u32>;
#[doc = "Field `WIFI_RST` writer - reg_wifi_rst"]
pub type WIFI_RST_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reg_wifi_rst"]
    #[inline(always)]
    pub fn wifi_rst(&self) -> WIFI_RST_R {
        WIFI_RST_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIFI_RST_EN")
            .field("wifi_rst", &format_args!("{}", self.wifi_rst().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WIFI_RST_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - reg_wifi_rst"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_rst(&mut self) -> WIFI_RST_W<WIFI_RST_EN_SPEC> {
        WIFI_RST_W::new(self, 0)
    }
}
#[doc = "APB_CTRL_WIFI_RST_EN_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wifi_rst_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wifi_rst_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WIFI_RST_EN_SPEC;
impl crate::RegisterSpec for WIFI_RST_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wifi_rst_en::R`](R) reader structure"]
impl crate::Readable for WIFI_RST_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wifi_rst_en::W`](W) writer structure"]
impl crate::Writable for WIFI_RST_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIFI_RST_EN to value 0"]
impl crate::Resettable for WIFI_RST_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
