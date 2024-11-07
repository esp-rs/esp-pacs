#[doc = "Register `UART0_SCLK_CONF` reader"]
pub type R = crate::R<UART0_SCLK_CONF_SPEC>;
#[doc = "Register `UART0_SCLK_CONF` writer"]
pub type W = crate::W<UART0_SCLK_CONF_SPEC>;
#[doc = "Field `UART0_SCLK_DIV_A` reader - The denominator of the frequency divider factor of the uart0 function clock."]
pub type UART0_SCLK_DIV_A_R = crate::FieldReader;
#[doc = "Field `UART0_SCLK_DIV_A` writer - The denominator of the frequency divider factor of the uart0 function clock."]
pub type UART0_SCLK_DIV_A_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `UART0_SCLK_DIV_B` reader - The numerator of the frequency divider factor of the uart0 function clock."]
pub type UART0_SCLK_DIV_B_R = crate::FieldReader;
#[doc = "Field `UART0_SCLK_DIV_B` writer - The numerator of the frequency divider factor of the uart0 function clock."]
pub type UART0_SCLK_DIV_B_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `UART0_SCLK_DIV_NUM` reader - The integral part of the frequency divider factor of the uart0 function clock."]
pub type UART0_SCLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `UART0_SCLK_DIV_NUM` writer - The integral part of the frequency divider factor of the uart0 function clock."]
pub type UART0_SCLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `UART0_SCLK_SEL` reader - set this field to select clock-source. 0: do not select anyone clock, 1: 80MHz, 2: FOSC, 3(default): XTAL."]
pub type UART0_SCLK_SEL_R = crate::FieldReader;
#[doc = "Field `UART0_SCLK_SEL` writer - set this field to select clock-source. 0: do not select anyone clock, 1: 80MHz, 2: FOSC, 3(default): XTAL."]
pub type UART0_SCLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UART0_SCLK_EN` reader - Set 1 to enable uart0 function clock"]
pub type UART0_SCLK_EN_R = crate::BitReader;
#[doc = "Field `UART0_SCLK_EN` writer - Set 1 to enable uart0 function clock"]
pub type UART0_SCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - The denominator of the frequency divider factor of the uart0 function clock."]
    #[inline(always)]
    pub fn uart0_sclk_div_a(&self) -> UART0_SCLK_DIV_A_R {
        UART0_SCLK_DIV_A_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - The numerator of the frequency divider factor of the uart0 function clock."]
    #[inline(always)]
    pub fn uart0_sclk_div_b(&self) -> UART0_SCLK_DIV_B_R {
        UART0_SCLK_DIV_B_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:19 - The integral part of the frequency divider factor of the uart0 function clock."]
    #[inline(always)]
    pub fn uart0_sclk_div_num(&self) -> UART0_SCLK_DIV_NUM_R {
        UART0_SCLK_DIV_NUM_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 20:21 - set this field to select clock-source. 0: do not select anyone clock, 1: 80MHz, 2: FOSC, 3(default): XTAL."]
    #[inline(always)]
    pub fn uart0_sclk_sel(&self) -> UART0_SCLK_SEL_R {
        UART0_SCLK_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Set 1 to enable uart0 function clock"]
    #[inline(always)]
    pub fn uart0_sclk_en(&self) -> UART0_SCLK_EN_R {
        UART0_SCLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART0_SCLK_CONF")
            .field("uart0_sclk_div_a", &self.uart0_sclk_div_a())
            .field("uart0_sclk_div_b", &self.uart0_sclk_div_b())
            .field("uart0_sclk_div_num", &self.uart0_sclk_div_num())
            .field("uart0_sclk_sel", &self.uart0_sclk_sel())
            .field("uart0_sclk_en", &self.uart0_sclk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - The denominator of the frequency divider factor of the uart0 function clock."]
    #[inline(always)]
    pub fn uart0_sclk_div_a(&mut self) -> UART0_SCLK_DIV_A_W<UART0_SCLK_CONF_SPEC> {
        UART0_SCLK_DIV_A_W::new(self, 0)
    }
    #[doc = "Bits 6:11 - The numerator of the frequency divider factor of the uart0 function clock."]
    #[inline(always)]
    pub fn uart0_sclk_div_b(&mut self) -> UART0_SCLK_DIV_B_W<UART0_SCLK_CONF_SPEC> {
        UART0_SCLK_DIV_B_W::new(self, 6)
    }
    #[doc = "Bits 12:19 - The integral part of the frequency divider factor of the uart0 function clock."]
    #[inline(always)]
    pub fn uart0_sclk_div_num(&mut self) -> UART0_SCLK_DIV_NUM_W<UART0_SCLK_CONF_SPEC> {
        UART0_SCLK_DIV_NUM_W::new(self, 12)
    }
    #[doc = "Bits 20:21 - set this field to select clock-source. 0: do not select anyone clock, 1: 80MHz, 2: FOSC, 3(default): XTAL."]
    #[inline(always)]
    pub fn uart0_sclk_sel(&mut self) -> UART0_SCLK_SEL_W<UART0_SCLK_CONF_SPEC> {
        UART0_SCLK_SEL_W::new(self, 20)
    }
    #[doc = "Bit 22 - Set 1 to enable uart0 function clock"]
    #[inline(always)]
    pub fn uart0_sclk_en(&mut self) -> UART0_SCLK_EN_W<UART0_SCLK_CONF_SPEC> {
        UART0_SCLK_EN_W::new(self, 22)
    }
}
#[doc = "UART0_SCLK configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`uart0_sclk_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart0_sclk_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UART0_SCLK_CONF_SPEC;
impl crate::RegisterSpec for UART0_SCLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart0_sclk_conf::R`](R) reader structure"]
impl crate::Readable for UART0_SCLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uart0_sclk_conf::W`](W) writer structure"]
impl crate::Writable for UART0_SCLK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART0_SCLK_CONF to value 0x0070_0000"]
impl crate::Resettable for UART0_SCLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0070_0000;
}
