#[doc = "Register `MEM_AUX_CTRL_1` reader"]
pub type R = crate::R<MEM_AUX_CTRL_1_SPEC>;
#[doc = "Register `MEM_AUX_CTRL_1` writer"]
pub type W = crate::W<MEM_AUX_CTRL_1_SPEC>;
#[doc = "Field `LSC_LUT_R_GR_MEM_AUX_CTRL` reader - this field configures the mem_aux of lsc r gr lut memory"]
pub type LSC_LUT_R_GR_MEM_AUX_CTRL_R = crate::FieldReader<u16>;
#[doc = "Field `LSC_LUT_R_GR_MEM_AUX_CTRL` writer - this field configures the mem_aux of lsc r gr lut memory"]
pub type LSC_LUT_R_GR_MEM_AUX_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `LSC_LUT_GB_B_MEM_AUX_CTRL` reader - this field configures the mem_aux of lsc gb b lut memory"]
pub type LSC_LUT_GB_B_MEM_AUX_CTRL_R = crate::FieldReader<u16>;
#[doc = "Field `LSC_LUT_GB_B_MEM_AUX_CTRL` writer - this field configures the mem_aux of lsc gb b lut memory"]
pub type LSC_LUT_GB_B_MEM_AUX_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - this field configures the mem_aux of lsc r gr lut memory"]
    #[inline(always)]
    pub fn lsc_lut_r_gr_mem_aux_ctrl(&self) -> LSC_LUT_R_GR_MEM_AUX_CTRL_R {
        LSC_LUT_R_GR_MEM_AUX_CTRL_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - this field configures the mem_aux of lsc gb b lut memory"]
    #[inline(always)]
    pub fn lsc_lut_gb_b_mem_aux_ctrl(&self) -> LSC_LUT_GB_B_MEM_AUX_CTRL_R {
        LSC_LUT_GB_B_MEM_AUX_CTRL_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_AUX_CTRL_1")
            .field(
                "lsc_lut_r_gr_mem_aux_ctrl",
                &self.lsc_lut_r_gr_mem_aux_ctrl(),
            )
            .field(
                "lsc_lut_gb_b_mem_aux_ctrl",
                &self.lsc_lut_gb_b_mem_aux_ctrl(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - this field configures the mem_aux of lsc r gr lut memory"]
    #[inline(always)]
    #[must_use]
    pub fn lsc_lut_r_gr_mem_aux_ctrl(
        &mut self,
    ) -> LSC_LUT_R_GR_MEM_AUX_CTRL_W<MEM_AUX_CTRL_1_SPEC> {
        LSC_LUT_R_GR_MEM_AUX_CTRL_W::new(self, 0)
    }
    #[doc = "Bits 16:29 - this field configures the mem_aux of lsc gb b lut memory"]
    #[inline(always)]
    #[must_use]
    pub fn lsc_lut_gb_b_mem_aux_ctrl(
        &mut self,
    ) -> LSC_LUT_GB_B_MEM_AUX_CTRL_W<MEM_AUX_CTRL_1_SPEC> {
        LSC_LUT_GB_B_MEM_AUX_CTRL_W::new(self, 16)
    }
}
#[doc = "mem aux control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_aux_ctrl_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_aux_ctrl_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_AUX_CTRL_1_SPEC;
impl crate::RegisterSpec for MEM_AUX_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_aux_ctrl_1::R`](R) reader structure"]
impl crate::Readable for MEM_AUX_CTRL_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_aux_ctrl_1::W`](W) writer structure"]
impl crate::Writable for MEM_AUX_CTRL_1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_AUX_CTRL_1 to value 0x1320_1320"]
impl crate::Resettable for MEM_AUX_CTRL_1_SPEC {
    const RESET_VALUE: u32 = 0x1320_1320;
}
