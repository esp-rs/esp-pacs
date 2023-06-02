#[doc = "Register `SPI_MEM_CACHE_FCTRL` reader"]
pub struct R(crate::R<SPI_MEM_CACHE_FCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_CACHE_FCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_CACHE_FCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_CACHE_FCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_CACHE_FCTRL` writer"]
pub struct W(crate::W<SPI_MEM_CACHE_FCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_CACHE_FCTRL_SPEC>;
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
impl From<crate::W<SPI_MEM_CACHE_FCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_CACHE_FCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_AXI_REQ_EN` reader - For SPI0, AXI master access enable, 1: enable, 0:disable."]
pub type SPI_MEM_AXI_REQ_EN_R = crate::BitReader;
#[doc = "Field `SPI_MEM_AXI_REQ_EN` writer - For SPI0, AXI master access enable, 1: enable, 0:disable."]
pub type SPI_MEM_AXI_REQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CACHE_FCTRL_SPEC, O>;
#[doc = "Field `SPI_MEM_CACHE_USR_ADDR_4BYTE` reader - For SPI0, cache read flash with 4 bytes address, 1: enable, 0:disable."]
pub type SPI_MEM_CACHE_USR_ADDR_4BYTE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_CACHE_USR_ADDR_4BYTE` writer - For SPI0, cache read flash with 4 bytes address, 1: enable, 0:disable."]
pub type SPI_MEM_CACHE_USR_ADDR_4BYTE_W<'a, const O: u8> =
    crate::BitWriter<'a, SPI_MEM_CACHE_FCTRL_SPEC, O>;
#[doc = "Field `SPI_MEM_CACHE_FLASH_USR_CMD` reader - For SPI0, cache read flash for user define command, 1: enable, 0:disable."]
pub type SPI_MEM_CACHE_FLASH_USR_CMD_R = crate::BitReader;
#[doc = "Field `SPI_MEM_CACHE_FLASH_USR_CMD` writer - For SPI0, cache read flash for user define command, 1: enable, 0:disable."]
pub type SPI_MEM_CACHE_FLASH_USR_CMD_W<'a, const O: u8> =
    crate::BitWriter<'a, SPI_MEM_CACHE_FCTRL_SPEC, O>;
#[doc = "Field `SPI_MEM_FDIN_DUAL` reader - For SPI0 flash, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type SPI_MEM_FDIN_DUAL_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FDIN_DUAL` writer - For SPI0 flash, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type SPI_MEM_FDIN_DUAL_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CACHE_FCTRL_SPEC, O>;
#[doc = "Field `SPI_MEM_FDOUT_DUAL` reader - For SPI0 flash, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type SPI_MEM_FDOUT_DUAL_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FDOUT_DUAL` writer - For SPI0 flash, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type SPI_MEM_FDOUT_DUAL_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CACHE_FCTRL_SPEC, O>;
#[doc = "Field `SPI_MEM_FADDR_DUAL` reader - For SPI0 flash, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type SPI_MEM_FADDR_DUAL_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FADDR_DUAL` writer - For SPI0 flash, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
pub type SPI_MEM_FADDR_DUAL_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CACHE_FCTRL_SPEC, O>;
#[doc = "Field `SPI_MEM_FDIN_QUAD` reader - For SPI0 flash, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type SPI_MEM_FDIN_QUAD_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FDIN_QUAD` writer - For SPI0 flash, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type SPI_MEM_FDIN_QUAD_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CACHE_FCTRL_SPEC, O>;
#[doc = "Field `SPI_MEM_FDOUT_QUAD` reader - For SPI0 flash, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type SPI_MEM_FDOUT_QUAD_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FDOUT_QUAD` writer - For SPI0 flash, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type SPI_MEM_FDOUT_QUAD_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CACHE_FCTRL_SPEC, O>;
#[doc = "Field `SPI_MEM_FADDR_QUAD` reader - For SPI0 flash, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type SPI_MEM_FADDR_QUAD_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FADDR_QUAD` writer - For SPI0 flash, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
pub type SPI_MEM_FADDR_QUAD_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_CACHE_FCTRL_SPEC, O>;
#[doc = "Field `SPI_SAME_AW_AR_ADDR_CHK_EN` reader - Set this bit to check AXI read/write the same address region."]
pub type SPI_SAME_AW_AR_ADDR_CHK_EN_R = crate::BitReader;
#[doc = "Field `SPI_CLOSE_AXI_INF_EN` reader - Set this bit to close AXI read/write transfer to MSPI, which means that only SLV_ERR will be replied to BRESP/RRESP."]
pub type SPI_CLOSE_AXI_INF_EN_R = crate::BitReader;
#[doc = "Field `SPI_CLOSE_AXI_INF_EN` writer - Set this bit to close AXI read/write transfer to MSPI, which means that only SLV_ERR will be replied to BRESP/RRESP."]
pub type SPI_CLOSE_AXI_INF_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, SPI_MEM_CACHE_FCTRL_SPEC, O>;
impl R {
    #[doc = "Bit 0 - For SPI0, AXI master access enable, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn spi_mem_axi_req_en(&self) -> SPI_MEM_AXI_REQ_EN_R {
        SPI_MEM_AXI_REQ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - For SPI0, cache read flash with 4 bytes address, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn spi_mem_cache_usr_addr_4byte(&self) -> SPI_MEM_CACHE_USR_ADDR_4BYTE_R {
        SPI_MEM_CACHE_USR_ADDR_4BYTE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - For SPI0, cache read flash for user define command, 1: enable, 0:disable."]
    #[inline(always)]
    pub fn spi_mem_cache_flash_usr_cmd(&self) -> SPI_MEM_CACHE_FLASH_USR_CMD_R {
        SPI_MEM_CACHE_FLASH_USR_CMD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - For SPI0 flash, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    pub fn spi_mem_fdin_dual(&self) -> SPI_MEM_FDIN_DUAL_R {
        SPI_MEM_FDIN_DUAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - For SPI0 flash, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    pub fn spi_mem_fdout_dual(&self) -> SPI_MEM_FDOUT_DUAL_R {
        SPI_MEM_FDOUT_DUAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - For SPI0 flash, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    pub fn spi_mem_faddr_dual(&self) -> SPI_MEM_FADDR_DUAL_R {
        SPI_MEM_FADDR_DUAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - For SPI0 flash, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    pub fn spi_mem_fdin_quad(&self) -> SPI_MEM_FDIN_QUAD_R {
        SPI_MEM_FDIN_QUAD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - For SPI0 flash, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    pub fn spi_mem_fdout_quad(&self) -> SPI_MEM_FDOUT_QUAD_R {
        SPI_MEM_FDOUT_QUAD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - For SPI0 flash, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    pub fn spi_mem_faddr_quad(&self) -> SPI_MEM_FADDR_QUAD_R {
        SPI_MEM_FADDR_QUAD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 30 - Set this bit to check AXI read/write the same address region."]
    #[inline(always)]
    pub fn spi_same_aw_ar_addr_chk_en(&self) -> SPI_SAME_AW_AR_ADDR_CHK_EN_R {
        SPI_SAME_AW_AR_ADDR_CHK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set this bit to close AXI read/write transfer to MSPI, which means that only SLV_ERR will be replied to BRESP/RRESP."]
    #[inline(always)]
    pub fn spi_close_axi_inf_en(&self) -> SPI_CLOSE_AXI_INF_EN_R {
        SPI_CLOSE_AXI_INF_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_CACHE_FCTRL")
            .field(
                "spi_mem_axi_req_en",
                &format_args!("{}", self.spi_mem_axi_req_en().bit()),
            )
            .field(
                "spi_mem_cache_usr_addr_4byte",
                &format_args!("{}", self.spi_mem_cache_usr_addr_4byte().bit()),
            )
            .field(
                "spi_mem_cache_flash_usr_cmd",
                &format_args!("{}", self.spi_mem_cache_flash_usr_cmd().bit()),
            )
            .field(
                "spi_mem_fdin_dual",
                &format_args!("{}", self.spi_mem_fdin_dual().bit()),
            )
            .field(
                "spi_mem_fdout_dual",
                &format_args!("{}", self.spi_mem_fdout_dual().bit()),
            )
            .field(
                "spi_mem_faddr_dual",
                &format_args!("{}", self.spi_mem_faddr_dual().bit()),
            )
            .field(
                "spi_mem_fdin_quad",
                &format_args!("{}", self.spi_mem_fdin_quad().bit()),
            )
            .field(
                "spi_mem_fdout_quad",
                &format_args!("{}", self.spi_mem_fdout_quad().bit()),
            )
            .field(
                "spi_mem_faddr_quad",
                &format_args!("{}", self.spi_mem_faddr_quad().bit()),
            )
            .field(
                "spi_same_aw_ar_addr_chk_en",
                &format_args!("{}", self.spi_same_aw_ar_addr_chk_en().bit()),
            )
            .field(
                "spi_close_axi_inf_en",
                &format_args!("{}", self.spi_close_axi_inf_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_CACHE_FCTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - For SPI0, AXI master access enable, 1: enable, 0:disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_axi_req_en(&mut self) -> SPI_MEM_AXI_REQ_EN_W<0> {
        SPI_MEM_AXI_REQ_EN_W::new(self)
    }
    #[doc = "Bit 1 - For SPI0, cache read flash with 4 bytes address, 1: enable, 0:disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_cache_usr_addr_4byte(&mut self) -> SPI_MEM_CACHE_USR_ADDR_4BYTE_W<1> {
        SPI_MEM_CACHE_USR_ADDR_4BYTE_W::new(self)
    }
    #[doc = "Bit 2 - For SPI0, cache read flash for user define command, 1: enable, 0:disable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_cache_flash_usr_cmd(&mut self) -> SPI_MEM_CACHE_FLASH_USR_CMD_W<2> {
        SPI_MEM_CACHE_FLASH_USR_CMD_W::new(self)
    }
    #[doc = "Bit 3 - For SPI0 flash, din phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fdin_dual(&mut self) -> SPI_MEM_FDIN_DUAL_W<3> {
        SPI_MEM_FDIN_DUAL_W::new(self)
    }
    #[doc = "Bit 4 - For SPI0 flash, dout phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fdout_dual(&mut self) -> SPI_MEM_FDOUT_DUAL_W<4> {
        SPI_MEM_FDOUT_DUAL_W::new(self)
    }
    #[doc = "Bit 5 - For SPI0 flash, address phase apply 2 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_dio."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_faddr_dual(&mut self) -> SPI_MEM_FADDR_DUAL_W<5> {
        SPI_MEM_FADDR_DUAL_W::new(self)
    }
    #[doc = "Bit 6 - For SPI0 flash, din phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fdin_quad(&mut self) -> SPI_MEM_FDIN_QUAD_W<6> {
        SPI_MEM_FDIN_QUAD_W::new(self)
    }
    #[doc = "Bit 7 - For SPI0 flash, dout phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fdout_quad(&mut self) -> SPI_MEM_FDOUT_QUAD_W<7> {
        SPI_MEM_FDOUT_QUAD_W::new(self)
    }
    #[doc = "Bit 8 - For SPI0 flash, address phase apply 4 signals. 1: enable 0: disable. The bit is the same with spi_mem_fread_qio."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_faddr_quad(&mut self) -> SPI_MEM_FADDR_QUAD_W<8> {
        SPI_MEM_FADDR_QUAD_W::new(self)
    }
    #[doc = "Bit 31 - Set this bit to close AXI read/write transfer to MSPI, which means that only SLV_ERR will be replied to BRESP/RRESP."]
    #[inline(always)]
    #[must_use]
    pub fn spi_close_axi_inf_en(&mut self) -> SPI_CLOSE_AXI_INF_EN_W<31> {
        SPI_CLOSE_AXI_INF_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI0 bit mode control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_cache_fctrl](index.html) module"]
pub struct SPI_MEM_CACHE_FCTRL_SPEC;
impl crate::RegisterSpec for SPI_MEM_CACHE_FCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_cache_fctrl::R](R) reader structure"]
impl crate::Readable for SPI_MEM_CACHE_FCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_cache_fctrl::W](W) writer structure"]
impl crate::Writable for SPI_MEM_CACHE_FCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_CACHE_FCTRL to value 0xc000_0000"]
impl crate::Resettable for SPI_MEM_CACHE_FCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0xc000_0000;
}
