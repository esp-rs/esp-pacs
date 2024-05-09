#[doc = "Register `COEX_PTI` reader"]
pub type R = crate::R<COEX_PTI_SPEC>;
#[doc = "Register `COEX_PTI` writer"]
pub type W = crate::W<COEX_PTI_SPEC>;
#[doc = "Field `COEX_PTI` reader - "]
pub type COEX_PTI_R = crate::FieldReader;
#[doc = "Field `COEX_PTI` writer - "]
pub type COEX_PTI_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `COEX_ACK_PTI` reader - "]
pub type COEX_ACK_PTI_R = crate::FieldReader;
#[doc = "Field `COEX_ACK_PTI` writer - "]
pub type COEX_ACK_PTI_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLOSE_RF_SEL` reader - "]
pub type CLOSE_RF_SEL_R = crate::BitReader;
#[doc = "Field `CLOSE_RF_SEL` writer - "]
pub type CLOSE_RF_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn coex_pti(&self) -> COEX_PTI_R {
        COEX_PTI_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn coex_ack_pti(&self) -> COEX_ACK_PTI_R {
        COEX_ACK_PTI_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn close_rf_sel(&self) -> CLOSE_RF_SEL_R {
        CLOSE_RF_SEL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COEX_PTI")
            .field("coex_pti", &format_args!("{}", self.coex_pti().bits()))
            .field(
                "coex_ack_pti",
                &format_args!("{}", self.coex_ack_pti().bits()),
            )
            .field(
                "close_rf_sel",
                &format_args!("{}", self.close_rf_sel().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COEX_PTI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn coex_pti(&mut self) -> COEX_PTI_W<COEX_PTI_SPEC> {
        COEX_PTI_W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn coex_ack_pti(&mut self) -> COEX_ACK_PTI_W<COEX_PTI_SPEC> {
        COEX_ACK_PTI_W::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn close_rf_sel(&mut self) -> CLOSE_RF_SEL_W<COEX_PTI_SPEC> {
        CLOSE_RF_SEL_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`coex_pti::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`coex_pti::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct COEX_PTI_SPEC;
impl crate::RegisterSpec for COEX_PTI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`coex_pti::R`](R) reader structure"]
impl crate::Readable for COEX_PTI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`coex_pti::W`](W) writer structure"]
impl crate::Writable for COEX_PTI_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COEX_PTI to value 0"]
impl crate::Resettable for COEX_PTI_SPEC {
    const RESET_VALUE: u32 = 0;
}
