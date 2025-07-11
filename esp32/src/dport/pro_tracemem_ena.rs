#[doc = "Register `PRO_TRACEMEM_ENA` reader"]
pub type R = crate::R<PRO_TRACEMEM_ENA_SPEC>;
#[doc = "Register `PRO_TRACEMEM_ENA` writer"]
pub type W = crate::W<PRO_TRACEMEM_ENA_SPEC>;
#[doc = "Field `PRO_TRACEMEM_ENA` reader - "]
pub type PRO_TRACEMEM_ENA_R = crate::BitReader;
#[doc = "Field `PRO_TRACEMEM_ENA` writer - "]
pub type PRO_TRACEMEM_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_tracemem_ena(&self) -> PRO_TRACEMEM_ENA_R {
        PRO_TRACEMEM_ENA_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_TRACEMEM_ENA")
            .field("pro_tracemem_ena", &self.pro_tracemem_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_tracemem_ena(&mut self) -> PRO_TRACEMEM_ENA_W<PRO_TRACEMEM_ENA_SPEC> {
        PRO_TRACEMEM_ENA_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_tracemem_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_tracemem_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_TRACEMEM_ENA_SPEC;
impl crate::RegisterSpec for PRO_TRACEMEM_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_tracemem_ena::R`](R) reader structure"]
impl crate::Readable for PRO_TRACEMEM_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_tracemem_ena::W`](W) writer structure"]
impl crate::Writable for PRO_TRACEMEM_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRO_TRACEMEM_ENA to value 0"]
impl crate::Resettable for PRO_TRACEMEM_ENA_SPEC {}
