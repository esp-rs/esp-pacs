#[doc = "Register `SPI_MEM_USER` reader"]
pub struct R(crate::R<SPI_MEM_USER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_USER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_USER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_USER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_USER` writer"]
pub struct W(crate::W<SPI_MEM_USER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_USER_SPEC>;
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
impl From<crate::W<SPI_MEM_USER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_USER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_CK_OUT_EDGE` reader - the bit combined with spi_mem_mosi_delay_mode bits to set mosi signal delay mode."]
pub type SPI_MEM_CK_OUT_EDGE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_CK_OUT_EDGE` writer - the bit combined with spi_mem_mosi_delay_mode bits to set mosi signal delay mode."]
pub type SPI_MEM_CK_OUT_EDGE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_USER_SPEC, O>;
#[doc = "Field `SPI_MEM_FWRITE_DUAL` reader - In the write operations read-data phase apply 2 signals"]
pub type SPI_MEM_FWRITE_DUAL_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FWRITE_DUAL` writer - In the write operations read-data phase apply 2 signals"]
pub type SPI_MEM_FWRITE_DUAL_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_USER_SPEC, O>;
#[doc = "Field `SPI_MEM_FWRITE_QUAD` reader - In the write operations read-data phase apply 4 signals"]
pub type SPI_MEM_FWRITE_QUAD_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FWRITE_QUAD` writer - In the write operations read-data phase apply 4 signals"]
pub type SPI_MEM_FWRITE_QUAD_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_USER_SPEC, O>;
#[doc = "Field `SPI_MEM_FWRITE_DIO` reader - In the write operations address phase and read-data phase apply 2 signals."]
pub type SPI_MEM_FWRITE_DIO_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FWRITE_DIO` writer - In the write operations address phase and read-data phase apply 2 signals."]
pub type SPI_MEM_FWRITE_DIO_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_USER_SPEC, O>;
#[doc = "Field `SPI_MEM_FWRITE_QIO` reader - In the write operations address phase and read-data phase apply 4 signals."]
pub type SPI_MEM_FWRITE_QIO_R = crate::BitReader;
#[doc = "Field `SPI_MEM_FWRITE_QIO` writer - In the write operations address phase and read-data phase apply 4 signals."]
pub type SPI_MEM_FWRITE_QIO_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_USER_SPEC, O>;
#[doc = "Field `SPI_MEM_USR_MISO_HIGHPART` reader - read-data phase only access to high-part of the buffer spi_mem_w8~spi_mem_w15. 1: enable 0: disable."]
pub type SPI_MEM_USR_MISO_HIGHPART_R = crate::BitReader;
#[doc = "Field `SPI_MEM_USR_MOSI_HIGHPART` reader - write-data phase only access to high-part of the buffer spi_mem_w8~spi_mem_w15. 1: enable 0: disable."]
pub type SPI_MEM_USR_MOSI_HIGHPART_R = crate::BitReader;
#[doc = "Field `SPI_MEM_USR_DUMMY_IDLE` reader - SPI clock is disable in dummy phase when the bit is enable."]
pub type SPI_MEM_USR_DUMMY_IDLE_R = crate::BitReader;
#[doc = "Field `SPI_MEM_USR_DUMMY_IDLE` writer - SPI clock is disable in dummy phase when the bit is enable."]
pub type SPI_MEM_USR_DUMMY_IDLE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_USER_SPEC, O>;
#[doc = "Field `SPI_MEM_USR_MOSI` reader - This bit enable the write-data phase of an operation."]
pub type SPI_MEM_USR_MOSI_R = crate::BitReader;
#[doc = "Field `SPI_MEM_USR_MOSI` writer - This bit enable the write-data phase of an operation."]
pub type SPI_MEM_USR_MOSI_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_USER_SPEC, O>;
#[doc = "Field `SPI_MEM_USR_MISO` reader - This bit enable the read-data phase of an operation."]
pub type SPI_MEM_USR_MISO_R = crate::BitReader;
#[doc = "Field `SPI_MEM_USR_MISO` writer - This bit enable the read-data phase of an operation."]
pub type SPI_MEM_USR_MISO_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_USER_SPEC, O>;
#[doc = "Field `SPI_MEM_USR_DUMMY` reader - This bit enable the dummy phase of an operation."]
pub type SPI_MEM_USR_DUMMY_R = crate::BitReader;
#[doc = "Field `SPI_MEM_USR_DUMMY` writer - This bit enable the dummy phase of an operation."]
pub type SPI_MEM_USR_DUMMY_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_USER_SPEC, O>;
#[doc = "Field `SPI_MEM_USR_ADDR` reader - This bit enable the address phase of an operation."]
pub type SPI_MEM_USR_ADDR_R = crate::BitReader;
#[doc = "Field `SPI_MEM_USR_ADDR` writer - This bit enable the address phase of an operation."]
pub type SPI_MEM_USR_ADDR_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_USER_SPEC, O>;
#[doc = "Field `SPI_MEM_USR_COMMAND` reader - This bit enable the command phase of an operation."]
pub type SPI_MEM_USR_COMMAND_R = crate::BitReader;
#[doc = "Field `SPI_MEM_USR_COMMAND` writer - This bit enable the command phase of an operation."]
pub type SPI_MEM_USR_COMMAND_W<'a, const O: u8> = crate::BitWriter<'a, SPI_MEM_USER_SPEC, O>;
impl R {
    #[doc = "Bit 9 - the bit combined with spi_mem_mosi_delay_mode bits to set mosi signal delay mode."]
    #[inline(always)]
    pub fn spi_mem_ck_out_edge(&self) -> SPI_MEM_CK_OUT_EDGE_R {
        SPI_MEM_CK_OUT_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - In the write operations read-data phase apply 2 signals"]
    #[inline(always)]
    pub fn spi_mem_fwrite_dual(&self) -> SPI_MEM_FWRITE_DUAL_R {
        SPI_MEM_FWRITE_DUAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - In the write operations read-data phase apply 4 signals"]
    #[inline(always)]
    pub fn spi_mem_fwrite_quad(&self) -> SPI_MEM_FWRITE_QUAD_R {
        SPI_MEM_FWRITE_QUAD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - In the write operations address phase and read-data phase apply 2 signals."]
    #[inline(always)]
    pub fn spi_mem_fwrite_dio(&self) -> SPI_MEM_FWRITE_DIO_R {
        SPI_MEM_FWRITE_DIO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - In the write operations address phase and read-data phase apply 4 signals."]
    #[inline(always)]
    pub fn spi_mem_fwrite_qio(&self) -> SPI_MEM_FWRITE_QIO_R {
        SPI_MEM_FWRITE_QIO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - read-data phase only access to high-part of the buffer spi_mem_w8~spi_mem_w15. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_usr_miso_highpart(&self) -> SPI_MEM_USR_MISO_HIGHPART_R {
        SPI_MEM_USR_MISO_HIGHPART_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - write-data phase only access to high-part of the buffer spi_mem_w8~spi_mem_w15. 1: enable 0: disable."]
    #[inline(always)]
    pub fn spi_mem_usr_mosi_highpart(&self) -> SPI_MEM_USR_MOSI_HIGHPART_R {
        SPI_MEM_USR_MOSI_HIGHPART_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SPI clock is disable in dummy phase when the bit is enable."]
    #[inline(always)]
    pub fn spi_mem_usr_dummy_idle(&self) -> SPI_MEM_USR_DUMMY_IDLE_R {
        SPI_MEM_USR_DUMMY_IDLE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - This bit enable the write-data phase of an operation."]
    #[inline(always)]
    pub fn spi_mem_usr_mosi(&self) -> SPI_MEM_USR_MOSI_R {
        SPI_MEM_USR_MOSI_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - This bit enable the read-data phase of an operation."]
    #[inline(always)]
    pub fn spi_mem_usr_miso(&self) -> SPI_MEM_USR_MISO_R {
        SPI_MEM_USR_MISO_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit enable the dummy phase of an operation."]
    #[inline(always)]
    pub fn spi_mem_usr_dummy(&self) -> SPI_MEM_USR_DUMMY_R {
        SPI_MEM_USR_DUMMY_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This bit enable the address phase of an operation."]
    #[inline(always)]
    pub fn spi_mem_usr_addr(&self) -> SPI_MEM_USR_ADDR_R {
        SPI_MEM_USR_ADDR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit enable the command phase of an operation."]
    #[inline(always)]
    pub fn spi_mem_usr_command(&self) -> SPI_MEM_USR_COMMAND_R {
        SPI_MEM_USR_COMMAND_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_USER")
            .field(
                "spi_mem_ck_out_edge",
                &format_args!("{}", self.spi_mem_ck_out_edge().bit()),
            )
            .field(
                "spi_mem_fwrite_dual",
                &format_args!("{}", self.spi_mem_fwrite_dual().bit()),
            )
            .field(
                "spi_mem_fwrite_quad",
                &format_args!("{}", self.spi_mem_fwrite_quad().bit()),
            )
            .field(
                "spi_mem_fwrite_dio",
                &format_args!("{}", self.spi_mem_fwrite_dio().bit()),
            )
            .field(
                "spi_mem_fwrite_qio",
                &format_args!("{}", self.spi_mem_fwrite_qio().bit()),
            )
            .field(
                "spi_mem_usr_miso_highpart",
                &format_args!("{}", self.spi_mem_usr_miso_highpart().bit()),
            )
            .field(
                "spi_mem_usr_mosi_highpart",
                &format_args!("{}", self.spi_mem_usr_mosi_highpart().bit()),
            )
            .field(
                "spi_mem_usr_dummy_idle",
                &format_args!("{}", self.spi_mem_usr_dummy_idle().bit()),
            )
            .field(
                "spi_mem_usr_mosi",
                &format_args!("{}", self.spi_mem_usr_mosi().bit()),
            )
            .field(
                "spi_mem_usr_miso",
                &format_args!("{}", self.spi_mem_usr_miso().bit()),
            )
            .field(
                "spi_mem_usr_dummy",
                &format_args!("{}", self.spi_mem_usr_dummy().bit()),
            )
            .field(
                "spi_mem_usr_addr",
                &format_args!("{}", self.spi_mem_usr_addr().bit()),
            )
            .field(
                "spi_mem_usr_command",
                &format_args!("{}", self.spi_mem_usr_command().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_USER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 9 - the bit combined with spi_mem_mosi_delay_mode bits to set mosi signal delay mode."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_ck_out_edge(&mut self) -> SPI_MEM_CK_OUT_EDGE_W<9> {
        SPI_MEM_CK_OUT_EDGE_W::new(self)
    }
    #[doc = "Bit 12 - In the write operations read-data phase apply 2 signals"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fwrite_dual(&mut self) -> SPI_MEM_FWRITE_DUAL_W<12> {
        SPI_MEM_FWRITE_DUAL_W::new(self)
    }
    #[doc = "Bit 13 - In the write operations read-data phase apply 4 signals"]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fwrite_quad(&mut self) -> SPI_MEM_FWRITE_QUAD_W<13> {
        SPI_MEM_FWRITE_QUAD_W::new(self)
    }
    #[doc = "Bit 14 - In the write operations address phase and read-data phase apply 2 signals."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fwrite_dio(&mut self) -> SPI_MEM_FWRITE_DIO_W<14> {
        SPI_MEM_FWRITE_DIO_W::new(self)
    }
    #[doc = "Bit 15 - In the write operations address phase and read-data phase apply 4 signals."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_fwrite_qio(&mut self) -> SPI_MEM_FWRITE_QIO_W<15> {
        SPI_MEM_FWRITE_QIO_W::new(self)
    }
    #[doc = "Bit 26 - SPI clock is disable in dummy phase when the bit is enable."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_usr_dummy_idle(&mut self) -> SPI_MEM_USR_DUMMY_IDLE_W<26> {
        SPI_MEM_USR_DUMMY_IDLE_W::new(self)
    }
    #[doc = "Bit 27 - This bit enable the write-data phase of an operation."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_usr_mosi(&mut self) -> SPI_MEM_USR_MOSI_W<27> {
        SPI_MEM_USR_MOSI_W::new(self)
    }
    #[doc = "Bit 28 - This bit enable the read-data phase of an operation."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_usr_miso(&mut self) -> SPI_MEM_USR_MISO_W<28> {
        SPI_MEM_USR_MISO_W::new(self)
    }
    #[doc = "Bit 29 - This bit enable the dummy phase of an operation."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_usr_dummy(&mut self) -> SPI_MEM_USR_DUMMY_W<29> {
        SPI_MEM_USR_DUMMY_W::new(self)
    }
    #[doc = "Bit 30 - This bit enable the address phase of an operation."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_usr_addr(&mut self) -> SPI_MEM_USR_ADDR_W<30> {
        SPI_MEM_USR_ADDR_W::new(self)
    }
    #[doc = "Bit 31 - This bit enable the command phase of an operation."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_usr_command(&mut self) -> SPI_MEM_USR_COMMAND_W<31> {
        SPI_MEM_USR_COMMAND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 user register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_user](index.html) module"]
pub struct SPI_MEM_USER_SPEC;
impl crate::RegisterSpec for SPI_MEM_USER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_user::R](R) reader structure"]
impl crate::Readable for SPI_MEM_USER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_user::W](W) writer structure"]
impl crate::Writable for SPI_MEM_USER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_USER to value 0x8000_0000"]
impl crate::Resettable for SPI_MEM_USER_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
