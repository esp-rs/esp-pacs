#[doc = "Register `MISC` reader"]
pub type R = crate::R<MISC_SPEC>;
#[doc = "Register `MISC` writer"]
pub type W = crate::W<MISC_SPEC>;
#[doc = "Field `TRANS_END` reader - The bit is used to indicate the spi0_mst_st controlled transmitting is done."]
pub type TRANS_END_R = crate::BitReader;
#[doc = "Field `TRANS_END` writer - The bit is used to indicate the spi0_mst_st controlled transmitting is done."]
pub type TRANS_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_END_INT_ENA` reader - The bit is used to enable the interrupt of spi0_mst_st controlled transmitting is done."]
pub type TRANS_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `TRANS_END_INT_ENA` writer - The bit is used to enable the interrupt of spi0_mst_st controlled transmitting is done."]
pub type TRANS_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSPI_ST_TRANS_END` reader - The bit is used to indicate the spi0_slv_st controlled transmitting is done."]
pub type CSPI_ST_TRANS_END_R = crate::BitReader;
#[doc = "Field `CSPI_ST_TRANS_END` writer - The bit is used to indicate the spi0_slv_st controlled transmitting is done."]
pub type CSPI_ST_TRANS_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSPI_ST_TRANS_END_INT_ENA` reader - The bit is used to enable the interrupt of spi0_slv_st controlled transmitting is done."]
pub type CSPI_ST_TRANS_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `CSPI_ST_TRANS_END_INT_ENA` writer - The bit is used to enable the interrupt of spi0_slv_st controlled transmitting is done."]
pub type CSPI_ST_TRANS_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_IDLE_EDGE` reader - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
pub type CK_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `CK_IDLE_EDGE` writer - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
pub type CK_IDLE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_KEEP_ACTIVE` reader - spi cs line keep low when the bit is set."]
pub type CS_KEEP_ACTIVE_R = crate::BitReader;
#[doc = "Field `CS_KEEP_ACTIVE` writer - spi cs line keep low when the bit is set."]
pub type CS_KEEP_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - The bit is used to indicate the spi0_mst_st controlled transmitting is done."]
    #[inline(always)]
    pub fn trans_end(&self) -> TRANS_END_R {
        TRANS_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The bit is used to enable the interrupt of spi0_mst_st controlled transmitting is done."]
    #[inline(always)]
    pub fn trans_end_int_ena(&self) -> TRANS_END_INT_ENA_R {
        TRANS_END_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit is used to indicate the spi0_slv_st controlled transmitting is done."]
    #[inline(always)]
    pub fn cspi_st_trans_end(&self) -> CSPI_ST_TRANS_END_R {
        CSPI_ST_TRANS_END_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The bit is used to enable the interrupt of spi0_slv_st controlled transmitting is done."]
    #[inline(always)]
    pub fn cspi_st_trans_end_int_ena(&self) -> CSPI_ST_TRANS_END_INT_ENA_R {
        CSPI_ST_TRANS_END_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn ck_idle_edge(&self) -> CK_IDLE_EDGE_R {
        CK_IDLE_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - spi cs line keep low when the bit is set."]
    #[inline(always)]
    pub fn cs_keep_active(&self) -> CS_KEEP_ACTIVE_R {
        CS_KEEP_ACTIVE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC")
            .field("trans_end", &format_args!("{}", self.trans_end().bit()))
            .field(
                "trans_end_int_ena",
                &format_args!("{}", self.trans_end_int_ena().bit()),
            )
            .field(
                "cspi_st_trans_end",
                &format_args!("{}", self.cspi_st_trans_end().bit()),
            )
            .field(
                "cspi_st_trans_end_int_ena",
                &format_args!("{}", self.cspi_st_trans_end_int_ena().bit()),
            )
            .field(
                "ck_idle_edge",
                &format_args!("{}", self.ck_idle_edge().bit()),
            )
            .field(
                "cs_keep_active",
                &format_args!("{}", self.cs_keep_active().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MISC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 3 - The bit is used to indicate the spi0_mst_st controlled transmitting is done."]
    #[inline(always)]
    #[must_use]
    pub fn trans_end(&mut self) -> TRANS_END_W<MISC_SPEC> {
        TRANS_END_W::new(self, 3)
    }
    #[doc = "Bit 4 - The bit is used to enable the interrupt of spi0_mst_st controlled transmitting is done."]
    #[inline(always)]
    #[must_use]
    pub fn trans_end_int_ena(&mut self) -> TRANS_END_INT_ENA_W<MISC_SPEC> {
        TRANS_END_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - The bit is used to indicate the spi0_slv_st controlled transmitting is done."]
    #[inline(always)]
    #[must_use]
    pub fn cspi_st_trans_end(&mut self) -> CSPI_ST_TRANS_END_W<MISC_SPEC> {
        CSPI_ST_TRANS_END_W::new(self, 5)
    }
    #[doc = "Bit 6 - The bit is used to enable the interrupt of spi0_slv_st controlled transmitting is done."]
    #[inline(always)]
    #[must_use]
    pub fn cspi_st_trans_end_int_ena(&mut self) -> CSPI_ST_TRANS_END_INT_ENA_W<MISC_SPEC> {
        CSPI_ST_TRANS_END_INT_ENA_W::new(self, 6)
    }
    #[doc = "Bit 9 - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    #[must_use]
    pub fn ck_idle_edge(&mut self) -> CK_IDLE_EDGE_W<MISC_SPEC> {
        CK_IDLE_EDGE_W::new(self, 9)
    }
    #[doc = "Bit 10 - spi cs line keep low when the bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn cs_keep_active(&mut self) -> CS_KEEP_ACTIVE_W<MISC_SPEC> {
        CS_KEEP_ACTIVE_W::new(self, 10)
    }
}
#[doc = "SPI0 misc register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISC_SPEC;
impl crate::RegisterSpec for MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc::R`](R) reader structure"]
impl crate::Readable for MISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`misc::W`](W) writer structure"]
impl crate::Writable for MISC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MISC to value 0"]
impl crate::Resettable for MISC_SPEC {
    const RESET_VALUE: u32 = 0;
}
