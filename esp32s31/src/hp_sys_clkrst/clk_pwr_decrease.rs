#[doc = "Register `CLK_PWR_DECREASE` reader"]
pub type R = crate::R<CLK_PWR_DECREASE_SPEC>;
#[doc = "Register `CLK_PWR_DECREASE` writer"]
pub type W = crate::W<CLK_PWR_DECREASE_SPEC>;
#[doc = "Field `APB_HW_DECREASE_DIV_NUM` reader - need_des"]
pub type APB_HW_DECREASE_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `APB_HW_DECREASE_DIV_NUM` writer - need_des"]
pub type APB_HW_DECREASE_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CPU_WFI_DECREASE_EN` reader - need_des"]
pub type CPU_WFI_DECREASE_EN_R = crate::BitReader;
#[doc = "Field `CPU_WFI_DECREASE_EN` writer - need_des"]
pub type CPU_WFI_DECREASE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn apb_hw_decrease_div_num(&self) -> APB_HW_DECREASE_DIV_NUM_R {
        APB_HW_DECREASE_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn cpu_wfi_decrease_en(&self) -> CPU_WFI_DECREASE_EN_R {
        CPU_WFI_DECREASE_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_PWR_DECREASE")
            .field("apb_hw_decrease_div_num", &self.apb_hw_decrease_div_num())
            .field("cpu_wfi_decrease_en", &self.cpu_wfi_decrease_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn apb_hw_decrease_div_num(
        &mut self,
    ) -> APB_HW_DECREASE_DIV_NUM_W<'_, CLK_PWR_DECREASE_SPEC> {
        APB_HW_DECREASE_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn cpu_wfi_decrease_en(&mut self) -> CPU_WFI_DECREASE_EN_W<'_, CLK_PWR_DECREASE_SPEC> {
        CPU_WFI_DECREASE_EN_W::new(self, 8)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_pwr_decrease::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_pwr_decrease::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_PWR_DECREASE_SPEC;
impl crate::RegisterSpec for CLK_PWR_DECREASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_pwr_decrease::R`](R) reader structure"]
impl crate::Readable for CLK_PWR_DECREASE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_pwr_decrease::W`](W) writer structure"]
impl crate::Writable for CLK_PWR_DECREASE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLK_PWR_DECREASE to value 0"]
impl crate::Resettable for CLK_PWR_DECREASE_SPEC {}
