#[doc = "Register `TCM_PERFORMACE_CTRL` reader"]
pub type R = crate::R<TCM_PERFORMACE_CTRL_SPEC>;
#[doc = "Register `TCM_PERFORMACE_CTRL` writer"]
pub type W = crate::W<TCM_PERFORMACE_CTRL_SPEC>;
#[doc = "Field `AHB_TO_TCM_MEM_EN_HOLD` reader - Set 1 to keep TCM mem enable hold when mem access is busy."]
pub type AHB_TO_TCM_MEM_EN_HOLD_R = crate::BitReader;
#[doc = "Field `AHB_TO_TCM_MEM_EN_HOLD` writer - Set 1 to keep TCM mem enable hold when mem access is busy."]
pub type AHB_TO_TCM_MEM_EN_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to keep TCM mem enable hold when mem access is busy."]
    #[inline(always)]
    pub fn ahb_to_tcm_mem_en_hold(&self) -> AHB_TO_TCM_MEM_EN_HOLD_R {
        AHB_TO_TCM_MEM_EN_HOLD_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCM_PERFORMACE_CTRL")
            .field("ahb_to_tcm_mem_en_hold", &self.ahb_to_tcm_mem_en_hold())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to keep TCM mem enable hold when mem access is busy."]
    #[inline(always)]
    pub fn ahb_to_tcm_mem_en_hold(
        &mut self,
    ) -> AHB_TO_TCM_MEM_EN_HOLD_W<'_, TCM_PERFORMACE_CTRL_SPEC> {
        AHB_TO_TCM_MEM_EN_HOLD_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_performace_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_performace_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCM_PERFORMACE_CTRL_SPEC;
impl crate::RegisterSpec for TCM_PERFORMACE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcm_performace_ctrl::R`](R) reader structure"]
impl crate::Readable for TCM_PERFORMACE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tcm_performace_ctrl::W`](W) writer structure"]
impl crate::Writable for TCM_PERFORMACE_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCM_PERFORMACE_CTRL to value 0"]
impl crate::Resettable for TCM_PERFORMACE_CTRL_SPEC {}
