#[doc = "Register `ECO_CELL_CTRL_48` reader"]
pub type R = crate::R<ECO_CELL_CTRL_48_SPEC>;
#[doc = "Register `ECO_CELL_CTRL_48` writer"]
pub type W = crate::W<ECO_CELL_CTRL_48_SPEC>;
#[doc = "Field `RDN_RESULT_48` reader - Reserved."]
pub type RDN_RESULT_48_R = crate::BitReader;
#[doc = "Field `RDN_ENA_48` reader - Reserved."]
pub type RDN_ENA_48_R = crate::BitReader;
#[doc = "Field `RDN_ENA_48` writer - Reserved."]
pub type RDN_ENA_48_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved."]
    #[inline(always)]
    pub fn rdn_result_48(&self) -> RDN_RESULT_48_R {
        RDN_RESULT_48_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved."]
    #[inline(always)]
    pub fn rdn_ena_48(&self) -> RDN_ENA_48_R {
        RDN_ENA_48_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECO_CELL_CTRL_48")
            .field("rdn_result_48", &self.rdn_result_48())
            .field("rdn_ena_48", &self.rdn_ena_48())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rdn_ena_48(&mut self) -> RDN_ENA_48_W<ECO_CELL_CTRL_48_SPEC> {
        RDN_ENA_48_W::new(self, 1)
    }
}
#[doc = "Reserved.\n\nYou can [`read`](crate::Reg::read) this register and get [`eco_cell_ctrl_48::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eco_cell_ctrl_48::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECO_CELL_CTRL_48_SPEC;
impl crate::RegisterSpec for ECO_CELL_CTRL_48_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eco_cell_ctrl_48::R`](R) reader structure"]
impl crate::Readable for ECO_CELL_CTRL_48_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eco_cell_ctrl_48::W`](W) writer structure"]
impl crate::Writable for ECO_CELL_CTRL_48_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECO_CELL_CTRL_48 to value 0"]
impl crate::Resettable for ECO_CELL_CTRL_48_SPEC {
    const RESET_VALUE: u32 = 0;
}
