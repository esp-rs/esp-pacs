#[doc = "Register `ECO_CELL_CTRL` reader"]
pub type R = crate::R<ECO_CELL_CTRL_SPEC>;
#[doc = "Register `ECO_CELL_CTRL` writer"]
pub type W = crate::W<ECO_CELL_CTRL_SPEC>;
#[doc = "Field `RDN_RESULT` reader - Reserved."]
pub type RDN_RESULT_R = crate::BitReader;
#[doc = "Field `RDN_ENA` reader - Reserved."]
pub type RDN_ENA_R = crate::BitReader;
#[doc = "Field `RDN_ENA` writer - Reserved."]
pub type RDN_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved."]
    #[inline(always)]
    pub fn rdn_result(&self) -> RDN_RESULT_R {
        RDN_RESULT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved."]
    #[inline(always)]
    pub fn rdn_ena(&self) -> RDN_ENA_R {
        RDN_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECO_CELL_CTRL")
            .field("rdn_result", &self.rdn_result())
            .field("rdn_ena", &self.rdn_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Reserved."]
    #[inline(always)]
    pub fn rdn_ena(&mut self) -> RDN_ENA_W<ECO_CELL_CTRL_SPEC> {
        RDN_ENA_W::new(self, 1)
    }
}
#[doc = "Reserved.\n\nYou can [`read`](crate::Reg::read) this register and get [`eco_cell_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eco_cell_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECO_CELL_CTRL_SPEC;
impl crate::RegisterSpec for ECO_CELL_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eco_cell_ctrl::R`](R) reader structure"]
impl crate::Readable for ECO_CELL_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eco_cell_ctrl::W`](W) writer structure"]
impl crate::Writable for ECO_CELL_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECO_CELL_CTRL to value 0"]
impl crate::Resettable for ECO_CELL_CTRL_SPEC {}
