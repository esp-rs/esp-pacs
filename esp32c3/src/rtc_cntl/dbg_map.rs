///Register `DBG_MAP` reader
pub type R = crate::R<DBG_MAP_SPEC>;
///Register `DBG_MAP` writer
pub type W = crate::W<DBG_MAP_SPEC>;
///Field `GPIO_PIN5_MUX_SEL` reader - use for debug
pub type GPIO_PIN5_MUX_SEL_R = crate::BitReader;
///Field `GPIO_PIN5_MUX_SEL` writer - use for debug
pub type GPIO_PIN5_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIO_PIN4_MUX_SEL` reader - use for debug
pub type GPIO_PIN4_MUX_SEL_R = crate::BitReader;
///Field `GPIO_PIN4_MUX_SEL` writer - use for debug
pub type GPIO_PIN4_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIO_PIN3_MUX_SEL` reader - use for debug
pub type GPIO_PIN3_MUX_SEL_R = crate::BitReader;
///Field `GPIO_PIN3_MUX_SEL` writer - use for debug
pub type GPIO_PIN3_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIO_PIN2_MUX_SEL` reader - use for debug
pub type GPIO_PIN2_MUX_SEL_R = crate::BitReader;
///Field `GPIO_PIN2_MUX_SEL` writer - use for debug
pub type GPIO_PIN2_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIO_PIN1_MUX_SEL` reader - use for debug
pub type GPIO_PIN1_MUX_SEL_R = crate::BitReader;
///Field `GPIO_PIN1_MUX_SEL` writer - use for debug
pub type GPIO_PIN1_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIO_PIN0_MUX_SEL` reader - use for debug
pub type GPIO_PIN0_MUX_SEL_R = crate::BitReader;
///Field `GPIO_PIN0_MUX_SEL` writer - use for debug
pub type GPIO_PIN0_MUX_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIO_PIN5_FUN_SEL` reader - use for debug
pub type GPIO_PIN5_FUN_SEL_R = crate::FieldReader;
///Field `GPIO_PIN5_FUN_SEL` writer - use for debug
pub type GPIO_PIN5_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `GPIO_PIN4_FUN_SEL` reader - use for debug
pub type GPIO_PIN4_FUN_SEL_R = crate::FieldReader;
///Field `GPIO_PIN4_FUN_SEL` writer - use for debug
pub type GPIO_PIN4_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `GPIO_PIN3_FUN_SEL` reader - use for debug
pub type GPIO_PIN3_FUN_SEL_R = crate::FieldReader;
///Field `GPIO_PIN3_FUN_SEL` writer - use for debug
pub type GPIO_PIN3_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `GPIO_PIN2_FUN_SEL` reader - use for debug
pub type GPIO_PIN2_FUN_SEL_R = crate::FieldReader;
///Field `GPIO_PIN2_FUN_SEL` writer - use for debug
pub type GPIO_PIN2_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `GPIO_PIN1_FUN_SEL` reader - use for debug
pub type GPIO_PIN1_FUN_SEL_R = crate::FieldReader;
///Field `GPIO_PIN1_FUN_SEL` writer - use for debug
pub type GPIO_PIN1_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `GPIO_PIN0_FUN_SEL` reader - use for debug
pub type GPIO_PIN0_FUN_SEL_R = crate::FieldReader;
///Field `GPIO_PIN0_FUN_SEL` writer - use for debug
pub type GPIO_PIN0_FUN_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 2 - use for debug
    #[inline(always)]
    pub fn gpio_pin5_mux_sel(&self) -> GPIO_PIN5_MUX_SEL_R {
        GPIO_PIN5_MUX_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - use for debug
    #[inline(always)]
    pub fn gpio_pin4_mux_sel(&self) -> GPIO_PIN4_MUX_SEL_R {
        GPIO_PIN4_MUX_SEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - use for debug
    #[inline(always)]
    pub fn gpio_pin3_mux_sel(&self) -> GPIO_PIN3_MUX_SEL_R {
        GPIO_PIN3_MUX_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - use for debug
    #[inline(always)]
    pub fn gpio_pin2_mux_sel(&self) -> GPIO_PIN2_MUX_SEL_R {
        GPIO_PIN2_MUX_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - use for debug
    #[inline(always)]
    pub fn gpio_pin1_mux_sel(&self) -> GPIO_PIN1_MUX_SEL_R {
        GPIO_PIN1_MUX_SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - use for debug
    #[inline(always)]
    pub fn gpio_pin0_mux_sel(&self) -> GPIO_PIN0_MUX_SEL_R {
        GPIO_PIN0_MUX_SEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - use for debug
    #[inline(always)]
    pub fn gpio_pin5_fun_sel(&self) -> GPIO_PIN5_FUN_SEL_R {
        GPIO_PIN5_FUN_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - use for debug
    #[inline(always)]
    pub fn gpio_pin4_fun_sel(&self) -> GPIO_PIN4_FUN_SEL_R {
        GPIO_PIN4_FUN_SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - use for debug
    #[inline(always)]
    pub fn gpio_pin3_fun_sel(&self) -> GPIO_PIN3_FUN_SEL_R {
        GPIO_PIN3_FUN_SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - use for debug
    #[inline(always)]
    pub fn gpio_pin2_fun_sel(&self) -> GPIO_PIN2_FUN_SEL_R {
        GPIO_PIN2_FUN_SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - use for debug
    #[inline(always)]
    pub fn gpio_pin1_fun_sel(&self) -> GPIO_PIN1_FUN_SEL_R {
        GPIO_PIN1_FUN_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - use for debug
    #[inline(always)]
    pub fn gpio_pin0_fun_sel(&self) -> GPIO_PIN0_FUN_SEL_R {
        GPIO_PIN0_FUN_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBG_MAP")
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
    ///Bit 2 - use for debug
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin5_mux_sel(&mut self) -> GPIO_PIN5_MUX_SEL_W<DBG_MAP_SPEC> {
        GPIO_PIN5_MUX_SEL_W::new(self, 2)
    }
    ///Bit 3 - use for debug
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin4_mux_sel(&mut self) -> GPIO_PIN4_MUX_SEL_W<DBG_MAP_SPEC> {
        GPIO_PIN4_MUX_SEL_W::new(self, 3)
    }
    ///Bit 4 - use for debug
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin3_mux_sel(&mut self) -> GPIO_PIN3_MUX_SEL_W<DBG_MAP_SPEC> {
        GPIO_PIN3_MUX_SEL_W::new(self, 4)
    }
    ///Bit 5 - use for debug
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin2_mux_sel(&mut self) -> GPIO_PIN2_MUX_SEL_W<DBG_MAP_SPEC> {
        GPIO_PIN2_MUX_SEL_W::new(self, 5)
    }
    ///Bit 6 - use for debug
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin1_mux_sel(&mut self) -> GPIO_PIN1_MUX_SEL_W<DBG_MAP_SPEC> {
        GPIO_PIN1_MUX_SEL_W::new(self, 6)
    }
    ///Bit 7 - use for debug
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin0_mux_sel(&mut self) -> GPIO_PIN0_MUX_SEL_W<DBG_MAP_SPEC> {
        GPIO_PIN0_MUX_SEL_W::new(self, 7)
    }
    ///Bits 8:11 - use for debug
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin5_fun_sel(&mut self) -> GPIO_PIN5_FUN_SEL_W<DBG_MAP_SPEC> {
        GPIO_PIN5_FUN_SEL_W::new(self, 8)
    }
    ///Bits 12:15 - use for debug
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin4_fun_sel(&mut self) -> GPIO_PIN4_FUN_SEL_W<DBG_MAP_SPEC> {
        GPIO_PIN4_FUN_SEL_W::new(self, 12)
    }
    ///Bits 16:19 - use for debug
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin3_fun_sel(&mut self) -> GPIO_PIN3_FUN_SEL_W<DBG_MAP_SPEC> {
        GPIO_PIN3_FUN_SEL_W::new(self, 16)
    }
    ///Bits 20:23 - use for debug
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin2_fun_sel(&mut self) -> GPIO_PIN2_FUN_SEL_W<DBG_MAP_SPEC> {
        GPIO_PIN2_FUN_SEL_W::new(self, 20)
    }
    ///Bits 24:27 - use for debug
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin1_fun_sel(&mut self) -> GPIO_PIN1_FUN_SEL_W<DBG_MAP_SPEC> {
        GPIO_PIN1_FUN_SEL_W::new(self, 24)
    }
    ///Bits 28:31 - use for debug
    #[inline(always)]
    #[must_use]
    pub fn gpio_pin0_fun_sel(&mut self) -> GPIO_PIN0_FUN_SEL_W<DBG_MAP_SPEC> {
        GPIO_PIN0_FUN_SEL_W::new(self, 28)
    }
}
/**rtc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`dbg_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DBG_MAP_SPEC;
impl crate::RegisterSpec for DBG_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dbg_map::R`](R) reader structure
impl crate::Readable for DBG_MAP_SPEC {}
///`write(|w| ..)` method takes [`dbg_map::W`](W) writer structure
impl crate::Writable for DBG_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DBG_MAP to value 0
impl crate::Resettable for DBG_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
