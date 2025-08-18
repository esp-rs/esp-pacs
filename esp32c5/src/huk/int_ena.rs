#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `PREP_DONE_INT_ENA` reader - The interrupt enable bit for the huk_prep_done_int interrupt"]
pub type PREP_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `PREP_DONE_INT_ENA` writer - The interrupt enable bit for the huk_prep_done_int interrupt"]
pub type PREP_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROC_DONE_INT_ENA` reader - The interrupt enable bit for the huk_proc_done_int interrupt"]
pub type PROC_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `PROC_DONE_INT_ENA` writer - The interrupt enable bit for the huk_proc_done_int interrupt"]
pub type PROC_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POST_DONE_INT_ENA` reader - The interrupt enable bit for the huk_post_done_int interrupt"]
pub type POST_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `POST_DONE_INT_ENA` writer - The interrupt enable bit for the huk_post_done_int interrupt"]
pub type POST_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the huk_prep_done_int interrupt"]
    #[inline(always)]
    pub fn prep_done_int_ena(&self) -> PREP_DONE_INT_ENA_R {
        PREP_DONE_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the huk_proc_done_int interrupt"]
    #[inline(always)]
    pub fn proc_done_int_ena(&self) -> PROC_DONE_INT_ENA_R {
        PROC_DONE_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the huk_post_done_int interrupt"]
    #[inline(always)]
    pub fn post_done_int_ena(&self) -> POST_DONE_INT_ENA_R {
        POST_DONE_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("prep_done_int_ena", &self.prep_done_int_ena())
            .field("proc_done_int_ena", &self.proc_done_int_ena())
            .field("post_done_int_ena", &self.post_done_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the huk_prep_done_int interrupt"]
    #[inline(always)]
    pub fn prep_done_int_ena(&mut self) -> PREP_DONE_INT_ENA_W<'_, INT_ENA_SPEC> {
        PREP_DONE_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the huk_proc_done_int interrupt"]
    #[inline(always)]
    pub fn proc_done_int_ena(&mut self) -> PROC_DONE_INT_ENA_W<'_, INT_ENA_SPEC> {
        PROC_DONE_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the huk_post_done_int interrupt"]
    #[inline(always)]
    pub fn post_done_int_ena(&mut self) -> POST_DONE_INT_ENA_W<'_, INT_ENA_SPEC> {
        POST_DONE_INT_ENA_W::new(self, 2)
    }
}
#[doc = "HUK Generator interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {}
