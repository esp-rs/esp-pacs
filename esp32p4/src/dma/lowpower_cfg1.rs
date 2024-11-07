#[doc = "Register `LOWPOWER_CFG1` reader"]
pub type R = crate::R<LOWPOWER_CFG1_SPEC>;
#[doc = "Register `LOWPOWER_CFG1` writer"]
pub type W = crate::W<LOWPOWER_CFG1_SPEC>;
#[doc = "Field `GLCH_LPDLY` reader - NA"]
pub type GLCH_LPDLY_R = crate::FieldReader;
#[doc = "Field `GLCH_LPDLY` writer - NA"]
pub type GLCH_LPDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SBIU_LPDLY` reader - NA"]
pub type SBIU_LPDLY_R = crate::FieldReader;
#[doc = "Field `SBIU_LPDLY` writer - NA"]
pub type SBIU_LPDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MXIF_LPDLY` reader - NA"]
pub type MXIF_LPDLY_R = crate::FieldReader;
#[doc = "Field `MXIF_LPDLY` writer - NA"]
pub type MXIF_LPDLY_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn glch_lpdly(&self) -> GLCH_LPDLY_R {
        GLCH_LPDLY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - NA"]
    #[inline(always)]
    pub fn sbiu_lpdly(&self) -> SBIU_LPDLY_R {
        SBIU_LPDLY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    pub fn mxif_lpdly(&self) -> MXIF_LPDLY_R {
        MXIF_LPDLY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOWPOWER_CFG1")
            .field("glch_lpdly", &self.glch_lpdly())
            .field("sbiu_lpdly", &self.sbiu_lpdly())
            .field("mxif_lpdly", &self.mxif_lpdly())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn glch_lpdly(&mut self) -> GLCH_LPDLY_W<LOWPOWER_CFG1_SPEC> {
        GLCH_LPDLY_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - NA"]
    #[inline(always)]
    pub fn sbiu_lpdly(&mut self) -> SBIU_LPDLY_W<LOWPOWER_CFG1_SPEC> {
        SBIU_LPDLY_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - NA"]
    #[inline(always)]
    pub fn mxif_lpdly(&mut self) -> MXIF_LPDLY_W<LOWPOWER_CFG1_SPEC> {
        MXIF_LPDLY_W::new(self, 16)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`lowpower_cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lowpower_cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOWPOWER_CFG1_SPEC;
impl crate::RegisterSpec for LOWPOWER_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lowpower_cfg1::R`](R) reader structure"]
impl crate::Readable for LOWPOWER_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lowpower_cfg1::W`](W) writer structure"]
impl crate::Writable for LOWPOWER_CFG1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOWPOWER_CFG1 to value 0x0040_4040"]
impl crate::Resettable for LOWPOWER_CFG1_SPEC {
    const RESET_VALUE: u32 = 0x0040_4040;
}
