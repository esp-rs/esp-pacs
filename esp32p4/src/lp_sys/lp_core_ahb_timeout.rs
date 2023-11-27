#[doc = "Register `LP_CORE_AHB_TIMEOUT` reader"]
pub type R = crate::R<LP_CORE_AHB_TIMEOUT_SPEC>;
#[doc = "Register `LP_CORE_AHB_TIMEOUT` writer"]
pub type W = crate::W<LP_CORE_AHB_TIMEOUT_SPEC>;
#[doc = "Field `EN` reader - set this field to 1 to enable lp core ahb timeout handle"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - set this field to 1 to enable lp core ahb timeout handle"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRES` reader - This field used to set lp core ahb bus timeout threshold"]
pub type THRES_R = crate::FieldReader<u16>;
#[doc = "Field `THRES` writer - This field used to set lp core ahb bus timeout threshold"]
pub type THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LP2HP_AHB_TIMEOUT_EN` reader - set this field to 1 to enable lp2hp ahb timeout handle"]
pub type LP2HP_AHB_TIMEOUT_EN_R = crate::BitReader;
#[doc = "Field `LP2HP_AHB_TIMEOUT_EN` writer - set this field to 1 to enable lp2hp ahb timeout handle"]
pub type LP2HP_AHB_TIMEOUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP2HP_AHB_TIMEOUT_THRES` reader - This field used to set lp2hp ahb bus timeout threshold"]
pub type LP2HP_AHB_TIMEOUT_THRES_R = crate::FieldReader;
#[doc = "Field `LP2HP_AHB_TIMEOUT_THRES` writer - This field used to set lp2hp ahb bus timeout threshold"]
pub type LP2HP_AHB_TIMEOUT_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - set this field to 1 to enable lp core ahb timeout handle"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:16 - This field used to set lp core ahb bus timeout threshold"]
    #[inline(always)]
    pub fn thres(&self) -> THRES_R {
        THRES_R::new(((self.bits >> 1) & 0xffff) as u16)
    }
    #[doc = "Bit 17 - set this field to 1 to enable lp2hp ahb timeout handle"]
    #[inline(always)]
    pub fn lp2hp_ahb_timeout_en(&self) -> LP2HP_AHB_TIMEOUT_EN_R {
        LP2HP_AHB_TIMEOUT_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:22 - This field used to set lp2hp ahb bus timeout threshold"]
    #[inline(always)]
    pub fn lp2hp_ahb_timeout_thres(&self) -> LP2HP_AHB_TIMEOUT_THRES_R {
        LP2HP_AHB_TIMEOUT_THRES_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_CORE_AHB_TIMEOUT")
            .field("en", &format_args!("{}", self.en().bit()))
            .field("thres", &format_args!("{}", self.thres().bits()))
            .field(
                "lp2hp_ahb_timeout_en",
                &format_args!("{}", self.lp2hp_ahb_timeout_en().bit()),
            )
            .field(
                "lp2hp_ahb_timeout_thres",
                &format_args!("{}", self.lp2hp_ahb_timeout_thres().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_CORE_AHB_TIMEOUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - set this field to 1 to enable lp core ahb timeout handle"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<LP_CORE_AHB_TIMEOUT_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bits 1:16 - This field used to set lp core ahb bus timeout threshold"]
    #[inline(always)]
    #[must_use]
    pub fn thres(&mut self) -> THRES_W<LP_CORE_AHB_TIMEOUT_SPEC> {
        THRES_W::new(self, 1)
    }
    #[doc = "Bit 17 - set this field to 1 to enable lp2hp ahb timeout handle"]
    #[inline(always)]
    #[must_use]
    pub fn lp2hp_ahb_timeout_en(&mut self) -> LP2HP_AHB_TIMEOUT_EN_W<LP_CORE_AHB_TIMEOUT_SPEC> {
        LP2HP_AHB_TIMEOUT_EN_W::new(self, 17)
    }
    #[doc = "Bits 18:22 - This field used to set lp2hp ahb bus timeout threshold"]
    #[inline(always)]
    #[must_use]
    pub fn lp2hp_ahb_timeout_thres(
        &mut self,
    ) -> LP2HP_AHB_TIMEOUT_THRES_W<LP_CORE_AHB_TIMEOUT_SPEC> {
        LP2HP_AHB_TIMEOUT_THRES_W::new(self, 18)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_core_ahb_timeout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_core_ahb_timeout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_CORE_AHB_TIMEOUT_SPEC;
impl crate::RegisterSpec for LP_CORE_AHB_TIMEOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_core_ahb_timeout::R`](R) reader structure"]
impl crate::Readable for LP_CORE_AHB_TIMEOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_core_ahb_timeout::W`](W) writer structure"]
impl crate::Writable for LP_CORE_AHB_TIMEOUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_CORE_AHB_TIMEOUT to value 0x007f_ffff"]
impl crate::Resettable for LP_CORE_AHB_TIMEOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0x007f_ffff;
}
