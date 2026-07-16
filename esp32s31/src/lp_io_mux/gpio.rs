#[doc = "Register `GPIO%s` reader"]
pub type R = crate::R<GPIO_SPEC>;
#[doc = "Register `GPIO%s` writer"]
pub type W = crate::W<GPIO_SPEC>;
#[doc = "Field `MCU_OE` reader - "]
pub type MCU_OE_R = crate::BitReader;
#[doc = "Field `MCU_OE` writer - "]
pub type MCU_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_SEL` reader - "]
pub type SLP_SEL_R = crate::BitReader;
#[doc = "Field `SLP_SEL` writer - "]
pub type SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_WPD` reader - "]
pub type MCU_WPD_R = crate::BitReader;
#[doc = "Field `MCU_WPD` writer - "]
pub type MCU_WPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_WPU` reader - "]
pub type MCU_WPU_R = crate::BitReader;
#[doc = "Field `MCU_WPU` writer - "]
pub type MCU_WPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_IE` reader - "]
pub type MCU_IE_R = crate::BitReader;
#[doc = "Field `MCU_IE` writer - "]
pub type MCU_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_DRV` reader - "]
pub type MCU_DRV_R = crate::FieldReader;
#[doc = "Field `MCU_DRV` writer - "]
pub type MCU_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FUN_WPD` reader - "]
pub type FUN_WPD_R = crate::BitReader;
#[doc = "Field `FUN_WPD` writer - "]
pub type FUN_WPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUN_WPU` reader - "]
pub type FUN_WPU_R = crate::BitReader;
#[doc = "Field `FUN_WPU` writer - "]
pub type FUN_WPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUN_IE` reader - "]
pub type FUN_IE_R = crate::BitReader;
#[doc = "Field `FUN_IE` writer - "]
pub type FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUN_DRV` reader - "]
pub type FUN_DRV_R = crate::FieldReader;
#[doc = "Field `FUN_DRV` writer - "]
pub type FUN_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCU_SEL` reader - "]
pub type MCU_SEL_R = crate::FieldReader;
#[doc = "Field `MCU_SEL` writer - "]
pub type MCU_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FILTER_EN` reader - "]
pub type FILTER_EN_R = crate::BitReader;
#[doc = "Field `FILTER_EN` writer - "]
pub type FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYS_EN` reader - "]
pub type HYS_EN_R = crate::BitReader;
#[doc = "Field `HYS_EN` writer - "]
pub type HYS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYS_SEL` reader - "]
pub type HYS_SEL_R = crate::BitReader;
#[doc = "Field `HYS_SEL` writer - "]
pub type HYS_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mcu_oe(&self) -> MCU_OE_R {
        MCU_OE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slp_sel(&self) -> SLP_SEL_R {
        SLP_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn mcu_wpd(&self) -> MCU_WPD_R {
        MCU_WPD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn mcu_wpu(&self) -> MCU_WPU_R {
        MCU_WPU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn mcu_ie(&self) -> MCU_IE_R {
        MCU_IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn mcu_drv(&self) -> MCU_DRV_R {
        MCU_DRV_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn fun_wpd(&self) -> FUN_WPD_R {
        FUN_WPD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn fun_wpu(&self) -> FUN_WPU_R {
        FUN_WPU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn fun_ie(&self) -> FUN_IE_R {
        FUN_IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn fun_drv(&self) -> FUN_DRV_R {
        FUN_DRV_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn mcu_sel(&self) -> MCU_SEL_R {
        MCU_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn filter_en(&self) -> FILTER_EN_R {
        FILTER_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn hys_en(&self) -> HYS_EN_R {
        HYS_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn hys_sel(&self) -> HYS_SEL_R {
        HYS_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO")
            .field("mcu_oe", &self.mcu_oe())
            .field("slp_sel", &self.slp_sel())
            .field("mcu_wpd", &self.mcu_wpd())
            .field("mcu_wpu", &self.mcu_wpu())
            .field("mcu_ie", &self.mcu_ie())
            .field("mcu_drv", &self.mcu_drv())
            .field("fun_wpd", &self.fun_wpd())
            .field("fun_wpu", &self.fun_wpu())
            .field("fun_ie", &self.fun_ie())
            .field("fun_drv", &self.fun_drv())
            .field("mcu_sel", &self.mcu_sel())
            .field("filter_en", &self.filter_en())
            .field("hys_en", &self.hys_en())
            .field("hys_sel", &self.hys_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn mcu_oe(&mut self) -> MCU_OE_W<'_, GPIO_SPEC> {
        MCU_OE_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slp_sel(&mut self) -> SLP_SEL_W<'_, GPIO_SPEC> {
        SLP_SEL_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn mcu_wpd(&mut self) -> MCU_WPD_W<'_, GPIO_SPEC> {
        MCU_WPD_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn mcu_wpu(&mut self) -> MCU_WPU_W<'_, GPIO_SPEC> {
        MCU_WPU_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn mcu_ie(&mut self) -> MCU_IE_W<'_, GPIO_SPEC> {
        MCU_IE_W::new(self, 4)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn mcu_drv(&mut self) -> MCU_DRV_W<'_, GPIO_SPEC> {
        MCU_DRV_W::new(self, 5)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn fun_wpd(&mut self) -> FUN_WPD_W<'_, GPIO_SPEC> {
        FUN_WPD_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn fun_wpu(&mut self) -> FUN_WPU_W<'_, GPIO_SPEC> {
        FUN_WPU_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn fun_ie(&mut self) -> FUN_IE_W<'_, GPIO_SPEC> {
        FUN_IE_W::new(self, 9)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn fun_drv(&mut self) -> FUN_DRV_W<'_, GPIO_SPEC> {
        FUN_DRV_W::new(self, 10)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn mcu_sel(&mut self) -> MCU_SEL_W<'_, GPIO_SPEC> {
        MCU_SEL_W::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn filter_en(&mut self) -> FILTER_EN_W<'_, GPIO_SPEC> {
        FILTER_EN_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn hys_en(&mut self) -> HYS_EN_W<'_, GPIO_SPEC> {
        HYS_EN_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn hys_sel(&mut self) -> HYS_SEL_W<'_, GPIO_SPEC> {
        HYS_SEL_W::new(self, 17)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_SPEC;
impl crate::RegisterSpec for GPIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio::R`](R) reader structure"]
impl crate::Readable for GPIO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio::W`](W) writer structure"]
impl crate::Writable for GPIO_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO%s to value 0x1800"]
impl crate::Resettable for GPIO_SPEC {
    const RESET_VALUE: u32 = 0x1800;
}
