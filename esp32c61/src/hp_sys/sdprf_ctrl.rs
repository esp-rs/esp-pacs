#[doc = "Register `SDPRF_CTRL` reader"]
pub type R = crate::R<SDPRF_CTRL_SPEC>;
#[doc = "Register `SDPRF_CTRL` writer"]
pub type W = crate::W<SDPRF_CTRL_SPEC>;
#[doc = "Field `SDPRF_MEM_AUX_CTRL` reader - reserved"]
pub type SDPRF_MEM_AUX_CTRL_R = crate::FieldReader<u32>;
#[doc = "Field `SDPRF_MEM_AUX_CTRL` writer - reserved"]
pub type SDPRF_MEM_AUX_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    pub fn sdprf_mem_aux_ctrl(&self) -> SDPRF_MEM_AUX_CTRL_R {
        SDPRF_MEM_AUX_CTRL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDPRF_CTRL")
            .field("sdprf_mem_aux_ctrl", &self.sdprf_mem_aux_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    pub fn sdprf_mem_aux_ctrl(&mut self) -> SDPRF_MEM_AUX_CTRL_W<'_, SDPRF_CTRL_SPEC> {
        SDPRF_MEM_AUX_CTRL_W::new(self, 0)
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`sdprf_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdprf_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDPRF_CTRL_SPEC;
impl crate::RegisterSpec for SDPRF_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdprf_ctrl::R`](R) reader structure"]
impl crate::Readable for SDPRF_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdprf_ctrl::W`](W) writer structure"]
impl crate::Writable for SDPRF_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDPRF_CTRL to value 0"]
impl crate::Resettable for SDPRF_CTRL_SPEC {}
