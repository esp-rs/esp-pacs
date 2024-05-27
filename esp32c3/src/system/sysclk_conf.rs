///Register `SYSCLK_CONF` reader
pub type R = crate::R<SYSCLK_CONF_SPEC>;
///Register `SYSCLK_CONF` writer
pub type W = crate::W<SYSCLK_CONF_SPEC>;
///Field `PRE_DIV_CNT` reader - reg_pre_div_cnt
pub type PRE_DIV_CNT_R = crate::FieldReader<u16>;
///Field `PRE_DIV_CNT` writer - reg_pre_div_cnt
pub type PRE_DIV_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `SOC_CLK_SEL` reader - reg_soc_clk_sel
pub type SOC_CLK_SEL_R = crate::FieldReader;
///Field `SOC_CLK_SEL` writer - reg_soc_clk_sel
pub type SOC_CLK_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `CLK_XTAL_FREQ` reader - reg_clk_xtal_freq
pub type CLK_XTAL_FREQ_R = crate::FieldReader;
///Field `CLK_DIV_EN` reader - reg_clk_div_en
pub type CLK_DIV_EN_R = crate::BitReader;
impl R {
    ///Bits 0:9 - reg_pre_div_cnt
    #[inline(always)]
    pub fn pre_div_cnt(&self) -> PRE_DIV_CNT_R {
        PRE_DIV_CNT_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:11 - reg_soc_clk_sel
    #[inline(always)]
    pub fn soc_clk_sel(&self) -> SOC_CLK_SEL_R {
        SOC_CLK_SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:18 - reg_clk_xtal_freq
    #[inline(always)]
    pub fn clk_xtal_freq(&self) -> CLK_XTAL_FREQ_R {
        CLK_XTAL_FREQ_R::new(((self.bits >> 12) & 0x7f) as u8)
    }
    ///Bit 19 - reg_clk_div_en
    #[inline(always)]
    pub fn clk_div_en(&self) -> CLK_DIV_EN_R {
        CLK_DIV_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCLK_CONF")
            .field("pre_div_cnt", &self.pre_div_cnt())
            .field("soc_clk_sel", &self.soc_clk_sel())
            .field("clk_xtal_freq", &self.clk_xtal_freq())
            .field("clk_div_en", &self.clk_div_en())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - reg_pre_div_cnt
    #[inline(always)]
    #[must_use]
    pub fn pre_div_cnt(&mut self) -> PRE_DIV_CNT_W<SYSCLK_CONF_SPEC> {
        PRE_DIV_CNT_W::new(self, 0)
    }
    ///Bits 10:11 - reg_soc_clk_sel
    #[inline(always)]
    #[must_use]
    pub fn soc_clk_sel(&mut self) -> SOC_CLK_SEL_W<SYSCLK_CONF_SPEC> {
        SOC_CLK_SEL_W::new(self, 10)
    }
}
/**system clock config register

You can [`read`](crate::generic::Reg::read) this register and get [`sysclk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysclk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYSCLK_CONF_SPEC;
impl crate::RegisterSpec for SYSCLK_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sysclk_conf::R`](R) reader structure
impl crate::Readable for SYSCLK_CONF_SPEC {}
///`write(|w| ..)` method takes [`sysclk_conf::W`](W) writer structure
impl crate::Writable for SYSCLK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SYSCLK_CONF to value 0x01
impl crate::Resettable for SYSCLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
