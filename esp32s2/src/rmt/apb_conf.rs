#[doc = "Register `APB_CONF` reader"]
pub struct R(crate::R<APB_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_CONF` writer"]
pub struct W(crate::W<APB_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_CONF_SPEC>;
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
impl From<crate::W<APB_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APB_FIFO_MASK` reader - 1'h1: access memory directly. 1'h0: access memory by FIFO."]
pub type APB_FIFO_MASK_R = crate::BitReader;
#[doc = "Field `APB_FIFO_MASK` writer - 1'h1: access memory directly. 1'h0: access memory by FIFO."]
pub type APB_FIFO_MASK_W<'a, const O: u8> = crate::BitWriter<'a, APB_CONF_SPEC, O>;
#[doc = "Field `MEM_TX_WRAP_EN` reader - This is the enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size."]
pub type MEM_TX_WRAP_EN_R = crate::BitReader;
#[doc = "Field `MEM_TX_WRAP_EN` writer - This is the enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size."]
pub type MEM_TX_WRAP_EN_W<'a, const O: u8> = crate::BitWriter<'a, APB_CONF_SPEC, O>;
#[doc = "Field `MEM_CLK_FORCE_ON` reader - Set this bit to enable the clock for RMT memory."]
pub type MEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `MEM_CLK_FORCE_ON` writer - Set this bit to enable the clock for RMT memory."]
pub type MEM_CLK_FORCE_ON_W<'a, const O: u8> = crate::BitWriter<'a, APB_CONF_SPEC, O>;
#[doc = "Field `MEM_FORCE_PD` reader - Set this bit to power down RMT memory."]
pub type MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `MEM_FORCE_PD` writer - Set this bit to power down RMT memory."]
pub type MEM_FORCE_PD_W<'a, const O: u8> = crate::BitWriter<'a, APB_CONF_SPEC, O>;
#[doc = "Field `MEM_FORCE_PU` reader - 1: Disable RMT memory light sleep power down function. 0: Power down RMT memory when RMT is in light sleep mode."]
pub type MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `MEM_FORCE_PU` writer - 1: Disable RMT memory light sleep power down function. 0: Power down RMT memory when RMT is in light sleep mode."]
pub type MEM_FORCE_PU_W<'a, const O: u8> = crate::BitWriter<'a, APB_CONF_SPEC, O>;
#[doc = "Field `CLK_EN` reader - RMT register clock gate enable signal. 1: Power up the drive clock of registers. 0: Power down the drive clock of registers"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - RMT register clock gate enable signal. 1: Power up the drive clock of registers. 0: Power down the drive clock of registers"]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, APB_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 0 - 1'h1: access memory directly. 1'h0: access memory by FIFO."]
    #[inline(always)]
    pub fn apb_fifo_mask(&self) -> APB_FIFO_MASK_R {
        APB_FIFO_MASK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size."]
    #[inline(always)]
    pub fn mem_tx_wrap_en(&self) -> MEM_TX_WRAP_EN_R {
        MEM_TX_WRAP_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to enable the clock for RMT memory."]
    #[inline(always)]
    pub fn mem_clk_force_on(&self) -> MEM_CLK_FORCE_ON_R {
        MEM_CLK_FORCE_ON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to power down RMT memory."]
    #[inline(always)]
    pub fn mem_force_pd(&self) -> MEM_FORCE_PD_R {
        MEM_FORCE_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1: Disable RMT memory light sleep power down function. 0: Power down RMT memory when RMT is in light sleep mode."]
    #[inline(always)]
    pub fn mem_force_pu(&self) -> MEM_FORCE_PU_R {
        MEM_FORCE_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31 - RMT register clock gate enable signal. 1: Power up the drive clock of registers. 0: Power down the drive clock of registers"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_CONF")
            .field(
                "apb_fifo_mask",
                &format_args!("{}", self.apb_fifo_mask().bit()),
            )
            .field(
                "mem_tx_wrap_en",
                &format_args!("{}", self.mem_tx_wrap_en().bit()),
            )
            .field(
                "mem_clk_force_on",
                &format_args!("{}", self.mem_clk_force_on().bit()),
            )
            .field(
                "mem_force_pd",
                &format_args!("{}", self.mem_force_pd().bit()),
            )
            .field(
                "mem_force_pu",
                &format_args!("{}", self.mem_force_pu().bit()),
            )
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - 1'h1: access memory directly. 1'h0: access memory by FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn apb_fifo_mask(&mut self) -> APB_FIFO_MASK_W<0> {
        APB_FIFO_MASK_W::new(self)
    }
    #[doc = "Bit 1 - This is the enable bit for wraparound mode: it will resume sending at the start when the data to be sent is more than its memory size."]
    #[inline(always)]
    #[must_use]
    pub fn mem_tx_wrap_en(&mut self) -> MEM_TX_WRAP_EN_W<1> {
        MEM_TX_WRAP_EN_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to enable the clock for RMT memory."]
    #[inline(always)]
    #[must_use]
    pub fn mem_clk_force_on(&mut self) -> MEM_CLK_FORCE_ON_W<2> {
        MEM_CLK_FORCE_ON_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to power down RMT memory."]
    #[inline(always)]
    #[must_use]
    pub fn mem_force_pd(&mut self) -> MEM_FORCE_PD_W<3> {
        MEM_FORCE_PD_W::new(self)
    }
    #[doc = "Bit 4 - 1: Disable RMT memory light sleep power down function. 0: Power down RMT memory when RMT is in light sleep mode."]
    #[inline(always)]
    #[must_use]
    pub fn mem_force_pu(&mut self) -> MEM_FORCE_PU_W<4> {
        MEM_FORCE_PU_W::new(self)
    }
    #[doc = "Bit 31 - RMT register clock gate enable signal. 1: Power up the drive clock of registers. 0: Power down the drive clock of registers"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<31> {
        CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RMT apb configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_conf](index.html) module"]
pub struct APB_CONF_SPEC;
impl crate::RegisterSpec for APB_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_conf::R](R) reader structure"]
impl crate::Readable for APB_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_conf::W](W) writer structure"]
impl crate::Writable for APB_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_CONF to value 0x04"]
impl crate::Resettable for APB_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
