#[doc = "Register `PBYTE` reader"]
pub type R = crate::R<PBYTE_SPEC>;
#[doc = "Register `PBYTE` writer"]
pub type W = crate::W<PBYTE_SPEC>;
#[doc = "Field `ORI_PBYTE` reader - configures bytes per pixel for ori img. 0: 0.5byte/pix, 1: 1byte/pix, 2: 1.5byte/pix, 3: 2byte/pix, 4: 3byte/pix"]
pub type ORI_PBYTE_R = crate::FieldReader;
#[doc = "Field `ORI_PBYTE` writer - configures bytes per pixel for ori img. 0: 0.5byte/pix, 1: 1byte/pix, 2: 1.5byte/pix, 3: 2byte/pix, 4: 3byte/pix"]
pub type ORI_PBYTE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - configures bytes per pixel for ori img. 0: 0.5byte/pix, 1: 1byte/pix, 2: 1.5byte/pix, 3: 2byte/pix, 4: 3byte/pix"]
    #[inline(always)]
    pub fn ori_pbyte(&self) -> ORI_PBYTE_R {
        ORI_PBYTE_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PBYTE")
            .field("ori_pbyte", &self.ori_pbyte())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - configures bytes per pixel for ori img. 0: 0.5byte/pix, 1: 1byte/pix, 2: 1.5byte/pix, 3: 2byte/pix, 4: 3byte/pix"]
    #[inline(always)]
    pub fn ori_pbyte(&mut self) -> ORI_PBYTE_W<'_, PBYTE_SPEC> {
        ORI_PBYTE_W::new(self, 0)
    }
}
#[doc = "image pbyte register\n\nYou can [`read`](crate::Reg::read) this register and get [`pbyte::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbyte::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PBYTE_SPEC;
impl crate::RegisterSpec for PBYTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbyte::R`](R) reader structure"]
impl crate::Readable for PBYTE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pbyte::W`](W) writer structure"]
impl crate::Writable for PBYTE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PBYTE to value 0x02"]
impl crate::Resettable for PBYTE_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
