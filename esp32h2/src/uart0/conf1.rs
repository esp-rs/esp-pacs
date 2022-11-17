#[doc = "Register `CONF1` reader"]
pub struct R(crate::R<CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF1` writer"]
pub struct W(crate::W<CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF1_SPEC>;
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
impl From<crate::W<CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFO_FULL_THRHD` reader - It will produce rxfifo_full_int interrupt when receiver receives more data than this register value."]
pub type RXFIFO_FULL_THRHD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXFIFO_FULL_THRHD` writer - It will produce rxfifo_full_int interrupt when receiver receives more data than this register value."]
pub type RXFIFO_FULL_THRHD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONF1_SPEC, u8, u8, 8, O>;
#[doc = "Field `TXFIFO_EMPTY_THRHD` reader - It will produce txfifo_empty_int interrupt when the data amount in Tx-FIFO is less than this register value."]
pub type TXFIFO_EMPTY_THRHD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXFIFO_EMPTY_THRHD` writer - It will produce txfifo_empty_int interrupt when the data amount in Tx-FIFO is less than this register value."]
pub type TXFIFO_EMPTY_THRHD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONF1_SPEC, u8, u8, 8, O>;
#[doc = "Field `CTS_INV` reader - Set this bit to inverse the level value of uart cts signal."]
pub type CTS_INV_R = crate::BitReader<bool>;
#[doc = "Field `CTS_INV` writer - Set this bit to inverse the level value of uart cts signal."]
pub type CTS_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF1_SPEC, bool, O>;
#[doc = "Field `DSR_INV` reader - Set this bit to inverse the level value of uart dsr signal."]
pub type DSR_INV_R = crate::BitReader<bool>;
#[doc = "Field `DSR_INV` writer - Set this bit to inverse the level value of uart dsr signal."]
pub type DSR_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF1_SPEC, bool, O>;
#[doc = "Field `RTS_INV` reader - Set this bit to inverse the level value of uart rts signal."]
pub type RTS_INV_R = crate::BitReader<bool>;
#[doc = "Field `RTS_INV` writer - Set this bit to inverse the level value of uart rts signal."]
pub type RTS_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF1_SPEC, bool, O>;
#[doc = "Field `DTR_INV` reader - Set this bit to inverse the level value of uart dtr signal."]
pub type DTR_INV_R = crate::BitReader<bool>;
#[doc = "Field `DTR_INV` writer - Set this bit to inverse the level value of uart dtr signal."]
pub type DTR_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF1_SPEC, bool, O>;
#[doc = "Field `SW_DTR` reader - This register is used to configure the software dtr signal which is used in software flow control."]
pub type SW_DTR_R = crate::BitReader<bool>;
#[doc = "Field `SW_DTR` writer - This register is used to configure the software dtr signal which is used in software flow control."]
pub type SW_DTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF1_SPEC, bool, O>;
#[doc = "Field `CLK_EN` reader - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
pub type CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `CLK_EN` writer - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - It will produce rxfifo_full_int interrupt when receiver receives more data than this register value."]
    #[inline(always)]
    pub fn rxfifo_full_thrhd(&self) -> RXFIFO_FULL_THRHD_R {
        RXFIFO_FULL_THRHD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - It will produce txfifo_empty_int interrupt when the data amount in Tx-FIFO is less than this register value."]
    #[inline(always)]
    pub fn txfifo_empty_thrhd(&self) -> TXFIFO_EMPTY_THRHD_R {
        TXFIFO_EMPTY_THRHD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Set this bit to inverse the level value of uart cts signal."]
    #[inline(always)]
    pub fn cts_inv(&self) -> CTS_INV_R {
        CTS_INV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set this bit to inverse the level value of uart dsr signal."]
    #[inline(always)]
    pub fn dsr_inv(&self) -> DSR_INV_R {
        DSR_INV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Set this bit to inverse the level value of uart rts signal."]
    #[inline(always)]
    pub fn rts_inv(&self) -> RTS_INV_R {
        RTS_INV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Set this bit to inverse the level value of uart dtr signal."]
    #[inline(always)]
    pub fn dtr_inv(&self) -> DTR_INV_R {
        DTR_INV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - This register is used to configure the software dtr signal which is used in software flow control."]
    #[inline(always)]
    pub fn sw_dtr(&self) -> SW_DTR_R {
        SW_DTR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - It will produce rxfifo_full_int interrupt when receiver receives more data than this register value."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_full_thrhd(&mut self) -> RXFIFO_FULL_THRHD_W<0> {
        RXFIFO_FULL_THRHD_W::new(self)
    }
    #[doc = "Bits 8:15 - It will produce txfifo_empty_int interrupt when the data amount in Tx-FIFO is less than this register value."]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_empty_thrhd(&mut self) -> TXFIFO_EMPTY_THRHD_W<8> {
        TXFIFO_EMPTY_THRHD_W::new(self)
    }
    #[doc = "Bit 16 - Set this bit to inverse the level value of uart cts signal."]
    #[inline(always)]
    #[must_use]
    pub fn cts_inv(&mut self) -> CTS_INV_W<16> {
        CTS_INV_W::new(self)
    }
    #[doc = "Bit 17 - Set this bit to inverse the level value of uart dsr signal."]
    #[inline(always)]
    #[must_use]
    pub fn dsr_inv(&mut self) -> DSR_INV_W<17> {
        DSR_INV_W::new(self)
    }
    #[doc = "Bit 18 - Set this bit to inverse the level value of uart rts signal."]
    #[inline(always)]
    #[must_use]
    pub fn rts_inv(&mut self) -> RTS_INV_W<18> {
        RTS_INV_W::new(self)
    }
    #[doc = "Bit 19 - Set this bit to inverse the level value of uart dtr signal."]
    #[inline(always)]
    #[must_use]
    pub fn dtr_inv(&mut self) -> DTR_INV_W<19> {
        DTR_INV_W::new(self)
    }
    #[doc = "Bit 20 - This register is used to configure the software dtr signal which is used in software flow control."]
    #[inline(always)]
    #[must_use]
    pub fn sw_dtr(&mut self) -> SW_DTR_W<20> {
        SW_DTR_W::new(self)
    }
    #[doc = "Bit 21 - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<21> {
        CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf1](index.html) module"]
pub struct CONF1_SPEC;
impl crate::RegisterSpec for CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf1::R](R) reader structure"]
impl crate::Readable for CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf1::W](W) writer structure"]
impl crate::Writable for CONF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF1 to value 0x6060"]
impl crate::Resettable for CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0x6060;
}
