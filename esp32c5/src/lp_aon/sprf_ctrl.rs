#[doc = "Register `SPRF_CTRL` reader"]
pub type R = crate::R<SPRF_CTRL_SPEC>;
#[doc = "Register `SPRF_CTRL` writer"]
pub type W = crate::W<SPRF_CTRL_SPEC>;
#[doc = "Field `SPRF_MEM_AUX_CTRL` reader - configure memory in lp system power status"]
pub type SPRF_MEM_AUX_CTRL_R = crate::FieldReader<u32>;
#[doc = "Field `SPRF_MEM_AUX_CTRL` writer - configure memory in lp system power status"]
pub type SPRF_MEM_AUX_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - configure memory in lp system power status"]
    #[inline(always)]
    pub fn sprf_mem_aux_ctrl(&self) -> SPRF_MEM_AUX_CTRL_R {
        SPRF_MEM_AUX_CTRL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPRF_CTRL")
            .field("sprf_mem_aux_ctrl", &self.sprf_mem_aux_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - configure memory in lp system power status"]
    #[inline(always)]
    pub fn sprf_mem_aux_ctrl(&mut self) -> SPRF_MEM_AUX_CTRL_W<SPRF_CTRL_SPEC> {
        SPRF_MEM_AUX_CTRL_W::new(self, 0)
    }
}
#[doc = "configure memory in lp system power status\n\nYou can [`read`](crate::Reg::read) this register and get [`sprf_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprf_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPRF_CTRL_SPEC;
impl crate::RegisterSpec for SPRF_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprf_ctrl::R`](R) reader structure"]
impl crate::Readable for SPRF_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sprf_ctrl::W`](W) writer structure"]
impl crate::Writable for SPRF_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPRF_CTRL to value 0x2070"]
impl crate::Resettable for SPRF_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x2070;
}
