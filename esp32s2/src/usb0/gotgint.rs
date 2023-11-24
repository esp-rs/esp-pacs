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
            .field("sesenddet", &format_args!("{}", self.sesenddet().bit()))
            .field(
                "sesreqsucstschng",
                &format_args!("{}", self.sesreqsucstschng().bit()),
            )
            .field(
                "hstnegsucstschng",
                &format_args!("{}", self.hstnegsucstschng().bit()),
            )
            .field("hstnegdet", &format_args!("{}", self.hstnegdet().bit()))
            .field("adevtoutchg", &format_args!("{}", self.adevtoutchg().bit()))
            .field("dbncedone", &format_args!("{}", self.dbncedone().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GOTGINT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gotgint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gotgint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GOTGINT_SPEC;
impl crate::RegisterSpec for GOTGINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gotgint::R`](R) reader structure"]
impl crate::Readable for GOTGINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gotgint::W`](W) writer structure"]
impl crate::Writable for GOTGINT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GOTGINT to value 0"]
impl crate::Resettable for GOTGINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
