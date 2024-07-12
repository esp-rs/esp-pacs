#[doc = "Register `GOTGINT` reader"]
pub type R = crate::R<GOTGINT_SPEC>;
#[doc = "Register `GOTGINT` writer"]
pub type W = crate::W<GOTGINT_SPEC>;
#[doc = "Field `SESENDDET` reader - "]
pub type SESENDDET_R = crate::BitReader;
#[doc = "Field `SESENDDET` writer - "]
pub type SESENDDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SESREQSUCSTSCHNG` reader - "]
pub type SESREQSUCSTSCHNG_R = crate::BitReader;
#[doc = "Field `SESREQSUCSTSCHNG` writer - "]
pub type SESREQSUCSTSCHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTNEGSUCSTSCHNG` reader - "]
pub type HSTNEGSUCSTSCHNG_R = crate::BitReader;
#[doc = "Field `HSTNEGSUCSTSCHNG` writer - "]
pub type HSTNEGSUCSTSCHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTNEGDET` reader - "]
pub type HSTNEGDET_R = crate::BitReader;
#[doc = "Field `HSTNEGDET` writer - "]
pub type HSTNEGDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADEVTOUTCHG` reader - "]
pub type ADEVTOUTCHG_R = crate::BitReader;
#[doc = "Field `ADEVTOUTCHG` writer - "]
pub type ADEVTOUTCHG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBNCEDONE` reader - "]
pub type DBNCEDONE_R = crate::BitReader;
#[doc = "Field `DBNCEDONE` writer - "]
pub type DBNCEDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sesenddet(&self) -> SESENDDET_R {
        SESENDDET_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sesreqsucstschng(&self) -> SESREQSUCSTSCHNG_R {
        SESREQSUCSTSCHNG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn hstnegsucstschng(&self) -> HSTNEGSUCSTSCHNG_R {
        HSTNEGSUCSTSCHNG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn hstnegdet(&self) -> HSTNEGDET_R {
        HSTNEGDET_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn adevtoutchg(&self) -> ADEVTOUTCHG_R {
        ADEVTOUTCHG_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn dbncedone(&self) -> DBNCEDONE_R {
        DBNCEDONE_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GOTGINT")
            .field("sesenddet", &self.sesenddet())
            .field("sesreqsucstschng", &self.sesreqsucstschng())
            .field("hstnegsucstschng", &self.hstnegsucstschng())
            .field("hstnegdet", &self.hstnegdet())
            .field("adevtoutchg", &self.adevtoutchg())
            .field("dbncedone", &self.dbncedone())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn sesenddet(&mut self) -> SESENDDET_W<GOTGINT_SPEC> {
        SESENDDET_W::new(self, 2)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn sesreqsucstschng(&mut self) -> SESREQSUCSTSCHNG_W<GOTGINT_SPEC> {
        SESREQSUCSTSCHNG_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn hstnegsucstschng(&mut self) -> HSTNEGSUCSTSCHNG_W<GOTGINT_SPEC> {
        HSTNEGSUCSTSCHNG_W::new(self, 9)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn hstnegdet(&mut self) -> HSTNEGDET_W<GOTGINT_SPEC> {
        HSTNEGDET_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn adevtoutchg(&mut self) -> ADEVTOUTCHG_W<GOTGINT_SPEC> {
        ADEVTOUTCHG_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn dbncedone(&mut self) -> DBNCEDONE_W<GOTGINT_SPEC> {
        DBNCEDONE_W::new(self, 19)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`gotgint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GOTGINT_SPEC;
impl crate::RegisterSpec for GOTGINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gotgint::R`](R) reader structure"]
impl crate::Readable for GOTGINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gotgint::W`](W) writer structure"]
impl crate::Writable for GOTGINT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GOTGINT to value 0"]
impl crate::Resettable for GOTGINT_SPEC {
    const RESET_VALUE: u32 = 0;
}
