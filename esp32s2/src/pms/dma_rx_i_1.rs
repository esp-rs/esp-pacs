#[doc = "Register `DMA_RX_I_1` reader"]
pub struct R(crate::R<DMA_RX_I_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_RX_I_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_RX_I_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_RX_I_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_RX_I_1` writer"]
pub struct W(crate::W<DMA_RX_I_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_RX_I_1_SPEC>;
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
impl From<crate::W<DMA_RX_I_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_RX_I_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_RX_I_SRAM_0_R` reader - Setting to 1 grants RX Copy DMA permission to read SRAM Block 0."]
pub type DMA_RX_I_SRAM_0_R_R = crate::BitReader;
#[doc = "Field `DMA_RX_I_SRAM_0_R` writer - Setting to 1 grants RX Copy DMA permission to read SRAM Block 0."]
pub type DMA_RX_I_SRAM_0_R_W<'a, const O: u8> = crate::BitWriter<'a, DMA_RX_I_1_SPEC, O>;
#[doc = "Field `DMA_RX_I_SRAM_0_W` reader - Setting to 1 grants RX Copy DMA permission to write SRAM Block 0."]
pub type DMA_RX_I_SRAM_0_W_R = crate::BitReader;
#[doc = "Field `DMA_RX_I_SRAM_0_W` writer - Setting to 1 grants RX Copy DMA permission to write SRAM Block 0."]
pub type DMA_RX_I_SRAM_0_W_W<'a, const O: u8> = crate::BitWriter<'a, DMA_RX_I_1_SPEC, O>;
#[doc = "Field `DMA_RX_I_SRAM_1_R` reader - Setting to 1 grants RX Copy DMA permission to read SRAM Block 1."]
pub type DMA_RX_I_SRAM_1_R_R = crate::BitReader;
#[doc = "Field `DMA_RX_I_SRAM_1_R` writer - Setting to 1 grants RX Copy DMA permission to read SRAM Block 1."]
pub type DMA_RX_I_SRAM_1_R_W<'a, const O: u8> = crate::BitWriter<'a, DMA_RX_I_1_SPEC, O>;
#[doc = "Field `DMA_RX_I_SRAM_1_W` reader - Setting to 1 grants RX Copy DMA permission to write SRAM Block 1."]
pub type DMA_RX_I_SRAM_1_W_R = crate::BitReader;
#[doc = "Field `DMA_RX_I_SRAM_1_W` writer - Setting to 1 grants RX Copy DMA permission to write SRAM Block 1."]
pub type DMA_RX_I_SRAM_1_W_W<'a, const O: u8> = crate::BitWriter<'a, DMA_RX_I_1_SPEC, O>;
#[doc = "Field `DMA_RX_I_SRAM_2_R` reader - Setting to 1 grants RX Copy DMA permission to read SRAM Block 2."]
pub type DMA_RX_I_SRAM_2_R_R = crate::BitReader;
#[doc = "Field `DMA_RX_I_SRAM_2_R` writer - Setting to 1 grants RX Copy DMA permission to read SRAM Block 2."]
pub type DMA_RX_I_SRAM_2_R_W<'a, const O: u8> = crate::BitWriter<'a, DMA_RX_I_1_SPEC, O>;
#[doc = "Field `DMA_RX_I_SRAM_2_W` reader - Setting to 1 grants RX Copy DMA permission to write SRAM Block 2."]
pub type DMA_RX_I_SRAM_2_W_R = crate::BitReader;
#[doc = "Field `DMA_RX_I_SRAM_2_W` writer - Setting to 1 grants RX Copy DMA permission to write SRAM Block 2."]
pub type DMA_RX_I_SRAM_2_W_W<'a, const O: u8> = crate::BitWriter<'a, DMA_RX_I_1_SPEC, O>;
#[doc = "Field `DMA_RX_I_SRAM_3_R` reader - Setting to 1 grants RX Copy DMA permission to read SRAM Block 3."]
pub type DMA_RX_I_SRAM_3_R_R = crate::BitReader;
#[doc = "Field `DMA_RX_I_SRAM_3_R` writer - Setting to 1 grants RX Copy DMA permission to read SRAM Block 3."]
pub type DMA_RX_I_SRAM_3_R_W<'a, const O: u8> = crate::BitWriter<'a, DMA_RX_I_1_SPEC, O>;
#[doc = "Field `DMA_RX_I_SRAM_3_W` reader - Setting to 1 grants RX Copy DMA permission to write SRAM Block 3."]
pub type DMA_RX_I_SRAM_3_W_R = crate::BitReader;
#[doc = "Field `DMA_RX_I_SRAM_3_W` writer - Setting to 1 grants RX Copy DMA permission to write SRAM Block 3."]
pub type DMA_RX_I_SRAM_3_W_W<'a, const O: u8> = crate::BitWriter<'a, DMA_RX_I_1_SPEC, O>;
#[doc = "Field `DMA_RX_I_SRAM_4_SPLTADDR` reader - Configure the split address of SRAM Block 4-21 for RX Copy DMA access."]
pub type DMA_RX_I_SRAM_4_SPLTADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DMA_RX_I_SRAM_4_SPLTADDR` writer - Configure the split address of SRAM Block 4-21 for RX Copy DMA access."]
pub type DMA_RX_I_SRAM_4_SPLTADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, DMA_RX_I_1_SPEC, 17, O, u32>;
#[doc = "Field `DMA_RX_I_SRAM_4_L_R` reader - Setting to 1 grants RX Copy DMA permission to read SRAM Block 4-21 low address region."]
pub type DMA_RX_I_SRAM_4_L_R_R = crate::BitReader;
#[doc = "Field `DMA_RX_I_SRAM_4_L_R` writer - Setting to 1 grants RX Copy DMA permission to read SRAM Block 4-21 low address region."]
pub type DMA_RX_I_SRAM_4_L_R_W<'a, const O: u8> = crate::BitWriter<'a, DMA_RX_I_1_SPEC, O>;
#[doc = "Field `DMA_RX_I_SRAM_4_L_W` reader - Setting to 1 grants RX Copy DMA permission to write SRAM Block 4-21 low address region."]
pub type DMA_RX_I_SRAM_4_L_W_R = crate::BitReader;
#[doc = "Field `DMA_RX_I_SRAM_4_L_W` writer - Setting to 1 grants RX Copy DMA permission to write SRAM Block 4-21 low address region."]
pub type DMA_RX_I_SRAM_4_L_W_W<'a, const O: u8> = crate::BitWriter<'a, DMA_RX_I_1_SPEC, O>;
#[doc = "Field `DMA_RX_I_SRAM_4_H_R` reader - Setting to 1 grants RX Copy DMA permission to read SRAM Block 4-21 high address region."]
pub type DMA_RX_I_SRAM_4_H_R_R = crate::BitReader;
#[doc = "Field `DMA_RX_I_SRAM_4_H_R` writer - Setting to 1 grants RX Copy DMA permission to read SRAM Block 4-21 high address region."]
pub type DMA_RX_I_SRAM_4_H_R_W<'a, const O: u8> = crate::BitWriter<'a, DMA_RX_I_1_SPEC, O>;
#[doc = "Field `DMA_RX_I_SRAM_4_H_W` reader - Setting to 1 grants RX Copy DMA permission to write SRAM Block 4~21 high address region."]
pub type DMA_RX_I_SRAM_4_H_W_R = crate::BitReader;
#[doc = "Field `DMA_RX_I_SRAM_4_H_W` writer - Setting to 1 grants RX Copy DMA permission to write SRAM Block 4~21 high address region."]
pub type DMA_RX_I_SRAM_4_H_W_W<'a, const O: u8> = crate::BitWriter<'a, DMA_RX_I_1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Setting to 1 grants RX Copy DMA permission to read SRAM Block 0."]
    #[inline(always)]
    pub fn dma_rx_i_sram_0_r(&self) -> DMA_RX_I_SRAM_0_R_R {
        DMA_RX_I_SRAM_0_R_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Setting to 1 grants RX Copy DMA permission to write SRAM Block 0."]
    #[inline(always)]
    pub fn dma_rx_i_sram_0_w(&self) -> DMA_RX_I_SRAM_0_W_R {
        DMA_RX_I_SRAM_0_W_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Setting to 1 grants RX Copy DMA permission to read SRAM Block 1."]
    #[inline(always)]
    pub fn dma_rx_i_sram_1_r(&self) -> DMA_RX_I_SRAM_1_R_R {
        DMA_RX_I_SRAM_1_R_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Setting to 1 grants RX Copy DMA permission to write SRAM Block 1."]
    #[inline(always)]
    pub fn dma_rx_i_sram_1_w(&self) -> DMA_RX_I_SRAM_1_W_R {
        DMA_RX_I_SRAM_1_W_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Setting to 1 grants RX Copy DMA permission to read SRAM Block 2."]
    #[inline(always)]
    pub fn dma_rx_i_sram_2_r(&self) -> DMA_RX_I_SRAM_2_R_R {
        DMA_RX_I_SRAM_2_R_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Setting to 1 grants RX Copy DMA permission to write SRAM Block 2."]
    #[inline(always)]
    pub fn dma_rx_i_sram_2_w(&self) -> DMA_RX_I_SRAM_2_W_R {
        DMA_RX_I_SRAM_2_W_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Setting to 1 grants RX Copy DMA permission to read SRAM Block 3."]
    #[inline(always)]
    pub fn dma_rx_i_sram_3_r(&self) -> DMA_RX_I_SRAM_3_R_R {
        DMA_RX_I_SRAM_3_R_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Setting to 1 grants RX Copy DMA permission to write SRAM Block 3."]
    #[inline(always)]
    pub fn dma_rx_i_sram_3_w(&self) -> DMA_RX_I_SRAM_3_W_R {
        DMA_RX_I_SRAM_3_W_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:24 - Configure the split address of SRAM Block 4-21 for RX Copy DMA access."]
    #[inline(always)]
    pub fn dma_rx_i_sram_4_spltaddr(&self) -> DMA_RX_I_SRAM_4_SPLTADDR_R {
        DMA_RX_I_SRAM_4_SPLTADDR_R::new((self.bits >> 8) & 0x0001_ffff)
    }
    #[doc = "Bit 25 - Setting to 1 grants RX Copy DMA permission to read SRAM Block 4-21 low address region."]
    #[inline(always)]
    pub fn dma_rx_i_sram_4_l_r(&self) -> DMA_RX_I_SRAM_4_L_R_R {
        DMA_RX_I_SRAM_4_L_R_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Setting to 1 grants RX Copy DMA permission to write SRAM Block 4-21 low address region."]
    #[inline(always)]
    pub fn dma_rx_i_sram_4_l_w(&self) -> DMA_RX_I_SRAM_4_L_W_R {
        DMA_RX_I_SRAM_4_L_W_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Setting to 1 grants RX Copy DMA permission to read SRAM Block 4-21 high address region."]
    #[inline(always)]
    pub fn dma_rx_i_sram_4_h_r(&self) -> DMA_RX_I_SRAM_4_H_R_R {
        DMA_RX_I_SRAM_4_H_R_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Setting to 1 grants RX Copy DMA permission to write SRAM Block 4~21 high address region."]
    #[inline(always)]
    pub fn dma_rx_i_sram_4_h_w(&self) -> DMA_RX_I_SRAM_4_H_W_R {
        DMA_RX_I_SRAM_4_H_W_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_RX_I_1")
            .field(
                "dma_rx_i_sram_0_r",
                &format_args!("{}", self.dma_rx_i_sram_0_r().bit()),
            )
            .field(
                "dma_rx_i_sram_0_w",
                &format_args!("{}", self.dma_rx_i_sram_0_w().bit()),
            )
            .field(
                "dma_rx_i_sram_1_r",
                &format_args!("{}", self.dma_rx_i_sram_1_r().bit()),
            )
            .field(
                "dma_rx_i_sram_1_w",
                &format_args!("{}", self.dma_rx_i_sram_1_w().bit()),
            )
            .field(
                "dma_rx_i_sram_2_r",
                &format_args!("{}", self.dma_rx_i_sram_2_r().bit()),
            )
            .field(
                "dma_rx_i_sram_2_w",
                &format_args!("{}", self.dma_rx_i_sram_2_w().bit()),
            )
            .field(
                "dma_rx_i_sram_3_r",
                &format_args!("{}", self.dma_rx_i_sram_3_r().bit()),
            )
            .field(
                "dma_rx_i_sram_3_w",
                &format_args!("{}", self.dma_rx_i_sram_3_w().bit()),
            )
            .field(
                "dma_rx_i_sram_4_spltaddr",
                &format_args!("{}", self.dma_rx_i_sram_4_spltaddr().bits()),
            )
            .field(
                "dma_rx_i_sram_4_l_r",
                &format_args!("{}", self.dma_rx_i_sram_4_l_r().bit()),
            )
            .field(
                "dma_rx_i_sram_4_l_w",
                &format_args!("{}", self.dma_rx_i_sram_4_l_w().bit()),
            )
            .field(
                "dma_rx_i_sram_4_h_r",
                &format_args!("{}", self.dma_rx_i_sram_4_h_r().bit()),
            )
            .field(
                "dma_rx_i_sram_4_h_w",
                &format_args!("{}", self.dma_rx_i_sram_4_h_w().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_RX_I_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Setting to 1 grants RX Copy DMA permission to read SRAM Block 0."]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_i_sram_0_r(&mut self) -> DMA_RX_I_SRAM_0_R_W<0> {
        DMA_RX_I_SRAM_0_R_W::new(self)
    }
    #[doc = "Bit 1 - Setting to 1 grants RX Copy DMA permission to write SRAM Block 0."]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_i_sram_0_w(&mut self) -> DMA_RX_I_SRAM_0_W_W<1> {
        DMA_RX_I_SRAM_0_W_W::new(self)
    }
    #[doc = "Bit 2 - Setting to 1 grants RX Copy DMA permission to read SRAM Block 1."]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_i_sram_1_r(&mut self) -> DMA_RX_I_SRAM_1_R_W<2> {
        DMA_RX_I_SRAM_1_R_W::new(self)
    }
    #[doc = "Bit 3 - Setting to 1 grants RX Copy DMA permission to write SRAM Block 1."]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_i_sram_1_w(&mut self) -> DMA_RX_I_SRAM_1_W_W<3> {
        DMA_RX_I_SRAM_1_W_W::new(self)
    }
    #[doc = "Bit 4 - Setting to 1 grants RX Copy DMA permission to read SRAM Block 2."]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_i_sram_2_r(&mut self) -> DMA_RX_I_SRAM_2_R_W<4> {
        DMA_RX_I_SRAM_2_R_W::new(self)
    }
    #[doc = "Bit 5 - Setting to 1 grants RX Copy DMA permission to write SRAM Block 2."]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_i_sram_2_w(&mut self) -> DMA_RX_I_SRAM_2_W_W<5> {
        DMA_RX_I_SRAM_2_W_W::new(self)
    }
    #[doc = "Bit 6 - Setting to 1 grants RX Copy DMA permission to read SRAM Block 3."]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_i_sram_3_r(&mut self) -> DMA_RX_I_SRAM_3_R_W<6> {
        DMA_RX_I_SRAM_3_R_W::new(self)
    }
    #[doc = "Bit 7 - Setting to 1 grants RX Copy DMA permission to write SRAM Block 3."]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_i_sram_3_w(&mut self) -> DMA_RX_I_SRAM_3_W_W<7> {
        DMA_RX_I_SRAM_3_W_W::new(self)
    }
    #[doc = "Bits 8:24 - Configure the split address of SRAM Block 4-21 for RX Copy DMA access."]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_i_sram_4_spltaddr(&mut self) -> DMA_RX_I_SRAM_4_SPLTADDR_W<8> {
        DMA_RX_I_SRAM_4_SPLTADDR_W::new(self)
    }
    #[doc = "Bit 25 - Setting to 1 grants RX Copy DMA permission to read SRAM Block 4-21 low address region."]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_i_sram_4_l_r(&mut self) -> DMA_RX_I_SRAM_4_L_R_W<25> {
        DMA_RX_I_SRAM_4_L_R_W::new(self)
    }
    #[doc = "Bit 26 - Setting to 1 grants RX Copy DMA permission to write SRAM Block 4-21 low address region."]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_i_sram_4_l_w(&mut self) -> DMA_RX_I_SRAM_4_L_W_W<26> {
        DMA_RX_I_SRAM_4_L_W_W::new(self)
    }
    #[doc = "Bit 27 - Setting to 1 grants RX Copy DMA permission to read SRAM Block 4-21 high address region."]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_i_sram_4_h_r(&mut self) -> DMA_RX_I_SRAM_4_H_R_W<27> {
        DMA_RX_I_SRAM_4_H_R_W::new(self)
    }
    #[doc = "Bit 28 - Setting to 1 grants RX Copy DMA permission to write SRAM Block 4~21 high address region."]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_i_sram_4_h_w(&mut self) -> DMA_RX_I_SRAM_4_H_W_W<28> {
        DMA_RX_I_SRAM_4_H_W_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX Copy DMA permission control register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_rx_i_1](index.html) module"]
pub struct DMA_RX_I_1_SPEC;
impl crate::RegisterSpec for DMA_RX_I_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_rx_i_1::R](R) reader structure"]
impl crate::Readable for DMA_RX_I_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_rx_i_1::W](W) writer structure"]
impl crate::Writable for DMA_RX_I_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_RX_I_1 to value 0x1e00_00ff"]
impl crate::Resettable for DMA_RX_I_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x1e00_00ff;
}
