#[doc = "Register `gpio33` reader"]
pub type R = crate::R<GPIO33_SPEC>;
#[doc = "Register `gpio33` writer"]
pub type W = crate::W<GPIO33_SPEC>;
#[doc = "Field `GPIO33_MCU_OE` reader - output enable on sleep mode"]
pub type GPIO33_MCU_OE_R = crate::BitReader;
#[doc = "Field `GPIO33_MCU_OE` writer - output enable on sleep mode"]
pub type GPIO33_MCU_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO33_SLP_SEL` reader - io sleep mode enable. set 1 to enable sleep mode."]
pub type GPIO33_SLP_SEL_R = crate::BitReader;
#[doc = "Field `GPIO33_SLP_SEL` writer - io sleep mode enable. set 1 to enable sleep mode."]
pub type GPIO33_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO33_MCU_WPD` reader - pull-down enable on sleep mode"]
pub type GPIO33_MCU_WPD_R = crate::BitReader;
#[doc = "Field `GPIO33_MCU_WPD` writer - pull-down enable on sleep mode"]
pub type GPIO33_MCU_WPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO33_MCU_WPU` reader - pull-up enable on sleep mode"]
pub type GPIO33_MCU_WPU_R = crate::BitReader;
#[doc = "Field `GPIO33_MCU_WPU` writer - pull-up enable on sleep mode"]
pub type GPIO33_MCU_WPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO33_MCU_IE` reader - input enable on sleep mode"]
pub type GPIO33_MCU_IE_R = crate::BitReader;
#[doc = "Field `GPIO33_MCU_IE` writer - input enable on sleep mode"]
pub type GPIO33_MCU_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO33_MCU_DRV` reader - select drive strenth on sleep mode"]
pub type GPIO33_MCU_DRV_R = crate::FieldReader;
#[doc = "Field `GPIO33_MCU_DRV` writer - select drive strenth on sleep mode"]
pub type GPIO33_MCU_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GPIO33_FUN_WPD` reader - pull-down enable"]
pub type GPIO33_FUN_WPD_R = crate::BitReader;
#[doc = "Field `GPIO33_FUN_WPD` writer - pull-down enable"]
pub type GPIO33_FUN_WPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO33_FUN_WPU` reader - pull-up enable"]
pub type GPIO33_FUN_WPU_R = crate::BitReader;
#[doc = "Field `GPIO33_FUN_WPU` writer - pull-up enable"]
pub type GPIO33_FUN_WPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO33_FUN_IE` reader - input enable"]
pub type GPIO33_FUN_IE_R = crate::BitReader;
#[doc = "Field `GPIO33_FUN_IE` writer - input enable"]
pub type GPIO33_FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO33_FUN_DRV` reader - select drive strenth, 0:5mA, 1:10mA, 2:20mA, 3:40mA"]
pub type GPIO33_FUN_DRV_R = crate::FieldReader;
#[doc = "Field `GPIO33_FUN_DRV` writer - select drive strenth, 0:5mA, 1:10mA, 2:20mA, 3:40mA"]
pub type GPIO33_FUN_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GPIO33_MCU_SEL` reader - 0:select function0, 1:select function1 ..."]
pub type GPIO33_MCU_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO33_MCU_SEL` writer - 0:select function0, 1:select function1 ..."]
pub type GPIO33_MCU_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO33_FILTER_EN` reader - input filter enable"]
pub type GPIO33_FILTER_EN_R = crate::BitReader;
#[doc = "Field `GPIO33_FILTER_EN` writer - input filter enable"]
pub type GPIO33_FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO33_RUE_I3C` reader - NA"]
pub type GPIO33_RUE_I3C_R = crate::BitReader;
#[doc = "Field `GPIO33_RUE_I3C` writer - NA"]
pub type GPIO33_RUE_I3C_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO33_RU_I3C` reader - NA"]
pub type GPIO33_RU_I3C_R = crate::FieldReader;
#[doc = "Field `GPIO33_RU_I3C` writer - NA"]
pub type GPIO33_RU_I3C_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GPIO33_RUE_SEL_I3C` reader - NA"]
pub type GPIO33_RUE_SEL_I3C_R = crate::BitReader;
#[doc = "Field `GPIO33_RUE_SEL_I3C` writer - NA"]
pub type GPIO33_RUE_SEL_I3C_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - output enable on sleep mode"]
    #[inline(always)]
    pub fn gpio33_mcu_oe(&self) -> GPIO33_MCU_OE_R {
        GPIO33_MCU_OE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - io sleep mode enable. set 1 to enable sleep mode."]
    #[inline(always)]
    pub fn gpio33_slp_sel(&self) -> GPIO33_SLP_SEL_R {
        GPIO33_SLP_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - pull-down enable on sleep mode"]
    #[inline(always)]
    pub fn gpio33_mcu_wpd(&self) -> GPIO33_MCU_WPD_R {
        GPIO33_MCU_WPD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - pull-up enable on sleep mode"]
    #[inline(always)]
    pub fn gpio33_mcu_wpu(&self) -> GPIO33_MCU_WPU_R {
        GPIO33_MCU_WPU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - input enable on sleep mode"]
    #[inline(always)]
    pub fn gpio33_mcu_ie(&self) -> GPIO33_MCU_IE_R {
        GPIO33_MCU_IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - select drive strenth on sleep mode"]
    #[inline(always)]
    pub fn gpio33_mcu_drv(&self) -> GPIO33_MCU_DRV_R {
        GPIO33_MCU_DRV_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - pull-down enable"]
    #[inline(always)]
    pub fn gpio33_fun_wpd(&self) -> GPIO33_FUN_WPD_R {
        GPIO33_FUN_WPD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - pull-up enable"]
    #[inline(always)]
    pub fn gpio33_fun_wpu(&self) -> GPIO33_FUN_WPU_R {
        GPIO33_FUN_WPU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - input enable"]
    #[inline(always)]
    pub fn gpio33_fun_ie(&self) -> GPIO33_FUN_IE_R {
        GPIO33_FUN_IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - select drive strenth, 0:5mA, 1:10mA, 2:20mA, 3:40mA"]
    #[inline(always)]
    pub fn gpio33_fun_drv(&self) -> GPIO33_FUN_DRV_R {
        GPIO33_FUN_DRV_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - 0:select function0, 1:select function1 ..."]
    #[inline(always)]
    pub fn gpio33_mcu_sel(&self) -> GPIO33_MCU_SEL_R {
        GPIO33_MCU_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - input filter enable"]
    #[inline(always)]
    pub fn gpio33_filter_en(&self) -> GPIO33_FILTER_EN_R {
        GPIO33_FILTER_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn gpio33_rue_i3c(&self) -> GPIO33_RUE_I3C_R {
        GPIO33_RUE_I3C_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - NA"]
    #[inline(always)]
    pub fn gpio33_ru_i3c(&self) -> GPIO33_RU_I3C_R {
        GPIO33_RU_I3C_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    pub fn gpio33_rue_sel_i3c(&self) -> GPIO33_RUE_SEL_I3C_R {
        GPIO33_RUE_SEL_I3C_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("gpio33")
            .field(
                "gpio33_mcu_oe",
                &format_args!("{}", self.gpio33_mcu_oe().bit()),
            )
            .field(
                "gpio33_slp_sel",
                &format_args!("{}", self.gpio33_slp_sel().bit()),
            )
            .field(
                "gpio33_mcu_wpd",
                &format_args!("{}", self.gpio33_mcu_wpd().bit()),
            )
            .field(
                "gpio33_mcu_wpu",
                &format_args!("{}", self.gpio33_mcu_wpu().bit()),
            )
            .field(
                "gpio33_mcu_ie",
                &format_args!("{}", self.gpio33_mcu_ie().bit()),
            )
            .field(
                "gpio33_mcu_drv",
                &format_args!("{}", self.gpio33_mcu_drv().bits()),
            )
            .field(
                "gpio33_fun_wpd",
                &format_args!("{}", self.gpio33_fun_wpd().bit()),
            )
            .field(
                "gpio33_fun_wpu",
                &format_args!("{}", self.gpio33_fun_wpu().bit()),
            )
            .field(
                "gpio33_fun_ie",
                &format_args!("{}", self.gpio33_fun_ie().bit()),
            )
            .field(
                "gpio33_fun_drv",
                &format_args!("{}", self.gpio33_fun_drv().bits()),
            )
            .field(
                "gpio33_mcu_sel",
                &format_args!("{}", self.gpio33_mcu_sel().bits()),
            )
            .field(
                "gpio33_filter_en",
                &format_args!("{}", self.gpio33_filter_en().bit()),
            )
            .field(
                "gpio33_rue_i3c",
                &format_args!("{}", self.gpio33_rue_i3c().bit()),
            )
            .field(
                "gpio33_ru_i3c",
                &format_args!("{}", self.gpio33_ru_i3c().bits()),
            )
            .field(
                "gpio33_rue_sel_i3c",
                &format_args!("{}", self.gpio33_rue_sel_i3c().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO33_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - output enable on sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpio33_mcu_oe(&mut self) -> GPIO33_MCU_OE_W<GPIO33_SPEC> {
        GPIO33_MCU_OE_W::new(self, 0)
    }
    #[doc = "Bit 1 - io sleep mode enable. set 1 to enable sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn gpio33_slp_sel(&mut self) -> GPIO33_SLP_SEL_W<GPIO33_SPEC> {
        GPIO33_SLP_SEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - pull-down enable on sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpio33_mcu_wpd(&mut self) -> GPIO33_MCU_WPD_W<GPIO33_SPEC> {
        GPIO33_MCU_WPD_W::new(self, 2)
    }
    #[doc = "Bit 3 - pull-up enable on sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpio33_mcu_wpu(&mut self) -> GPIO33_MCU_WPU_W<GPIO33_SPEC> {
        GPIO33_MCU_WPU_W::new(self, 3)
    }
    #[doc = "Bit 4 - input enable on sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpio33_mcu_ie(&mut self) -> GPIO33_MCU_IE_W<GPIO33_SPEC> {
        GPIO33_MCU_IE_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - select drive strenth on sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn gpio33_mcu_drv(&mut self) -> GPIO33_MCU_DRV_W<GPIO33_SPEC> {
        GPIO33_MCU_DRV_W::new(self, 5)
    }
    #[doc = "Bit 7 - pull-down enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio33_fun_wpd(&mut self) -> GPIO33_FUN_WPD_W<GPIO33_SPEC> {
        GPIO33_FUN_WPD_W::new(self, 7)
    }
    #[doc = "Bit 8 - pull-up enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio33_fun_wpu(&mut self) -> GPIO33_FUN_WPU_W<GPIO33_SPEC> {
        GPIO33_FUN_WPU_W::new(self, 8)
    }
    #[doc = "Bit 9 - input enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio33_fun_ie(&mut self) -> GPIO33_FUN_IE_W<GPIO33_SPEC> {
        GPIO33_FUN_IE_W::new(self, 9)
    }
    #[doc = "Bits 10:11 - select drive strenth, 0:5mA, 1:10mA, 2:20mA, 3:40mA"]
    #[inline(always)]
    #[must_use]
    pub fn gpio33_fun_drv(&mut self) -> GPIO33_FUN_DRV_W<GPIO33_SPEC> {
        GPIO33_FUN_DRV_W::new(self, 10)
    }
    #[doc = "Bits 12:14 - 0:select function0, 1:select function1 ..."]
    #[inline(always)]
    #[must_use]
    pub fn gpio33_mcu_sel(&mut self) -> GPIO33_MCU_SEL_W<GPIO33_SPEC> {
        GPIO33_MCU_SEL_W::new(self, 12)
    }
    #[doc = "Bit 15 - input filter enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio33_filter_en(&mut self) -> GPIO33_FILTER_EN_W<GPIO33_SPEC> {
        GPIO33_FILTER_EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn gpio33_rue_i3c(&mut self) -> GPIO33_RUE_I3C_W<GPIO33_SPEC> {
        GPIO33_RUE_I3C_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn gpio33_ru_i3c(&mut self) -> GPIO33_RU_I3C_W<GPIO33_SPEC> {
        GPIO33_RU_I3C_W::new(self, 17)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn gpio33_rue_sel_i3c(&mut self) -> GPIO33_RUE_SEL_I3C_W<GPIO33_SPEC> {
        GPIO33_RUE_SEL_I3C_W::new(self, 19)
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
#[doc = "iomux control register for gpio33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio33::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio33::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO33_SPEC;
impl crate::RegisterSpec for GPIO33_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio33::R`](R) reader structure"]
impl crate::Readable for GPIO33_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio33::W`](W) writer structure"]
impl crate::Writable for GPIO33_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets gpio33 to value 0x0800"]
impl crate::Resettable for GPIO33_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800;
}
