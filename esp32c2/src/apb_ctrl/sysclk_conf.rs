///Register `SYSCLK_CONF` reader
pub type R = crate::R<SYSCLK_CONF_SPEC>;
///Register `SYSCLK_CONF` writer
pub type W = crate::W<SYSCLK_CONF_SPEC>;
///Field `PRE_DIV_CNT` reader - reg_pre_div_cnt
pub type PRE_DIV_CNT_R = crate::FieldReader<u16>;
///Field `PRE_DIV_CNT` writer - reg_pre_div_cnt
pub type PRE_DIV_CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `CLK_320M_EN` reader - reg_clk_320m_en
pub type CLK_320M_EN_R = crate::BitReader;
///Field `CLK_320M_EN` writer - reg_clk_320m_en
pub type CLK_320M_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_EN` reader - reg_clk_en
pub type CLK_EN_R = crate::BitReader;
///Field `CLK_EN` writer - reg_clk_en
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RST_TICK_CNT` reader - reg_rst_tick_cnt
pub type RST_TICK_CNT_R = crate::BitReader;
///Field `RST_TICK_CNT` writer - reg_rst_tick_cnt
pub type RST_TICK_CNT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:9 - reg_pre_div_cnt
    #[inline(always)]
    pub fn pre_div_cnt(&self) -> PRE_DIV_CNT_R {
        PRE_DIV_CNT_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 10 - reg_clk_320m_en
    #[inline(always)]
    pub fn clk_320m_en(&self) -> CLK_320M_EN_R {
        CLK_320M_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - reg_clk_en
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - reg_rst_tick_cnt
    #[inline(always)]
    pub fn rst_tick_cnt(&self) -> RST_TICK_CNT_R {
        RST_TICK_CNT_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSCLK_CONF")
            .field("pre_div_cnt", &self.pre_div_cnt())
            .field("clk_320m_en", &self.clk_320m_en())
            .field("clk_en", &self.clk_en())
            .field("rst_tick_cnt", &self.rst_tick_cnt())
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
    ///Bit 10 - reg_clk_320m_en
    #[inline(always)]
    #[must_use]
    pub fn clk_320m_en(&mut self) -> CLK_320M_EN_W<SYSCLK_CONF_SPEC> {
        CLK_320M_EN_W::new(self, 10)
    }
    ///Bit 11 - reg_clk_en
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<SYSCLK_CONF_SPEC> {
        CLK_EN_W::new(self, 11)
    }
    ///Bit 12 - reg_rst_tick_cnt
    #[inline(always)]
    #[must_use]
    pub fn rst_tick_cnt(&mut self) -> RST_TICK_CNT_W<SYSCLK_CONF_SPEC> {
        RST_TICK_CNT_W::new(self, 12)
    }
}
/**APB_CTRL_SYSCLK_CONF_REG

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
