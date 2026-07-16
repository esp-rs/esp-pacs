#[doc = "Register `CPU_WFI_DELAY_CTRL0` reader"]
pub type R = crate::R<CPU_WFI_DELAY_CTRL0_SPEC>;
#[doc = "Register `CPU_WFI_DELAY_CTRL0` writer"]
pub type W = crate::W<CPU_WFI_DELAY_CTRL0_SPEC>;
#[doc = "Field `CPUICM_DELAY_NUM` reader - need_des"]
pub type CPUICM_DELAY_NUM_R = crate::FieldReader;
#[doc = "Field `CPUICM_DELAY_NUM` writer - need_des"]
pub type CPUICM_DELAY_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - need_des"]
    #[inline(always)]
    pub fn cpuicm_delay_num(&self) -> CPUICM_DELAY_NUM_R {
        CPUICM_DELAY_NUM_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPU_WFI_DELAY_CTRL0")
            .field("cpuicm_delay_num", &self.cpuicm_delay_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - need_des"]
    #[inline(always)]
    pub fn cpuicm_delay_num(&mut self) -> CPUICM_DELAY_NUM_W<'_, CPU_WFI_DELAY_CTRL0_SPEC> {
        CPUICM_DELAY_NUM_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu_wfi_delay_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu_wfi_delay_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPU_WFI_DELAY_CTRL0_SPEC;
impl crate::RegisterSpec for CPU_WFI_DELAY_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu_wfi_delay_ctrl0::R`](R) reader structure"]
impl crate::Readable for CPU_WFI_DELAY_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpu_wfi_delay_ctrl0::W`](W) writer structure"]
impl crate::Writable for CPU_WFI_DELAY_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPU_WFI_DELAY_CTRL0 to value 0"]
impl crate::Resettable for CPU_WFI_DELAY_CTRL0_SPEC {}
