#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `UNDERRUN` reader - write 1 to enables dpi_underrun_int_st field of MIPI_DSI_BRG_INT_ST_REG controlled by dpi_underrun interrupt signal"]
pub type UNDERRUN_R = crate::BitReader;
#[doc = "Field `UNDERRUN` writer - write 1 to enables dpi_underrun_int_st field of MIPI_DSI_BRG_INT_ST_REG controlled by dpi_underrun interrupt signal"]
pub type UNDERRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - write 1 to enables dpi_underrun_int_st field of MIPI_DSI_BRG_INT_ST_REG controlled by dpi_underrun interrupt signal"]
    #[inline(always)]
    pub fn underrun(&self) -> UNDERRUN_R {
        UNDERRUN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("underrun", &self.underrun())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - write 1 to enables dpi_underrun_int_st field of MIPI_DSI_BRG_INT_ST_REG controlled by dpi_underrun interrupt signal"]
    #[inline(always)]
    pub fn underrun(&mut self) -> UNDERRUN_W<INT_ENA_SPEC> {
        UNDERRUN_W::new(self, 0)
    }
}
#[doc = "dsi_bridge interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
