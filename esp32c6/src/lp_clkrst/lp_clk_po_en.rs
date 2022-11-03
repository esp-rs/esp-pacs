#[doc = "Register `LP_CLK_PO_EN` reader"]
pub struct R(crate::R<LP_CLK_PO_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LP_CLK_PO_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LP_CLK_PO_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LP_CLK_PO_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LP_CLK_PO_EN` writer"]
pub struct W(crate::W<LP_CLK_PO_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LP_CLK_PO_EN_SPEC>;
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
impl From<crate::W<LP_CLK_PO_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LP_CLK_PO_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AON_SLOW_OEN` reader - need_des"]
pub type AON_SLOW_OEN_R = crate::BitReader<bool>;
#[doc = "Field `AON_SLOW_OEN` writer - need_des"]
pub type AON_SLOW_OEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LP_CLK_PO_EN_SPEC, bool, O>;
#[doc = "Field `AON_FAST_OEN` reader - need_des"]
pub type AON_FAST_OEN_R = crate::BitReader<bool>;
#[doc = "Field `AON_FAST_OEN` writer - need_des"]
pub type AON_FAST_OEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LP_CLK_PO_EN_SPEC, bool, O>;
#[doc = "Field `SOSC_OEN` reader - need_des"]
pub type SOSC_OEN_R = crate::BitReader<bool>;
#[doc = "Field `SOSC_OEN` writer - need_des"]
pub type SOSC_OEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LP_CLK_PO_EN_SPEC, bool, O>;
#[doc = "Field `FOSC_OEN` reader - need_des"]
pub type FOSC_OEN_R = crate::BitReader<bool>;
#[doc = "Field `FOSC_OEN` writer - need_des"]
pub type FOSC_OEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LP_CLK_PO_EN_SPEC, bool, O>;
#[doc = "Field `OSC32K_OEN` reader - need_des"]
pub type OSC32K_OEN_R = crate::BitReader<bool>;
#[doc = "Field `OSC32K_OEN` writer - need_des"]
pub type OSC32K_OEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LP_CLK_PO_EN_SPEC, bool, O>;
#[doc = "Field `XTAL32K_OEN` reader - need_des"]
pub type XTAL32K_OEN_R = crate::BitReader<bool>;
#[doc = "Field `XTAL32K_OEN` writer - need_des"]
pub type XTAL32K_OEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LP_CLK_PO_EN_SPEC, bool, O>;
#[doc = "Field `CORE_EFUSE_OEN` reader - need_des"]
pub type CORE_EFUSE_OEN_R = crate::BitReader<bool>;
#[doc = "Field `CORE_EFUSE_OEN` writer - need_des"]
pub type CORE_EFUSE_OEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LP_CLK_PO_EN_SPEC, bool, O>;
#[doc = "Field `SLOW_OEN` reader - need_des"]
pub type SLOW_OEN_R = crate::BitReader<bool>;
#[doc = "Field `SLOW_OEN` writer - need_des"]
pub type SLOW_OEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LP_CLK_PO_EN_SPEC, bool, O>;
#[doc = "Field `FAST_OEN` reader - need_des"]
pub type FAST_OEN_R = crate::BitReader<bool>;
#[doc = "Field `FAST_OEN` writer - need_des"]
pub type FAST_OEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LP_CLK_PO_EN_SPEC, bool, O>;
#[doc = "Field `RNG_OEN` reader - need_des"]
pub type RNG_OEN_R = crate::BitReader<bool>;
#[doc = "Field `RNG_OEN` writer - need_des"]
pub type RNG_OEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LP_CLK_PO_EN_SPEC, bool, O>;
#[doc = "Field `LPBUS_OEN` reader - need_des"]
pub type LPBUS_OEN_R = crate::BitReader<bool>;
#[doc = "Field `LPBUS_OEN` writer - need_des"]
pub type LPBUS_OEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LP_CLK_PO_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn aon_slow_oen(&self) -> AON_SLOW_OEN_R {
        AON_SLOW_OEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn aon_fast_oen(&self) -> AON_FAST_OEN_R {
        AON_FAST_OEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn sosc_oen(&self) -> SOSC_OEN_R {
        SOSC_OEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn fosc_oen(&self) -> FOSC_OEN_R {
        FOSC_OEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn osc32k_oen(&self) -> OSC32K_OEN_R {
        OSC32K_OEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn xtal32k_oen(&self) -> XTAL32K_OEN_R {
        XTAL32K_OEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn core_efuse_oen(&self) -> CORE_EFUSE_OEN_R {
        CORE_EFUSE_OEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn slow_oen(&self) -> SLOW_OEN_R {
        SLOW_OEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn fast_oen(&self) -> FAST_OEN_R {
        FAST_OEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - need_des"]
    #[inline(always)]
    pub fn rng_oen(&self) -> RNG_OEN_R {
        RNG_OEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn lpbus_oen(&self) -> LPBUS_OEN_R {
        LPBUS_OEN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn aon_slow_oen(&mut self) -> AON_SLOW_OEN_W<0> {
        AON_SLOW_OEN_W::new(self)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn aon_fast_oen(&mut self) -> AON_FAST_OEN_W<1> {
        AON_FAST_OEN_W::new(self)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sosc_oen(&mut self) -> SOSC_OEN_W<2> {
        SOSC_OEN_W::new(self)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn fosc_oen(&mut self) -> FOSC_OEN_W<3> {
        FOSC_OEN_W::new(self)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn osc32k_oen(&mut self) -> OSC32K_OEN_W<4> {
        OSC32K_OEN_W::new(self)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn xtal32k_oen(&mut self) -> XTAL32K_OEN_W<5> {
        XTAL32K_OEN_W::new(self)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn core_efuse_oen(&mut self) -> CORE_EFUSE_OEN_W<6> {
        CORE_EFUSE_OEN_W::new(self)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn slow_oen(&mut self) -> SLOW_OEN_W<7> {
        SLOW_OEN_W::new(self)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn fast_oen(&mut self) -> FAST_OEN_W<8> {
        FAST_OEN_W::new(self)
    }
    #[doc = "Bit 9 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn rng_oen(&mut self) -> RNG_OEN_W<9> {
        RNG_OEN_W::new(self)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lpbus_oen(&mut self) -> LPBUS_OEN_W<10> {
        LPBUS_OEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lp_clk_po_en](index.html) module"]
pub struct LP_CLK_PO_EN_SPEC;
impl crate::RegisterSpec for LP_CLK_PO_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lp_clk_po_en::R](R) reader structure"]
impl crate::Readable for LP_CLK_PO_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lp_clk_po_en::W](W) writer structure"]
impl crate::Writable for LP_CLK_PO_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_CLK_PO_EN to value 0x07ff"]
impl crate::Resettable for LP_CLK_PO_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0x07ff;
}
