#[doc = "Register `MEM_AUX_CTRL_4` reader"]
pub type R = crate::R<MEM_AUX_CTRL_4_SPEC>;
#[doc = "Register `MEM_AUX_CTRL_4` writer"]
pub type W = crate::W<MEM_AUX_CTRL_4_SPEC>;
#[doc = "Field `SHARP_MATRIX_UV_MEM_AUX_CTRL` reader - this field configures the mem_aux of sharp uv line buffer memory"]
pub type SHARP_MATRIX_UV_MEM_AUX_CTRL_R = crate::FieldReader<u16>;
#[doc = "Field `SHARP_MATRIX_UV_MEM_AUX_CTRL` writer - this field configures the mem_aux of sharp uv line buffer memory"]
pub type SHARP_MATRIX_UV_MEM_AUX_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - this field configures the mem_aux of sharp uv line buffer memory"]
    #[inline(always)]
    pub fn sharp_matrix_uv_mem_aux_ctrl(&self) -> SHARP_MATRIX_UV_MEM_AUX_CTRL_R {
        SHARP_MATRIX_UV_MEM_AUX_CTRL_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_AUX_CTRL_4")
            .field(
                "sharp_matrix_uv_mem_aux_ctrl",
                &self.sharp_matrix_uv_mem_aux_ctrl(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:13 - this field configures the mem_aux of sharp uv line buffer memory"]
    #[inline(always)]
    #[must_use]
    pub fn sharp_matrix_uv_mem_aux_ctrl(
        &mut self,
    ) -> SHARP_MATRIX_UV_MEM_AUX_CTRL_W<MEM_AUX_CTRL_4_SPEC> {
        SHARP_MATRIX_UV_MEM_AUX_CTRL_W::new(self, 0)
    }
}
#[doc = "mem aux control register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_aux_ctrl_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_aux_ctrl_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_AUX_CTRL_4_SPEC;
impl crate::RegisterSpec for MEM_AUX_CTRL_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_aux_ctrl_4::R`](R) reader structure"]
impl crate::Readable for MEM_AUX_CTRL_4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_aux_ctrl_4::W`](W) writer structure"]
impl crate::Writable for MEM_AUX_CTRL_4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_AUX_CTRL_4 to value 0x1320"]
impl crate::Resettable for MEM_AUX_CTRL_4_SPEC {
    const RESET_VALUE: u32 = 0x1320;
}
