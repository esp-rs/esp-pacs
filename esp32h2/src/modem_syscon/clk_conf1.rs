///Register `CLK_CONF1` reader
pub type R = crate::R<CLK_CONF1_SPEC>;
///Register `CLK_CONF1` writer
pub type W = crate::W<CLK_CONF1_SPEC>;
///Field `CLK_FE_16M_EN` reader -
pub type CLK_FE_16M_EN_R = crate::BitReader;
///Field `CLK_FE_16M_EN` writer -
pub type CLK_FE_16M_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_FE_32M_EN` reader -
pub type CLK_FE_32M_EN_R = crate::BitReader;
///Field `CLK_FE_32M_EN` writer -
pub type CLK_FE_32M_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_FE_SDM_EN` reader -
pub type CLK_FE_SDM_EN_R = crate::BitReader;
///Field `CLK_FE_SDM_EN` writer -
pub type CLK_FE_SDM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_FE_ADC_EN` reader -
pub type CLK_FE_ADC_EN_R = crate::BitReader;
///Field `CLK_FE_ADC_EN` writer -
pub type CLK_FE_ADC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_FE_APB_EN` reader -
pub type CLK_FE_APB_EN_R = crate::BitReader;
///Field `CLK_FE_APB_EN` writer -
pub type CLK_FE_APB_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_BT_APB_EN` reader -
pub type CLK_BT_APB_EN_R = crate::BitReader;
///Field `CLK_BT_APB_EN` writer -
pub type CLK_BT_APB_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_BT_EN` reader -
pub type CLK_BT_EN_R = crate::BitReader;
///Field `CLK_BT_EN` writer -
pub type CLK_BT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 12
    #[inline(always)]
    pub fn clk_fe_16m_en(&self) -> CLK_FE_16M_EN_R {
        CLK_FE_16M_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13
    #[inline(always)]
    pub fn clk_fe_32m_en(&self) -> CLK_FE_32M_EN_R {
        CLK_FE_32M_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14
    #[inline(always)]
    pub fn clk_fe_sdm_en(&self) -> CLK_FE_SDM_EN_R {
        CLK_FE_SDM_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15
    #[inline(always)]
    pub fn clk_fe_adc_en(&self) -> CLK_FE_ADC_EN_R {
        CLK_FE_ADC_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16
    #[inline(always)]
    pub fn clk_fe_apb_en(&self) -> CLK_FE_APB_EN_R {
        CLK_FE_APB_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17
    #[inline(always)]
    pub fn clk_bt_apb_en(&self) -> CLK_BT_APB_EN_R {
        CLK_BT_APB_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18
    #[inline(always)]
    pub fn clk_bt_en(&self) -> CLK_BT_EN_R {
        CLK_BT_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_CONF1")
            .field("clk_fe_16m_en", &self.clk_fe_16m_en())
            .field("clk_fe_32m_en", &self.clk_fe_32m_en())
            .field("clk_fe_sdm_en", &self.clk_fe_sdm_en())
            .field("clk_fe_adc_en", &self.clk_fe_adc_en())
            .field("clk_fe_apb_en", &self.clk_fe_apb_en())
            .field("clk_bt_apb_en", &self.clk_bt_apb_en())
            .field("clk_bt_en", &self.clk_bt_en())
            .finish()
    }
}
impl W {
    ///Bit 12
    #[inline(always)]
    #[must_use]
    pub fn clk_fe_16m_en(&mut self) -> CLK_FE_16M_EN_W<CLK_CONF1_SPEC> {
        CLK_FE_16M_EN_W::new(self, 12)
    }
    ///Bit 13
    #[inline(always)]
    #[must_use]
    pub fn clk_fe_32m_en(&mut self) -> CLK_FE_32M_EN_W<CLK_CONF1_SPEC> {
        CLK_FE_32M_EN_W::new(self, 13)
    }
    ///Bit 14
    #[inline(always)]
    #[must_use]
    pub fn clk_fe_sdm_en(&mut self) -> CLK_FE_SDM_EN_W<CLK_CONF1_SPEC> {
        CLK_FE_SDM_EN_W::new(self, 14)
    }
    ///Bit 15
    #[inline(always)]
    #[must_use]
    pub fn clk_fe_adc_en(&mut self) -> CLK_FE_ADC_EN_W<CLK_CONF1_SPEC> {
        CLK_FE_ADC_EN_W::new(self, 15)
    }
    ///Bit 16
    #[inline(always)]
    #[must_use]
    pub fn clk_fe_apb_en(&mut self) -> CLK_FE_APB_EN_W<CLK_CONF1_SPEC> {
        CLK_FE_APB_EN_W::new(self, 16)
    }
    ///Bit 17
    #[inline(always)]
    #[must_use]
    pub fn clk_bt_apb_en(&mut self) -> CLK_BT_APB_EN_W<CLK_CONF1_SPEC> {
        CLK_BT_APB_EN_W::new(self, 17)
    }
    ///Bit 18
    #[inline(always)]
    #[must_use]
    pub fn clk_bt_en(&mut self) -> CLK_BT_EN_W<CLK_CONF1_SPEC> {
        CLK_BT_EN_W::new(self, 18)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`clk_conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CLK_CONF1_SPEC;
impl crate::RegisterSpec for CLK_CONF1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`clk_conf1::R`](R) reader structure
impl crate::Readable for CLK_CONF1_SPEC {}
///`write(|w| ..)` method takes [`clk_conf1::W`](W) writer structure
impl crate::Writable for CLK_CONF1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CLK_CONF1 to value 0
impl crate::Resettable for CLK_CONF1_SPEC {
    const RESET_VALUE: u32 = 0;
}
