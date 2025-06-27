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
pub type CLK_SDM_INV_PHASE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - xxxx"]
    #[inline(always)]
    pub fn clk_sdm_inv_phase_ena(&self) -> CLK_SDM_INV_PHASE_ENA_R {
        CLK_SDM_INV_PHASE_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - xxxx"]
    #[inline(always)]
    pub fn clk_sdm_inv_phase_sel(&self) -> CLK_SDM_INV_PHASE_SEL_R {
        CLK_SDM_INV_PHASE_SEL_R::new(((self.bits >> 1) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDM_INV_PHASE_CONF")
            .field("clk_sdm_inv_phase_ena", &self.clk_sdm_inv_phase_ena())
            .field("clk_sdm_inv_phase_sel", &self.clk_sdm_inv_phase_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - xxxx"]
    #[inline(always)]
    pub fn clk_sdm_inv_phase_ena(&mut self) -> CLK_SDM_INV_PHASE_ENA_W<SDM_INV_PHASE_CONF_SPEC> {
        CLK_SDM_INV_PHASE_ENA_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - xxxx"]
    #[inline(always)]
    pub fn clk_sdm_inv_phase_sel(&mut self) -> CLK_SDM_INV_PHASE_SEL_W<SDM_INV_PHASE_CONF_SPEC> {
        CLK_SDM_INV_PHASE_SEL_W::new(self, 1)
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
#[doc = "`reset()` method sets SDM_INV_PHASE_CONF to value 0"]
impl crate::Resettable for SDM_INV_PHASE_CONF_SPEC {}
