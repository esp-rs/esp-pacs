#[doc = "Register `UART0_PD_CTRL` reader"]
pub type R = crate::R<UART0_PD_CTRL_SPEC>;
#[doc = "Register `UART0_PD_CTRL` writer"]
pub type W = crate::W<UART0_PD_CTRL_SPEC>;
#[doc = "Field `UART0_MEM_FORCE_PU` reader - Set this bit to force power down UART0 memory."]
pub type UART0_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `UART0_MEM_FORCE_PU` writer - Set this bit to force power down UART0 memory."]
pub type UART0_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART0_MEM_FORCE_PD` reader - Set this bit to force power up UART0 memory."]
pub type UART0_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `UART0_MEM_FORCE_PD` writer - Set this bit to force power up UART0 memory."]
pub type UART0_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Set this bit to force power down UART0 memory."]
    #[inline(always)]
    pub fn uart0_mem_force_pu(&self) -> UART0_MEM_FORCE_PU_R {
        UART0_MEM_FORCE_PU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to force power up UART0 memory."]
    #[inline(always)]
    pub fn uart0_mem_force_pd(&self) -> UART0_MEM_FORCE_PD_R {
        UART0_MEM_FORCE_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART0_PD_CTRL")
            .field("uart0_mem_force_pu", &self.uart0_mem_force_pu())
            .field("uart0_mem_force_pd", &self.uart0_mem_force_pd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Set this bit to force power down UART0 memory."]
    #[inline(always)]
    #[must_use]
    pub fn uart0_mem_force_pu(&mut self) -> UART0_MEM_FORCE_PU_W<UART0_PD_CTRL_SPEC> {
        UART0_MEM_FORCE_PU_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to force power up UART0 memory."]
    #[inline(always)]
    #[must_use]
    pub fn uart0_mem_force_pd(&mut self) -> UART0_MEM_FORCE_PD_W<UART0_PD_CTRL_SPEC> {
        UART0_MEM_FORCE_PD_W::new(self, 2)
    }
}
#[doc = "UART0 power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_pd_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_pd_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART0_PD_CTRL_SPEC;
impl crate::RegisterSpec for UART0_PD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart0_pd_ctrl::R`](R) reader structure"]
impl crate::Readable for UART0_PD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart0_pd_ctrl::W`](W) writer structure"]
impl crate::Writable for UART0_PD_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART0_PD_CTRL to value 0x02"]
impl crate::Resettable for UART0_PD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
