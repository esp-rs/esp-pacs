#[doc = "Register `FIFO_CONF` reader"]
pub struct R(crate::R<FIFO_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFO_CONF` writer"]
pub struct W(crate::W<FIFO_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_CONF_SPEC>;
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
impl From<crate::W<FIFO_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_DATA_NUM` reader - "]
pub type RX_DATA_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_DATA_NUM` writer - "]
pub type RX_DATA_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFO_CONF_SPEC, u8, u8, 6, O>;
#[doc = "Field `TX_DATA_NUM` reader - "]
pub type TX_DATA_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_DATA_NUM` writer - "]
pub type TX_DATA_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFO_CONF_SPEC, u8, u8, 6, O>;
#[doc = "Field `DSCR_EN` reader - "]
pub type DSCR_EN_R = crate::BitReader<bool>;
#[doc = "Field `DSCR_EN` writer - "]
pub type DSCR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFO_CONF_SPEC, bool, O>;
#[doc = "Field `TX_FIFO_MOD` reader - "]
pub type TX_FIFO_MOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_FIFO_MOD` writer - "]
pub type TX_FIFO_MOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFO_CONF_SPEC, u8, u8, 3, O>;
#[doc = "Field `RX_FIFO_MOD` reader - "]
pub type RX_FIFO_MOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_FIFO_MOD` writer - "]
pub type RX_FIFO_MOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FIFO_CONF_SPEC, u8, u8, 3, O>;
#[doc = "Field `TX_FIFO_MOD_FORCE_EN` reader - "]
pub type TX_FIFO_MOD_FORCE_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_FIFO_MOD_FORCE_EN` writer - "]
pub type TX_FIFO_MOD_FORCE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FIFO_CONF_SPEC, bool, O>;
#[doc = "Field `RX_FIFO_MOD_FORCE_EN` reader - "]
pub type RX_FIFO_MOD_FORCE_EN_R = crate::BitReader<bool>;
#[doc = "Field `RX_FIFO_MOD_FORCE_EN` writer - "]
pub type RX_FIFO_MOD_FORCE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FIFO_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rx_data_num(&self) -> RX_DATA_NUM_R {
        RX_DATA_NUM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn tx_data_num(&self) -> TX_DATA_NUM_R {
        TX_DATA_NUM_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dscr_en(&self) -> DSCR_EN_R {
        DSCR_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn tx_fifo_mod(&self) -> TX_FIFO_MOD_R {
        TX_FIFO_MOD_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rx_fifo_mod(&self) -> RX_FIFO_MOD_R {
        RX_FIFO_MOD_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tx_fifo_mod_force_en(&self) -> TX_FIFO_MOD_FORCE_EN_R {
        TX_FIFO_MOD_FORCE_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rx_fifo_mod_force_en(&self) -> RX_FIFO_MOD_FORCE_EN_R {
        RX_FIFO_MOD_FORCE_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rx_data_num(&mut self) -> RX_DATA_NUM_W<0> {
        RX_DATA_NUM_W::new(self)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn tx_data_num(&mut self) -> TX_DATA_NUM_W<6> {
        TX_DATA_NUM_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dscr_en(&mut self) -> DSCR_EN_W<12> {
        DSCR_EN_W::new(self)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn tx_fifo_mod(&mut self) -> TX_FIFO_MOD_W<13> {
        TX_FIFO_MOD_W::new(self)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rx_fifo_mod(&mut self) -> RX_FIFO_MOD_W<16> {
        RX_FIFO_MOD_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tx_fifo_mod_force_en(&mut self) -> TX_FIFO_MOD_FORCE_EN_W<19> {
        TX_FIFO_MOD_FORCE_EN_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rx_fifo_mod_force_en(&mut self) -> RX_FIFO_MOD_FORCE_EN_W<20> {
        RX_FIFO_MOD_FORCE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_conf](index.html) module"]
pub struct FIFO_CONF_SPEC;
impl crate::RegisterSpec for FIFO_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_conf::R](R) reader structure"]
impl crate::Readable for FIFO_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo_conf::W](W) writer structure"]
impl crate::Writable for FIFO_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFO_CONF to value 0x1820"]
impl crate::Resettable for FIFO_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1820
    }
}
