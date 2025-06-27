#[doc = "Register `PRO_EXTMEM_REG_DATE` reader"]
pub type R = crate::R<PRO_EXTMEM_REG_DATE_SPEC>;
#[doc = "Register `PRO_EXTMEM_REG_DATE` writer"]
pub type W = crate::W<PRO_EXTMEM_REG_DATE_SPEC>;
#[doc = "Field `PRO_EXTMEM_REG_DATE` reader - Reserved."]
pub type PRO_EXTMEM_REG_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `PRO_EXTMEM_REG_DATE` writer - Reserved."]
pub type PRO_EXTMEM_REG_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Reserved."]
    #[inline(always)]
    pub fn pro_extmem_reg_date(&self) -> PRO_EXTMEM_REG_DATE_R {
        PRO_EXTMEM_REG_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_EXTMEM_REG_DATE")
            .field("pro_extmem_reg_date", &self.pro_extmem_reg_date())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:27 - Reserved."]
    #[inline(always)]
    pub fn pro_extmem_reg_date(&mut self) -> PRO_EXTMEM_REG_DATE_W<PRO_EXTMEM_REG_DATE_SPEC> {
        PRO_EXTMEM_REG_DATE_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_extmem_reg_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pro_extmem_reg_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_EXTMEM_REG_DATE_SPEC;
impl crate::RegisterSpec for PRO_EXTMEM_REG_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_extmem_reg_date::R`](R) reader structure"]
impl crate::Readable for PRO_EXTMEM_REG_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_extmem_reg_date::W`](W) writer structure"]
impl crate::Writable for PRO_EXTMEM_REG_DATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRO_EXTMEM_REG_DATE to value 0x0190_4180"]
impl crate::Resettable for PRO_EXTMEM_REG_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0190_4180;
}
