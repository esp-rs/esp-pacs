#[doc = "Register `PIN` reader"]
pub struct R(crate::R<PIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIN` writer"]
pub struct W(crate::W<PIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIN_SPEC>;
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
impl From<crate::W<PIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS0_DIS` reader - SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin"]
pub type CS0_DIS_R = crate::BitReader;
#[doc = "Field `CS0_DIS` writer - SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin"]
pub type CS0_DIS_W<'a, const O: u8> = crate::BitWriter<'a, PIN_SPEC, O>;
#[doc = "Field `CS1_DIS` reader - SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin"]
pub type CS1_DIS_R = crate::BitReader;
#[doc = "Field `CS1_DIS` writer - SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin"]
pub type CS1_DIS_W<'a, const O: u8> = crate::BitWriter<'a, PIN_SPEC, O>;
#[doc = "Field `CS2_DIS` reader - SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin"]
pub type CS2_DIS_R = crate::BitReader;
#[doc = "Field `CS2_DIS` writer - SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin"]
pub type CS2_DIS_W<'a, const O: u8> = crate::BitWriter<'a, PIN_SPEC, O>;
#[doc = "Field `CK_DIS` reader - 1: spi clk out disable 0: spi clk out enable"]
pub type CK_DIS_R = crate::BitReader;
#[doc = "Field `CK_DIS` writer - 1: spi clk out disable 0: spi clk out enable"]
pub type CK_DIS_W<'a, const O: u8> = crate::BitWriter<'a, PIN_SPEC, O>;
#[doc = "Field `MASTER_CS_POL` reader - In the master mode the bits are the polarity of spi cs line the value is equivalent to spi_cs ^ spi_master_cs_pol."]
pub type MASTER_CS_POL_R = crate::FieldReader;
#[doc = "Field `MASTER_CS_POL` writer - In the master mode the bits are the polarity of spi cs line the value is equivalent to spi_cs ^ spi_master_cs_pol."]
pub type MASTER_CS_POL_W<'a, const O: u8> = crate::FieldWriter<'a, PIN_SPEC, 3, O>;
#[doc = "Field `MASTER_CK_SEL` reader - In the master mode spi cs line is enable as spi clk it is combined with spi_cs0_dis spi_cs1_dis spi_cs2_dis."]
pub type MASTER_CK_SEL_R = crate::FieldReader;
#[doc = "Field `MASTER_CK_SEL` writer - In the master mode spi cs line is enable as spi clk it is combined with spi_cs0_dis spi_cs1_dis spi_cs2_dis."]
pub type MASTER_CK_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, PIN_SPEC, 3, O>;
#[doc = "Field `CK_IDLE_EDGE` reader - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
pub type CK_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `CK_IDLE_EDGE` writer - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
pub type CK_IDLE_EDGE_W<'a, const O: u8> = crate::BitWriter<'a, PIN_SPEC, O>;
#[doc = "Field `CS_KEEP_ACTIVE` reader - spi cs line keep low when the bit is set."]
pub type CS_KEEP_ACTIVE_R = crate::BitReader;
#[doc = "Field `CS_KEEP_ACTIVE` writer - spi cs line keep low when the bit is set."]
pub type CS_KEEP_ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, PIN_SPEC, O>;
impl R {
    #[doc = "Bit 0 - SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin"]
    #[inline(always)]
    pub fn cs0_dis(&self) -> CS0_DIS_R {
        CS0_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin"]
    #[inline(always)]
    pub fn cs1_dis(&self) -> CS1_DIS_R {
        CS1_DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin"]
    #[inline(always)]
    pub fn cs2_dis(&self) -> CS2_DIS_R {
        CS2_DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - 1: spi clk out disable 0: spi clk out enable"]
    #[inline(always)]
    pub fn ck_dis(&self) -> CK_DIS_R {
        CK_DIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - In the master mode the bits are the polarity of spi cs line the value is equivalent to spi_cs ^ spi_master_cs_pol."]
    #[inline(always)]
    pub fn master_cs_pol(&self) -> MASTER_CS_POL_R {
        MASTER_CS_POL_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 11:13 - In the master mode spi cs line is enable as spi clk it is combined with spi_cs0_dis spi_cs1_dis spi_cs2_dis."]
    #[inline(always)]
    pub fn master_ck_sel(&self) -> MASTER_CK_SEL_R {
        MASTER_CK_SEL_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 29 - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn ck_idle_edge(&self) -> CK_IDLE_EDGE_R {
        CK_IDLE_EDGE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - spi cs line keep low when the bit is set."]
    #[inline(always)]
    pub fn cs_keep_active(&self) -> CS_KEEP_ACTIVE_R {
        CS_KEEP_ACTIVE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIN")
            .field("cs0_dis", &format_args!("{}", self.cs0_dis().bit()))
            .field("cs1_dis", &format_args!("{}", self.cs1_dis().bit()))
            .field("cs2_dis", &format_args!("{}", self.cs2_dis().bit()))
            .field("ck_dis", &format_args!("{}", self.ck_dis().bit()))
            .field(
                "master_cs_pol",
                &format_args!("{}", self.master_cs_pol().bits()),
            )
            .field(
                "master_ck_sel",
                &format_args!("{}", self.master_ck_sel().bits()),
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
impl core::fmt::Debug for crate::generic::Reg<PIN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - SPI CS0 pin enable, 1: disable CS0, 0: spi_cs0 signal is from/to CS0 pin"]
    #[inline(always)]
    #[must_use]
    pub fn cs0_dis(&mut self) -> CS0_DIS_W<0> {
        CS0_DIS_W::new(self)
    }
    #[doc = "Bit 1 - SPI CS1 pin enable, 1: disable CS1, 0: spi_cs1 signal is from/to CS1 pin"]
    #[inline(always)]
    #[must_use]
    pub fn cs1_dis(&mut self) -> CS1_DIS_W<1> {
        CS1_DIS_W::new(self)
    }
    #[doc = "Bit 2 - SPI CS2 pin enable, 1: disable CS2, 0: spi_cs2 signal is from/to CS2 pin"]
    #[inline(always)]
    #[must_use]
    pub fn cs2_dis(&mut self) -> CS2_DIS_W<2> {
        CS2_DIS_W::new(self)
    }
    #[doc = "Bit 5 - 1: spi clk out disable 0: spi clk out enable"]
    #[inline(always)]
    #[must_use]
    pub fn ck_dis(&mut self) -> CK_DIS_W<5> {
        CK_DIS_W::new(self)
    }
    #[doc = "Bits 6:8 - In the master mode the bits are the polarity of spi cs line the value is equivalent to spi_cs ^ spi_master_cs_pol."]
    #[inline(always)]
    #[must_use]
    pub fn master_cs_pol(&mut self) -> MASTER_CS_POL_W<6> {
        MASTER_CS_POL_W::new(self)
    }
    #[doc = "Bits 11:13 - In the master mode spi cs line is enable as spi clk it is combined with spi_cs0_dis spi_cs1_dis spi_cs2_dis."]
    #[inline(always)]
    #[must_use]
    pub fn master_ck_sel(&mut self) -> MASTER_CK_SEL_W<11> {
        MASTER_CK_SEL_W::new(self)
    }
    #[doc = "Bit 29 - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    #[must_use]
    pub fn ck_idle_edge(&mut self) -> CK_IDLE_EDGE_W<29> {
        CK_IDLE_EDGE_W::new(self)
    }
    #[doc = "Bit 30 - spi cs line keep low when the bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn cs_keep_active(&mut self) -> CS_KEEP_ACTIVE_W<30> {
        CS_KEEP_ACTIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin](index.html) module"]
pub struct PIN_SPEC;
impl crate::RegisterSpec for PIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pin::R](R) reader structure"]
impl crate::Readable for PIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pin::W](W) writer structure"]
impl crate::Writable for PIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIN to value 0x06"]
impl crate::Resettable for PIN_SPEC {
    const RESET_VALUE: Self::Ux = 0x06;
}
