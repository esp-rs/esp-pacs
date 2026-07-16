#[doc = "Register `UART_MEM_LP_CTRL` reader"]
pub type R = crate::R<UART_MEM_LP_CTRL_SPEC>;
#[doc = "Register `UART_MEM_LP_CTRL` writer"]
pub type W = crate::W<UART_MEM_LP_CTRL_SPEC>;
#[doc = "Field `UART_MEM_LP_MODE` reader - "]
pub type UART_MEM_LP_MODE_R = crate::FieldReader;
#[doc = "Field `UART_MEM_LP_MODE` writer - "]
pub type UART_MEM_LP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UART_MEM_LP_EN` reader - "]
pub type UART_MEM_LP_EN_R = crate::BitReader;
#[doc = "Field `UART_MEM_LP_EN` writer - "]
pub type UART_MEM_LP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART_MEM_LP_FORCE_CTRL` reader - "]
pub type UART_MEM_LP_FORCE_CTRL_R = crate::BitReader;
#[doc = "Field `UART_MEM_LP_FORCE_CTRL` writer - "]
pub type UART_MEM_LP_FORCE_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn uart_mem_lp_mode(&self) -> UART_MEM_LP_MODE_R {
        UART_MEM_LP_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uart_mem_lp_en(&self) -> UART_MEM_LP_EN_R {
        UART_MEM_LP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn uart_mem_lp_force_ctrl(&self) -> UART_MEM_LP_FORCE_CTRL_R {
        UART_MEM_LP_FORCE_CTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART_MEM_LP_CTRL")
            .field("uart_mem_lp_mode", &self.uart_mem_lp_mode())
            .field("uart_mem_lp_en", &self.uart_mem_lp_en())
            .field("uart_mem_lp_force_ctrl", &self.uart_mem_lp_force_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn uart_mem_lp_mode(&mut self) -> UART_MEM_LP_MODE_W<'_, UART_MEM_LP_CTRL_SPEC> {
        UART_MEM_LP_MODE_W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn uart_mem_lp_en(&mut self) -> UART_MEM_LP_EN_W<'_, UART_MEM_LP_CTRL_SPEC> {
        UART_MEM_LP_EN_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn uart_mem_lp_force_ctrl(
        &mut self,
    ) -> UART_MEM_LP_FORCE_CTRL_W<'_, UART_MEM_LP_CTRL_SPEC> {
        UART_MEM_LP_FORCE_CTRL_W::new(self, 3)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_mem_lp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart_mem_lp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART_MEM_LP_CTRL_SPEC;
impl crate::RegisterSpec for UART_MEM_LP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_mem_lp_ctrl::R`](R) reader structure"]
impl crate::Readable for UART_MEM_LP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart_mem_lp_ctrl::W`](W) writer structure"]
impl crate::Writable for UART_MEM_LP_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UART_MEM_LP_CTRL to value 0x02"]
impl crate::Resettable for UART_MEM_LP_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
