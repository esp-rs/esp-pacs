#[doc = "Register `F2S_APB_BRG_CNTL` reader"]
pub type R = crate::R<F2S_APB_BRG_CNTL_SPEC>;
#[doc = "Register `F2S_APB_BRG_CNTL` writer"]
pub type W = crate::W<F2S_APB_BRG_CNTL_SPEC>;
#[doc = "Field `F2S_APB_POSTW_EN` reader - reserved"]
pub type F2S_APB_POSTW_EN_R = crate::BitReader;
#[doc = "Field `F2S_APB_POSTW_EN` writer - reserved"]
pub type F2S_APB_POSTW_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn f2s_apb_postw_en(&self) -> F2S_APB_POSTW_EN_R {
        F2S_APB_POSTW_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("F2S_APB_BRG_CNTL")
            .field("f2s_apb_postw_en", &self.f2s_apb_postw_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - reserved"]
    #[inline(always)]
    pub fn f2s_apb_postw_en(&mut self) -> F2S_APB_POSTW_EN_W<F2S_APB_BRG_CNTL_SPEC> {
        F2S_APB_POSTW_EN_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`f2s_apb_brg_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`f2s_apb_brg_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct F2S_APB_BRG_CNTL_SPEC;
impl crate::RegisterSpec for F2S_APB_BRG_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`f2s_apb_brg_cntl::R`](R) reader structure"]
impl crate::Readable for F2S_APB_BRG_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`f2s_apb_brg_cntl::W`](W) writer structure"]
impl crate::Writable for F2S_APB_BRG_CNTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets F2S_APB_BRG_CNTL to value 0"]
impl crate::Resettable for F2S_APB_BRG_CNTL_SPEC {}
