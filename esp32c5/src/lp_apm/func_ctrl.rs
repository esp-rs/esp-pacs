#[doc = "Register `FUNC_CTRL` reader"]
pub type R = crate::R<FUNC_CTRL_SPEC>;
#[doc = "Register `FUNC_CTRL` writer"]
pub type W = crate::W<FUNC_CTRL_SPEC>;
#[doc = "Field `M0_FUNC_EN` reader - PMS M0 function enable"]
pub type M0_FUNC_EN_R = crate::BitReader;
#[doc = "Field `M0_FUNC_EN` writer - PMS M0 function enable"]
pub type M0_FUNC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M1_FUNC_EN` reader - PMS M1 function enable"]
pub type M1_FUNC_EN_R = crate::BitReader;
#[doc = "Field `M1_FUNC_EN` writer - PMS M1 function enable"]
pub type M1_FUNC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PMS M0 function enable"]
    #[inline(always)]
    pub fn m0_func_en(&self) -> M0_FUNC_EN_R {
        M0_FUNC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PMS M1 function enable"]
    #[inline(always)]
    pub fn m1_func_en(&self) -> M1_FUNC_EN_R {
        M1_FUNC_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC_CTRL")
            .field("m0_func_en", &self.m0_func_en())
            .field("m1_func_en", &self.m1_func_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - PMS M0 function enable"]
    #[inline(always)]
    pub fn m0_func_en(&mut self) -> M0_FUNC_EN_W<FUNC_CTRL_SPEC> {
        M0_FUNC_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - PMS M1 function enable"]
    #[inline(always)]
    pub fn m1_func_en(&mut self) -> M1_FUNC_EN_W<FUNC_CTRL_SPEC> {
        M1_FUNC_EN_W::new(self, 1)
    }
}
#[doc = "APM function control register\n\nYou can [`read`](crate::Reg::read) this register and get [`func_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
