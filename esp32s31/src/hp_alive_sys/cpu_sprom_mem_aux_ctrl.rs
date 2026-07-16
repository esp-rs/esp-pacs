#[doc = "Register `CPU_SPROM_MEM_AUX_CTRL` reader"]
pub type R = crate::R<CPU_SPROM_MEM_AUX_CTRL_SPEC>;
#[doc = "Register `CPU_SPROM_MEM_AUX_CTRL` writer"]
pub type W = crate::W<CPU_SPROM_MEM_AUX_CTRL_SPEC>;
#[doc = "Field `CPU_SPROM_MEM_AUX_CTRL` reader - sprom mem aux ctrl , control CPU power domain"]
pub type CPU_SPROM_MEM_AUX_CTRL_R = crate::FieldReader<u32>;
#[doc = "Field `CPU_SPROM_MEM_AUX_CTRL` writer - sprom mem aux ctrl , control CPU power domain"]
pub type CPU_SPROM_MEM_AUX_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - sprom mem aux ctrl , control CPU power domain"]
    #[inline(always)]
    pub fn cpu_sprom_mem_aux_ctrl(&self) -> CPU_SPROM_MEM_AUX_CTRL_R {
        CPU_SPROM_MEM_AUX_CTRL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_SPROM_MEM_AUX_CTRL")
            .field("cpu_sprom_mem_aux_ctrl", &self.cpu_sprom_mem_aux_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - sprom mem aux ctrl , control CPU power domain"]
    #[inline(always)]
    pub fn cpu_sprom_mem_aux_ctrl(
        &mut self,
    ) -> CPU_SPROM_MEM_AUX_CTRL_W<'_, CPU_SPROM_MEM_AUX_CTRL_SPEC> {
        CPU_SPROM_MEM_AUX_CTRL_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_sprom_mem_aux_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_sprom_mem_aux_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_SPROM_MEM_AUX_CTRL_SPEC;
impl crate::RegisterSpec for CPU_SPROM_MEM_AUX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_sprom_mem_aux_ctrl::R`](R) reader structure"]
impl crate::Readable for CPU_SPROM_MEM_AUX_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_sprom_mem_aux_ctrl::W`](W) writer structure"]
impl crate::Writable for CPU_SPROM_MEM_AUX_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_SPROM_MEM_AUX_CTRL to value 0x70"]
impl crate::Resettable for CPU_SPROM_MEM_AUX_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x70;
}
