///Register `DAC_CONF` reader
pub type R = crate::R<DAC_CONF_SPEC>;
///Register `DAC_CONF` writer
pub type W = crate::W<DAC_CONF_SPEC>;
///Field `DAC_CLK_DIV` reader -
pub type DAC_CLK_DIV_R = crate::FieldReader;
///Field `DAC_CLK_DIV` writer -
pub type DAC_CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DAC_CLK_PAD_SEL` reader -
pub type DAC_CLK_PAD_SEL_R = crate::BitReader;
///Field `DAC_CLK_PAD_SEL` writer -
pub type DAC_CLK_PAD_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7
    #[inline(always)]
    pub fn dac_clk_div(&self) -> DAC_CLK_DIV_R {
        DAC_CLK_DIV_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 8
    #[inline(always)]
    pub fn dac_clk_pad_sel(&self) -> DAC_CLK_PAD_SEL_R {
        DAC_CLK_PAD_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_CONF")
            .field("dac_clk_div", &self.dac_clk_div())
            .field("dac_clk_pad_sel", &self.dac_clk_pad_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:7
    #[inline(always)]
    #[must_use]
    pub fn dac_clk_div(&mut self) -> DAC_CLK_DIV_W<DAC_CONF_SPEC> {
        DAC_CLK_DIV_W::new(self, 0)
    }
    ///Bit 8
    #[inline(always)]
    #[must_use]
    pub fn dac_clk_pad_sel(&mut self) -> DAC_CLK_PAD_SEL_W<DAC_CONF_SPEC> {
        DAC_CLK_PAD_SEL_W::new(self, 8)
    }
}
/**

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
///`reset()` method sets DAC_CONF to value 0x28
impl crate::Resettable for DAC_CONF_SPEC {
    const RESET_VALUE: u32 = 0x28;
}
