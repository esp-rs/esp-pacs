#[doc = "Register `GPIO5` reader"]
pub type R = crate::R<GPIO5_SPEC>;
#[doc = "Register `GPIO5` writer"]
pub type W = crate::W<GPIO5_SPEC>;
#[doc = "Field `LP_GPIO5_MCU_OE` reader - need des"]
pub type LP_GPIO5_MCU_OE_R = crate::BitReader;
#[doc = "Field `LP_GPIO5_MCU_OE` writer - need des"]
pub type LP_GPIO5_MCU_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_GPIO5_SLP_SEL` reader - need des"]
pub type LP_GPIO5_SLP_SEL_R = crate::BitReader;
#[doc = "Field `LP_GPIO5_SLP_SEL` writer - need des"]
pub type LP_GPIO5_SLP_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_GPIO5_MCU_WPD` reader - need des"]
pub type LP_GPIO5_MCU_WPD_R = crate::BitReader;
#[doc = "Field `LP_GPIO5_MCU_WPD` writer - need des"]
pub type LP_GPIO5_MCU_WPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_GPIO5_MCU_WPU` reader - need des"]
pub type LP_GPIO5_MCU_WPU_R = crate::BitReader;
#[doc = "Field `LP_GPIO5_MCU_WPU` writer - need des"]
pub type LP_GPIO5_MCU_WPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_GPIO5_MCU_IE` reader - need des"]
pub type LP_GPIO5_MCU_IE_R = crate::BitReader;
#[doc = "Field `LP_GPIO5_MCU_IE` writer - need des"]
pub type LP_GPIO5_MCU_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_GPIO5_MCU_DRV` reader - need des"]
pub type LP_GPIO5_MCU_DRV_R = crate::FieldReader;
#[doc = "Field `LP_GPIO5_MCU_DRV` writer - need des"]
pub type LP_GPIO5_MCU_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_GPIO5_FUN_WPD` reader - need des"]
pub type LP_GPIO5_FUN_WPD_R = crate::BitReader;
#[doc = "Field `LP_GPIO5_FUN_WPD` writer - need des"]
pub type LP_GPIO5_FUN_WPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_GPIO5_FUN_WPU` reader - need des"]
pub type LP_GPIO5_FUN_WPU_R = crate::BitReader;
#[doc = "Field `LP_GPIO5_FUN_WPU` writer - need des"]
pub type LP_GPIO5_FUN_WPU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_GPIO5_FUN_IE` reader - need des"]
pub type LP_GPIO5_FUN_IE_R = crate::BitReader;
#[doc = "Field `LP_GPIO5_FUN_IE` writer - need des"]
pub type LP_GPIO5_FUN_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_GPIO5_FUN_DRV` reader - need des"]
pub type LP_GPIO5_FUN_DRV_R = crate::FieldReader;
#[doc = "Field `LP_GPIO5_FUN_DRV` writer - need des"]
pub type LP_GPIO5_FUN_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_GPIO5_MCU_SEL` reader - need des"]
pub type LP_GPIO5_MCU_SEL_R = crate::FieldReader;
#[doc = "Field `LP_GPIO5_MCU_SEL` writer - need des"]
pub type LP_GPIO5_MCU_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - need des"]
    #[inline(always)]
    pub fn lp_gpio5_mcu_oe(&self) -> LP_GPIO5_MCU_OE_R {
        LP_GPIO5_MCU_OE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need des"]
    #[inline(always)]
    pub fn lp_gpio5_slp_sel(&self) -> LP_GPIO5_SLP_SEL_R {
        LP_GPIO5_SLP_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need des"]
    #[inline(always)]
    pub fn lp_gpio5_mcu_wpd(&self) -> LP_GPIO5_MCU_WPD_R {
        LP_GPIO5_MCU_WPD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need des"]
    #[inline(always)]
    pub fn lp_gpio5_mcu_wpu(&self) -> LP_GPIO5_MCU_WPU_R {
        LP_GPIO5_MCU_WPU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need des"]
    #[inline(always)]
    pub fn lp_gpio5_mcu_ie(&self) -> LP_GPIO5_MCU_IE_R {
        LP_GPIO5_MCU_IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - need des"]
    #[inline(always)]
    pub fn lp_gpio5_mcu_drv(&self) -> LP_GPIO5_MCU_DRV_R {
        LP_GPIO5_MCU_DRV_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - need des"]
    #[inline(always)]
    pub fn lp_gpio5_fun_wpd(&self) -> LP_GPIO5_FUN_WPD_R {
        LP_GPIO5_FUN_WPD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - need des"]
    #[inline(always)]
    pub fn lp_gpio5_fun_wpu(&self) -> LP_GPIO5_FUN_WPU_R {
        LP_GPIO5_FUN_WPU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - need des"]
    #[inline(always)]
    pub fn lp_gpio5_fun_ie(&self) -> LP_GPIO5_FUN_IE_R {
        LP_GPIO5_FUN_IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - need des"]
    #[inline(always)]
    pub fn lp_gpio5_fun_drv(&self) -> LP_GPIO5_FUN_DRV_R {
        LP_GPIO5_FUN_DRV_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - need des"]
    #[inline(always)]
    pub fn lp_gpio5_mcu_sel(&self) -> LP_GPIO5_MCU_SEL_R {
        LP_GPIO5_MCU_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO5")
            .field(
                "lp_gpio5_mcu_oe",
                &format_args!("{}", self.lp_gpio5_mcu_oe().bit()),
            )
            .field(
                "lp_gpio5_slp_sel",
                &format_args!("{}", self.lp_gpio5_slp_sel().bit()),
            )
            .field(
                "lp_gpio5_mcu_wpd",
                &format_args!("{}", self.lp_gpio5_mcu_wpd().bit()),
            )
            .field(
                "lp_gpio5_mcu_wpu",
                &format_args!("{}", self.lp_gpio5_mcu_wpu().bit()),
            )
            .field(
                "lp_gpio5_mcu_ie",
                &format_args!("{}", self.lp_gpio5_mcu_ie().bit()),
            )
            .field(
                "lp_gpio5_mcu_drv",
                &format_args!("{}", self.lp_gpio5_mcu_drv().bits()),
            )
            .field(
                "lp_gpio5_fun_wpd",
                &format_args!("{}", self.lp_gpio5_fun_wpd().bit()),
            )
            .field(
                "lp_gpio5_fun_wpu",
                &format_args!("{}", self.lp_gpio5_fun_wpu().bit()),
            )
            .field(
                "lp_gpio5_fun_ie",
                &format_args!("{}", self.lp_gpio5_fun_ie().bit()),
            )
            .field(
                "lp_gpio5_fun_drv",
                &format_args!("{}", self.lp_gpio5_fun_drv().bits()),
            )
            .field(
                "lp_gpio5_mcu_sel",
                &format_args!("{}", self.lp_gpio5_mcu_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio5_mcu_oe(&mut self) -> LP_GPIO5_MCU_OE_W<GPIO5_SPEC> {
        LP_GPIO5_MCU_OE_W::new(self, 0)
    }
    #[doc = "Bit 1 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio5_slp_sel(&mut self) -> LP_GPIO5_SLP_SEL_W<GPIO5_SPEC> {
        LP_GPIO5_SLP_SEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio5_mcu_wpd(&mut self) -> LP_GPIO5_MCU_WPD_W<GPIO5_SPEC> {
        LP_GPIO5_MCU_WPD_W::new(self, 2)
    }
    #[doc = "Bit 3 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio5_mcu_wpu(&mut self) -> LP_GPIO5_MCU_WPU_W<GPIO5_SPEC> {
        LP_GPIO5_MCU_WPU_W::new(self, 3)
    }
    #[doc = "Bit 4 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio5_mcu_ie(&mut self) -> LP_GPIO5_MCU_IE_W<GPIO5_SPEC> {
        LP_GPIO5_MCU_IE_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio5_mcu_drv(&mut self) -> LP_GPIO5_MCU_DRV_W<GPIO5_SPEC> {
        LP_GPIO5_MCU_DRV_W::new(self, 5)
    }
    #[doc = "Bit 7 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio5_fun_wpd(&mut self) -> LP_GPIO5_FUN_WPD_W<GPIO5_SPEC> {
        LP_GPIO5_FUN_WPD_W::new(self, 7)
    }
    #[doc = "Bit 8 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio5_fun_wpu(&mut self) -> LP_GPIO5_FUN_WPU_W<GPIO5_SPEC> {
        LP_GPIO5_FUN_WPU_W::new(self, 8)
    }
    #[doc = "Bit 9 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio5_fun_ie(&mut self) -> LP_GPIO5_FUN_IE_W<GPIO5_SPEC> {
        LP_GPIO5_FUN_IE_W::new(self, 9)
    }
    #[doc = "Bits 10:11 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio5_fun_drv(&mut self) -> LP_GPIO5_FUN_DRV_W<GPIO5_SPEC> {
        LP_GPIO5_FUN_DRV_W::new(self, 10)
    }
    #[doc = "Bits 12:14 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio5_mcu_sel(&mut self) -> LP_GPIO5_MCU_SEL_W<GPIO5_SPEC> {
        LP_GPIO5_MCU_SEL_W::new(self, 12)
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
#[doc = "need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO5_SPEC;
impl crate::RegisterSpec for GPIO5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio5::R`](R) reader structure"]
impl crate::Readable for GPIO5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio5::W`](W) writer structure"]
impl crate::Writable for GPIO5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO5 to value 0"]
impl crate::Resettable for GPIO5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
