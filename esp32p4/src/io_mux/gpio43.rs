#[doc = "Register `gpio43` reader"]
pub type R = crate::R<GPIO43_SPEC>;
#[doc = "Register `gpio43` writer"]
pub type W = crate::W<GPIO43_SPEC>;
#[doc = "Field `GPIO43_MCU_OE` reader - output enable on sleep mode"]
pub type GPIO43_MCU_OE_R = crate::BitReader;
#[doc = "Field `GPIO43_MCU_OE` writer - output enable on sleep mode"]
pub type GPIO43_MCU_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO43_SLP_SEL` reader - io sleep mode enable. set 1 to enable sleep mode."]
pub type GPIO43_SLP_SEL_R = crate::BitReader;
#[doc = "Field `GPIO43_SLP_SEL` writer - io sleep mode enable. set 1 to enable sleep mode."]
pub type GPIO43_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO43_MCU_WPD` reader - pull-down enable on sleep mode"]
pub type GPIO43_MCU_WPD_R = crate::BitReader;
#[doc = "Field `GPIO43_MCU_WPD` writer - pull-down enable on sleep mode"]
pub type GPIO43_MCU_WPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO43_MCU_WPU` reader - pull-up enable on sleep mode"]
pub type GPIO43_MCU_WPU_R = crate::BitReader;
#[doc = "Field `GPIO43_MCU_WPU` writer - pull-up enable on sleep mode"]
pub type GPIO43_MCU_WPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO43_MCU_IE` reader - input enable on sleep mode"]
pub type GPIO43_MCU_IE_R = crate::BitReader;
#[doc = "Field `GPIO43_MCU_IE` writer - input enable on sleep mode"]
pub type GPIO43_MCU_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO43_MCU_DRV` reader - select drive strenth on sleep mode"]
pub type GPIO43_MCU_DRV_R = crate::FieldReader;
#[doc = "Field `GPIO43_MCU_DRV` writer - select drive strenth on sleep mode"]
pub type GPIO43_MCU_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GPIO43_FUN_WPD` reader - pull-down enable"]
pub type GPIO43_FUN_WPD_R = crate::BitReader;
#[doc = "Field `GPIO43_FUN_WPD` writer - pull-down enable"]
pub type GPIO43_FUN_WPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO43_FUN_WPU` reader - pull-up enable"]
pub type GPIO43_FUN_WPU_R = crate::BitReader;
#[doc = "Field `GPIO43_FUN_WPU` writer - pull-up enable"]
pub type GPIO43_FUN_WPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO43_FUN_IE` reader - input enable"]
pub type GPIO43_FUN_IE_R = crate::BitReader;
#[doc = "Field `GPIO43_FUN_IE` writer - input enable"]
pub type GPIO43_FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO43_FUN_DRV` reader - select drive strenth, 0:5mA, 1:10mA, 2:20mA, 3:40mA"]
pub type GPIO43_FUN_DRV_R = crate::FieldReader;
#[doc = "Field `GPIO43_FUN_DRV` writer - select drive strenth, 0:5mA, 1:10mA, 2:20mA, 3:40mA"]
pub type GPIO43_FUN_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GPIO43_MCU_SEL` reader - 0:select function0, 1:select function1 ..."]
pub type GPIO43_MCU_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO43_MCU_SEL` writer - 0:select function0, 1:select function1 ..."]
pub type GPIO43_MCU_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO43_FILTER_EN` reader - input filter enable"]
pub type GPIO43_FILTER_EN_R = crate::BitReader;
#[doc = "Field `GPIO43_FILTER_EN` writer - input filter enable"]
pub type GPIO43_FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - output enable on sleep mode"]
    #[inline(always)]
    pub fn gpio43_mcu_oe(&self) -> GPIO43_MCU_OE_R {
        GPIO43_MCU_OE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - io sleep mode enable. set 1 to enable sleep mode."]
    #[inline(always)]
    pub fn gpio43_slp_sel(&self) -> GPIO43_SLP_SEL_R {
        GPIO43_SLP_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - pull-down enable on sleep mode"]
    #[inline(always)]
    pub fn gpio43_mcu_wpd(&self) -> GPIO43_MCU_WPD_R {
        GPIO43_MCU_WPD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - pull-up enable on sleep mode"]
    #[inline(always)]
    pub fn gpio43_mcu_wpu(&self) -> GPIO43_MCU_WPU_R {
        GPIO43_MCU_WPU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - input enable on sleep mode"]
    #[inline(always)]
    pub fn gpio43_mcu_ie(&self) -> GPIO43_MCU_IE_R {
        GPIO43_MCU_IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - select drive strenth on sleep mode"]
    #[inline(always)]
    pub fn gpio43_mcu_drv(&self) -> GPIO43_MCU_DRV_R {
        GPIO43_MCU_DRV_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - pull-down enable"]
    #[inline(always)]
    pub fn gpio43_fun_wpd(&self) -> GPIO43_FUN_WPD_R {
        GPIO43_FUN_WPD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - pull-up enable"]
    #[inline(always)]
    pub fn gpio43_fun_wpu(&self) -> GPIO43_FUN_WPU_R {
        GPIO43_FUN_WPU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - input enable"]
    #[inline(always)]
    pub fn gpio43_fun_ie(&self) -> GPIO43_FUN_IE_R {
        GPIO43_FUN_IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - select drive strenth, 0:5mA, 1:10mA, 2:20mA, 3:40mA"]
    #[inline(always)]
    pub fn gpio43_fun_drv(&self) -> GPIO43_FUN_DRV_R {
        GPIO43_FUN_DRV_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - 0:select function0, 1:select function1 ..."]
    #[inline(always)]
    pub fn gpio43_mcu_sel(&self) -> GPIO43_MCU_SEL_R {
        GPIO43_MCU_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - input filter enable"]
    #[inline(always)]
    pub fn gpio43_filter_en(&self) -> GPIO43_FILTER_EN_R {
        GPIO43_FILTER_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("gpio43")
            .field(
                "gpio43_mcu_oe",
                &format_args!("{}", self.gpio43_mcu_oe().bit()),
            )
            .field(
                "gpio43_slp_sel",
                &format_args!("{}", self.gpio43_slp_sel().bit()),
            )
            .field(
                "gpio43_mcu_wpd",
                &format_args!("{}", self.gpio43_mcu_wpd().bit()),
            )
            .field(
                "gpio43_mcu_wpu",
                &format_args!("{}", self.gpio43_mcu_wpu().bit()),
            )
            .field(
                "gpio43_mcu_ie",
                &format_args!("{}", self.gpio43_mcu_ie().bit()),
            )
            .field(
                "gpio43_mcu_drv",
                &format_args!("{}", self.gpio43_mcu_drv().bits()),
            )
            .field(
                "gpio43_fun_wpd",
                &format_args!("{}", self.gpio43_fun_wpd().bit()),
            )
            .field(
                "gpio43_fun_wpu",
                &format_args!("{}", self.gpio43_fun_wpu().bit()),
            )
            .field(
                "gpio43_fun_ie",
                &format_args!("{}", self.gpio43_fun_ie().bit()),
            )
            .field(
                "gpio43_fun_drv",
                &format_args!("{}", self.gpio43_fun_drv().bits()),
            )
            .field(
                "gpio43_mcu_sel",
                &format_args!("{}", self.gpio43_mcu_sel().bits()),
            )
            .field(
                "gpio43_filter_en",
                &format_args!("{}", self.gpio43_filter_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO43_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - output enable on sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpio43_mcu_oe(&mut self) -> GPIO43_MCU_OE_W<GPIO43_SPEC> {
        GPIO43_MCU_OE_W::new(self, 0)
    }
    #[doc = "Bit 1 - io sleep mode enable. set 1 to enable sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn gpio43_slp_sel(&mut self) -> GPIO43_SLP_SEL_W<GPIO43_SPEC> {
        GPIO43_SLP_SEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - pull-down enable on sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpio43_mcu_wpd(&mut self) -> GPIO43_MCU_WPD_W<GPIO43_SPEC> {
        GPIO43_MCU_WPD_W::new(self, 2)
    }
    #[doc = "Bit 3 - pull-up enable on sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpio43_mcu_wpu(&mut self) -> GPIO43_MCU_WPU_W<GPIO43_SPEC> {
        GPIO43_MCU_WPU_W::new(self, 3)
    }
    #[doc = "Bit 4 - input enable on sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpio43_mcu_ie(&mut self) -> GPIO43_MCU_IE_W<GPIO43_SPEC> {
        GPIO43_MCU_IE_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - select drive strenth on sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpio43_mcu_drv(&mut self) -> GPIO43_MCU_DRV_W<GPIO43_SPEC> {
        GPIO43_MCU_DRV_W::new(self, 5)
    }
    #[doc = "Bit 7 - pull-down enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio43_fun_wpd(&mut self) -> GPIO43_FUN_WPD_W<GPIO43_SPEC> {
        GPIO43_FUN_WPD_W::new(self, 7)
    }
    #[doc = "Bit 8 - pull-up enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio43_fun_wpu(&mut self) -> GPIO43_FUN_WPU_W<GPIO43_SPEC> {
        GPIO43_FUN_WPU_W::new(self, 8)
    }
    #[doc = "Bit 9 - input enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio43_fun_ie(&mut self) -> GPIO43_FUN_IE_W<GPIO43_SPEC> {
        GPIO43_FUN_IE_W::new(self, 9)
    }
    #[doc = "Bits 10:11 - select drive strenth, 0:5mA, 1:10mA, 2:20mA, 3:40mA"]
    #[inline(always)]
    #[must_use]
    pub fn gpio43_fun_drv(&mut self) -> GPIO43_FUN_DRV_W<GPIO43_SPEC> {
        GPIO43_FUN_DRV_W::new(self, 10)
    }
    #[doc = "Bits 12:14 - 0:select function0, 1:select function1 ..."]
    #[inline(always)]
    #[must_use]
    pub fn gpio43_mcu_sel(&mut self) -> GPIO43_MCU_SEL_W<GPIO43_SPEC> {
        GPIO43_MCU_SEL_W::new(self, 12)
    }
    #[doc = "Bit 15 - input filter enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio43_filter_en(&mut self) -> GPIO43_FILTER_EN_W<GPIO43_SPEC> {
        GPIO43_FILTER_EN_W::new(self, 15)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "iomux control register for gpio43\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio43::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio43::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO43_SPEC;
impl crate::RegisterSpec for GPIO43_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio43::R`](R) reader structure"]
impl crate::Readable for GPIO43_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio43::W`](W) writer structure"]
impl crate::Writable for GPIO43_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpio43 to value 0x0800"]
impl crate::Resettable for GPIO43_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800;
}
