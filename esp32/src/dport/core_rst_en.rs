#[doc = "Register `CORE_RST_EN` reader"]
pub struct R(crate::R<CORE_RST_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_RST_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_RST_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_RST_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_RST_EN` writer"]
pub struct W(crate::W<CORE_RST_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_RST_EN_SPEC>;
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
impl From<crate::W<CORE_RST_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_RST_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_RST` reader - "]
pub type CORE_RST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CORE_RST` writer - "]
pub type CORE_RST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CORE_RST_EN_SPEC, u8, u8, 8, O>;
#[doc = "Field `BB_RST` reader - "]
pub type BB_RST_R = crate::BitReader<bool>;
#[doc = "Field `BB_RST` writer - "]
pub type BB_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CORE_RST_EN_SPEC, bool, O>;
#[doc = "Field `FE_RST` reader - "]
pub type FE_RST_R = crate::BitReader<bool>;
#[doc = "Field `FE_RST` writer - "]
pub type FE_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CORE_RST_EN_SPEC, bool, O>;
#[doc = "Field `MAC_RST` reader - "]
pub type MAC_RST_R = crate::BitReader<bool>;
#[doc = "Field `MAC_RST` writer - "]
pub type MAC_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CORE_RST_EN_SPEC, bool, O>;
#[doc = "Field `BT_RST` reader - "]
pub type BT_RST_R = crate::BitReader<bool>;
#[doc = "Field `BT_RST` writer - "]
pub type BT_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CORE_RST_EN_SPEC, bool, O>;
#[doc = "Field `BTMAC_RST` reader - "]
pub type BTMAC_RST_R = crate::BitReader<bool>;
#[doc = "Field `BTMAC_RST` writer - "]
pub type BTMAC_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CORE_RST_EN_SPEC, bool, O>;
#[doc = "Field `SDIO_RST` reader - "]
pub type SDIO_RST_R = crate::BitReader<bool>;
#[doc = "Field `SDIO_RST` writer - "]
pub type SDIO_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CORE_RST_EN_SPEC, bool, O>;
#[doc = "Field `SDIO_HOST_RST` reader - "]
pub type SDIO_HOST_RST_R = crate::BitReader<bool>;
#[doc = "Field `SDIO_HOST_RST` writer - "]
pub type SDIO_HOST_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CORE_RST_EN_SPEC, bool, O>;
#[doc = "Field `EMAC_RST` reader - "]
pub type EMAC_RST_R = crate::BitReader<bool>;
#[doc = "Field `EMAC_RST` writer - "]
pub type EMAC_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CORE_RST_EN_SPEC, bool, O>;
#[doc = "Field `MACPWR_RST` reader - "]
pub type MACPWR_RST_R = crate::BitReader<bool>;
#[doc = "Field `MACPWR_RST` writer - "]
pub type MACPWR_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CORE_RST_EN_SPEC, bool, O>;
#[doc = "Field `RW_BTMAC_RST` reader - "]
pub type RW_BTMAC_RST_R = crate::BitReader<bool>;
#[doc = "Field `RW_BTMAC_RST` writer - "]
pub type RW_BTMAC_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CORE_RST_EN_SPEC, bool, O>;
#[doc = "Field `RW_BTLP_RST` reader - "]
pub type RW_BTLP_RST_R = crate::BitReader<bool>;
#[doc = "Field `RW_BTLP_RST` writer - "]
pub type RW_BTLP_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CORE_RST_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn core_rst(&self) -> CORE_RST_R {
        CORE_RST_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bb_rst(&self) -> BB_RST_R {
        BB_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fe_rst(&self) -> FE_RST_R {
        FE_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn mac_rst(&self) -> MAC_RST_R {
        MAC_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn bt_rst(&self) -> BT_RST_R {
        BT_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn btmac_rst(&self) -> BTMAC_RST_R {
        BTMAC_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sdio_rst(&self) -> SDIO_RST_R {
        SDIO_RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sdio_host_rst(&self) -> SDIO_HOST_RST_R {
        SDIO_HOST_RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn emac_rst(&self) -> EMAC_RST_R {
        EMAC_RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn macpwr_rst(&self) -> MACPWR_RST_R {
        MACPWR_RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rw_btmac_rst(&self) -> RW_BTMAC_RST_R {
        RW_BTMAC_RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rw_btlp_rst(&self) -> RW_BTLP_RST_R {
        RW_BTLP_RST_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn core_rst(&mut self) -> CORE_RST_W<0> {
        CORE_RST_W::new(self)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn bb_rst(&mut self) -> BB_RST_W<0> {
        BB_RST_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn fe_rst(&mut self) -> FE_RST_W<1> {
        FE_RST_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn mac_rst(&mut self) -> MAC_RST_W<2> {
        MAC_RST_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn bt_rst(&mut self) -> BT_RST_W<3> {
        BT_RST_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn btmac_rst(&mut self) -> BTMAC_RST_W<4> {
        BTMAC_RST_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_rst(&mut self) -> SDIO_RST_W<5> {
        SDIO_RST_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_host_rst(&mut self) -> SDIO_HOST_RST_W<6> {
        SDIO_HOST_RST_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn emac_rst(&mut self) -> EMAC_RST_W<7> {
        EMAC_RST_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn macpwr_rst(&mut self) -> MACPWR_RST_W<8> {
        MACPWR_RST_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn rw_btmac_rst(&mut self) -> RW_BTMAC_RST_W<9> {
        RW_BTMAC_RST_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn rw_btlp_rst(&mut self) -> RW_BTLP_RST_W<10> {
        RW_BTLP_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_rst_en](index.html) module"]
pub struct CORE_RST_EN_SPEC;
impl crate::RegisterSpec for CORE_RST_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_rst_en::R](R) reader structure"]
impl crate::Readable for CORE_RST_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_rst_en::W](W) writer structure"]
impl crate::Writable for CORE_RST_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CORE_RST_EN to value 0"]
impl crate::Resettable for CORE_RST_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
