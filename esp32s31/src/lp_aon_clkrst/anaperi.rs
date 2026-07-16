#[doc = "Register `ANAPERI` reader"]
pub type R = crate::R<ANAPERI_SPEC>;
#[doc = "Register `ANAPERI` writer"]
pub type W = crate::W<ANAPERI_SPEC>;
#[doc = "Field `LP_ANAPERI_RST_EN` reader - need_des"]
pub type LP_ANAPERI_RST_EN_R = crate::BitReader;
#[doc = "Field `LP_ANAPERI_RST_EN` writer - need_des"]
pub type LP_ANAPERI_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_anaperi_rst_en(&self) -> LP_ANAPERI_RST_EN_R {
        LP_ANAPERI_RST_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANAPERI")
            .field("lp_anaperi_rst_en", &self.lp_anaperi_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_anaperi_rst_en(&mut self) -> LP_ANAPERI_RST_EN_W<'_, ANAPERI_SPEC> {
        LP_ANAPERI_RST_EN_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`anaperi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anaperi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ANAPERI_SPEC;
impl crate::RegisterSpec for ANAPERI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`anaperi::R`](R) reader structure"]
impl crate::Readable for ANAPERI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`anaperi::W`](W) writer structure"]
impl crate::Writable for ANAPERI_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ANAPERI to value 0"]
impl crate::Resettable for ANAPERI_SPEC {}
