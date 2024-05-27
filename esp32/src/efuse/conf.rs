#[doc = "Register `CONF` reader"]
pub type R = crate::R<CONF_SPEC>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<CONF_SPEC>;
#[doc = "Field `OP_CODE` reader - "]
pub type OP_CODE_R = crate::FieldReader<u16>;
#[doc = "Field `OP_CODE` writer - "]
pub type OP_CODE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FORCE_NO_WR_RD_DIS` reader - "]
pub type FORCE_NO_WR_RD_DIS_R = crate::BitReader;
#[doc = "Field `FORCE_NO_WR_RD_DIS` writer - "]
pub type FORCE_NO_WR_RD_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn op_code(&self) -> OP_CODE_R {
        OP_CODE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn force_no_wr_rd_dis(&self) -> FORCE_NO_WR_RD_DIS_R {
        FORCE_NO_WR_RD_DIS_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field("op_code", &self.op_code())
            .field("force_no_wr_rd_dis", &self.force_no_wr_rd_dis())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn op_code(&mut self) -> OP_CODE_W<CONF_SPEC> {
        OP_CODE_W::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn force_no_wr_rd_dis(&mut self) -> FORCE_NO_WR_RD_DIS_W<CONF_SPEC> {
        FORCE_NO_WR_RD_DIS_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf::R`](R) reader structure"]
impl crate::Readable for CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf::W`](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF to value 0x0001_0000"]
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: u32 = 0x0001_0000;
}
