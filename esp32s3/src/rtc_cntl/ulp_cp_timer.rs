#[doc = "Register `ULP_CP_TIMER` reader"]
pub type R = crate::R<ULP_CP_TIMER_SPEC>;
#[doc = "Register `ULP_CP_TIMER` writer"]
pub type W = crate::W<ULP_CP_TIMER_SPEC>;
#[doc = "Field `ULP_CP_PC_INIT` reader - ULP-coprocessor PC initial address"]
pub type ULP_CP_PC_INIT_R = crate::FieldReader<u16>;
#[doc = "Field `ULP_CP_PC_INIT` writer - ULP-coprocessor PC initial address"]
pub type ULP_CP_PC_INIT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `ULP_CP_GPIO_WAKEUP_ENA` reader - ULP-coprocessor wakeup by GPIO enable"]
pub type ULP_CP_GPIO_WAKEUP_ENA_R = crate::BitReader;
#[doc = "Field `ULP_CP_GPIO_WAKEUP_ENA` writer - ULP-coprocessor wakeup by GPIO enable"]
pub type ULP_CP_GPIO_WAKEUP_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULP_CP_GPIO_WAKEUP_CLR` writer - ULP-coprocessor wakeup by GPIO state clear"]
pub type ULP_CP_GPIO_WAKEUP_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULP_CP_SLP_TIMER_EN` reader - ULP-coprocessor timer enable bit"]
pub type ULP_CP_SLP_TIMER_EN_R = crate::BitReader;
#[doc = "Field `ULP_CP_SLP_TIMER_EN` writer - ULP-coprocessor timer enable bit"]
pub type ULP_CP_SLP_TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - ULP-coprocessor PC initial address"]
    #[inline(always)]
    pub fn ulp_cp_pc_init(&self) -> ULP_CP_PC_INIT_R {
        ULP_CP_PC_INIT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 29 - ULP-coprocessor wakeup by GPIO enable"]
    #[inline(always)]
    pub fn ulp_cp_gpio_wakeup_ena(&self) -> ULP_CP_GPIO_WAKEUP_ENA_R {
        ULP_CP_GPIO_WAKEUP_ENA_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - ULP-coprocessor timer enable bit"]
    #[inline(always)]
    pub fn ulp_cp_slp_timer_en(&self) -> ULP_CP_SLP_TIMER_EN_R {
        ULP_CP_SLP_TIMER_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ULP_CP_TIMER")
            .field("ulp_cp_pc_init", &self.ulp_cp_pc_init())
            .field("ulp_cp_gpio_wakeup_ena", &self.ulp_cp_gpio_wakeup_ena())
            .field("ulp_cp_slp_timer_en", &self.ulp_cp_slp_timer_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - ULP-coprocessor PC initial address"]
    #[inline(always)]
    pub fn ulp_cp_pc_init(&mut self) -> ULP_CP_PC_INIT_W<'_, ULP_CP_TIMER_SPEC> {
        ULP_CP_PC_INIT_W::new(self, 0)
    }
    #[doc = "Bit 29 - ULP-coprocessor wakeup by GPIO enable"]
    #[inline(always)]
    pub fn ulp_cp_gpio_wakeup_ena(&mut self) -> ULP_CP_GPIO_WAKEUP_ENA_W<'_, ULP_CP_TIMER_SPEC> {
        ULP_CP_GPIO_WAKEUP_ENA_W::new(self, 29)
    }
    #[doc = "Bit 30 - ULP-coprocessor wakeup by GPIO state clear"]
    #[inline(always)]
    pub fn ulp_cp_gpio_wakeup_clr(&mut self) -> ULP_CP_GPIO_WAKEUP_CLR_W<'_, ULP_CP_TIMER_SPEC> {
        ULP_CP_GPIO_WAKEUP_CLR_W::new(self, 30)
    }
    #[doc = "Bit 31 - ULP-coprocessor timer enable bit"]
    #[inline(always)]
    pub fn ulp_cp_slp_timer_en(&mut self) -> ULP_CP_SLP_TIMER_EN_W<'_, ULP_CP_TIMER_SPEC> {
        ULP_CP_SLP_TIMER_EN_W::new(self, 31)
    }
}
#[doc = "configure ulp\n\nYou can [`read`](crate::Reg::read) this register and get [`ulp_cp_timer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ulp_cp_timer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ULP_CP_TIMER_SPEC;
impl crate::RegisterSpec for ULP_CP_TIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ulp_cp_timer::R`](R) reader structure"]
impl crate::Readable for ULP_CP_TIMER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ulp_cp_timer::W`](W) writer structure"]
impl crate::Writable for ULP_CP_TIMER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ULP_CP_TIMER to value 0"]
impl crate::Resettable for ULP_CP_TIMER_SPEC {}
