#[doc = "Register `SYS_SDPRF_MEM_AUX_CTRL` reader"]
pub type R = crate::R<SYS_SDPRF_MEM_AUX_CTRL_SPEC>;
#[doc = "Register `SYS_SDPRF_MEM_AUX_CTRL` writer"]
pub type W = crate::W<SYS_SDPRF_MEM_AUX_CTRL_SPEC>;
#[doc = "Field `SYS_SDPRF_MEM_AUX_CTRL` reader - "]
pub type SYS_SDPRF_MEM_AUX_CTRL_R = crate::FieldReader<u32>;
#[doc = "Field `SYS_SDPRF_MEM_AUX_CTRL` writer - "]
pub type SYS_SDPRF_MEM_AUX_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sys_sdprf_mem_aux_ctrl(&self) -> SYS_SDPRF_MEM_AUX_CTRL_R {
        SYS_SDPRF_MEM_AUX_CTRL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS_SDPRF_MEM_AUX_CTRL")
            .field("sys_sdprf_mem_aux_ctrl", &self.sys_sdprf_mem_aux_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sys_sdprf_mem_aux_ctrl(
        &mut self,
    ) -> SYS_SDPRF_MEM_AUX_CTRL_W<'_, SYS_SDPRF_MEM_AUX_CTRL_SPEC> {
        SYS_SDPRF_MEM_AUX_CTRL_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_sdprf_mem_aux_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_sdprf_mem_aux_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SDPRF_MEM_AUX_CTRL_SPEC;
impl crate::RegisterSpec for SYS_SDPRF_MEM_AUX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_sdprf_mem_aux_ctrl::R`](R) reader structure"]
impl crate::Readable for SYS_SDPRF_MEM_AUX_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_sdprf_mem_aux_ctrl::W`](W) writer structure"]
impl crate::Writable for SYS_SDPRF_MEM_AUX_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_SDPRF_MEM_AUX_CTRL to value 0x2070"]
impl crate::Resettable for SYS_SDPRF_MEM_AUX_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x2070;
}
