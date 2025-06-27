#[doc = "Register `CLK_CONF1` reader"]
pub type R = crate::R<CLK_CONF1_SPEC>;
#[doc = "Register `CLK_CONF1` writer"]
pub type W = crate::W<CLK_CONF1_SPEC>;
#[doc = "Field `CLK_FE_16M_EN` reader - "]
pub type CLK_FE_16M_EN_R = crate::BitReader;
#[doc = "Field `CLK_FE_16M_EN` writer - "]
pub type CLK_FE_16M_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_FE_32M_EN` reader - "]
pub type CLK_FE_32M_EN_R = crate::BitReader;
#[doc = "Field `CLK_FE_32M_EN` writer - "]
pub type CLK_FE_32M_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_FE_SDM_EN` reader - "]
pub type CLK_FE_SDM_EN_R = crate::BitReader;
#[doc = "Field `CLK_FE_SDM_EN` writer - "]
pub type CLK_FE_SDM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_FE_ADC_EN` reader - "]
pub type CLK_FE_ADC_EN_R = crate::BitReader;
#[doc = "Field `CLK_FE_ADC_EN` writer - "]
pub type CLK_FE_ADC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_FE_APB_EN` reader - "]
pub type CLK_FE_APB_EN_R = crate::BitReader;
#[doc = "Field `CLK_FE_APB_EN` writer - "]
pub type CLK_FE_APB_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_BT_APB_EN` reader - "]
pub type CLK_BT_APB_EN_R = crate::BitReader;
#[doc = "Field `CLK_BT_APB_EN` writer - "]
pub type CLK_BT_APB_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_BT_EN` reader - "]
pub type CLK_BT_EN_R = crate::BitReader;
#[doc = "Field `CLK_BT_EN` writer - "]
pub type CLK_BT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn clk_fe_16m_en(&self) -> CLK_FE_16M_EN_R {
        CLK_FE_16M_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn clk_fe_32m_en(&self) -> CLK_FE_32M_EN_R {
        CLK_FE_32M_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn clk_fe_sdm_en(&self) -> CLK_FE_SDM_EN_R {
        CLK_FE_SDM_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn clk_fe_adc_en(&self) -> CLK_FE_ADC_EN_R {
        CLK_FE_ADC_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn clk_fe_apb_en(&self) -> CLK_FE_APB_EN_R {
        CLK_FE_APB_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn clk_bt_apb_en(&self) -> CLK_BT_APB_EN_R {
        CLK_BT_APB_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
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
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn clk_fe_16m_en(&mut self) -> CLK_FE_16M_EN_W<CLK_CONF1_SPEC> {
        CLK_FE_16M_EN_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn clk_fe_32m_en(&mut self) -> CLK_FE_32M_EN_W<CLK_CONF1_SPEC> {
        CLK_FE_32M_EN_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn clk_fe_sdm_en(&mut self) -> CLK_FE_SDM_EN_W<CLK_CONF1_SPEC> {
        CLK_FE_SDM_EN_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn clk_fe_adc_en(&mut self) -> CLK_FE_ADC_EN_W<CLK_CONF1_SPEC> {
        CLK_FE_ADC_EN_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn clk_fe_apb_en(&mut self) -> CLK_FE_APB_EN_W<CLK_CONF1_SPEC> {
        CLK_FE_APB_EN_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn clk_bt_apb_en(&mut self) -> CLK_BT_APB_EN_W<CLK_CONF1_SPEC> {
        CLK_BT_APB_EN_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn clk_bt_en(&mut self) -> CLK_BT_EN_W<CLK_CONF1_SPEC> {
        CLK_BT_EN_W::new(self, 18)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_CONF1_SPEC;
impl crate::RegisterSpec for CLK_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_conf1::R`](R) reader structure"]
impl crate::Readable for CLK_CONF1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_conf1::W`](W) writer structure"]
impl crate::Writable for CLK_CONF1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLK_CONF1 to value 0"]
impl crate::Resettable for CLK_CONF1_SPEC {}
