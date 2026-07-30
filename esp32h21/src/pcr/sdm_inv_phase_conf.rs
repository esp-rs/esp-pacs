#[doc = "Register `SDM_INV_PHASE_CONF` reader"]
pub type R = crate::R<SDM_INV_PHASE_CONF_SPEC>;
#[doc = "Register `SDM_INV_PHASE_CONF` writer"]
pub type W = crate::W<SDM_INV_PHASE_CONF_SPEC>;
#[doc = "Field `CLK_SDM_INV_PHASE_ENA` reader - xxxx"]
pub type CLK_SDM_INV_PHASE_ENA_R = crate::BitReader;
#[doc = "Field `CLK_SDM_INV_PHASE_ENA` writer - xxxx"]
pub type CLK_SDM_INV_PHASE_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SDM_INV_PHASE_SEL` reader - xxxx"]
pub type CLK_SDM_INV_PHASE_SEL_R = crate::FieldReader;
#[doc = "Field `CLK_SDM_INV_PHASE_SEL` writer - xxxx"]
pub type CLK_SDM_INV_PHASE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLK_TXLO_INV_PHASE_ENA` reader - xxxx"]
pub type CLK_TXLO_INV_PHASE_ENA_R = crate::BitReader;
#[doc = "Field `CLK_TXLO_INV_PHASE_ENA` writer - xxxx"]
pub type CLK_TXLO_INV_PHASE_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SDM_TXLO_INV_PHASE_SEL` reader - xxxx"]
pub type CLK_SDM_TXLO_INV_PHASE_SEL_R = crate::FieldReader;
#[doc = "Field `CLK_SDM_TXLO_INV_PHASE_SEL` writer - xxxx"]
pub type CLK_SDM_TXLO_INV_PHASE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLK_SDM_EN` reader - xxxx"]
pub type CLK_SDM_EN_R = crate::BitReader;
#[doc = "Field `CLK_SDM_EN` writer - xxxx"]
pub type CLK_SDM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_TXLO_DIV2_EN` reader - xxxx"]
pub type CLK_TXLO_DIV2_EN_R = crate::BitReader;
#[doc = "Field `CLK_TXLO_DIV2_EN` writer - xxxx"]
pub type CLK_TXLO_DIV2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - xxxx"]
    #[inline(always)]
    pub fn clk_sdm_inv_phase_ena(&self) -> CLK_SDM_INV_PHASE_ENA_R {
        CLK_SDM_INV_PHASE_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - xxxx"]
    #[inline(always)]
    pub fn clk_sdm_inv_phase_sel(&self) -> CLK_SDM_INV_PHASE_SEL_R {
        CLK_SDM_INV_PHASE_SEL_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - xxxx"]
    #[inline(always)]
    pub fn clk_txlo_inv_phase_ena(&self) -> CLK_TXLO_INV_PHASE_ENA_R {
        CLK_TXLO_INV_PHASE_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:9 - xxxx"]
    #[inline(always)]
    pub fn clk_sdm_txlo_inv_phase_sel(&self) -> CLK_SDM_TXLO_INV_PHASE_SEL_R {
        CLK_SDM_TXLO_INV_PHASE_SEL_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - xxxx"]
    #[inline(always)]
    pub fn clk_sdm_en(&self) -> CLK_SDM_EN_R {
        CLK_SDM_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - xxxx"]
    #[inline(always)]
    pub fn clk_txlo_div2_en(&self) -> CLK_TXLO_DIV2_EN_R {
        CLK_TXLO_DIV2_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDM_INV_PHASE_CONF")
            .field("clk_sdm_inv_phase_ena", &self.clk_sdm_inv_phase_ena())
            .field("clk_sdm_inv_phase_sel", &self.clk_sdm_inv_phase_sel())
            .field("clk_txlo_inv_phase_ena", &self.clk_txlo_inv_phase_ena())
            .field(
                "clk_sdm_txlo_inv_phase_sel",
                &self.clk_sdm_txlo_inv_phase_sel(),
            )
            .field("clk_sdm_en", &self.clk_sdm_en())
            .field("clk_txlo_div2_en", &self.clk_txlo_div2_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - xxxx"]
    #[inline(always)]
    pub fn clk_sdm_inv_phase_ena(
        &mut self,
    ) -> CLK_SDM_INV_PHASE_ENA_W<'_, SDM_INV_PHASE_CONF_SPEC> {
        CLK_SDM_INV_PHASE_ENA_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - xxxx"]
    #[inline(always)]
    pub fn clk_sdm_inv_phase_sel(
        &mut self,
    ) -> CLK_SDM_INV_PHASE_SEL_W<'_, SDM_INV_PHASE_CONF_SPEC> {
        CLK_SDM_INV_PHASE_SEL_W::new(self, 1)
    }
    #[doc = "Bit 5 - xxxx"]
    #[inline(always)]
    pub fn clk_txlo_inv_phase_ena(
        &mut self,
    ) -> CLK_TXLO_INV_PHASE_ENA_W<'_, SDM_INV_PHASE_CONF_SPEC> {
        CLK_TXLO_INV_PHASE_ENA_W::new(self, 5)
    }
    #[doc = "Bits 6:9 - xxxx"]
    #[inline(always)]
    pub fn clk_sdm_txlo_inv_phase_sel(
        &mut self,
    ) -> CLK_SDM_TXLO_INV_PHASE_SEL_W<'_, SDM_INV_PHASE_CONF_SPEC> {
        CLK_SDM_TXLO_INV_PHASE_SEL_W::new(self, 6)
    }
    #[doc = "Bit 10 - xxxx"]
    #[inline(always)]
    pub fn clk_sdm_en(&mut self) -> CLK_SDM_EN_W<'_, SDM_INV_PHASE_CONF_SPEC> {
        CLK_SDM_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - xxxx"]
    #[inline(always)]
    pub fn clk_txlo_div2_en(&mut self) -> CLK_TXLO_DIV2_EN_W<'_, SDM_INV_PHASE_CONF_SPEC> {
        CLK_TXLO_DIV2_EN_W::new(self, 11)
    }
}
#[doc = "xxxx\n\nYou can [`read`](crate::Reg::read) this register and get [`sdm_inv_phase_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdm_inv_phase_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDM_INV_PHASE_CONF_SPEC;
impl crate::RegisterSpec for SDM_INV_PHASE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdm_inv_phase_conf::R`](R) reader structure"]
impl crate::Readable for SDM_INV_PHASE_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdm_inv_phase_conf::W`](W) writer structure"]
impl crate::Writable for SDM_INV_PHASE_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDM_INV_PHASE_CONF to value 0x0400"]
impl crate::Resettable for SDM_INV_PHASE_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0400;
}
