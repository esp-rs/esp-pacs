#[doc = "Register `USER` reader"]
pub struct R(crate::R<USER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USER` writer"]
pub struct W(crate::W<USER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USER_SPEC>;
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
impl From<crate::W<USER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CK_OUT_EDGE` reader - This bit, combined with SPI_MEM_CK_IDLE_EDGE bit, is used to change the clock mode 0~3 of SPI_CLK."]
pub type CK_OUT_EDGE_R = crate::BitReader;
#[doc = "Field `CK_OUT_EDGE` writer - This bit, combined with SPI_MEM_CK_IDLE_EDGE bit, is used to change the clock mode 0~3 of SPI_CLK."]
pub type CK_OUT_EDGE_W<'a, const O: u8> = crate::BitWriter<'a, USER_SPEC, O>;
#[doc = "Field `FWRITE_DUAL` reader - Set this bit to enable 2-bm in DOUT phase in SPI1 write operation."]
pub type FWRITE_DUAL_R = crate::BitReader;
#[doc = "Field `FWRITE_DUAL` writer - Set this bit to enable 2-bm in DOUT phase in SPI1 write operation."]
pub type FWRITE_DUAL_W<'a, const O: u8> = crate::BitWriter<'a, USER_SPEC, O>;
#[doc = "Field `FWRITE_QUAD` reader - Set this bit to enable 4-bm in DOUT phase in SPI1 write operation."]
pub type FWRITE_QUAD_R = crate::BitReader;
#[doc = "Field `FWRITE_QUAD` writer - Set this bit to enable 4-bm in DOUT phase in SPI1 write operation."]
pub type FWRITE_QUAD_W<'a, const O: u8> = crate::BitWriter<'a, USER_SPEC, O>;
#[doc = "Field `FWRITE_DIO` reader - Set this bit to enable 2-bm in ADDR and DOUT phase in SPI1 write operation."]
pub type FWRITE_DIO_R = crate::BitReader;
#[doc = "Field `FWRITE_DIO` writer - Set this bit to enable 2-bm in ADDR and DOUT phase in SPI1 write operation."]
pub type FWRITE_DIO_W<'a, const O: u8> = crate::BitWriter<'a, USER_SPEC, O>;
#[doc = "Field `FWRITE_QIO` reader - Set this bit to enable 4-bit-mode(4-bm) in ADDR and DOUT phase in SPI1 write operation."]
pub type FWRITE_QIO_R = crate::BitReader;
#[doc = "Field `FWRITE_QIO` writer - Set this bit to enable 4-bit-mode(4-bm) in ADDR and DOUT phase in SPI1 write operation."]
pub type FWRITE_QIO_W<'a, const O: u8> = crate::BitWriter<'a, USER_SPEC, O>;
#[doc = "Field `USR_MISO_HIGHPART` reader - DIN phase only access to high-part of the buffer SPI_MEM_W8_REG~SPI_MEM_W15_REG. 1: enable 0: disable."]
pub type USR_MISO_HIGHPART_R = crate::BitReader;
#[doc = "Field `USR_MISO_HIGHPART` writer - DIN phase only access to high-part of the buffer SPI_MEM_W8_REG~SPI_MEM_W15_REG. 1: enable 0: disable."]
pub type USR_MISO_HIGHPART_W<'a, const O: u8> = crate::BitWriter<'a, USER_SPEC, O>;
#[doc = "Field `USR_MOSI_HIGHPART` reader - DOUT phase only access to high-part of the buffer SPI_MEM_W8_REG~SPI_MEM_W15_REG. 1: enable 0: disable."]
pub type USR_MOSI_HIGHPART_R = crate::BitReader;
#[doc = "Field `USR_MOSI_HIGHPART` writer - DOUT phase only access to high-part of the buffer SPI_MEM_W8_REG~SPI_MEM_W15_REG. 1: enable 0: disable."]
pub type USR_MOSI_HIGHPART_W<'a, const O: u8> = crate::BitWriter<'a, USER_SPEC, O>;
#[doc = "Field `USR_DUMMY_IDLE` reader - SPI_CLK is disabled(No clock edges) in DUMMY phase when the bit is enable."]
pub type USR_DUMMY_IDLE_R = crate::BitReader;
#[doc = "Field `USR_DUMMY_IDLE` writer - SPI_CLK is disabled(No clock edges) in DUMMY phase when the bit is enable."]
pub type USR_DUMMY_IDLE_W<'a, const O: u8> = crate::BitWriter<'a, USER_SPEC, O>;
#[doc = "Field `USR_MOSI` reader - Set this bit to enable the DOUT phase of an write-data operation."]
pub type USR_MOSI_R = crate::BitReader;
#[doc = "Field `USR_MOSI` writer - Set this bit to enable the DOUT phase of an write-data operation."]
pub type USR_MOSI_W<'a, const O: u8> = crate::BitWriter<'a, USER_SPEC, O>;
#[doc = "Field `USR_MISO` reader - Set this bit to enable enable the DIN phase of a read-data operation."]
pub type USR_MISO_R = crate::BitReader;
#[doc = "Field `USR_MISO` writer - Set this bit to enable enable the DIN phase of a read-data operation."]
pub type USR_MISO_W<'a, const O: u8> = crate::BitWriter<'a, USER_SPEC, O>;
#[doc = "Field `USR_DUMMY` reader - Set this bit to enable enable the DUMMY phase of an operation."]
pub type USR_DUMMY_R = crate::BitReader;
#[doc = "Field `USR_DUMMY` writer - Set this bit to enable enable the DUMMY phase of an operation."]
pub type USR_DUMMY_W<'a, const O: u8> = crate::BitWriter<'a, USER_SPEC, O>;
#[doc = "Field `USR_ADDR` reader - Set this bit to enable enable the ADDR phase of an operation."]
pub type USR_ADDR_R = crate::BitReader;
#[doc = "Field `USR_ADDR` writer - Set this bit to enable enable the ADDR phase of an operation."]
pub type USR_ADDR_W<'a, const O: u8> = crate::BitWriter<'a, USER_SPEC, O>;
#[doc = "Field `USR_COMMAND` reader - Set this bit to enable enable the CMD phase of an operation."]
pub type USR_COMMAND_R = crate::BitReader;
#[doc = "Field `USR_COMMAND` writer - Set this bit to enable enable the CMD phase of an operation."]
pub type USR_COMMAND_W<'a, const O: u8> = crate::BitWriter<'a, USER_SPEC, O>;
impl R {
    #[doc = "Bit 9 - This bit, combined with SPI_MEM_CK_IDLE_EDGE bit, is used to change the clock mode 0~3 of SPI_CLK."]
    #[inline(always)]
    pub fn ck_out_edge(&self) -> CK_OUT_EDGE_R {
        CK_OUT_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Set this bit to enable 2-bm in DOUT phase in SPI1 write operation."]
    #[inline(always)]
    pub fn fwrite_dual(&self) -> FWRITE_DUAL_R {
        FWRITE_DUAL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set this bit to enable 4-bm in DOUT phase in SPI1 write operation."]
    #[inline(always)]
    pub fn fwrite_quad(&self) -> FWRITE_QUAD_R {
        FWRITE_QUAD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set this bit to enable 2-bm in ADDR and DOUT phase in SPI1 write operation."]
    #[inline(always)]
    pub fn fwrite_dio(&self) -> FWRITE_DIO_R {
        FWRITE_DIO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set this bit to enable 4-bit-mode(4-bm) in ADDR and DOUT phase in SPI1 write operation."]
    #[inline(always)]
    pub fn fwrite_qio(&self) -> FWRITE_QIO_R {
        FWRITE_QIO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - DIN phase only access to high-part of the buffer SPI_MEM_W8_REG~SPI_MEM_W15_REG. 1: enable 0: disable."]
    #[inline(always)]
    pub fn usr_miso_highpart(&self) -> USR_MISO_HIGHPART_R {
        USR_MISO_HIGHPART_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DOUT phase only access to high-part of the buffer SPI_MEM_W8_REG~SPI_MEM_W15_REG. 1: enable 0: disable."]
    #[inline(always)]
    pub fn usr_mosi_highpart(&self) -> USR_MOSI_HIGHPART_R {
        USR_MOSI_HIGHPART_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SPI_CLK is disabled(No clock edges) in DUMMY phase when the bit is enable."]
    #[inline(always)]
    pub fn usr_dummy_idle(&self) -> USR_DUMMY_IDLE_R {
        USR_DUMMY_IDLE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Set this bit to enable the DOUT phase of an write-data operation."]
    #[inline(always)]
    pub fn usr_mosi(&self) -> USR_MOSI_R {
        USR_MOSI_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Set this bit to enable enable the DIN phase of a read-data operation."]
    #[inline(always)]
    pub fn usr_miso(&self) -> USR_MISO_R {
        USR_MISO_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Set this bit to enable enable the DUMMY phase of an operation."]
    #[inline(always)]
    pub fn usr_dummy(&self) -> USR_DUMMY_R {
        USR_DUMMY_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Set this bit to enable enable the ADDR phase of an operation."]
    #[inline(always)]
    pub fn usr_addr(&self) -> USR_ADDR_R {
        USR_ADDR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set this bit to enable enable the CMD phase of an operation."]
    #[inline(always)]
    pub fn usr_command(&self) -> USR_COMMAND_R {
        USR_COMMAND_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USER")
            .field("ck_out_edge", &format_args!("{}", self.ck_out_edge().bit()))
            .field("fwrite_dual", &format_args!("{}", self.fwrite_dual().bit()))
            .field("fwrite_quad", &format_args!("{}", self.fwrite_quad().bit()))
            .field("fwrite_dio", &format_args!("{}", self.fwrite_dio().bit()))
            .field("fwrite_qio", &format_args!("{}", self.fwrite_qio().bit()))
            .field(
                "usr_miso_highpart",
                &format_args!("{}", self.usr_miso_highpart().bit()),
            )
            .field(
                "usr_mosi_highpart",
                &format_args!("{}", self.usr_mosi_highpart().bit()),
            )
            .field(
                "usr_dummy_idle",
                &format_args!("{}", self.usr_dummy_idle().bit()),
            )
            .field("usr_mosi", &format_args!("{}", self.usr_mosi().bit()))
            .field("usr_miso", &format_args!("{}", self.usr_miso().bit()))
            .field("usr_dummy", &format_args!("{}", self.usr_dummy().bit()))
            .field("usr_addr", &format_args!("{}", self.usr_addr().bit()))
            .field("usr_command", &format_args!("{}", self.usr_command().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<USER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 9 - This bit, combined with SPI_MEM_CK_IDLE_EDGE bit, is used to change the clock mode 0~3 of SPI_CLK."]
    #[inline(always)]
    #[must_use]
    pub fn ck_out_edge(&mut self) -> CK_OUT_EDGE_W<9> {
        CK_OUT_EDGE_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to enable 2-bm in DOUT phase in SPI1 write operation."]
    #[inline(always)]
    #[must_use]
    pub fn fwrite_dual(&mut self) -> FWRITE_DUAL_W<12> {
        FWRITE_DUAL_W::new(self)
    }
    #[doc = "Bit 13 - Set this bit to enable 4-bm in DOUT phase in SPI1 write operation."]
    #[inline(always)]
    #[must_use]
    pub fn fwrite_quad(&mut self) -> FWRITE_QUAD_W<13> {
        FWRITE_QUAD_W::new(self)
    }
    #[doc = "Bit 14 - Set this bit to enable 2-bm in ADDR and DOUT phase in SPI1 write operation."]
    #[inline(always)]
    #[must_use]
    pub fn fwrite_dio(&mut self) -> FWRITE_DIO_W<14> {
        FWRITE_DIO_W::new(self)
    }
    #[doc = "Bit 15 - Set this bit to enable 4-bit-mode(4-bm) in ADDR and DOUT phase in SPI1 write operation."]
    #[inline(always)]
    #[must_use]
    pub fn fwrite_qio(&mut self) -> FWRITE_QIO_W<15> {
        FWRITE_QIO_W::new(self)
    }
    #[doc = "Bit 24 - DIN phase only access to high-part of the buffer SPI_MEM_W8_REG~SPI_MEM_W15_REG. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn usr_miso_highpart(&mut self) -> USR_MISO_HIGHPART_W<24> {
        USR_MISO_HIGHPART_W::new(self)
    }
    #[doc = "Bit 25 - DOUT phase only access to high-part of the buffer SPI_MEM_W8_REG~SPI_MEM_W15_REG. 1: enable 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn usr_mosi_highpart(&mut self) -> USR_MOSI_HIGHPART_W<25> {
        USR_MOSI_HIGHPART_W::new(self)
    }
    #[doc = "Bit 26 - SPI_CLK is disabled(No clock edges) in DUMMY phase when the bit is enable."]
    #[inline(always)]
    #[must_use]
    pub fn usr_dummy_idle(&mut self) -> USR_DUMMY_IDLE_W<26> {
        USR_DUMMY_IDLE_W::new(self)
    }
    #[doc = "Bit 27 - Set this bit to enable the DOUT phase of an write-data operation."]
    #[inline(always)]
    #[must_use]
    pub fn usr_mosi(&mut self) -> USR_MOSI_W<27> {
        USR_MOSI_W::new(self)
    }
    #[doc = "Bit 28 - Set this bit to enable enable the DIN phase of a read-data operation."]
    #[inline(always)]
    #[must_use]
    pub fn usr_miso(&mut self) -> USR_MISO_W<28> {
        USR_MISO_W::new(self)
    }
    #[doc = "Bit 29 - Set this bit to enable enable the DUMMY phase of an operation."]
    #[inline(always)]
    #[must_use]
    pub fn usr_dummy(&mut self) -> USR_DUMMY_W<29> {
        USR_DUMMY_W::new(self)
    }
    #[doc = "Bit 30 - Set this bit to enable enable the ADDR phase of an operation."]
    #[inline(always)]
    #[must_use]
    pub fn usr_addr(&mut self) -> USR_ADDR_W<30> {
        USR_ADDR_W::new(self)
    }
    #[doc = "Bit 31 - Set this bit to enable enable the CMD phase of an operation."]
    #[inline(always)]
    #[must_use]
    pub fn usr_command(&mut self) -> USR_COMMAND_W<31> {
        USR_COMMAND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 user register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [user](index.html) module"]
pub struct USER_SPEC;
impl crate::RegisterSpec for USER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [user::R](R) reader structure"]
impl crate::Readable for USER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [user::W](W) writer structure"]
impl crate::Writable for USER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USER to value 0x8000_0000"]
impl crate::Resettable for USER_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
