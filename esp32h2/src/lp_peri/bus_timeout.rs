#[doc = "Register `BUS_TIMEOUT` reader"]
pub type R = crate::R<BUS_TIMEOUT_SPEC>;
#[doc = "Register `BUS_TIMEOUT` writer"]
pub type W = crate::W<BUS_TIMEOUT_SPEC>;
#[doc = "Field `LP_PERI_TIMEOUT_THRES` reader - need_des"]
pub type LP_PERI_TIMEOUT_THRES_R = crate::FieldReader<u16>;
#[doc = "Field `LP_PERI_TIMEOUT_THRES` writer - need_des"]
pub type LP_PERI_TIMEOUT_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LP_PERI_TIMEOUT_INT_CLEAR` writer - need_des"]
pub type LP_PERI_TIMEOUT_INT_CLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_PERI_TIMEOUT_PROTECT_EN` reader - need_des"]
pub type LP_PERI_TIMEOUT_PROTECT_EN_R = crate::BitReader;
#[doc = "Field `LP_PERI_TIMEOUT_PROTECT_EN` writer - need_des"]
pub type LP_PERI_TIMEOUT_PROTECT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 14:29 - need_des"]
    #[inline(always)]
    pub fn lp_peri_timeout_thres(&self) -> LP_PERI_TIMEOUT_THRES_R {
        LP_PERI_TIMEOUT_THRES_R::new(((self.bits >> 14) & 0xffff) as u16)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_peri_timeout_protect_en(&self) -> LP_PERI_TIMEOUT_PROTECT_EN_R {
        LP_PERI_TIMEOUT_PROTECT_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUS_TIMEOUT")
            .field(
                "lp_peri_timeout_thres",
                &format_args!("{}", self.lp_peri_timeout_thres().bits()),
            )
            .field(
                "lp_peri_timeout_protect_en",
                &format_args!("{}", self.lp_peri_timeout_protect_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BUS_TIMEOUT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 14:29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_peri_timeout_thres(&mut self) -> LP_PERI_TIMEOUT_THRES_W<BUS_TIMEOUT_SPEC> {
        LP_PERI_TIMEOUT_THRES_W::new(self, 14)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_peri_timeout_int_clear(&mut self) -> LP_PERI_TIMEOUT_INT_CLEAR_W<BUS_TIMEOUT_SPEC> {
        LP_PERI_TIMEOUT_INT_CLEAR_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_peri_timeout_protect_en(&mut self) -> LP_PERI_TIMEOUT_PROTECT_EN_W<BUS_TIMEOUT_SPEC> {
        LP_PERI_TIMEOUT_PROTECT_EN_W::new(self, 31)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_timeout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_timeout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUS_TIMEOUT_SPEC;
impl crate::RegisterSpec for BUS_TIMEOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_timeout::R`](R) reader structure"]
impl crate::Readable for BUS_TIMEOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bus_timeout::W`](W) writer structure"]
impl crate::Writable for BUS_TIMEOUT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUS_TIMEOUT to value 0xbfff_c000"]
impl crate::Resettable for BUS_TIMEOUT_SPEC {
    const RESET_VALUE: u32 = 0xbfff_c000;
}
