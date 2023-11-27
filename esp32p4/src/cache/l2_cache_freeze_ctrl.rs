#[doc = "Register `L2_CACHE_FREEZE_CTRL` reader"]
pub type R = crate::R<L2_CACHE_FREEZE_CTRL_SPEC>;
#[doc = "Register `L2_CACHE_FREEZE_CTRL` writer"]
pub type W = crate::W<L2_CACHE_FREEZE_CTRL_SPEC>;
#[doc = "Field `L2_CACHE_FREEZE_EN` reader - The bit is used to enable freeze operation on L2-Cache. It can be cleared by software."]
pub type L2_CACHE_FREEZE_EN_R = crate::BitReader;
#[doc = "Field `L2_CACHE_FREEZE_EN` writer - The bit is used to enable freeze operation on L2-Cache. It can be cleared by software."]
pub type L2_CACHE_FREEZE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_CACHE_FREEZE_MODE` reader - The bit is used to configure mode of freeze operation L2-Cache. 0: a miss-access will not stuck. 1: a miss-access will stuck."]
pub type L2_CACHE_FREEZE_MODE_R = crate::BitReader;
#[doc = "Field `L2_CACHE_FREEZE_MODE` writer - The bit is used to configure mode of freeze operation L2-Cache. 0: a miss-access will not stuck. 1: a miss-access will stuck."]
pub type L2_CACHE_FREEZE_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2_CACHE_FREEZE_DONE` reader - The bit is used to indicate whether freeze operation on L2-Cache is finished or not. 0: not finished. 1: finished."]
pub type L2_CACHE_FREEZE_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 20 - The bit is used to enable freeze operation on L2-Cache. It can be cleared by software."]
    #[inline(always)]
    pub fn l2_cache_freeze_en(&self) -> L2_CACHE_FREEZE_EN_R {
        L2_CACHE_FREEZE_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The bit is used to configure mode of freeze operation L2-Cache. 0: a miss-access will not stuck. 1: a miss-access will stuck."]
    #[inline(always)]
    pub fn l2_cache_freeze_mode(&self) -> L2_CACHE_FREEZE_MODE_R {
        L2_CACHE_FREEZE_MODE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The bit is used to indicate whether freeze operation on L2-Cache is finished or not. 0: not finished. 1: finished."]
    #[inline(always)]
    pub fn l2_cache_freeze_done(&self) -> L2_CACHE_FREEZE_DONE_R {
        L2_CACHE_FREEZE_DONE_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_FREEZE_CTRL")
            .field(
                "l2_cache_freeze_en",
                &format_args!("{}", self.l2_cache_freeze_en().bit()),
            )
            .field(
                "l2_cache_freeze_mode",
                &format_args!("{}", self.l2_cache_freeze_mode().bit()),
            )
            .field(
                "l2_cache_freeze_done",
                &format_args!("{}", self.l2_cache_freeze_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_FREEZE_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 20 - The bit is used to enable freeze operation on L2-Cache. It can be cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_freeze_en(&mut self) -> L2_CACHE_FREEZE_EN_W<L2_CACHE_FREEZE_CTRL_SPEC> {
        L2_CACHE_FREEZE_EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - The bit is used to configure mode of freeze operation L2-Cache. 0: a miss-access will not stuck. 1: a miss-access will stuck."]
    #[inline(always)]
    #[must_use]
    pub fn l2_cache_freeze_mode(&mut self) -> L2_CACHE_FREEZE_MODE_W<L2_CACHE_FREEZE_CTRL_SPEC> {
        L2_CACHE_FREEZE_MODE_W::new(self, 21)
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
#[doc = "Cache Freeze control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_freeze_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_freeze_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2_CACHE_FREEZE_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_FREEZE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_cache_freeze_ctrl::R`](R) reader structure"]
impl crate::Readable for L2_CACHE_FREEZE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`l2_cache_freeze_ctrl::W`](W) writer structure"]
impl crate::Writable for L2_CACHE_FREEZE_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets L2_CACHE_FREEZE_CTRL to value 0"]
impl crate::Resettable for L2_CACHE_FREEZE_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
