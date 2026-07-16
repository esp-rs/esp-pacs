#[doc = "Register `HUK` reader"]
pub type R = crate::R<HUK_SPEC>;
#[doc = "Register `HUK` writer"]
pub type W = crate::W<HUK_SPEC>;
#[doc = "Field `LP_HUK_RST_EN` reader - need_des"]
pub type LP_HUK_RST_EN_R = crate::BitReader;
#[doc = "Field `LP_HUK_RST_EN` writer - need_des"]
pub type LP_HUK_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_huk_rst_en(&self) -> LP_HUK_RST_EN_R {
        LP_HUK_RST_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUK")
            .field("lp_huk_rst_en", &self.lp_huk_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_huk_rst_en(&mut self) -> LP_HUK_RST_EN_W<'_, HUK_SPEC> {
        LP_HUK_RST_EN_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`huk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HUK_SPEC;
impl crate::RegisterSpec for HUK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`huk::R`](R) reader structure"]
impl crate::Readable for HUK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`huk::W`](W) writer structure"]
impl crate::Writable for HUK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HUK to value 0"]
impl crate::Resettable for HUK_SPEC {}
