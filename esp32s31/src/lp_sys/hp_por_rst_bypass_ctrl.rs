#[doc = "Register `HP_POR_RST_BYPASS_CTRL` reader"]
pub type R = crate::R<HP_POR_RST_BYPASS_CTRL_SPEC>;
#[doc = "Register `HP_POR_RST_BYPASS_CTRL` writer"]
pub type W = crate::W<HP_POR_RST_BYPASS_CTRL_SPEC>;
#[doc = "Field `HP_PO_CPU_RSTN_BYPASS_CTRL` reader - "]
pub type HP_PO_CPU_RSTN_BYPASS_CTRL_R = crate::FieldReader;
#[doc = "Field `HP_PO_CPU_RSTN_BYPASS_CTRL` writer - "]
pub type HP_PO_CPU_RSTN_BYPASS_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HP_PO_CNNT_RSTN_BYPASS_CTRL` reader - "]
pub type HP_PO_CNNT_RSTN_BYPASS_CTRL_R = crate::FieldReader;
#[doc = "Field `HP_PO_CNNT_RSTN_BYPASS_CTRL` writer - "]
pub type HP_PO_CNNT_RSTN_BYPASS_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HP_PO_ALIVE_RSTN_BYPASS_CTRL` reader - "]
pub type HP_PO_ALIVE_RSTN_BYPASS_CTRL_R = crate::FieldReader;
#[doc = "Field `HP_PO_ALIVE_RSTN_BYPASS_CTRL` writer - "]
pub type HP_PO_ALIVE_RSTN_BYPASS_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HP_PO_RSTN_BYPASS_CTRL` reader - "]
pub type HP_PO_RSTN_BYPASS_CTRL_R = crate::FieldReader;
#[doc = "Field `HP_PO_RSTN_BYPASS_CTRL` writer - "]
pub type HP_PO_RSTN_BYPASS_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn hp_po_cpu_rstn_bypass_ctrl(&self) -> HP_PO_CPU_RSTN_BYPASS_CTRL_R {
        HP_PO_CPU_RSTN_BYPASS_CTRL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn hp_po_cnnt_rstn_bypass_ctrl(&self) -> HP_PO_CNNT_RSTN_BYPASS_CTRL_R {
        HP_PO_CNNT_RSTN_BYPASS_CTRL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn hp_po_alive_rstn_bypass_ctrl(&self) -> HP_PO_ALIVE_RSTN_BYPASS_CTRL_R {
        HP_PO_ALIVE_RSTN_BYPASS_CTRL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn hp_po_rstn_bypass_ctrl(&self) -> HP_PO_RSTN_BYPASS_CTRL_R {
        HP_PO_RSTN_BYPASS_CTRL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_POR_RST_BYPASS_CTRL")
            .field(
                "hp_po_cpu_rstn_bypass_ctrl",
                &self.hp_po_cpu_rstn_bypass_ctrl(),
            )
            .field(
                "hp_po_cnnt_rstn_bypass_ctrl",
                &self.hp_po_cnnt_rstn_bypass_ctrl(),
            )
            .field(
                "hp_po_alive_rstn_bypass_ctrl",
                &self.hp_po_alive_rstn_bypass_ctrl(),
            )
            .field("hp_po_rstn_bypass_ctrl", &self.hp_po_rstn_bypass_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn hp_po_cpu_rstn_bypass_ctrl(
        &mut self,
    ) -> HP_PO_CPU_RSTN_BYPASS_CTRL_W<'_, HP_POR_RST_BYPASS_CTRL_SPEC> {
        HP_PO_CPU_RSTN_BYPASS_CTRL_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn hp_po_cnnt_rstn_bypass_ctrl(
        &mut self,
    ) -> HP_PO_CNNT_RSTN_BYPASS_CTRL_W<'_, HP_POR_RST_BYPASS_CTRL_SPEC> {
        HP_PO_CNNT_RSTN_BYPASS_CTRL_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn hp_po_alive_rstn_bypass_ctrl(
        &mut self,
    ) -> HP_PO_ALIVE_RSTN_BYPASS_CTRL_W<'_, HP_POR_RST_BYPASS_CTRL_SPEC> {
        HP_PO_ALIVE_RSTN_BYPASS_CTRL_W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn hp_po_rstn_bypass_ctrl(
        &mut self,
    ) -> HP_PO_RSTN_BYPASS_CTRL_W<'_, HP_POR_RST_BYPASS_CTRL_SPEC> {
        HP_PO_RSTN_BYPASS_CTRL_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_por_rst_bypass_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_por_rst_bypass_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_POR_RST_BYPASS_CTRL_SPEC;
impl crate::RegisterSpec for HP_POR_RST_BYPASS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_por_rst_bypass_ctrl::R`](R) reader structure"]
impl crate::Readable for HP_POR_RST_BYPASS_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_por_rst_bypass_ctrl::W`](W) writer structure"]
impl crate::Writable for HP_POR_RST_BYPASS_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_POR_RST_BYPASS_CTRL to value 0xffff_ffff"]
impl crate::Resettable for HP_POR_RST_BYPASS_CTRL_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
