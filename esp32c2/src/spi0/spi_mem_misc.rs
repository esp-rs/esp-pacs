#[doc = "Register `SPI_MEM_MISC` reader"]
pub struct R(crate::R<SPI_MEM_MISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_MISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_MISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_MISC` writer"]
pub struct W(crate::W<SPI_MEM_MISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_MISC_SPEC>;
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
impl From<crate::W<SPI_MEM_MISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_MISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_TRANS_END` reader - The bit is used to indicate the spi0_mst_st controlled transmitting is done."]
pub type SPI_MEM_TRANS_END_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_TRANS_END` writer - The bit is used to indicate the spi0_mst_st controlled transmitting is done."]
pub type SPI_MEM_TRANS_END_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_MISC_SPEC, bool, 3>;
#[doc = "Field `SPI_MEM_TRANS_END_INT_ENA` reader - The bit is used to enable the interrupt of spi0_mst_st controlled transmitting is done."]
pub type SPI_MEM_TRANS_END_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_TRANS_END_INT_ENA` writer - The bit is used to enable the interrupt of spi0_mst_st controlled transmitting is done."]
pub type SPI_MEM_TRANS_END_INT_ENA_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_MISC_SPEC, bool, 4>;
#[doc = "Field `SPI_MEM_CSPI_ST_TRANS_END` reader - The bit is used to indicate the spi0_slv_st controlled transmitting is done."]
pub type SPI_MEM_CSPI_ST_TRANS_END_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_CSPI_ST_TRANS_END` writer - The bit is used to indicate the spi0_slv_st controlled transmitting is done."]
pub type SPI_MEM_CSPI_ST_TRANS_END_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_MISC_SPEC, bool, 5>;
#[doc = "Field `SPI_MEM_CSPI_ST_TRANS_END_INT_ENA` reader - The bit is used to enable the interrupt of spi0_slv_st controlled transmitting is done."]
pub type SPI_MEM_CSPI_ST_TRANS_END_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_CSPI_ST_TRANS_END_INT_ENA` writer - The bit is used to enable the interrupt of spi0_slv_st controlled transmitting is done."]
pub type SPI_MEM_CSPI_ST_TRANS_END_INT_ENA_W<'a> =
    crate::BitWriter<'a, u32, SPI_MEM_MISC_SPEC, bool, 6>;
#[doc = "Field `SPI_MEM_CK_IDLE_EDGE` reader - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
pub type SPI_MEM_CK_IDLE_EDGE_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_CK_IDLE_EDGE` writer - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
pub type SPI_MEM_CK_IDLE_EDGE_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_MISC_SPEC, bool, 9>;
#[doc = "Field `SPI_MEM_CS_KEEP_ACTIVE` reader - spi cs line keep low when the bit is set."]
pub type SPI_MEM_CS_KEEP_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MEM_CS_KEEP_ACTIVE` writer - spi cs line keep low when the bit is set."]
pub type SPI_MEM_CS_KEEP_ACTIVE_W<'a> = crate::BitWriter<'a, u32, SPI_MEM_MISC_SPEC, bool, 10>;
impl R {
    #[doc = "Bit 3 - The bit is used to indicate the spi0_mst_st controlled transmitting is done."]
    #[inline(always)]
    pub fn spi_mem_trans_end(&self) -> SPI_MEM_TRANS_END_R {
        SPI_MEM_TRANS_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to enable the interrupt of spi0_mst_st controlled transmitting is done."]
    #[inline(always)]
    pub fn spi_mem_trans_end_int_ena(&self) -> SPI_MEM_TRANS_END_INT_ENA_R {
        SPI_MEM_TRANS_END_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit is used to indicate the spi0_slv_st controlled transmitting is done."]
    #[inline(always)]
    pub fn spi_mem_cspi_st_trans_end(&self) -> SPI_MEM_CSPI_ST_TRANS_END_R {
        SPI_MEM_CSPI_ST_TRANS_END_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The bit is used to enable the interrupt of spi0_slv_st controlled transmitting is done."]
    #[inline(always)]
    pub fn spi_mem_cspi_st_trans_end_int_ena(&self) -> SPI_MEM_CSPI_ST_TRANS_END_INT_ENA_R {
        SPI_MEM_CSPI_ST_TRANS_END_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn spi_mem_ck_idle_edge(&self) -> SPI_MEM_CK_IDLE_EDGE_R {
        SPI_MEM_CK_IDLE_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - spi cs line keep low when the bit is set."]
    #[inline(always)]
    pub fn spi_mem_cs_keep_active(&self) -> SPI_MEM_CS_KEEP_ACTIVE_R {
        SPI_MEM_CS_KEEP_ACTIVE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - The bit is used to indicate the spi0_mst_st controlled transmitting is done."]
    #[inline(always)]
    pub fn spi_mem_trans_end(&mut self) -> SPI_MEM_TRANS_END_W {
        SPI_MEM_TRANS_END_W::new(self)
    }
    #[doc = "Bit 4 - The bit is used to enable the interrupt of spi0_mst_st controlled transmitting is done."]
    #[inline(always)]
    pub fn spi_mem_trans_end_int_ena(&mut self) -> SPI_MEM_TRANS_END_INT_ENA_W {
        SPI_MEM_TRANS_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - The bit is used to indicate the spi0_slv_st controlled transmitting is done."]
    #[inline(always)]
    pub fn spi_mem_cspi_st_trans_end(&mut self) -> SPI_MEM_CSPI_ST_TRANS_END_W {
        SPI_MEM_CSPI_ST_TRANS_END_W::new(self)
    }
    #[doc = "Bit 6 - The bit is used to enable the interrupt of spi0_slv_st controlled transmitting is done."]
    #[inline(always)]
    pub fn spi_mem_cspi_st_trans_end_int_ena(&mut self) -> SPI_MEM_CSPI_ST_TRANS_END_INT_ENA_W {
        SPI_MEM_CSPI_ST_TRANS_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 9 - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn spi_mem_ck_idle_edge(&mut self) -> SPI_MEM_CK_IDLE_EDGE_W {
        SPI_MEM_CK_IDLE_EDGE_W::new(self)
    }
    #[doc = "Bit 10 - spi cs line keep low when the bit is set."]
    #[inline(always)]
    pub fn spi_mem_cs_keep_active(&mut self) -> SPI_MEM_CS_KEEP_ACTIVE_W {
        SPI_MEM_CS_KEEP_ACTIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 misc register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_misc](index.html) module"]
pub struct SPI_MEM_MISC_SPEC;
impl crate::RegisterSpec for SPI_MEM_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_misc::R](R) reader structure"]
impl crate::Readable for SPI_MEM_MISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_misc::W](W) writer structure"]
impl crate::Writable for SPI_MEM_MISC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_MEM_MISC to value 0"]
impl crate::Resettable for SPI_MEM_MISC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
