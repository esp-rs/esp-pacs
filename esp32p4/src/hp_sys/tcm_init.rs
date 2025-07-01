#[doc = "Register `TCM_INIT` reader"]
pub type R = crate::R<TCM_INIT_SPEC>;
#[doc = "Register `TCM_INIT` writer"]
pub type W = crate::W<TCM_INIT_SPEC>;
#[doc = "Field `REG_TCM_INIT_EN` reader - NA"]
pub type REG_TCM_INIT_EN_R = crate::BitReader;
#[doc = "Field `REG_TCM_INIT_EN` writer - NA"]
pub type REG_TCM_INIT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TCM_INIT_CNT_RESET` reader - Set 1 to reset tcm init cnt"]
pub type REG_TCM_INIT_CNT_RESET_R = crate::BitReader;
#[doc = "Field `REG_TCM_INIT_CNT_RESET` writer - Set 1 to reset tcm init cnt"]
pub type REG_TCM_INIT_CNT_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TCM_INIT_DONE` reader - NA"]
pub type REG_TCM_INIT_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_tcm_init_en(&self) -> REG_TCM_INIT_EN_R {
        REG_TCM_INIT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to reset tcm init cnt"]
    #[inline(always)]
    pub fn reg_tcm_init_cnt_reset(&self) -> REG_TCM_INIT_CNT_RESET_R {
        REG_TCM_INIT_CNT_RESET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn reg_tcm_init_done(&self) -> REG_TCM_INIT_DONE_R {
        REG_TCM_INIT_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCM_INIT")
            .field("reg_tcm_init_en", &self.reg_tcm_init_en())
            .field("reg_tcm_init_cnt_reset", &self.reg_tcm_init_cnt_reset())
            .field("reg_tcm_init_done", &self.reg_tcm_init_done())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_tcm_init_en(&mut self) -> REG_TCM_INIT_EN_W<TCM_INIT_SPEC> {
        REG_TCM_INIT_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 1 to reset tcm init cnt"]
    #[inline(always)]
    pub fn reg_tcm_init_cnt_reset(&mut self) -> REG_TCM_INIT_CNT_RESET_W<TCM_INIT_SPEC> {
        REG_TCM_INIT_CNT_RESET_W::new(self, 1)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_init::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_init::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCM_INIT_SPEC;
impl crate::RegisterSpec for TCM_INIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcm_init::R`](R) reader structure"]
impl crate::Readable for TCM_INIT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tcm_init::W`](W) writer structure"]
impl crate::Writable for TCM_INIT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCM_INIT to value 0x02"]
impl crate::Resettable for TCM_INIT_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
