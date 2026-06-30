#[doc = "Register `LP_AONCLKRST_HUK` reader"]
pub type R = crate::R<LP_AONCLKRST_HUK_SPEC>;
#[doc = "Register `LP_AONCLKRST_HUK` writer"]
pub type W = crate::W<LP_AONCLKRST_HUK_SPEC>;
#[doc = "Field `LP_AONCLKRST_LP_HUK_RST_EN` reader - need_des"]
pub type LP_AONCLKRST_LP_HUK_RST_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_LP_HUK_RST_EN` writer - need_des"]
pub type LP_AONCLKRST_LP_HUK_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_lp_huk_rst_en(&self) -> LP_AONCLKRST_LP_HUK_RST_EN_R {
        LP_AONCLKRST_LP_HUK_RST_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_HUK")
            .field(
                "lp_aonclkrst_lp_huk_rst_en",
                &self.lp_aonclkrst_lp_huk_rst_en(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_lp_huk_rst_en(
        &mut self,
    ) -> LP_AONCLKRST_LP_HUK_RST_EN_W<'_, LP_AONCLKRST_HUK_SPEC> {
        LP_AONCLKRST_LP_HUK_RST_EN_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_huk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_huk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_HUK_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_HUK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_huk::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_HUK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_huk::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_HUK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_AONCLKRST_HUK to value 0"]
impl crate::Resettable for LP_AONCLKRST_HUK_SPEC {}
