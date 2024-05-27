///Register `CLK_CONF` reader
pub type R = crate::R<CLK_CONF_SPEC>;
///Register `CLK_CONF` writer
pub type W = crate::W<CLK_CONF_SPEC>;
///Field `SCLK_DIV_NUM` reader - the integral part of the fractional divisor for i2c module
pub type SCLK_DIV_NUM_R = crate::FieldReader;
///Field `SCLK_DIV_NUM` writer - the integral part of the fractional divisor for i2c module
pub type SCLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SCLK_DIV_A` reader - the numerator of the fractional part of the fractional divisor for i2c module
pub type SCLK_DIV_A_R = crate::FieldReader;
///Field `SCLK_DIV_A` writer - the numerator of the fractional part of the fractional divisor for i2c module
pub type SCLK_DIV_A_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `SCLK_DIV_B` reader - the denominator of the fractional part of the fractional divisor for i2c module
pub type SCLK_DIV_B_R = crate::FieldReader;
///Field `SCLK_DIV_B` writer - the denominator of the fractional part of the fractional divisor for i2c module
pub type SCLK_DIV_B_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `SCLK_SEL` reader - The clock selection for i2c module:0-XTAL,1-CLK_8MHz.
pub type SCLK_SEL_R = crate::BitReader;
///Field `SCLK_SEL` writer - The clock selection for i2c module:0-XTAL,1-CLK_8MHz.
pub type SCLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCLK_ACTIVE` reader - The clock switch for i2c module
pub type SCLK_ACTIVE_R = crate::BitReader;
///Field `SCLK_ACTIVE` writer - The clock switch for i2c module
pub type SCLK_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - the integral part of the fractional divisor for i2c module
    #[inline(always)]
    pub fn sclk_div_num(&self) -> SCLK_DIV_NUM_R {
        SCLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:13 - the numerator of the fractional part of the fractional divisor for i2c module
    #[inline(always)]
    pub fn sclk_div_a(&self) -> SCLK_DIV_A_R {
        SCLK_DIV_A_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    ///Bits 14:19 - the denominator of the fractional part of the fractional divisor for i2c module
    #[inline(always)]
    pub fn sclk_div_b(&self) -> SCLK_DIV_B_R {
        SCLK_DIV_B_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    ///Bit 20 - The clock selection for i2c module:0-XTAL,1-CLK_8MHz.
    #[inline(always)]
    pub fn sclk_sel(&self) -> SCLK_SEL_R {
        SCLK_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - The clock switch for i2c module
    #[inline(always)]
    pub fn sclk_active(&self) -> SCLK_ACTIVE_R {
        SCLK_ACTIVE_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_CONF")
            .field("sclk_div_num", &self.sclk_div_num())
            .field("sclk_div_a", &self.sclk_div_a())
            .field("sclk_div_b", &self.sclk_div_b())
            .field("sclk_sel", &self.sclk_sel())
            .field("sclk_active", &self.sclk_active())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - the integral part of the fractional divisor for i2c module
    #[inline(always)]
    #[must_use]
    pub fn sclk_div_num(&mut self) -> SCLK_DIV_NUM_W<CLK_CONF_SPEC> {
        SCLK_DIV_NUM_W::new(self, 0)
    }
    ///Bits 8:13 - the numerator of the fractional part of the fractional divisor for i2c module
    #[inline(always)]
    #[must_use]
    pub fn sclk_div_a(&mut self) -> SCLK_DIV_A_W<CLK_CONF_SPEC> {
        SCLK_DIV_A_W::new(self, 8)
    }
    ///Bits 14:19 - the denominator of the fractional part of the fractional divisor for i2c module
    #[inline(always)]
    #[must_use]
    pub fn sclk_div_b(&mut self) -> SCLK_DIV_B_W<CLK_CONF_SPEC> {
        SCLK_DIV_B_W::new(self, 14)
    }
    ///Bit 20 - The clock selection for i2c module:0-XTAL,1-CLK_8MHz.
    #[inline(always)]
    #[must_use]
    pub fn sclk_sel(&mut self) -> SCLK_SEL_W<CLK_CONF_SPEC> {
        SCLK_SEL_W::new(self, 20)
    }
    ///Bit 21 - The clock switch for i2c module
    #[inline(always)]
    #[must_use]
    pub fn sclk_active(&mut self) -> SCLK_ACTIVE_W<CLK_CONF_SPEC> {
        SCLK_ACTIVE_W::new(self, 21)
    }
}
/**I2C CLK configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CLK_CONF_SPEC;
impl crate::RegisterSpec for CLK_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`clk_conf::R`](R) reader structure
impl crate::Readable for CLK_CONF_SPEC {}
///`write(|w| ..)` method takes [`clk_conf::W`](W) writer structure
impl crate::Writable for CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CLK_CONF to value 0x0020_0000
impl crate::Resettable for CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0020_0000;
}
