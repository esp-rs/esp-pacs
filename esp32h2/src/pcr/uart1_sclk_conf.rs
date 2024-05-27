///Register `UART1_SCLK_CONF` reader
pub type R = crate::R<UART1_SCLK_CONF_SPEC>;
///Register `UART1_SCLK_CONF` writer
pub type W = crate::W<UART1_SCLK_CONF_SPEC>;
///Field `UART1_SCLK_DIV_A` reader - The denominator of the frequency divider factor of the uart1 function clock.
pub type UART1_SCLK_DIV_A_R = crate::FieldReader;
///Field `UART1_SCLK_DIV_A` writer - The denominator of the frequency divider factor of the uart1 function clock.
pub type UART1_SCLK_DIV_A_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `UART1_SCLK_DIV_B` reader - The numerator of the frequency divider factor of the uart1 function clock.
pub type UART1_SCLK_DIV_B_R = crate::FieldReader;
///Field `UART1_SCLK_DIV_B` writer - The numerator of the frequency divider factor of the uart1 function clock.
pub type UART1_SCLK_DIV_B_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `UART1_SCLK_DIV_NUM` reader - The integral part of the frequency divider factor of the uart1 function clock.
pub type UART1_SCLK_DIV_NUM_R = crate::FieldReader;
///Field `UART1_SCLK_DIV_NUM` writer - The integral part of the frequency divider factor of the uart1 function clock.
pub type UART1_SCLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `UART1_SCLK_SEL` reader - set this field to select clock-source. 0: do not select anyone clock, 1: 80MHz, 2: FOSC, 3(default): XTAL.
pub type UART1_SCLK_SEL_R = crate::FieldReader;
///Field `UART1_SCLK_SEL` writer - set this field to select clock-source. 0: do not select anyone clock, 1: 80MHz, 2: FOSC, 3(default): XTAL.
pub type UART1_SCLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `UART1_SCLK_EN` reader - Set 1 to enable uart0 function clock
pub type UART1_SCLK_EN_R = crate::BitReader;
///Field `UART1_SCLK_EN` writer - Set 1 to enable uart0 function clock
pub type UART1_SCLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:5 - The denominator of the frequency divider factor of the uart1 function clock.
    #[inline(always)]
    pub fn uart1_sclk_div_a(&self) -> UART1_SCLK_DIV_A_R {
        UART1_SCLK_DIV_A_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:11 - The numerator of the frequency divider factor of the uart1 function clock.
    #[inline(always)]
    pub fn uart1_sclk_div_b(&self) -> UART1_SCLK_DIV_B_R {
        UART1_SCLK_DIV_B_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    ///Bits 12:19 - The integral part of the frequency divider factor of the uart1 function clock.
    #[inline(always)]
    pub fn uart1_sclk_div_num(&self) -> UART1_SCLK_DIV_NUM_R {
        UART1_SCLK_DIV_NUM_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    ///Bits 20:21 - set this field to select clock-source. 0: do not select anyone clock, 1: 80MHz, 2: FOSC, 3(default): XTAL.
    #[inline(always)]
    pub fn uart1_sclk_sel(&self) -> UART1_SCLK_SEL_R {
        UART1_SCLK_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - Set 1 to enable uart0 function clock
    #[inline(always)]
    pub fn uart1_sclk_en(&self) -> UART1_SCLK_EN_R {
        UART1_SCLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART1_SCLK_CONF")
            .field("uart1_sclk_div_a", &self.uart1_sclk_div_a())
            .field("uart1_sclk_div_b", &self.uart1_sclk_div_b())
            .field("uart1_sclk_div_num", &self.uart1_sclk_div_num())
            .field("uart1_sclk_sel", &self.uart1_sclk_sel())
            .field("uart1_sclk_en", &self.uart1_sclk_en())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - The denominator of the frequency divider factor of the uart1 function clock.
    #[inline(always)]
    #[must_use]
    pub fn uart1_sclk_div_a(&mut self) -> UART1_SCLK_DIV_A_W<UART1_SCLK_CONF_SPEC> {
        UART1_SCLK_DIV_A_W::new(self, 0)
    }
    ///Bits 6:11 - The numerator of the frequency divider factor of the uart1 function clock.
    #[inline(always)]
    #[must_use]
    pub fn uart1_sclk_div_b(&mut self) -> UART1_SCLK_DIV_B_W<UART1_SCLK_CONF_SPEC> {
        UART1_SCLK_DIV_B_W::new(self, 6)
    }
    ///Bits 12:19 - The integral part of the frequency divider factor of the uart1 function clock.
    #[inline(always)]
    #[must_use]
    pub fn uart1_sclk_div_num(&mut self) -> UART1_SCLK_DIV_NUM_W<UART1_SCLK_CONF_SPEC> {
        UART1_SCLK_DIV_NUM_W::new(self, 12)
    }
    ///Bits 20:21 - set this field to select clock-source. 0: do not select anyone clock, 1: 80MHz, 2: FOSC, 3(default): XTAL.
    #[inline(always)]
    #[must_use]
    pub fn uart1_sclk_sel(&mut self) -> UART1_SCLK_SEL_W<UART1_SCLK_CONF_SPEC> {
        UART1_SCLK_SEL_W::new(self, 20)
    }
    ///Bit 22 - Set 1 to enable uart0 function clock
    #[inline(always)]
    #[must_use]
    pub fn uart1_sclk_en(&mut self) -> UART1_SCLK_EN_W<UART1_SCLK_CONF_SPEC> {
        UART1_SCLK_EN_W::new(self, 22)
    }
}
/**UART1_SCLK configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`uart1_sclk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart1_sclk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct UART1_SCLK_CONF_SPEC;
impl crate::RegisterSpec for UART1_SCLK_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`uart1_sclk_conf::R`](R) reader structure
impl crate::Readable for UART1_SCLK_CONF_SPEC {}
///`write(|w| ..)` method takes [`uart1_sclk_conf::W`](W) writer structure
impl crate::Writable for UART1_SCLK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets UART1_SCLK_CONF to value 0x0070_0000
impl crate::Resettable for UART1_SCLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0070_0000;
}
