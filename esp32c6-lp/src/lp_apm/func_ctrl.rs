#[doc = "Register `FUNC_CTRL` reader"]
pub type R = crate::R<FUNC_CTRL_SPEC>;
#[doc = "Register `FUNC_CTRL` writer"]
pub type W = crate::W<FUNC_CTRL_SPEC>;
#[doc = "Field `M_PMS_FUNC_EN(0-1)` reader - PMS M%s function enable"]
pub type M_PMS_FUNC_EN_R = crate::BitReader;
#[doc = "Field `M_PMS_FUNC_EN(0-1)` writer - PMS M%s function enable"]
pub type M_PMS_FUNC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "PMS M(0-1) function enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `M0_PMS_FUNC_EN` field"]
    #[inline(always)]
    pub fn m_pms_func_en(&self, n: u8) -> M_PMS_FUNC_EN_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        M_PMS_FUNC_EN_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "PMS M(0-1) function enable"]
    #[inline(always)]
    pub fn m_pms_func_en_iter(&self) -> impl Iterator<Item = M_PMS_FUNC_EN_R> + '_ {
        (0..2).map(move |n| M_PMS_FUNC_EN_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - PMS M0 function enable"]
    #[inline(always)]
    pub fn m0_pms_func_en(&self) -> M_PMS_FUNC_EN_R {
        M_PMS_FUNC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PMS M1 function enable"]
    #[inline(always)]
    pub fn m1_pms_func_en(&self) -> M_PMS_FUNC_EN_R {
        M_PMS_FUNC_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC_CTRL")
            .field("m0_pms_func_en", &self.m0_pms_func_en().bit())
            .field("m1_pms_func_en", &self.m1_pms_func_en().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FUNC_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "PMS M(0-1) function enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `M0_PMS_FUNC_EN` field"]
    #[inline(always)]
    #[must_use]
    pub fn m_pms_func_en(&mut self, n: u8) -> M_PMS_FUNC_EN_W<FUNC_CTRL_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        M_PMS_FUNC_EN_W::new(self, n)
    }
    #[doc = "Bit 0 - PMS M0 function enable"]
    #[inline(always)]
    #[must_use]
    pub fn m0_pms_func_en(&mut self) -> M_PMS_FUNC_EN_W<FUNC_CTRL_SPEC> {
        M_PMS_FUNC_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - PMS M1 function enable"]
    #[inline(always)]
    #[must_use]
    pub fn m1_pms_func_en(&mut self) -> M_PMS_FUNC_EN_W<FUNC_CTRL_SPEC> {
        M_PMS_FUNC_EN_W::new(self, 1)
    }
}
#[doc = "PMS function control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FUNC_CTRL_SPEC;
impl crate::RegisterSpec for FUNC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func_ctrl::R`](R) reader structure"]
impl crate::Readable for FUNC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`func_ctrl::W`](W) writer structure"]
impl crate::Writable for FUNC_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FUNC_CTRL to value 0x03"]
impl crate::Resettable for FUNC_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
