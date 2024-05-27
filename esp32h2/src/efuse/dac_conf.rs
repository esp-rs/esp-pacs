///Register `DAC_CONF` reader
pub type R = crate::R<DAC_CONF_SPEC>;
///Register `DAC_CONF` writer
pub type W = crate::W<DAC_CONF_SPEC>;
///Field `DAC_CLK_DIV` reader - Controls the division factor of the rising clock of the programming voltage.
pub type DAC_CLK_DIV_R = crate::FieldReader;
///Field `DAC_CLK_DIV` writer - Controls the division factor of the rising clock of the programming voltage.
pub type DAC_CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DAC_CLK_PAD_SEL` reader - Don't care.
pub type DAC_CLK_PAD_SEL_R = crate::BitReader;
///Field `DAC_CLK_PAD_SEL` writer - Don't care.
pub type DAC_CLK_PAD_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC_NUM` reader - Controls the rising period of the programming voltage.
pub type DAC_NUM_R = crate::FieldReader;
///Field `DAC_NUM` writer - Controls the rising period of the programming voltage.
pub type DAC_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `OE_CLR` reader - Reduces the power supply of the programming voltage.
pub type OE_CLR_R = crate::BitReader;
///Field `OE_CLR` writer - Reduces the power supply of the programming voltage.
pub type OE_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Controls the division factor of the rising clock of the programming voltage.
    #[inline(always)]
    pub fn dac_clk_div(&self) -> DAC_CLK_DIV_R {
        DAC_CLK_DIV_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - Don't care.
    #[inline(always)]
    pub fn dac_clk_pad_sel(&self) -> DAC_CLK_PAD_SEL_R {
        DAC_CLK_PAD_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:16 - Controls the rising period of the programming voltage.
    #[inline(always)]
    pub fn dac_num(&self) -> DAC_NUM_R {
        DAC_NUM_R::new(((self.bits >> 9) & 0xff) as u8)
    }
    ///Bit 17 - Reduces the power supply of the programming voltage.
    #[inline(always)]
    pub fn oe_clr(&self) -> OE_CLR_R {
        OE_CLR_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_CONF")
            .field("dac_clk_div", &self.dac_clk_div())
            .field("dac_clk_pad_sel", &self.dac_clk_pad_sel())
            .field("dac_num", &self.dac_num())
            .field("oe_clr", &self.oe_clr())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Controls the division factor of the rising clock of the programming voltage.
    #[inline(always)]
    #[must_use]
    pub fn dac_clk_div(&mut self) -> DAC_CLK_DIV_W<DAC_CONF_SPEC> {
        DAC_CLK_DIV_W::new(self, 0)
    }
    ///Bit 8 - Don't care.
    #[inline(always)]
    #[must_use]
    pub fn dac_clk_pad_sel(&mut self) -> DAC_CLK_PAD_SEL_W<DAC_CONF_SPEC> {
        DAC_CLK_PAD_SEL_W::new(self, 8)
    }
    ///Bits 9:16 - Controls the rising period of the programming voltage.
    #[inline(always)]
    #[must_use]
    pub fn dac_num(&mut self) -> DAC_NUM_W<DAC_CONF_SPEC> {
        DAC_NUM_W::new(self, 9)
    }
    ///Bit 17 - Reduces the power supply of the programming voltage.
    #[inline(always)]
    #[must_use]
    pub fn oe_clr(&mut self) -> OE_CLR_W<DAC_CONF_SPEC> {
        OE_CLR_W::new(self, 17)
    }
}
/**Controls the eFuse programming voltage.

You can [`read`](crate::generic::Reg::read) this register and get [`dac_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DAC_CONF_SPEC;
impl crate::RegisterSpec for DAC_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dac_conf::R`](R) reader structure
impl crate::Readable for DAC_CONF_SPEC {}
///`write(|w| ..)` method takes [`dac_conf::W`](W) writer structure
impl crate::Writable for DAC_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DAC_CONF to value 0x0001_fe17
impl crate::Resettable for DAC_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0001_fe17;
}
