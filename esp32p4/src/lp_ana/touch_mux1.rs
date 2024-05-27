#[doc = "Register `TOUCH_MUX1` reader"]
pub type R = crate::R<TOUCH_MUX1_SPEC>;
#[doc = "Register `TOUCH_MUX1` writer"]
pub type W = crate::W<TOUCH_MUX1_SPEC>;
#[doc = "Field `TOUCH_START` reader - need_des"]
pub type TOUCH_START_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_START` writer - need_des"]
pub type TOUCH_START_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `TOUCH_XPD` reader - need_des"]
pub type TOUCH_XPD_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_XPD` writer - need_des"]
pub type TOUCH_XPD_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - need_des"]
    #[inline(always)]
    pub fn touch_start(&self) -> TOUCH_START_R {
        TOUCH_START_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 15:29 - need_des"]
    #[inline(always)]
    pub fn touch_xpd(&self) -> TOUCH_XPD_R {
        TOUCH_XPD_R::new(((self.bits >> 15) & 0x7fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_MUX1")
            .field("touch_start", &self.touch_start())
            .field("touch_xpd", &self.touch_xpd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:14 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_start(&mut self) -> TOUCH_START_W<TOUCH_MUX1_SPEC> {
        TOUCH_START_W::new(self, 0)
    }
    #[doc = "Bits 15:29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_xpd(&mut self) -> TOUCH_XPD_W<TOUCH_MUX1_SPEC> {
        TOUCH_XPD_W::new(self, 15)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_mux1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_mux1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_MUX1_SPEC;
impl crate::RegisterSpec for TOUCH_MUX1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_mux1::R`](R) reader structure"]
impl crate::Readable for TOUCH_MUX1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_mux1::W`](W) writer structure"]
impl crate::Writable for TOUCH_MUX1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOUCH_MUX1 to value 0"]
impl crate::Resettable for TOUCH_MUX1_SPEC {
    const RESET_VALUE: u32 = 0;
}
