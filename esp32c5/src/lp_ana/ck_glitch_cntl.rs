#[doc = "Register `CK_GLITCH_CNTL` reader"]
pub type R = crate::R<CK_GLITCH_CNTL_SPEC>;
#[doc = "Register `CK_GLITCH_CNTL` writer"]
pub type W = crate::W<CK_GLITCH_CNTL_SPEC>;
#[doc = "Field `PWR_GLITCH_RESET_ENA` reader - enable powerglitch or not"]
pub type PWR_GLITCH_RESET_ENA_R = crate::FieldReader;
#[doc = "Field `PWR_GLITCH_RESET_ENA` writer - enable powerglitch or not"]
pub type PWR_GLITCH_RESET_ENA_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CK_GLITCH_RESET_ENA` reader - reserved"]
pub type CK_GLITCH_RESET_ENA_R = crate::BitReader;
#[doc = "Field `CK_GLITCH_RESET_ENA` writer - reserved"]
pub type CK_GLITCH_RESET_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 27:30 - enable powerglitch or not"]
    #[inline(always)]
    pub fn pwr_glitch_reset_ena(&self) -> PWR_GLITCH_RESET_ENA_R {
        PWR_GLITCH_RESET_ENA_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - reserved"]
    #[inline(always)]
    pub fn ck_glitch_reset_ena(&self) -> CK_GLITCH_RESET_ENA_R {
        CK_GLITCH_RESET_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CK_GLITCH_CNTL")
            .field("pwr_glitch_reset_ena", &self.pwr_glitch_reset_ena())
            .field("ck_glitch_reset_ena", &self.ck_glitch_reset_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bits 27:30 - enable powerglitch or not"]
    #[inline(always)]
    pub fn pwr_glitch_reset_ena(&mut self) -> PWR_GLITCH_RESET_ENA_W<CK_GLITCH_CNTL_SPEC> {
        PWR_GLITCH_RESET_ENA_W::new(self, 27)
    }
    #[doc = "Bit 31 - reserved"]
    #[inline(always)]
    pub fn ck_glitch_reset_ena(&mut self) -> CK_GLITCH_RESET_ENA_W<CK_GLITCH_CNTL_SPEC> {
        CK_GLITCH_RESET_ENA_W::new(self, 31)
    }
}
#[doc = "Configure power glitch\n\nYou can [`read`](crate::Reg::read) this register and get [`ck_glitch_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ck_glitch_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CK_GLITCH_CNTL_SPEC;
impl crate::RegisterSpec for CK_GLITCH_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ck_glitch_cntl::R`](R) reader structure"]
impl crate::Readable for CK_GLITCH_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ck_glitch_cntl::W`](W) writer structure"]
impl crate::Writable for CK_GLITCH_CNTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CK_GLITCH_CNTL to value 0"]
impl crate::Resettable for CK_GLITCH_CNTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
