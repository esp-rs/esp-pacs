#[doc = "Register `DLY_NUM_DUMP_CONF` reader"]
pub type R = crate::R<DLY_NUM_DUMP_CONF_SPEC>;
#[doc = "Register `DLY_NUM_DUMP_CONF` writer"]
pub type W = crate::W<DLY_NUM_DUMP_CONF_SPEC>;
#[doc = "Field `DUMP_EN` reader - needs field desc"]
pub type DUMP_EN_R = crate::BitReader;
#[doc = "Field `DUMP_EN` writer - needs field desc"]
pub type DUMP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUMP_CELL_SEL` reader - needs field desc"]
pub type DUMP_CELL_SEL_R = crate::FieldReader;
#[doc = "Field `DUMP_CELL_SEL` writer - needs field desc"]
pub type DUMP_CELL_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DUMP_EOF_NUM` reader - needs field desc"]
pub type DUMP_EOF_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `DUMP_EOF_NUM` writer - needs field desc"]
pub type DUMP_EOF_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - needs field desc"]
    #[inline(always)]
    pub fn dump_en(&self) -> DUMP_EN_R {
        DUMP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - needs field desc"]
    #[inline(always)]
    pub fn dump_cell_sel(&self) -> DUMP_CELL_SEL_R {
        DUMP_CELL_SEL_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:23 - needs field desc"]
    #[inline(always)]
    pub fn dump_eof_num(&self) -> DUMP_EOF_NUM_R {
        DUMP_EOF_NUM_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLY_NUM_DUMP_CONF")
            .field("dump_en", &self.dump_en())
            .field("dump_cell_sel", &self.dump_cell_sel())
            .field("dump_eof_num", &self.dump_eof_num())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - needs field desc"]
    #[inline(always)]
    pub fn dump_en(&mut self) -> DUMP_EN_W<'_, DLY_NUM_DUMP_CONF_SPEC> {
        DUMP_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:7 - needs field desc"]
    #[inline(always)]
    pub fn dump_cell_sel(&mut self) -> DUMP_CELL_SEL_W<'_, DLY_NUM_DUMP_CONF_SPEC> {
        DUMP_CELL_SEL_W::new(self, 1)
    }
    #[doc = "Bits 8:23 - needs field desc"]
    #[inline(always)]
    pub fn dump_eof_num(&mut self) -> DUMP_EOF_NUM_W<'_, DLY_NUM_DUMP_CONF_SPEC> {
        DUMP_EOF_NUM_W::new(self, 8)
    }
}
#[doc = "needs field desc\n\nYou can [`read`](crate::Reg::read) this register and get [`dly_num_dump_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dly_num_dump_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLY_NUM_DUMP_CONF_SPEC;
impl crate::RegisterSpec for DLY_NUM_DUMP_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dly_num_dump_conf::R`](R) reader structure"]
impl crate::Readable for DLY_NUM_DUMP_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dly_num_dump_conf::W`](W) writer structure"]
impl crate::Writable for DLY_NUM_DUMP_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DLY_NUM_DUMP_CONF to value 0x00ff_ff00"]
impl crate::Resettable for DLY_NUM_DUMP_CONF_SPEC {
    const RESET_VALUE: u32 = 0x00ff_ff00;
}
