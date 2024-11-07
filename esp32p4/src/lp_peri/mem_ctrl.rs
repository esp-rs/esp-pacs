#[doc = "Register `MEM_CTRL` reader"]
pub type R = crate::R<MEM_CTRL_SPEC>;
#[doc = "Register `MEM_CTRL` writer"]
pub type W = crate::W<MEM_CTRL_SPEC>;
#[doc = "Field `LP_UART_WAKEUP_FLAG_CLR` writer - need_des"]
pub type LP_UART_WAKEUP_FLAG_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_UART_WAKEUP_FLAG` reader - need_des"]
pub type LP_UART_WAKEUP_FLAG_R = crate::BitReader;
#[doc = "Field `LP_UART_WAKEUP_FLAG` writer - need_des"]
pub type LP_UART_WAKEUP_FLAG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_UART_WAKEUP_EN` reader - need_des"]
pub type LP_UART_WAKEUP_EN_R = crate::BitReader;
#[doc = "Field `LP_UART_WAKEUP_EN` writer - need_des"]
pub type LP_UART_WAKEUP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_UART_MEM_FORCE_PD` reader - need_des"]
pub type LP_UART_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `LP_UART_MEM_FORCE_PD` writer - need_des"]
pub type LP_UART_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_UART_MEM_FORCE_PU` reader - need_des"]
pub type LP_UART_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `LP_UART_MEM_FORCE_PU` writer - need_des"]
pub type LP_UART_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn lp_uart_wakeup_flag(&self) -> LP_UART_WAKEUP_FLAG_R {
        LP_UART_WAKEUP_FLAG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_uart_wakeup_en(&self) -> LP_UART_WAKEUP_EN_R {
        LP_UART_WAKEUP_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_uart_mem_force_pd(&self) -> LP_UART_MEM_FORCE_PD_R {
        LP_UART_MEM_FORCE_PD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_uart_mem_force_pu(&self) -> LP_UART_MEM_FORCE_PU_R {
        LP_UART_MEM_FORCE_PU_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_CTRL")
            .field("lp_uart_wakeup_flag", &self.lp_uart_wakeup_flag())
            .field("lp_uart_wakeup_en", &self.lp_uart_wakeup_en())
            .field("lp_uart_mem_force_pd", &self.lp_uart_mem_force_pd())
            .field("lp_uart_mem_force_pu", &self.lp_uart_mem_force_pu())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn lp_uart_wakeup_flag_clr(&mut self) -> LP_UART_WAKEUP_FLAG_CLR_W<MEM_CTRL_SPEC> {
        LP_UART_WAKEUP_FLAG_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn lp_uart_wakeup_flag(&mut self) -> LP_UART_WAKEUP_FLAG_W<MEM_CTRL_SPEC> {
        LP_UART_WAKEUP_FLAG_W::new(self, 1)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_uart_wakeup_en(&mut self) -> LP_UART_WAKEUP_EN_W<MEM_CTRL_SPEC> {
        LP_UART_WAKEUP_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_uart_mem_force_pd(&mut self) -> LP_UART_MEM_FORCE_PD_W<MEM_CTRL_SPEC> {
        LP_UART_MEM_FORCE_PD_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_uart_mem_force_pu(&mut self) -> LP_UART_MEM_FORCE_PU_W<MEM_CTRL_SPEC> {
        LP_UART_MEM_FORCE_PU_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_CTRL_SPEC;
impl crate::RegisterSpec for MEM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_ctrl::R`](R) reader structure"]
impl crate::Readable for MEM_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_ctrl::W`](W) writer structure"]
impl crate::Writable for MEM_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_CTRL to value 0x8000_0000"]
impl crate::Resettable for MEM_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x8000_0000;
}
