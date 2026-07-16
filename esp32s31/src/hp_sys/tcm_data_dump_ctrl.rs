#[doc = "Register `TCM_DATA_DUMP_CTRL` reader"]
pub type R = crate::R<TCM_DATA_DUMP_CTRL_SPEC>;
#[doc = "Register `TCM_DATA_DUMP_CTRL` writer"]
pub type W = crate::W<TCM_DATA_DUMP_CTRL_SPEC>;
#[doc = "Field `TCM_DATA_DUMP_EN` reader - hp tcm_data dump switch"]
pub type TCM_DATA_DUMP_EN_R = crate::FieldReader<u32>;
#[doc = "Field `TCM_DATA_DUMP_EN` writer - hp tcm_data dump switch"]
pub type TCM_DATA_DUMP_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - hp tcm_data dump switch"]
    #[inline(always)]
    pub fn tcm_data_dump_en(&self) -> TCM_DATA_DUMP_EN_R {
        TCM_DATA_DUMP_EN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCM_DATA_DUMP_CTRL")
            .field("tcm_data_dump_en", &self.tcm_data_dump_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - hp tcm_data dump switch"]
    #[inline(always)]
    pub fn tcm_data_dump_en(&mut self) -> TCM_DATA_DUMP_EN_W<'_, TCM_DATA_DUMP_CTRL_SPEC> {
        TCM_DATA_DUMP_EN_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`tcm_data_dump_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcm_data_dump_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TCM_DATA_DUMP_CTRL_SPEC;
impl crate::RegisterSpec for TCM_DATA_DUMP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcm_data_dump_ctrl::R`](R) reader structure"]
impl crate::Readable for TCM_DATA_DUMP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tcm_data_dump_ctrl::W`](W) writer structure"]
impl crate::Writable for TCM_DATA_DUMP_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCM_DATA_DUMP_CTRL to value 0"]
impl crate::Resettable for TCM_DATA_DUMP_CTRL_SPEC {}
