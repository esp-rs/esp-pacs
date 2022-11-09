#[doc = "Register `PD_CONF` reader"]
pub struct R(crate::R<PD_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PD_CONF` writer"]
pub struct W(crate::W<PD_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PD_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_RAM_FORCE_PD` reader - Set this bit to force power down DMA internal memory."]
pub type DMA_RAM_FORCE_PD_R = crate::BitReader<bool>;
#[doc = "Field `DMA_RAM_FORCE_PD` writer - Set this bit to force power down DMA internal memory."]
pub type DMA_RAM_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_CONF_SPEC, bool, O>;
#[doc = "Field `DMA_RAM_FORCE_PU` reader - Set this bit to force power up DMA internal memory"]
pub type DMA_RAM_FORCE_PU_R = crate::BitReader<bool>;
#[doc = "Field `DMA_RAM_FORCE_PU` writer - Set this bit to force power up DMA internal memory"]
pub type DMA_RAM_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_CONF_SPEC, bool, O>;
#[doc = "Field `DMA_RAM_CLK_FO` reader - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
pub type DMA_RAM_CLK_FO_R = crate::BitReader<bool>;
#[doc = "Field `DMA_RAM_CLK_FO` writer - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
pub type DMA_RAM_CLK_FO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PD_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 4 - Set this bit to force power down DMA internal memory."]
    #[inline(always)]
    pub fn dma_ram_force_pd(&self) -> DMA_RAM_FORCE_PD_R {
        DMA_RAM_FORCE_PD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to force power up DMA internal memory"]
    #[inline(always)]
    pub fn dma_ram_force_pu(&self) -> DMA_RAM_FORCE_PU_R {
        DMA_RAM_FORCE_PU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
    #[inline(always)]
    pub fn dma_ram_clk_fo(&self) -> DMA_RAM_CLK_FO_R {
        DMA_RAM_CLK_FO_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Set this bit to force power down DMA internal memory."]
    #[inline(always)]
    #[must_use]
    pub fn dma_ram_force_pd(&mut self) -> DMA_RAM_FORCE_PD_W<4> {
        DMA_RAM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to force power up DMA internal memory"]
    #[inline(always)]
    #[must_use]
    pub fn dma_ram_force_pu(&mut self) -> DMA_RAM_FORCE_PU_W<5> {
        DMA_RAM_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 6 - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
    #[inline(always)]
    #[must_use]
    pub fn dma_ram_clk_fo(&mut self) -> DMA_RAM_CLK_FO_W<6> {
        DMA_RAM_CLK_FO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "reserved\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd_conf](index.html) module"]
pub struct PD_CONF_SPEC;
impl crate::RegisterSpec for PD_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd_conf::R](R) reader structure"]
impl crate::Readable for PD_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd_conf::W](W) writer structure"]
impl crate::Writable for PD_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PD_CONF to value 0x20"]
impl crate::Resettable for PD_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
