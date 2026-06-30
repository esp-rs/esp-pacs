#[doc = "Register `HP_TCM_DATA_DUMP_CTRL` reader"]
pub type R = crate::R<HP_TCM_DATA_DUMP_CTRL_SPEC>;
#[doc = "Register `HP_TCM_DATA_DUMP_CTRL` writer"]
pub type W = crate::W<HP_TCM_DATA_DUMP_CTRL_SPEC>;
#[doc = "Field `HP_TCM_DATA_DUMP_EN` reader - hp tcm_data dump switch"]
pub type HP_TCM_DATA_DUMP_EN_R = crate::FieldReader<u32>;
#[doc = "Field `HP_TCM_DATA_DUMP_EN` writer - hp tcm_data dump switch"]
pub type HP_TCM_DATA_DUMP_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - hp tcm_data dump switch"]
    #[inline(always)]
    pub fn hp_tcm_data_dump_en(&self) -> HP_TCM_DATA_DUMP_EN_R {
        HP_TCM_DATA_DUMP_EN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_TCM_DATA_DUMP_CTRL")
            .field("hp_tcm_data_dump_en", &self.hp_tcm_data_dump_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - hp tcm_data dump switch"]
    #[inline(always)]
    pub fn hp_tcm_data_dump_en(&mut self) -> HP_TCM_DATA_DUMP_EN_W<'_, HP_TCM_DATA_DUMP_CTRL_SPEC> {
        HP_TCM_DATA_DUMP_EN_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_tcm_data_dump_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_tcm_data_dump_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_TCM_DATA_DUMP_CTRL_SPEC;
impl crate::RegisterSpec for HP_TCM_DATA_DUMP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_tcm_data_dump_ctrl::R`](R) reader structure"]
impl crate::Readable for HP_TCM_DATA_DUMP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_tcm_data_dump_ctrl::W`](W) writer structure"]
impl crate::Writable for HP_TCM_DATA_DUMP_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_TCM_DATA_DUMP_CTRL to value 0"]
impl crate::Resettable for HP_TCM_DATA_DUMP_CTRL_SPEC {}
