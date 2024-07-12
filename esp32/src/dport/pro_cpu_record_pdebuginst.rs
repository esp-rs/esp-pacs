#[doc = "Register `PRO_CPU_RECORD_PDEBUGINST` reader"]
pub type R = crate::R<PRO_CPU_RECORD_PDEBUGINST_SPEC>;
#[doc = "Register `PRO_CPU_RECORD_PDEBUGINST` writer"]
pub type W = crate::W<PRO_CPU_RECORD_PDEBUGINST_SPEC>;
#[doc = "Field `RECORD_PRO_PDEBUGINST` reader - "]
pub type RECORD_PRO_PDEBUGINST_R = crate::FieldReader<u32>;
#[doc = "Field `RECORD_PDEBUGINST_SZ` reader - "]
pub type RECORD_PDEBUGINST_SZ_R = crate::FieldReader;
#[doc = "Field `RECORD_PDEBUGINST_SZ` writer - "]
pub type RECORD_PDEBUGINST_SZ_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RECORD_PDEBUGINST_ISRC` reader - "]
pub type RECORD_PDEBUGINST_ISRC_R = crate::FieldReader;
#[doc = "Field `RECORD_PDEBUGINST_ISRC` writer - "]
pub type RECORD_PDEBUGINST_ISRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RECORD_PDEBUGINST_LOOP_REP` reader - "]
pub type RECORD_PDEBUGINST_LOOP_REP_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGINST_LOOP_REP` writer - "]
pub type RECORD_PDEBUGINST_LOOP_REP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECORD_PDEBUGINST_LOOP` reader - "]
pub type RECORD_PDEBUGINST_LOOP_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGINST_LOOP` writer - "]
pub type RECORD_PDEBUGINST_LOOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RECORD_PDEBUGINST_CINTL` reader - "]
pub type RECORD_PDEBUGINST_CINTL_R = crate::FieldReader;
#[doc = "Field `RECORD_PDEBUGINST_CINTL` writer - "]
pub type RECORD_PDEBUGINST_CINTL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn record_pro_pdebuginst(&self) -> RECORD_PRO_PDEBUGINST_R {
        RECORD_PRO_PDEBUGINST_R::new(self.bits)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn record_pdebuginst_sz(&self) -> RECORD_PDEBUGINST_SZ_R {
        RECORD_PDEBUGINST_SZ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn record_pdebuginst_isrc(&self) -> RECORD_PDEBUGINST_ISRC_R {
        RECORD_PDEBUGINST_ISRC_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn record_pdebuginst_loop_rep(&self) -> RECORD_PDEBUGINST_LOOP_REP_R {
        RECORD_PDEBUGINST_LOOP_REP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn record_pdebuginst_loop(&self) -> RECORD_PDEBUGINST_LOOP_R {
        RECORD_PDEBUGINST_LOOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn record_pdebuginst_cintl(&self) -> RECORD_PDEBUGINST_CINTL_R {
        RECORD_PDEBUGINST_CINTL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CPU_RECORD_PDEBUGINST")
            .field("record_pro_pdebuginst", &self.record_pro_pdebuginst())
            .field("record_pdebuginst_sz", &self.record_pdebuginst_sz())
            .field("record_pdebuginst_isrc", &self.record_pdebuginst_isrc())
            .field(
                "record_pdebuginst_loop_rep",
                &self.record_pdebuginst_loop_rep(),
            )
            .field("record_pdebuginst_loop", &self.record_pdebuginst_loop())
            .field("record_pdebuginst_cintl", &self.record_pdebuginst_cintl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebuginst_sz(
        &mut self,
    ) -> RECORD_PDEBUGINST_SZ_W<PRO_CPU_RECORD_PDEBUGINST_SPEC> {
        RECORD_PDEBUGINST_SZ_W::new(self, 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebuginst_isrc(
        &mut self,
    ) -> RECORD_PDEBUGINST_ISRC_W<PRO_CPU_RECORD_PDEBUGINST_SPEC> {
        RECORD_PDEBUGINST_ISRC_W::new(self, 12)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebuginst_loop_rep(
        &mut self,
    ) -> RECORD_PDEBUGINST_LOOP_REP_W<PRO_CPU_RECORD_PDEBUGINST_SPEC> {
        RECORD_PDEBUGINST_LOOP_REP_W::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebuginst_loop(
        &mut self,
    ) -> RECORD_PDEBUGINST_LOOP_W<PRO_CPU_RECORD_PDEBUGINST_SPEC> {
        RECORD_PDEBUGINST_LOOP_W::new(self, 21)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebuginst_cintl(
        &mut self,
    ) -> RECORD_PDEBUGINST_CINTL_W<PRO_CPU_RECORD_PDEBUGINST_SPEC> {
        RECORD_PDEBUGINST_CINTL_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_cpu_record_pdebuginst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_cpu_record_pdebuginst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_CPU_RECORD_PDEBUGINST_SPEC;
impl crate::RegisterSpec for PRO_CPU_RECORD_PDEBUGINST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_cpu_record_pdebuginst::R`](R) reader structure"]
impl crate::Readable for PRO_CPU_RECORD_PDEBUGINST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_cpu_record_pdebuginst::W`](W) writer structure"]
impl crate::Writable for PRO_CPU_RECORD_PDEBUGINST_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_CPU_RECORD_PDEBUGINST to value 0"]
impl crate::Resettable for PRO_CPU_RECORD_PDEBUGINST_SPEC {
    const RESET_VALUE: u32 = 0;
}
