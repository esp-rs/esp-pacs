#[doc = "Register `CPU_SPRF_MEM_AUX_CTRL` reader"]
pub type R = crate::R<CPU_SPRF_MEM_AUX_CTRL_SPEC>;
#[doc = "Register `CPU_SPRF_MEM_AUX_CTRL` writer"]
pub type W = crate::W<CPU_SPRF_MEM_AUX_CTRL_SPEC>;
#[doc = "Field `CPU_SPRF_MEM_AUX_CTRL` reader - need_des"]
pub type CPU_SPRF_MEM_AUX_CTRL_R = crate::FieldReader<u32>;
#[doc = "Field `CPU_SPRF_MEM_AUX_CTRL` writer - need_des"]
pub type CPU_SPRF_MEM_AUX_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn cpu_sprf_mem_aux_ctrl(&self) -> CPU_SPRF_MEM_AUX_CTRL_R {
        CPU_SPRF_MEM_AUX_CTRL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_SPRF_MEM_AUX_CTRL")
            .field("cpu_sprf_mem_aux_ctrl", &self.cpu_sprf_mem_aux_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn cpu_sprf_mem_aux_ctrl(
        &mut self,
    ) -> CPU_SPRF_MEM_AUX_CTRL_W<'_, CPU_SPRF_MEM_AUX_CTRL_SPEC> {
        CPU_SPRF_MEM_AUX_CTRL_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_sprf_mem_aux_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_sprf_mem_aux_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_SPRF_MEM_AUX_CTRL_SPEC;
impl crate::RegisterSpec for CPU_SPRF_MEM_AUX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_sprf_mem_aux_ctrl::R`](R) reader structure"]
impl crate::Readable for CPU_SPRF_MEM_AUX_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_sprf_mem_aux_ctrl::W`](W) writer structure"]
impl crate::Writable for CPU_SPRF_MEM_AUX_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_SPRF_MEM_AUX_CTRL to value 0x2070"]
impl crate::Resettable for CPU_SPRF_MEM_AUX_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x2070;
}
