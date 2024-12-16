#[doc = "Register `CNTL_DBG_MAP` reader"]
pub type R = crate::R<CNTL_DBG_MAP_SPEC>;
#[doc = "Register `CNTL_DBG_MAP` writer"]
pub type W = crate::W<CNTL_DBG_MAP_SPEC>;
#[doc = "Field `GPIO_PIN5_MUX_SEL` reader - Need add desc"]
pub type GPIO_PIN5_MUX_SEL_R = crate::BitReader;
#[doc = "Field `GPIO_PIN5_MUX_SEL` writer - Need add desc"]
pub type GPIO_PIN5_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO_PIN4_MUX_SEL` reader - Need add desc"]
pub type GPIO_PIN4_MUX_SEL_R = crate::BitReader;
#[doc = "Field `GPIO_PIN4_MUX_SEL` writer - Need add desc"]
pub type GPIO_PIN4_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO_PIN3_MUX_SEL` reader - Need add desc"]
pub type GPIO_PIN3_MUX_SEL_R = crate::BitReader;
#[doc = "Field `GPIO_PIN3_MUX_SEL` writer - Need add desc"]
pub type GPIO_PIN3_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO_PIN2_MUX_SEL` reader - Need add desc"]
pub type GPIO_PIN2_MUX_SEL_R = crate::BitReader;
#[doc = "Field `GPIO_PIN2_MUX_SEL` writer - Need add desc"]
pub type GPIO_PIN2_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO_PIN1_MUX_SEL` reader - Need add desc"]
pub type GPIO_PIN1_MUX_SEL_R = crate::BitReader;
#[doc = "Field `GPIO_PIN1_MUX_SEL` writer - Need add desc"]
pub type GPIO_PIN1_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO_PIN0_MUX_SEL` reader - Need add desc"]
pub type GPIO_PIN0_MUX_SEL_R = crate::BitReader;
#[doc = "Field `GPIO_PIN0_MUX_SEL` writer - Need add desc"]
pub type GPIO_PIN0_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO_PIN5_FUN_SEL` reader - Need add desc"]
pub type GPIO_PIN5_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO_PIN5_FUN_SEL` writer - Need add desc"]
pub type GPIO_PIN5_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GPIO_PIN4_FUN_SEL` reader - Need add desc"]
pub type GPIO_PIN4_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO_PIN4_FUN_SEL` writer - Need add desc"]
pub type GPIO_PIN4_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GPIO_PIN3_FUN_SEL` reader - Need add desc"]
pub type GPIO_PIN3_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO_PIN3_FUN_SEL` writer - Need add desc"]
pub type GPIO_PIN3_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GPIO_PIN2_FUN_SEL` reader - Need add desc"]
pub type GPIO_PIN2_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO_PIN2_FUN_SEL` writer - Need add desc"]
pub type GPIO_PIN2_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GPIO_PIN1_FUN_SEL` reader - Need add desc"]
pub type GPIO_PIN1_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO_PIN1_FUN_SEL` writer - Need add desc"]
pub type GPIO_PIN1_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GPIO_PIN0_FUN_SEL` reader - Need add desc"]
pub type GPIO_PIN0_FUN_SEL_R = crate::FieldReader;
#[doc = "Field `GPIO_PIN0_FUN_SEL` writer - Need add desc"]
pub type GPIO_PIN0_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 2 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin5_mux_sel(&self) -> GPIO_PIN5_MUX_SEL_R {
        GPIO_PIN5_MUX_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin4_mux_sel(&self) -> GPIO_PIN4_MUX_SEL_R {
        GPIO_PIN4_MUX_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin3_mux_sel(&self) -> GPIO_PIN3_MUX_SEL_R {
        GPIO_PIN3_MUX_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin2_mux_sel(&self) -> GPIO_PIN2_MUX_SEL_R {
        GPIO_PIN2_MUX_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin1_mux_sel(&self) -> GPIO_PIN1_MUX_SEL_R {
        GPIO_PIN1_MUX_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin0_mux_sel(&self) -> GPIO_PIN0_MUX_SEL_R {
        GPIO_PIN0_MUX_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin5_fun_sel(&self) -> GPIO_PIN5_FUN_SEL_R {
        GPIO_PIN5_FUN_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin4_fun_sel(&self) -> GPIO_PIN4_FUN_SEL_R {
        GPIO_PIN4_FUN_SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin3_fun_sel(&self) -> GPIO_PIN3_FUN_SEL_R {
        GPIO_PIN3_FUN_SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin2_fun_sel(&self) -> GPIO_PIN2_FUN_SEL_R {
        GPIO_PIN2_FUN_SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin1_fun_sel(&self) -> GPIO_PIN1_FUN_SEL_R {
        GPIO_PIN1_FUN_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin0_fun_sel(&self) -> GPIO_PIN0_FUN_SEL_R {
        GPIO_PIN0_FUN_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTL_DBG_MAP")
            .field("gpio_pin5_mux_sel", &self.gpio_pin5_mux_sel())
            .field("gpio_pin4_mux_sel", &self.gpio_pin4_mux_sel())
            .field("gpio_pin3_mux_sel", &self.gpio_pin3_mux_sel())
            .field("gpio_pin2_mux_sel", &self.gpio_pin2_mux_sel())
            .field("gpio_pin1_mux_sel", &self.gpio_pin1_mux_sel())
            .field("gpio_pin0_mux_sel", &self.gpio_pin0_mux_sel())
            .field("gpio_pin5_fun_sel", &self.gpio_pin5_fun_sel())
            .field("gpio_pin4_fun_sel", &self.gpio_pin4_fun_sel())
            .field("gpio_pin3_fun_sel", &self.gpio_pin3_fun_sel())
            .field("gpio_pin2_fun_sel", &self.gpio_pin2_fun_sel())
            .field("gpio_pin1_fun_sel", &self.gpio_pin1_fun_sel())
            .field("gpio_pin0_fun_sel", &self.gpio_pin0_fun_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin5_mux_sel(&mut self) -> GPIO_PIN5_MUX_SEL_W<CNTL_DBG_MAP_SPEC> {
        GPIO_PIN5_MUX_SEL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin4_mux_sel(&mut self) -> GPIO_PIN4_MUX_SEL_W<CNTL_DBG_MAP_SPEC> {
        GPIO_PIN4_MUX_SEL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin3_mux_sel(&mut self) -> GPIO_PIN3_MUX_SEL_W<CNTL_DBG_MAP_SPEC> {
        GPIO_PIN3_MUX_SEL_W::new(self, 4)
    }
    #[doc = "Bit 5 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin2_mux_sel(&mut self) -> GPIO_PIN2_MUX_SEL_W<CNTL_DBG_MAP_SPEC> {
        GPIO_PIN2_MUX_SEL_W::new(self, 5)
    }
    #[doc = "Bit 6 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin1_mux_sel(&mut self) -> GPIO_PIN1_MUX_SEL_W<CNTL_DBG_MAP_SPEC> {
        GPIO_PIN1_MUX_SEL_W::new(self, 6)
    }
    #[doc = "Bit 7 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin0_mux_sel(&mut self) -> GPIO_PIN0_MUX_SEL_W<CNTL_DBG_MAP_SPEC> {
        GPIO_PIN0_MUX_SEL_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin5_fun_sel(&mut self) -> GPIO_PIN5_FUN_SEL_W<CNTL_DBG_MAP_SPEC> {
        GPIO_PIN5_FUN_SEL_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin4_fun_sel(&mut self) -> GPIO_PIN4_FUN_SEL_W<CNTL_DBG_MAP_SPEC> {
        GPIO_PIN4_FUN_SEL_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin3_fun_sel(&mut self) -> GPIO_PIN3_FUN_SEL_W<CNTL_DBG_MAP_SPEC> {
        GPIO_PIN3_FUN_SEL_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin2_fun_sel(&mut self) -> GPIO_PIN2_FUN_SEL_W<CNTL_DBG_MAP_SPEC> {
        GPIO_PIN2_FUN_SEL_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin1_fun_sel(&mut self) -> GPIO_PIN1_FUN_SEL_W<CNTL_DBG_MAP_SPEC> {
        GPIO_PIN1_FUN_SEL_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Need add desc"]
    #[inline(always)]
    pub fn gpio_pin0_fun_sel(&mut self) -> GPIO_PIN0_FUN_SEL_W<CNTL_DBG_MAP_SPEC> {
        GPIO_PIN0_FUN_SEL_W::new(self, 28)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cntl_dbg_map::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntl_dbg_map::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTL_DBG_MAP_SPEC;
impl crate::RegisterSpec for CNTL_DBG_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntl_dbg_map::R`](R) reader structure"]
impl crate::Readable for CNTL_DBG_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cntl_dbg_map::W`](W) writer structure"]
impl crate::Writable for CNTL_DBG_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNTL_DBG_MAP to value 0"]
impl crate::Resettable for CNTL_DBG_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
